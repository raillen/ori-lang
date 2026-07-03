# PowerShell script to build and install Ori globally to C:\ori-lang

$globalDir = "C:\ori-lang"
$binDir = "$globalDir\bin"
$stdlibDir = "$globalDir\stdlib"
$runtimeDir = "$globalDir\runtime"

Write-Host "Creating global directories at $globalDir..."
if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Force -Path $binDir | Out-Null }
if (-not (Test-Path $stdlibDir)) { New-Item -ItemType Directory -Force -Path $stdlibDir | Out-Null }
if (-not (Test-Path $runtimeDir)) { New-Item -ItemType Directory -Force -Path $runtimeDir | Out-Null }

Write-Host "Building ori-driver (release)..."
cargo build -p ori-driver --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build ori-driver."
    exit 1
}

Write-Host "Building ori-runtime (release)..."
cargo build -p ori-runtime --release --lib
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build ori-runtime."
    exit 1
}

Write-Host "Copying ori.exe to $binDir..."
Copy-Item "target\release\ori.exe" "$binDir\ori.exe" -Force

Write-Host "Copying standard library to $stdlibDir..."
Copy-Item "stdlib\*" "$stdlibDir\" -Recurse -Force

Write-Host "Copying runtime artifacts to $runtimeDir..."
# Since runtime artifacts are OS specific, we will just copy the entire runtime directory
Copy-Item "runtime\*" "$runtimeDir\" -Recurse -Force

Write-Host "Updating C:\ori-lang\runtime with freshly built release runtime..."
$triple = "x86_64-pc-windows-msvc" # Assume Windows for this powershell script by default, though we can be smarter
if (Test-Path "target\release\ori_runtime.dll") {
    if (-not (Test-Path "$runtimeDir\$triple")) { New-Item -ItemType Directory -Force -Path "$runtimeDir\$triple" | Out-Null }
    Copy-Item "target\release\ori_runtime.dll" "$runtimeDir\$triple\" -Force
    Copy-Item "target\release\ori_runtime.lib" "$runtimeDir\$triple\" -Force
}

Write-Host "Global update complete!"
Write-Host "Please ensure that $binDir is in your system PATH environment variable."

