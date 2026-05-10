# Decision 084 - Net Error Catalog

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / networking
- Scope: std.net error variants and classification

## Summary

This decision defines the official set of error variants for the `net` module in Zenith. The goal is to provide a typed, actionable, and platform-independent classification of common network failures.

## Decision

The `net.Error` enum shall contain the following variants:

| Variant | Semantics |
| :--- | :--- |
| `ConnectionRefused` | Reachable host explicitly rejected the connection. |
| `HostUnreachable` | No route to the host or DNS resolution failed. |
| `Timeout` | Operation did not complete within the specified duration. |
| `AddressInUse` | Local address/port is already tied to another socket. |
| `AlreadyConnected` | Socket is already in a connected state. |
| `NotConnected` | Operation requires a connection that doesn't exist. |
| `NetworkDown` | The local network interface or infrastructure is inactive. |
| `Overflow` | Received data exceeds the provided buffer or internal limit. |
| `PeerReset` | Connection was forcibly closed by the remote peer. |
| `SystemLimit` | OS resource limits reached (e.g., out of file descriptors). |
| `Unknown` | Fallback for non-classified platform errors. |

## Rationale

Zenith avoids exposing raw `errno` or `HRESULT` codes to ensure that error handling logic remains portable and readable. By providing a curated set of variants, we encourage developers to handle expected network conditions (like timeouts or refused connections) explicitly while treating system-level failures as distinct categories.

## Handling Examples

```zt
match connection.read_some(max: 1024)
case success(value data):
    process(data)
case success(none):
    print("Connection closed by peer")
case error(net.Error.Timeout):
    print("Wait time exceeded")
case error(net.Error.PeerReset):
    print("Network drop")
case error(e):
    print("Other network error: " + e.as_text())
end
```
