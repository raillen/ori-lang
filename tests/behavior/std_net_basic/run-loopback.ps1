param(
    [string]$Executable = "build/std-net-basic.exe",
    [int]$Port = 41234
)

$ErrorActionPreference = "Stop"

$serverScript = Join-Path $PSScriptRoot "loopback-server.ps1"
$resolvedExecutable = if ([System.IO.Path]::IsPathRooted($Executable)) { $Executable } else { Join-Path $PSScriptRoot $Executable }
$readyFile = Join-Path ([System.IO.Path]::GetTempPath()) ("zt-std-net-basic-{0}.ready" -f [System.Guid]::NewGuid().ToString("N"))
$server = Start-Process -FilePath "powershell" -ArgumentList @(
    "-NoProfile",
    "-ExecutionPolicy", "Bypass",
    "-File", $serverScript,
    "-Port", $Port,
    "-ReadyFile", $readyFile
) -PassThru -WindowStyle Hidden

try {
    $deadline = (Get-Date).AddSeconds(5)
    while (-not (Test-Path -LiteralPath $readyFile)) {
        if ($server.HasExited) {
            exit 1
        }
        if ((Get-Date) -gt $deadline) {
            exit 1
        }
        Start-Sleep -Milliseconds 100
    }
    & $resolvedExecutable
    $exitCode = $LASTEXITCODE
    $runningServer = Get-Process -Id $server.Id -ErrorAction SilentlyContinue
    if ($null -ne $runningServer) {
        Wait-Process -Id $server.Id -Timeout 5 -ErrorAction SilentlyContinue
    }
    exit $exitCode
} finally {
    Remove-Item -LiteralPath $readyFile -Force -ErrorAction SilentlyContinue
    $existingServer = Get-Process -Id $server.Id -ErrorAction SilentlyContinue
    if ($null -ne $existingServer) {
        try {
            Stop-Process -InputObject $existingServer -Force -ErrorAction SilentlyContinue
        } catch {
        }
    }
}
