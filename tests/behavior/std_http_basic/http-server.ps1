param(
    [int]$Port = 41235
)

$ErrorActionPreference = "Stop"

$listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Parse("127.0.0.1"), $Port)
$listener.Start()
try {
    for ($i = 0; $i -lt 2; $i++) {
        $client = $listener.AcceptTcpClient()
        try {
            $stream = $client.GetStream()
            try {
                $buffer = New-Object byte[] 4096
                $count = $stream.Read($buffer, 0, $buffer.Length)
                $request = [System.Text.Encoding]::UTF8.GetString($buffer, 0, $count)

                if ($request.StartsWith("POST /echo ")) {
                    $body = "post echo: zenith"
                } else {
                    $body = "hello from std.http"
                }

                $response = "HTTP/1.1 200 OK`r`nContent-Length: $($body.Length)`r`nConnection: close`r`nContent-Type: text/plain`r`n`r`n$body"
                $bytes = [System.Text.Encoding]::UTF8.GetBytes($response)
                $stream.Write($bytes, 0, $bytes.Length)
                $stream.Flush()
            } finally {
                $stream.Dispose()
            }
        } finally {
            $client.Dispose()
        }
    }
} finally {
    $listener.Stop()
}
