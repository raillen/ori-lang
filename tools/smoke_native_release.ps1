[CmdletBinding()]
param(
    [string]$PackageRoot = "",
    [switch]$SkipBuild,
    [switch]$KeepPackage
)

$ErrorActionPreference = "Stop"

function Get-HostTriple {
    $rustcVersion = & rustc -vV
    if ($LASTEXITCODE -ne 0) {
        throw "rustc -vV failed; install Rust before running the native release smoke test."
    }

    foreach ($line in $rustcVersion) {
        if ($line -like "host:*") {
            return $line.Substring(5).Trim()
        }
    }

    throw "Could not detect the Rust host target from rustc -vV."
}

function Get-OriExeName {
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        return "ori.exe"
    }

    return "ori"
}

function Get-OutputExeName([string]$Name) {
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        return "$Name.exe"
    }

    return $Name
}

function Invoke-Checked([scriptblock]$Command, [string]$Description) {
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed with exit code $LASTEXITCODE."
    }
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$hostTriple = Get-HostTriple
$targetRoot = if ($env:CARGO_TARGET_DIR) {
    [System.IO.Path]::GetFullPath($env:CARGO_TARGET_DIR)
} else {
    Join-Path $repoRoot "target"
}

if ([string]::IsNullOrWhiteSpace($PackageRoot)) {
    $PackageRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("ori-native-release-smoke-" + [System.Guid]::NewGuid().ToString("N"))
} else {
    $PackageRoot = [System.IO.Path]::GetFullPath($PackageRoot)
}

$packageRootPath = $PackageRoot
$oriExeName = Get-OriExeName
$sourceOri = Join-Path $targetRoot (Join-Path "release" $oriExeName)
$packageOri = Join-Path $packageRootPath $oriExeName
$examplesDir = Join-Path $packageRootPath "examples"
$runtimeDir = Join-Path $packageRootPath "runtime"

Push-Location $repoRoot
try {
    if (-not $SkipBuild) {
        Invoke-Checked { cargo build -p ori-driver --release } "cargo build -p ori-driver --release"
    }

    if (-not (Test-Path -LiteralPath $sourceOri -PathType Leaf)) {
        throw "Release compiler was not found at $sourceOri."
    }

    if (Test-Path -LiteralPath $packageRootPath) {
        Remove-Item -LiteralPath $packageRootPath -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $packageRootPath | Out-Null
    New-Item -ItemType Directory -Force -Path $examplesDir | Out-Null

    Copy-Item -LiteralPath $sourceOri -Destination $packageOri -Force
    Copy-Item -LiteralPath (Join-Path $repoRoot "examples/hello_world.orl") -Destination (Join-Path $examplesDir "hello_world.orl") -Force
    Copy-Item -LiteralPath (Join-Path $repoRoot "examples/async_demo.orl") -Destination (Join-Path $examplesDir "async_demo.orl") -Force

    $stageArgs = @{
        Target = $hostTriple
        Profile = "release"
        OutputRoot = $runtimeDir
    }
    if ($SkipBuild) {
        $stageArgs.SkipBuild = $true
    }
    Invoke-Checked { & (Join-Path $PSScriptRoot "stage_native_runtime.ps1") @stageArgs } "stage_native_runtime.ps1"

    $testSource = @'
namespace app.package_smoke

import ori.test as test
import ori.task as task

@test
func package_smoke_test()
    check 1 + 1 == 2
    test.assert(true, "package smoke test")
end

@test
async func package_async_smoke_test()
    await task.sleep(1)
    test.assert(true, "package async smoke test")
end
'@
    $testPath = Join-Path $examplesDir "package_smoke_test.orl"
    Set-Content -LiteralPath $testPath -Value $testSource -Encoding ASCII

    $previousRequirePackagedRuntime = $env:ORI_REQUIRE_PACKAGED_RUNTIME
    $env:ORI_REQUIRE_PACKAGED_RUNTIME = "1"
    Push-Location $packageRootPath
    try {
        $helloExe = Join-Path $packageRootPath (Get-OutputExeName "hello_world")
        Invoke-Checked { & $packageOri compile (Join-Path "examples" "hello_world.orl") --out $helloExe } "ori compile in packaged release folder"
        $helloOutput = & $helloExe
        if ($LASTEXITCODE -ne 0) {
            throw "compiled hello_world executable failed with exit code $LASTEXITCODE."
        }
        if (($helloOutput -join "`n") -notmatch "The answer is: 42") {
            throw "compiled hello_world executable did not print the expected answer."
        }

        $asyncExe = Join-Path $packageRootPath (Get-OutputExeName "async_demo")
        Invoke-Checked { & $packageOri compile (Join-Path "examples" "async_demo.orl") --out $asyncExe } "ori compile async_demo in packaged release folder"
        $asyncOutput = & $asyncExe
        if ($LASTEXITCODE -ne 0) {
            throw "compiled async_demo executable failed with exit code $LASTEXITCODE."
        }
        if (($asyncOutput -join "`n") -notmatch "^42$") {
            throw "compiled async_demo executable did not print the expected async answer."
        }

        Invoke-Checked { & $packageOri test (Join-Path "examples" "package_smoke_test.orl") } "ori test in packaged release folder"

        $runtimeTripleDir = Join-Path $runtimeDir $hostTriple
        $cdylibName = if ($IsWindows -or $env:OS -eq "Windows_NT") {
            "ori_runtime.dll"
        } elseif ($hostTriple -like "*apple-darwin*") {
            "libori_runtime.dylib"
        } else {
            "libori_runtime.so"
        }
        $cdylibPath = Join-Path $runtimeTripleDir $cdylibName
        if (-not (Test-Path -LiteralPath $cdylibPath -PathType Leaf)) {
            throw "packaged runtime cdylib was not staged at $cdylibPath."
        }

        $jitOutput = & $packageOri run (Join-Path "examples" "hello_world.orl")
        if ($LASTEXITCODE -ne 0) {
            throw "ori run (JIT default) failed with exit code $LASTEXITCODE."
        }
        if (($jitOutput -join "`n") -notmatch "The answer is: 42") {
            throw "ori run (JIT default) did not print the expected answer."
        }
    } finally {
        Pop-Location
        if ($null -eq $previousRequirePackagedRuntime) {
            Remove-Item Env:\ORI_REQUIRE_PACKAGED_RUNTIME -ErrorAction SilentlyContinue
        } else {
            $env:ORI_REQUIRE_PACKAGED_RUNTIME = $previousRequirePackagedRuntime
        }
    }

    Write-Host "native release smoke passed: $packageRootPath"
} finally {
    Pop-Location
    if (-not $KeepPackage -and (Test-Path -LiteralPath $packageRootPath)) {
        Remove-Item -LiteralPath $packageRootPath -Recurse -Force
    }
}
