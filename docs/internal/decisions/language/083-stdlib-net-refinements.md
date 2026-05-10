# Decision 083 - Stdlib Net Refinements

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / networking
- Scope: std.net per-operation timeouts, DNS resilience, and buffering policy

## Summary

Following the consolidation of Zenith's core memory and error models, `std.net` is refined to provide a balance between low-level control (raw unbuffered sockets) and high-level resilience (transparent multi-IP DNS and optional granular timeouts).

## Decision

Accepted refinements:

- **Per-Operation Timeouts**: `net.Connection` methods such as `read_some` and `write_all` accept an optional `timeout: time.Duration`. If omitted, the operation uses the connection's default timeout (defined at `connect`).
- **Resilient DNS**: `net.connect` iterates through all IP addresses returned by DNS resolution. It attempts to connect to each sequentially until success or the global timeout is reached.
- **Unbuffered Raw Connections**: `net.Connection` remains a thin, unbuffered wrapper around the system socket to ensure explicit and predictable IO.
- **Manual vs Managed Close**: `connection.close()` remains the canonical way to explicitly close the socket and handle potential errors. However, the connection's drop handler (ARC reduction to zero) must ensure the socket is closed to prevent resource leaks.
- **Categorized Errors**: Recoverable networking errors are mapped to Zenith-specific categories in `net.Error`, shielding users from platform-specific error codes.

## Rationale

Zenith's "Reading-First" philosophy requires that networking code be robust without excessive boilerplate. 

Mandatory timeouts prevent silent freezes, while optional per-operation overrides provide the flexibility needed for asymmetric protocols. 

The resilient DNS behavior matches developer expectations for modern applications, moving the burden of retry logic from the user to the standard library.

Keeping the base connection unbuffered ensures that users have a clear understanding of when syscalls occur, satisfying the "Explicit" pillar. Ergonomics for buffered reads (like `read_line`) are provided via separate wrappers.

## Canonical Shape

```zt
import std.net as net
import std.time as time

const conn = net.connect(
    host: "example.com",
    port: 80,
    timeout: time.seconds(10)
)?

// Uses default 10s timeout
conn.write_all(request)?

// Explicit 2s timeout for this specific read
const chunk = conn.read_some(max: 1024, timeout: time.seconds(2))?
```
