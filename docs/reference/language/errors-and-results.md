# Errors and Results Reference

> Surface: reference
> Status: current

## The Rule

Zenith separates absence from failure.

| Situation | Use |
| --- | --- |
| The value may simply not exist | `optional<T>` |
| The operation can fail and the caller needs the reason | `result<T,E>` |
| The function succeeds but returns no data | `void` |

This is part of the language philosophy: make normal paths and failure paths visible.

## Optional

Use `optional<T>` when absence is expected.

```zt
func find_name(id: int) -> optional<text>
    if id == 1
        return "Ada"
    end

    return none
end
```

Manual handling:

```zt
match find_name(2)
    case value name ->
        return name
    case none ->
        return "missing"
end
```

Propagation with `?` may be used where the current compiler supports it:

```zt
func add_one(ok: bool) -> optional<int>
    const value: int = parse_score(ok)?
    return value + 1
end
```

For public examples, prefer explicit `match` when the goal is teaching.

Short helpers:

| Helper | Returns | Use when |
| --- | --- | --- |
| `maybe.is_some()` | `bool` | you only need to know that a value exists |
| `maybe.is_none()` | `bool` | you only need to know that a value is absent |
| `maybe.or(default_value)` | `T` | you want a simple fallback value |

Outside the current public helper subset:

| Helper | Returns | Meaning |
| --- | --- | --- |
| `maybe.or_error(message)` | `result<T,text>` | convert expected absence into a recoverable error |

Use explicit `match` today when absence must become a new error.
Use `maybe.or_return(return_value)` when absence should return from the
enclosing function.

## Result

Use `result<T,E>` when an operation can fail.

```zt
func parse_count(value: text) -> result<int, text>
    if value == ""
        return error("empty value")
    end

    return success(10)
end
```

Success with payload:

```zt
return success(10)
```

Success without payload:

```zt
return success()
```

Error:

```zt
return error("missing file")
```

Handling:

```zt
match parse_count("10")
    case success(count) ->
        return count
    case error(message) ->
        panic(message)
end
```

Short helpers:

| Helper | Returns | Use when |
| --- | --- | --- |
| `result_value.is_success()` | `bool` | you only need to know the operation succeeded |
| `result_value.is_error()` | `bool` | you only need to know the operation failed |

Current helper:

| Helper | Returns | Use when |
| --- | --- | --- |
| `result_value.or_wrap(context)` | `result<T, core.Error>` | you want to preserve the original error and add short text context |

Outside the current public helper subset:

| Helper | Returns | Meaning |
| --- | --- | --- |
| `result_value.or_panic(message)` | `T` | extract success value or panic with a custom message |

## `result<void,E>`

Use `result<void,E>` when the operation can fail but success has no data.

```zt
func write_config(path: text) -> result<void, text>
    if path == ""
        return error("path is empty")
    end

    return success()
end
```

This is clearer than returning a boolean whose meaning must be guessed.

## No Null

Zenith does not use `null` as the user-facing absence model.

Use `optional<T>`.

That keeps absence typed, visible and matchable.
