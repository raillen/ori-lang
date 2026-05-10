# Decision 073 - General Streams Policy

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / runtime / networking architecture
- Scope: public stream abstraction, concrete IO types, runtime internals and future stream traits

## Summary

Zenith does not introduce a public `std.stream` module in the MVP.

The MVP uses concrete types and functions in each module instead of exposing a universal `Stream` abstraction to users.

The runtime C implementation may still use an internal stream abstraction when that reduces duplication, but this is not part of the user-facing language or stdlib surface.

## Decision

Accepted direction:

- no public `std.stream` module in the MVP
- no generic `Stream<T>` type in the MVP
- no universal user-facing `read(...)` / `write(...)` stream API in the MVP
- `std.io` keeps concrete console-oriented types such as `io.Input` and `io.Output`
- `std.fs` keeps file-whole APIs in the MVP instead of exposing open file streams
- `std.net` should start with concrete network types such as `net.Connection`
- binary read/write APIs should first appear on concrete module-owned types
- the runtime C may use internal stream-like structs or vtables if useful
- public stream traits are deferred until concrete APIs prove the shape

## Concrete First-Wave Shape

The first-wave user model should remain concrete and readable:

```zt
import std.io as io
import std.fs as fs

io.write("Hello")

const file_text: text = fs.read_text("notes.txt")?
fs.write_text("copy.txt", file_text)?
```

Networking should follow the same concrete direction:

```zt
import std.net as net
import std.time as time

const connection: net.Connection = net.connect(
    host: "example.com",
    port: 80,
    timeout: time.seconds(10)
)?

connection.write_all(hex bytes "47 45 54")?
const maybe_chunk: optional<bytes> = connection.read_some(max: 4096)?
connection.close()?
```

The exact `std.net` API is defined later by Decision 075. The example only shows the accepted direction: concrete connection-owned operations instead of a generic public stream layer.

## Internal Runtime Direction

The C runtime may use a shared internal representation for stream-like objects.

Possible internal implementation tools:

- function pointer tables
- tagged handles
- opaque runtime structs
- platform-specific adapters for files, console handles and sockets

This is an implementation detail.

User code should not depend on a public runtime stream type in the MVP.

## Deferred Public Traits

Public stream-like traits are deferred.

Possible future traits:

```zt
trait ReadableBytes
    mut func read_bytes(max: int) -> result<bytes, Error>
end

trait WritableBytes
    mut func write_bytes(data: bytes) -> result<int, Error>
end

trait ReadableText
    mut func read_text() -> result<text, Error>
end

trait WritableText
    mut func write_text(value: text) -> result<int, Error>
end
```

These are intentionally not accepted for the MVP because they require more decisions around:

- error type unification
- EOF semantics
- partial reads
- buffering
- timeout policy
- close/drop behavior
- binary/text encoding boundaries
- mutating receiver conventions

## Rationale

A universal stream abstraction is powerful, but it is not cheap.

Introducing it too early would force the MVP to settle several hard API contracts before `std.net`, TLS, files and process pipes have shown their real needs.

Concrete module-owned APIs are easier to read, easier to document and easier to implement in the first C backend.

This direction matches Zenith's preference for explicit, predictable code over premature abstraction.

## Non-Canonical Forms

Public generic stream type in the MVP:

```zt
const stream: Stream<bytes> = net.connect(...)
```

Generic stream module as the main user API:

```zt
stream.write(connection, data)
stream.read(connection, max: 4096)
```

Forcing `std.fs`, `std.io`, `std.net` and `std.os.process` to share one public error type before their concrete APIs are stable.

## Out of Scope

This decision does not yet define:

- `std.net`
- `std.net.tls`
- websocket APIs
- process pipes
- open file handles
- timeout behavior
- socket/address details beyond the first-wave model in Decision 075
- public stream traits after the MVP

