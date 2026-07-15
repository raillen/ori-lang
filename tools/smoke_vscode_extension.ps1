[CmdletBinding()]
param(
    [string]$WorkspaceRoot = "",
    [switch]$SkipCargoBuild,
    [switch]$SkipNpmInstall,
    [switch]$SkipLspE2e,
    [switch]$KeepWorkspace
)

$ErrorActionPreference = "Stop"

function Get-OriExeName {
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        return "ori.exe"
    }
    return "ori"
}

function Get-LspExeName {
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        return "ori-lsp.exe"
    }
    return "ori-lsp"
}

function Invoke-Checked([scriptblock]$Command, [string]$Description) {
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed with exit code $LASTEXITCODE."
    }
}

function Assert-SmokeRoot([string]$Path) {
    $full = [System.IO.Path]::GetFullPath($Path)
    $leaf = Split-Path -Leaf $full
    if (-not $leaf.StartsWith("ori-vscode-extension-smoke-")) {
        throw "Refusing to remove smoke workspace with unexpected name: $full"
    }
    return $full
}

function Assert-JsonFile([string]$Path) {
    Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json | Out-Null
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$extensionRoot = Join-Path $repoRoot "extensions/vscode-orl"
$targetRoot = if ($env:CARGO_TARGET_DIR) {
    [System.IO.Path]::GetFullPath($env:CARGO_TARGET_DIR)
} else {
    # Workspace crates live under compiler/
    Join-Path $repoRoot "compiler/target"
}

if ([string]::IsNullOrWhiteSpace($WorkspaceRoot)) {
    $WorkspaceRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("ori-vscode-extension-smoke-" + [System.Guid]::NewGuid().ToString("N"))
}
$workspaceRootPath = Assert-SmokeRoot $WorkspaceRoot

$oriExe = Join-Path $targetRoot (Join-Path "debug" (Get-OriExeName))
$lspExe = Join-Path $targetRoot (Join-Path "debug" (Get-LspExeName))
$projectRoot = Join-Path $workspaceRootPath "demo"

Push-Location $repoRoot
try {
    if (-not $SkipCargoBuild) {
        Invoke-Checked { cargo build --manifest-path (Join-Path $RepoRoot "compiler/Cargo.toml") -p ori-driver -p ori-lsp } "cargo build -p ori-driver -p ori-lsp"
    }

    if (-not (Test-Path -LiteralPath $oriExe -PathType Leaf)) {
        throw "Ori compiler was not found at $oriExe."
    }
    if (-not (Test-Path -LiteralPath $lspExe -PathType Leaf)) {
        throw "Ori LSP server was not found at $lspExe."
    }

    Push-Location $extensionRoot
    try {
        if (-not $SkipNpmInstall -and -not (Test-Path -LiteralPath "node_modules" -PathType Container)) {
            Invoke-Checked { npm install } "npm install"
        }
        Invoke-Checked { npm run compile } "npm run compile"
        Assert-JsonFile (Join-Path $extensionRoot "package.json")
        Assert-JsonFile (Join-Path $extensionRoot "language-configuration.json")
        Assert-JsonFile (Join-Path $extensionRoot "snippets/ori.json")
        Assert-JsonFile (Join-Path $extensionRoot "syntaxes/ori.tmLanguage.json")
    } finally {
        Pop-Location
    }

    if (-not $SkipLspE2e) {
        Invoke-Checked { cargo test --manifest-path (Join-Path $RepoRoot "compiler/Cargo.toml") -p ori-lsp --test e2e } "cargo test -p ori-lsp --test e2e"
    }

    if (Test-Path -LiteralPath $workspaceRootPath) {
        Remove-Item -LiteralPath $workspaceRootPath -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $workspaceRootPath | Out-Null

    Invoke-Checked { & $oriExe new $projectRoot --name vscode_smoke } "ori new outside repository"

    $settingsDir = Join-Path $projectRoot ".vscode"
    New-Item -ItemType Directory -Force -Path $settingsDir | Out-Null
    $settings = @{
        "ori.lsp.path" = $lspExe
        "ori.compiler.path" = $oriExe
        "ori.stdlib.root" = Join-Path $repoRoot "stdlib"
    } | ConvertTo-Json
    Set-Content -LiteralPath (Join-Path $settingsDir "settings.json") -Value $settings -Encoding ASCII

    Invoke-Checked { & $oriExe check (Join-Path $projectRoot "ori.proj") } "ori check outside repository"
    Invoke-Checked { & $oriExe run (Join-Path $projectRoot "src/main.orl") } "ori run outside repository"
    Invoke-Checked { & $oriExe test (Join-Path $projectRoot "src/main.orl") } "ori test outside repository"
    Invoke-Checked { & $oriExe doc check (Join-Path $projectRoot "ori.proj") } "ori doc check outside repository"
    Invoke-Checked { & $oriExe summary (Join-Path $projectRoot "ori.proj") } "ori summary outside repository"
    Invoke-Checked { & $oriExe fmt (Join-Path $projectRoot "src/main.orl") } "ori fmt outside repository"

    Write-Host "VS Code extension smoke passed: $workspaceRootPath"
} finally {
    Pop-Location
    if (-not $KeepWorkspace -and (Test-Path -LiteralPath $workspaceRootPath)) {
        Remove-Item -LiteralPath $workspaceRootPath -Recurse -Force
    }
}
