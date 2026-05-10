param(
    [int]$Port = 41234,
    [string]$ReadyFile = ""
)

$ErrorActionPreference = "Stop"

$listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Parse("127.0.0.1"), $Port)
$listener.Start()
if ($ReadyFile -ne "") {
    [System.IO.File]::WriteAllText($ReadyFile, "ready")
}
try {
    $client = $listener.AcceptTcpClient()
    try {
        $stream = $client.GetStream()
        try {
            $buffer = New-Object byte[] 1024
            $read = $stream.Read($buffer, 0, $buffer.Length)
            if ($read -gt 0) {
                $stream.Write($buffer, 0, $read)
                $stream.Flush()
            }
        } finally {
            $stream.Dispose()
        }
    } finally {
        $client.Dispose()
    }
} finally {
    $listener.Stop()
}
