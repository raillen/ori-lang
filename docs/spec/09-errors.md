# Ori Language Specification — Chapter 09: Errors and Propagation

> Status: normative
> Audience: compiler implementers

---

## Overview

Ori has two distinct constructs for absence and failure:

| Construct | Use | Propagation |
|---|---|---|
| `optional<T>` | A value that may be absent | `?` or `if some` |
| `result<T, E>` | An operation that may fail | `?` |

There are no exceptions. There is no `null`. There is no `throw`/`catch`.

All failure paths are visible in function signatures and in the code that
handles them.

---

## `optional<T>`

`optional<T>` represents a value that may or may not be present.

```ori
func find_user(id: int) -> optional<User>
    if id <= 0
        return none
    end
    return some(database.lookup(id))
end
```

Constructors:
- `some(value)` — wraps a value.
- `none` — the absent value.

### Unwrapping `optional<T>`

**Pattern match** (explicit, always safe):

```ori
match find_user(42)
case some(user):
    greet(user)
case none:
    io.print("not found")
end
```

**`if some` binding** (concise single-branch handling):

```ori
if some(user) = find_user(42)
    greet(user)
end
```

**Planned `.or(fallback)`** — unwrap or use a default:

```ori
const name: string = find_name(id).or("Anonymous")
```

**Planned `.or_return(value)`** — unwrap or return from the enclosing function:

```ori
const user: User = find_user(id).or_return(none)
-- If find_user returns none, the enclosing function returns none immediately.
-- The enclosing function must return optional<_>.
```

These helpers are not accepted by the current checker/runtime. Use `?`,
`if some(...) = ...`, or `match` today.

**`?` propagation** — unwrap or propagate absence:

```ori
func get_user_name(id: int) -> optional<string>
    const user: User = find_user(id)?
    -- If none: return none from get_user_name
    -- If some(u): bind u to user
    return some(user.name)
end
```

---

## `result<T, E>`

`result<T, E>` represents an operation that either succeeds with value `T` or
fails with error `E`.

```ori
func read_config(path: string) -> result<Config, string>
    if path == ""
        return error("empty path")
    end
    const raw: string = ori.fs.read_text(path)?
    return success(parse_config(raw)?)
end
```

Constructors:
- `success(value)` — the success variant.
- `error(value)` — the failure variant.

### `?` Propagation on `result<T, E>`

```ori
func start(path: string) -> result<void, string>
    const config: Config = read_config(path)?
    -- If error(e): return error(e) from start
    -- If success(v): bind v to config
    apply_config(config)
    return success()
end
```

Rules for `?` on `result<T, E>`:
1. The enclosing function must return `result<_, F>` where `F` is compatible with `E`.
2. If `E == F`: the error is propagated as-is.
3. If `E != F`: a compile error. Use explicit conversion. `.or_wrap()` is planned.

### Planned `.or_wrap(context)`

Adds a context string to an existing error without losing the original:

```ori
const config: Config = read_config(path).or_wrap("loading configuration")?
```

If `read_config` returns `error("empty path")`, the result becomes
`error("loading configuration: empty path")`.

Current status: `.or_wrap(...)` is not accepted by the current checker/runtime.
Use `?` with matching error types or handle the error with `match` today.

### Pattern Match on `result<T, E>`

```ori
match load_data(path)
case success(data):
    process(data)
case error(msg):
    io.print(f"failed: {msg}")
end
```

---

## `?` — The Propagation Operator

`?` is the unified propagation operator. It works on both `optional<T>` and
`result<T, E>`.

**Behavior summary:**

| Type | On success | On failure |
|---|---|---|
| `optional<T>` | Unwraps to `T` | Returns `none` from enclosing function |
| `result<T, E>` | Unwraps to `T` | Returns `error(e)` from enclosing function |

**Compatibility rules:**

The enclosing function's return type must be compatible:

| Expression type | Required enclosing return type |
|---|---|
| `optional<T>?` | `optional<_>` |
| `result<T, E>?` | `result<_, E>` or `result<_, F>` where `E` converts to `F` |

Using `?` in a function that returns `void` or an incompatible type is a
compile error.

Backend status:

- The native backend supports `?` propagation.
- The C backend supports `?` propagation for `optional<T>` and
  `result<T, E>` when the enclosing function returns a compatible
  `optional<_>` or `result<_, E>`.

---

## Error Types

Any type may be used as the error branch of `result<T, E>`.

Current implementation: `ori.core.Error` exists as a marker trait. The richer
`message()`/`cause()` trait-method contract below is planned and documents the
intended stable shape for future stdlib APIs:

```ori
trait Error
    func message() -> string

    func cause() -> optional<any<Error>>
        return none
    end
end
```

Using `Error` is not required by the compiler today. Future rich stdlib APIs may
require it. The current `ori.Error` value type is importable and stores
`code: string` plus `message: string`; current `ori.io`/`ori.fs` helpers still
use `string` errors or `optional<T>` where documented.

### Defining Error Types

Use `struct` + `implement Error for`:

```ori
struct ValidationError
    field: string
    reason: string
end

implement Error for ValidationError
    func message() -> string
        return f"validation failed on '{self.field}': {self.reason}"
    end
end
```

### Error Union with `enum`

When a function may fail with multiple distinct error types, use an enum:

```ori
enum AppError
    Network(error: NetworkError)
    Validation(error: ValidationError)
    Parse(error: ParseError)
end

implement Error for AppError
    func message() -> string
        match self
        case .Network(error):
            return error.message()
        case .Validation(error):
            return error.message()
        case .Parse(error):
            return error.message()
        end
    end
end

func run(input: string) -> result<Output, AppError>
```

This guarantees exhaustive handling at the call site.

---

## Panic

A panic is a non-recoverable error. It terminates the program (or the current
thread/job, depending on the runtime).

Sources of panic:
- `panic("message")` — explicit panic.
- `check condition` — assertion failure.
- `todo()` — unimplemented path reached.
- `unreachable()` — supposedly unreachable path reached.
- Integer division by zero.
- Index out of bounds.
- Field contract violation.
- Failed type narrowing (`is` check followed by wrong-type access).

Panics are not catchable at the language level. Use `result<T, E>` for
expected failures.

---

## No Exceptions

Ori has no exceptions, no `throw`, no `catch`, no `try` blocks.

The reason: exceptions are invisible in function signatures. A function that
throws can fail in ways not visible to the caller, requiring knowledge of the
implementation to handle correctly. In Ori, every function that can fail
says so in its return type.

---

## Common Patterns

### Chaining fallible operations

```ori
func process(path: string) -> result<Output, string>
    const raw: string   = ori.fs.read_text(path)?
    const parsed: Input = parse(raw)?
    const output: Output = transform(parsed)?
    return success(output)
end
```

### Converting error types

```ori
func run(path: string) -> result<void, AppError>
    -- result<Config, string> is not result<void, AppError>.
    -- Explicit conversion is needed today.
    match read_config(path)
    case success(c):
        apply_config(c)
        return success()
    case error(msg):
        return error(AppError.Parse(ParseError(message: msg)))
    end
end
```

### Early return from nested optional

```ori
func find_display_name(id: int) -> optional<string>
    const user: User = find_user(id)?
    const profile: Profile = find_profile(user.profile_id)?
    return some(profile.display_name)
end
```
