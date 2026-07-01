[CmdletBinding()]
param(
    [string]$PackageRoot = "",
    [string]$ArchivePath = "",
    [switch]$SkipBuild,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

function Get-HostTriple {
    $rustcVersion = & rustc -Vv
    if ($LASTEXITCODE -ne 0) {
        throw "rustc -Vv failed; install Rust before packaging Ori."
    }

    foreach ($line in $rustcVersion) {
        if ($line -like "host:*") {
            return $line.Substring(5).Trim()
        }
    }

    throw "Could not detect the Rust host target from rustc -Vv."
}

function Get-WorkspaceVersion([string]$RepoRoot) {
    $cargoToml = Join-Path $RepoRoot "Cargo.toml"
    $match = Select-String -LiteralPath $cargoToml -Pattern '^\s*version\s*=\s*"([^"]+)"' | Select-Object -First 1
    if ($null -eq $match) {
        throw "Could not find workspace version in $cargoToml."
    }

    return $match.Matches[0].Groups[1].Value
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$version = Get-WorkspaceVersion $repoRoot
$hostTriple = Get-HostTriple
$distRoot = Join-Path $repoRoot "target/dist"

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
