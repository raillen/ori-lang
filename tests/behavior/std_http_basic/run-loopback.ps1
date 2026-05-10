param(
    [string]$Executable = "build/std-http-basic.exe",
    [int]$Port = 41235
)

$ErrorActionPreference = "Stop"

$serverScript = Join-Path $PSScriptRoot "http-server.ps1"
$resolvedExecutable = if ([System.IO.Path]::IsPathRooted($Executable)) { $Executable } else { Join-Path $PSScriptRoot $Executable }
$server = Start-Process -FilePath "powershell" -ArgumentList @(
    "-NoProfile",
    "-ExecutionPolicy", "Bypass",
    "-File", $serverScript,
    "-Port", $Port
) -PassThru -WindowStyle Hidden

try {
    Start-Sleep -Milliseconds 250
    & $resolvedExecutable
    $exitCode = $LASTEXITCODE
    $runningServer = Get-Process -Id $server.Id -ErrorAction SilentlyContinue
    if ($null -ne $runningServer) {
        Wait-Process -Id $server.Id -Timeout 5 -ErrorAction SilentlyContinue
    }
    exit $exitCode
} finally {
    $existingServer = Get-Process -Id $server.Id -ErrorAction SilentlyContinue
    if ($null -ne $existingServer) {
        try {
            Stop-Process -InputObject $existingServer -Force -ErrorAction SilentlyContinue
        } catch {
        }
    }
}
