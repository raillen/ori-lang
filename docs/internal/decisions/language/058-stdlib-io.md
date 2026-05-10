# Decision 058 - Stdlib Io Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / io
- Scope: standard input/output/error streams, typed stream handles, textual IO, stream defaults, flush policy and IO error type

## Summary

`std.io` is the small, textual, synchronous IO module for standard streams.

It exposes typed default input/output/error handles and uses free functions with optional named stream parameters.

The MVP keeps the surface ergonomic by hiding explicit flush from ordinary user code.

## Decision

Canonical import:

```zt
import std.io as io
```

Accepted principal types:

- `io.Input`
- `io.Output`
- `io.Error`

Canonical default streams:

- `io.input: io.Input`
- `io.output: io.Output`
- `io.error: io.Output`

Canonical operations:

```zt
io.write("Hello")
io.write("error\n", to: io.error)

io.print(user)
io.print(user, to: io.error)

const line: result<optional<text>, io.Error> = io.read_line()
const all: result<text, io.Error> = io.read_all()
```

Normative rules for this cut:

- `std.io` is limited to standard textual input/output/error concerns
- the module uses free functions, not method-style stream calls, as the canonical surface
- `io.Input` and `io.Output` are the accepted stream-handle types for the MVP
- `io.error` is an `io.Output`, not a separate special error-output type
- a generic `io.Stream` abstraction is not part of the MVP surface
- output operations accept an optional `to:` output handle
- input operations accept an optional `from:` input handle
- when omitted, output defaults to `io.output`
- when omitted, input defaults to `io.input`
- the standard streams are named `input`, `output` and `error`, not `stdin`, `stdout` and `stderr`
- `write` accepts only `text`
- `print` accepts values renderable through `TextRepresentable<T>`
- `write` and `print` do not append a newline automatically
- `read_line` returns `result<optional<text>, io.Error>`
- `read_all` returns `result<text, io.Error>`
- EOF on `read_line` is represented as `success(none)`
- IO failure uses a typed `io.Error`, not plain `text`
- `flush()` is not part of the MVP user-facing API

## API Direction

Accepted API direction:

```zt
func write(value: text, to: io.Output = io.output) -> result<void, io.Error>

func print<Item>(value: Item, to: io.Output = io.output) -> result<void, io.Error>
where Item is TextRepresentable<Item>

func read_line(from: io.Input = io.input) -> result<optional<text>, io.Error>
func read_all(from: io.Input = io.input) -> result<text, io.Error>
```

## Flush Policy

The MVP should not require users to remember manual flush in ordinary interactive programs.

Recommended runtime policy:

- `io.error` uses aggressive flushing
- `io.output` is line-buffered when attached to a terminal
- the runtime may flush `io.output` automatically before blocking reads on `io.input`
- the runtime flushes standard output streams on normal process completion

Example target behavior:

```zt
io.write("Name: ")
const name: optional<text> = io.read_line()?
```

The user should not need an explicit `flush()` for this common prompt/read flow.

## Read Semantics

`read_line` strips line terminators from the returned line.

It treats ordinary line endings such as `\n` and `\r\n` as line terminators.

If EOF occurs after collecting text for a final unterminated line, that line is returned first.

The next `read_line` returns `success(none)`.

`read_all` reads from the current position to EOF.

## Error Type

`std.io` uses a typed `io.Error`.

This is intentionally aligned with the updated `main` contract so user programs can return `result<void, io.Error>` or `result<int, io.Error>` directly.

## Rationale

Free functions with optional `to:` and `from:` parameters keep the common case short without forcing object-method syntax or explicit stream values everywhere.

Giving `std.io` explicit `Input` and `Output` handle types makes the module semantically cleaner and prepares it for future integration with files and processes without changing the surface model.

Using `input`, `output` and `error` reads better than traditional C names while preserving explicit destinations when needed.

Keeping `std.io` textual and synchronous prevents it from absorbing filesystems, process management or terminal-control concerns too early.

Hiding flush as a runtime policy reduces friction in the MVP.

## Non-Canonical Forms

Method-style stream calls as the only canonical API:

```zt
io.output.write("Hello")
```

Always requiring explicit stream arguments:

```zt
io.write(io.output, "Hello")
```

Automatic newline in `write` or `print`:

```zt
io.print("Hello")
-- implicitly writes "\n"
```

Returning plain `text` errors by default:

```zt
func read_line() -> result<optional<text>, text>
```

Introducing a generic untyped `io.Stream` into the public MVP surface.

## Out of Scope

This decision does not yet define:

- file IO
- binary IO
- buffering configuration APIs
- terminal colors/cursor APIs
- stream seeking
- stream close semantics for user-managed resources
- async IO
