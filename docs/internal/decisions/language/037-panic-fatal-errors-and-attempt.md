# Decision 037 - Panic, Fatal Errors And Attempt

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / runtime
- Scope: `panic`, fatal errors, contract panic, recoverability, future `attempt/rescue/finally`

## Summary

Zenith separates recoverable failure from fatal failure.

Recoverable failure uses `result<T, E>`.

Expected absence uses `optional<T>`.

Fatal failure uses `panic`.

Future `attempt/rescue/finally` is accepted as a possible structured recovery syntax for `result` and `optional` propagation, not for catching `panic`.

## Decision

Explicit panic:

```zt
panic("unreachable state")
```

Canonical recoverable error:

```zt
func load_user(id: int) -> result<User, text>
    ...
end
```

Canonical handling through `match`:

```zt
match load_user(id)
case success(user):
    show_user(user)
case error(message):
    show_error(message)
end
```

Canonical propagation:

```zt
const user: User = load_user(id)?
```

Normative rules for this cut:

- `panic(message: text)` is a core builtin
- `panic` represents fatal failure
- `panic` is not recoverable through ordinary language control flow in the MVP
- `panic` does not replace `result`
- recoverable failure must use `result<T, E>`
- expected absence must use `optional<T>`
- `where` contract failure produces contract panic
- list index out of bounds produces panic
- missing `map[key]` produces panic
- fatal runtime errors produce panic
- `panic(...)` never returns
- an internal `never` type may exist, but `never` is not required as a user-visible MVP type
- traditional `try/catch` is not Zenith's error model
- future `attempt/rescue/finally` is not exception handling and does not catch panic
- panic capture, if introduced later, must use separate explicit syntax at controlled boundaries

## Panic Use

`panic` is for impossible states, broken invariants and fatal runtime conditions.

Example:

```zt
func required(value: optional<int>) -> int
    match value
    case some(number):
        return number
    case none:
        panic("expected value")
    end
end
```

The compiler may accept `panic(...)` in positions where a value would otherwise be required because `panic` does not return.

## Contract Panic

A failed `where` contract produces contract panic.

Example:

```zt
struct Player
    hp: int where it >= 0
end

const player: Player = Player(hp: -1)
```

Diagnostic direction:

```text
error[runtime.contract]:
  Contract failed.

what happened:
  Player.hp must satisfy: it >= 0

value:
  -1

where:
  src/app/player.zt:2:13
```

Exact formatting belongs to Decision 038, but panic output must remain explicit and human-readable.

## Runtime Panic

List bounds:

```zt
const value: int = numbers[99]
```

Runtime direction:

```text
error[runtime.bounds]:
  List index out of bounds.

index:
  99

length:
  3
```

Missing map key:

```zt
const score: int = scores["Julia"]
```

Runtime direction:

```text
error[runtime.map_key]:
  Map key was not found.
```

When the key is safely representable as text, the runtime may include it.

## No Traditional Exceptions

Zenith does not use traditional exception handling as its normal error model.

This is not Zenith's canonical direction:

```zt
try
    ...
catch error
    ...
end
```

The language should not encourage hidden exceptional control flow for expected failures.

Expected failure belongs in function signatures:

```zt
func read_file(path: text) -> result<text, text>
```

not in hidden throw/catch behavior.

## Future Attempt/Rescue/Finally

`attempt/rescue/finally` is accepted as a future direction for ergonomic handling of `result` or `optional` propagation.

It is not part of the MVP executable subset unless implemented explicitly later.

Conceptual `result` example:

```zt
attempt
    show_loading(true)
    const config: Config = load_config()?
    const user: User = load_user(config.user_id)?
    render(user)
rescue message:
    show_error(message)
finally
    show_loading(false)
end
```

Semantics:

- inside `attempt`, `?` does not immediately return from the surrounding function
- instead, `?` transfers control to `rescue`
- `rescue` handles the propagated `result` error or `optional` absence
- `finally` is optional
- `finally` runs on successful completion of `attempt`
- `finally` runs after `rescue`
- `finally` runs before leaving through `return` inside `attempt` or `rescue`
- `finally` is not the primary resource-release mechanism
- scope/ownership remains the primary cleanup mechanism
- `attempt/rescue/finally` does not catch `panic`

Optional direction:

```zt
attempt
    const user: User = find_user(id)?
    show_user(user)
rescue none:
    show_not_found()
end
```

First-cut restriction direction:

- an `attempt` block should use one propagation channel
- mixing `result` and `optional` in the same `attempt` is out of scope for the first cut
- `?` inside `finally` requires a future separate decision

## Why Attempt Does Not Catch Panic

`panic` means bug, broken contract or fatal runtime condition.

If ordinary `attempt/rescue` caught panic, users could start using `panic` as recoverable control flow, weakening the distinction between:

- expected failure: `result`
- expected absence: `optional`
- fatal invariant violation: `panic`

This would make function signatures less honest.

Example non-canonical direction:

```zt
func load_user(id: int) -> User
    if not file_exists("user.txt")
        panic("file not found")
    end

    ...
end
```

This should instead be:

```zt
func load_user(id: int) -> result<User, text>
    ...
end
```

## Future Panic Recovery

If panic capture is needed, it must use separate explicit syntax and be restricted to controlled boundaries.

Possible future direction:

```zt
recover panic
    run_app()
case info:
    log_panic(info)
    return 1
end
```

Valid future use cases:

- `main` boundary
- test runner
- host/embedder boundary
- isolated worker/task boundary

This is not normal application control flow and is out of scope for the MVP.

## Cleanup

For non-fatal early return and `?` propagation, cleanup of live managed values is required.

For fatal panic, the semantic direction is human-readable reporting and ordered cleanup when viable. However, the initial C backend may abort directly on fatal panic paths.

Zenith should not promise `finally` as the resource cleanup foundation. Scope and ownership should handle resources.

## Rationale

Keeping fatal and recoverable failure separate preserves readable control flow.

`result` makes expected failure visible in the type.

`optional` makes absence visible in the type.

`panic` remains reserved for broken invariants and unrecoverable states.

Future `attempt/rescue/finally` can reduce nested `match` boilerplate while preserving explicit, typed error channels.

Separating future panic recovery into `recover panic` prevents accidental reintroduction of traditional exception semantics.

## Non-Canonical Forms

Using panic for expected failure is not canonical:

```zt
func read_file(path: text) -> text
    if missing(path)
        panic("file not found")
    end
end
```

Catching panic through `attempt/rescue` is not canonical:

```zt
attempt
    panic("broken")
rescue message:
    ...
end
```

Traditional `try/catch` syntax is not canonical:

```zt
try
    ...
catch error
    ...
end
```

## Out of Scope

This decision does not yet define:

- concrete `attempt/rescue/finally` implementation
- `attempt` type inference rules
- `?` inside `finally`
- concrete `PanicInfo` type
- `recover panic` implementation
- stack traces
- panic hooks
- test assertions for panic
- host/embedder panic APIs
