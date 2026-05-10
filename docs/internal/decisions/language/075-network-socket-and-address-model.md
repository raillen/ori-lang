# Decision 075 - Network Socket And Address Model

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / runtime / networking
- Scope: first-wave `std.net` socket model, addressing, DNS boundary, TCP client scope and connection operations

## Summary

The first `std.net` cut is TCP client networking only.

The MVP does not introduce public `net.Address`, `net.Port`, `net.Protocol`, UDP sockets, server listeners or raw sockets.

The user connects with explicit `host: text`, `port: int` and `timeout: time.Duration`.

DNS resolution is handled inside `net.connect(...)` in the MVP.

## Decision

Accepted first-wave model:

- `std.net` starts with TCP client connections only
- no UDP in the MVP
- no TCP server/listener API in the MVP
- no raw socket API in the MVP
- no Unix domain socket API in the MVP
- no public `net.Address` type in the MVP
- no public `net.Port` type in the MVP
- no public `net.Protocol` enum in the MVP
- `host` is passed directly as `text`
- `host` may be a DNS name, IPv4 literal or IPv6 literal
- `port` is passed as `int`
- `port` must be in the range `1..65535`
- DNS resolution is performed by `net.connect(...)`
- no separate `net.resolve(...)` API exists in the MVP
- timeout policy follows Decision 074
- connection operations are methods on `net.Connection`
- mutating connection methods use `mut func` because they may change connection state

Canonical import:

```zt
import std.net as net
```

## First-Wave Types

Accepted public type direction:

```zt
net.Connection
net.Error
```

Deferred public types:

```zt
net.Address
net.Port
net.Protocol
net.Listener
net.UdpSocket
```

`net.Connection` represents a connected TCP client socket.

`net.Error` is the recoverable error family for networking operations.

## Connect API Direction

Accepted connect shape:

```zt
net.connect(
    host: text,
    port: int where port >= 1 and port <= 65535,
    timeout: time.Duration
) -> result<net.Connection, net.Error>
```

Example:

```zt
import std.net as net
import std.time as time

const connection: net.Connection = net.connect(
    host: "example.com",
    port: 80,
    timeout: time.seconds(10)
)?
```

Rules:

- `host` must not be empty
- `port` must be between `1` and `65535`
- `timeout` is required
- DNS lookup is part of `connect`
- DNS failure is reported through `net.Error`
- connection failure is reported through `net.Error`
- timeout is reported as `net.Error.Timeout`

## Connection Operation Direction

Accepted operation naming direction:

```zt
connection.read_some(max: int where max > 0) -> result<optional<bytes>, net.Error>
connection.write_all(data: bytes) -> result<void, net.Error>
connection.close() -> result<void, net.Error>
```

Semantics:

- `read_some!` reads up to `max` bytes
- `read_some!` may return fewer bytes than `max`
- `read_some!` returns `success(none)` when the peer cleanly closes the connection
- `read_some!` returns `success(value chunk)` when bytes are read
- `chunk` should be non-empty when `max > 0`
- `write_all!` attempts to write the full byte sequence
- `write_all!` either succeeds fully or returns an error
- `close!` closes the connection handle
- using a closed connection is a recoverable networking error, not undefined behavior

Example:

```zt
import std.net as net
import std.time as time

const connection: net.Connection = net.connect(
    host: "example.com",
    port: 80,
    timeout: time.seconds(10)
)?

connection.write_all(hex bytes "47 45 54 20 2F 20")?

const maybe_chunk: optional<bytes> = connection.read_some(max: 4096)?

match maybe_chunk
case value chunk:
    handle_chunk(chunk)
case none:
    handle_closed_connection()
end

connection.close()?
```

## Why No Address Type Yet

A public `net.Address` type is useful later, but it adds several decisions that are not needed for the first TCP client cut:

- resolved versus unresolved addresses
- IPv4 versus IPv6 representation
- multiple DNS results
- address lists
- local bind addresses
- display formatting
- equality and hashing semantics
- whether a port belongs inside the address

For the MVP, `host: text` and `port: int` are easier to read and easier to implement.

## Why No Resolve API Yet

A separate `net.resolve(...)` API is useful when the user needs control over DNS results.

The MVP does not need that control.

Keeping DNS inside `connect` makes common code clearer:

```zt
net.connect(host: "example.com", port: 443, timeout: time.seconds(10))?
```

Instead of:

```zt
const addresses: list<net.Address> = net.resolve("example.com")?
const connection: net.Connection = net.connect(addresses[0], timeout: time.seconds(10))?
```

The second form is more powerful, but it is not the beginner-friendly default.

## Rationale

TCP client networking is the smallest useful network target.

It supports HTTP clients, simple protocol clients, local service checks and future TLS layering without forcing the MVP to solve server sockets, UDP semantics or address objects.

Using `host: text` and `port: int` follows Zenith's readability goal: the call says exactly what the user is trying to connect to.

The `read_some!` / `write_all!` naming avoids a common stream ambiguity: reads may be partial, while writes in the MVP are all-or-error.

## Non-Canonical Forms

Address object as the primary MVP call shape:

```zt
const address: net.Address = net.Address(host: "example.com", port: 80)
const connection: net.Connection = net.connect(address, timeout: time.seconds(10))?
```

Separate resolve as mandatory ceremony:

```zt
const addresses: list<net.Address> = net.resolve("example.com")?
const connection: net.Connection = net.connect(addresses[0], timeout: time.seconds(10))?
```

UDP in the MVP:

```zt
const socket: net.UdpSocket = net.udp()
```

Server listener in the MVP:

```zt
const listener: net.Listener = net.listen(port: 8080)?
```

Generic protocol parameter in the MVP:

```zt
net.connect(host: "example.com", port: 80, protocol: net.Protocol.Tcp, timeout: time.seconds(10))?
```

## Out of Scope

This decision does not yet define:

- `std.net.tls`
- websocket APIs
- UDP sockets
- TCP server/listener APIs
- raw sockets
- Unix domain sockets
- public resolved address objects
- public DNS resolver APIs
- local bind address configuration
- proxy support
- HTTP APIs
