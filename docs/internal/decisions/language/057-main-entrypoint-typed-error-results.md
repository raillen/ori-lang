# Decision 057 - Main Entrypoint Typed Error Results

- Status: accepted
- Date: 2026-04-17
- Type: language / tooling / entrypoint
- Scope: `main` return contract, typed error propagation, process boundary

## Summary

Zenith keeps `func main()` as the implicit no-return entrypoint form.

When `main` returns a value, the return type must be explicit.

The entrypoint now allows typed `result` errors, not only `text`, as long as the error type can be rendered as text at the process boundary.

This decision amends the earlier entrypoint decision.

## Decision

Accepted canonical entrypoint forms:

```zt
func main()
    io.write("Hello")
end
```

```zt
func main() -> int
    return 0
end
```

```zt
func main() -> result<void, io.Error>
    const line: optional<text> = io.read_line()?
    return success()
end
```

```zt
func main() -> result<int, io.Error>
    return success(0)
end
```

Normative rules for this cut:

- omitting `-> Type` on `main` means no return value
- non-void `main` forms must declare the return type explicitly
- return-type inference for non-void `main` is not part of the language design
- `main` may return `int`
- `main` may return `result<void, Error>`
- `main` may return `result<int, Error>`
- in those `result` forms, the error type does not need to be `text`
- the error type must be renderable as text at the process boundary
- the intended mechanism is the core/text representation contract such as `TextRepresentable<Error>`
- on `error(value)`, the runtime/driver converts the error value to text, writes it to stderr and exits with code `1`
- on `success()` the exit code is `0`
- on `success(code)` the exit code is `code`

## Rationale

The explicit return type on non-void `main` keeps the entry contract visible.

This matters because `main` controls process exit semantics and user-visible error behavior.

Allowing typed errors at the entry boundary keeps stdlib modules such as `std.io` and `std.fs` free to define meaningful error types instead of collapsing everything to `text`.

This is a better architectural fit than inferring arbitrary non-void `main` returns from the body.

## Non-Canonical Forms

Implicitly inferred non-void `main`:

```zt
func main()
    return 0
end
```

Implicitly inferred `result` main:

```zt
func main()
    const value = io.read_line()?
    return success()
end
```

Restricting `main` errors to `text` only:

```zt
func main() -> result<void, text>
    ...
end
```

## Out of Scope

This decision does not yet define:

- the exact trait/typeclass contract checked for entrypoint error rendering
- async entrypoints
- non-`result` typed failure channels
- host-specific exit-code remapping policies
