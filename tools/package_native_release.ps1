param(
    [string]$PackageRoot = "",
    [string]$ArchivePath = "",
    [switch]$SkipBuild,
    [switch]$Overwrite
)

$ErrorActionPreference = "Stop"
# PowerShell 7.4+ may treat native stderr as terminating errors under Stop.
$PSNativeCommandUseErrorActionPreference = $false

function Get-HostTriple {
    $text = (& rustc -Vv | Out-String)
    if ($LASTEXITCODE -ne 0) {
        throw "rustc -Vv failed; install Rust before packaging Ori. Output: $text"
    }
    foreach ($line in ($text -split "`r?`n")) {
        if ($line -match '^host:\s*(.+)$') {
            return $Matches[1].Trim()
        }
    }
    throw "Could not detect the Rust host target from rustc -Vv. Output: $text"
}

function Get-WorkspaceVersion([string]$RepoRoot) {
    $cargoToml = Join-Path $RepoRoot "compiler/Cargo.toml"
    if (-not (Test-Path -LiteralPath $cargoToml -PathType Leaf)) {
        $cargoToml = Join-Path $RepoRoot "Cargo.toml"
    }
    $inWorkspacePackage = $false
    foreach ($line in Get-Content -LiteralPath $cargoToml) {
        $text = [string]$line
        if ($text.Trim() -eq "[workspace.package]") {
            $inWorkspacePackage = $true
            continue
        }
        if ($inWorkspacePackage -and $text.Trim().StartsWith("[")) {
            break
        }
        if ($inWorkspacePackage -and $text -match '^\s*version\s*=\s*"([^"]+)"') {
            return $Matches[1]
        }
    }
    throw "Could not find [workspace.package] version in $cargoToml."
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$version = Get-WorkspaceVersion $repoRoot
$hostTriple = Get-HostTriple
$compilerTarget = if ($env:CARGO_TARGET_DIR) {
    [System.IO.Path]::GetFullPath($env:CARGO_TARGET_DIR)
} else {
    Join-Path $repoRoot "compiler/target"
}
$distRoot = Join-Path $compilerTarget "dist"

if ([string]::IsNullOrWhiteSpace($PackageRoot)) {
    $PackageRoot = Join-Path $distRoot "ori-$version-$hostTriple"
} else {
    $PackageRoot = [System.IO.Path]::GetFullPath($PackageRoot)
}

if ([string]::IsNullOrWhiteSpace($ArchivePath)) {
    $ArchivePath = Join-Path $distRoot "ori-$version-$hostTriple.zip"
} else {
    $ArchivePath = [System.IO.Path]::GetFullPath($ArchivePath)
}

New-Item -ItemType Directory -Force -Path (Split-Path -Parent $ArchivePath) | Out-Null

$smokeArgs = @{
    PackageRoot = $PackageRoot
    KeepPackage = $true
}
if ($SkipBuild) {
    $smokeArgs.SkipBuild = $true
}

& (Join-Path $PSScriptRoot "smoke_native_release.ps1") @smokeArgs
if ($LASTEXITCODE -ne 0) {
    throw "smoke_native_release.ps1 failed with exit code $LASTEXITCODE."
}

if ((Test-Path -LiteralPath $ArchivePath) -and -not $Force) {
    throw "Archive already exists at $ArchivePath. Pass -Force to replace it."
}
if (Test-Path -LiteralPath $ArchivePath) {
    Remove-Item -LiteralPath $ArchivePath -Force
}

Compress-Archive -LiteralPath $PackageRoot -DestinationPath $ArchivePath -Force

Write-Host "native release package: $PackageRoot"
Write-Host "native release archive: $ArchivePath"
