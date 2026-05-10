# Decision 074 - Network Blocking And Timeout Policy

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / runtime / networking
- Scope: blocking network behavior, explicit timeout policy, result errors and MVP exclusions
- Amended by: Decision 075 for concrete connection operation names and read EOF shape

## Summary

Zenith networking is synchronous/blocking in the MVP.

`std.net` does not introduce `async`, `await`, event loops, callbacks or non-blocking polling in the MVP.

Network operations must still avoid hidden indefinite waits: connection creation requires an explicit timeout, and the connection carries that timeout as the default for later operations.

## Decision

Accepted direction:

- `std.net` is blocking/synchronous in the MVP
- no `async` / `await` in the MVP
- no event loop API in the MVP
- no callback-based networking API in the MVP
- no user-facing non-blocking polling API in the MVP
- network operations use `time.Duration`
- `net.connect(...)` requires `timeout: time.Duration`
- `net.Connection` stores that timeout as the default operation timeout
- `read`, `write` and `close` return `result`
- timeout is represented as a recoverable `net.Error.Timeout`
- timeout is not `panic`
- timeout is not EOF
- timeout is not absence of data
- there is no hidden global network timeout in the MVP
- explicit infinite wait is not part of the MVP
- `std.io` console input may remain blocking without timeout
- `std.fs` file-whole APIs do not require timeout in the MVP
- process timeout policy remains separate and can be discussed with process APIs

## Canonical Shape

The intended first-wave network style is:

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

connection.close()?
```

Decision 075 defines the first-wave `std.net` connection operation names. This example shows the accepted timeout model:

- timeout is explicit at connection creation
- later operations use the connection timeout by default
- recoverable failures use `result`

## Why Timeout On Connect

Requiring timeout on every `read` and `write` would be explicit but noisy:

```zt
connection.read_some(max: 4096, timeout: time.seconds(5))?
connection.write_all(data, timeout: time.seconds(5))?
```

Using a hidden global default would be shorter but too magical:

```zt
net.connect(host: "example.com", port: 80)?
```

The accepted compromise is to make timeout explicit at the connection boundary and then let the connection carry that policy.

This keeps source code readable without hiding the most important operational decision.

## Timeout Semantics

Timeout means the requested network operation did not finish within the configured duration.

It is a recoverable error:

```zt
net.Error.Timeout
```

Timeout does not mean:

- the peer closed the connection
- EOF was reached
- no data exists yet
- the operation returned partial success
- the program should panic

Any future API that supports partial reads or writes must define partial-result behavior explicitly instead of using timeout as an implicit partial-success signal.

## Rationale

Synchronous networking is much easier to implement in the first C runtime and much easier to read in beginner-facing examples.

At the same time, invisible indefinite waits are bad for Zenith's philosophy because they make programs appear frozen without a clear cause.

Requiring a timeout at the connection boundary makes the latency policy visible while avoiding repetitive timeout arguments on every operation.

Deferring async and non-blocking APIs prevents the MVP from taking on an event-loop model before the core language and stdlib are stable.

## Non-Canonical Forms

Hidden default timeout:

```zt
const connection: net.Connection = net.connect(host: "example.com", port: 80)?
```

Mandatory timeout repetition on every operation as the primary style:

```zt
connection.read_some(max: 4096, timeout: time.seconds(10))?
connection.write_all(data, timeout: time.seconds(10))?
```

Async networking syntax in the MVP:

```zt
const connection: net.Connection = await net.connect(...)
```

Timeout as panic:

```zt
panic("network timeout")
```

Timeout as optional absence:

```zt
const chunk: optional<bytes> = connection.read_some(max: 4096)
```

## Out of Scope

This decision does not yet define:

- socket/address details beyond the first-wave model in Decision 075
- `std.net` final API surface
- TLS behavior
- websocket behavior
- non-blocking APIs
- event-loop APIs
- async/await language support
- explicit infinite-wait syntax
- process timeout policy
