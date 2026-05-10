# Types Reference

> Surface: reference
> Status: current

## Primitive Types

| Type | Meaning |
| --- | --- |
| `int` | signed integer |
| `float` | floating-point number |
| `bool` | `true` or `false` |
| `text` | managed text |
| `bytes` | binary buffer |
| `void` | no useful value |

## Generic Containers

| Type | Meaning |
| --- | --- |
| `list<T>` | ordered list |
| `map<K,V>` | key/value map |
| `optional<T>` | value or `none` |
| `result<T,E>` | `success(T)` or `error(E)` |
| `lazy<T>` | explicit lazy value |
| `any Trait` or `any<Trait>` | dynamic trait object subset |

## `void`

`void` means: this operation has no useful value to return.

Use it when the result is only success/failure or side effect.

```zt
func log(message: text)
    -- No useful return value.
    return
end
```

`void` is especially important inside generic types:

```zt
func save(path: text, content: text) -> result<void, text>
    if path == ""
        return error("path is empty")
    end

    -- The operation succeeded, but there is no payload.
    return success()
end
```

Philosophy:

- `void` avoids fake return values like `0` or `true`;
- the function name carries the intent;
- `result<void, E>` says clearly: it can fail, but success has no data.

## `optional<T>`

`optional<T>` means: a value may be present or absent.

Zenith uses `none` instead of user-facing `null`.

```zt
func find_name(id: int) -> optional<text>
    if id == 1
        return "Ada"
    end

    return none
end
```

Handle it with `match`:

```zt
match find_name(1)
    case value name ->
        return name
    case none ->
        return "anonymous"
end
```

Use `optional<T>` when absence is normal, not exceptional.

Small helpers:

- `value.is_some()` checks if an optional has a value.
- `value.is_none()` checks if an optional is absent.
- `value.or(default_value)` returns the contained value, or the fallback.
- `value.or_return(return_value)` returns from the enclosing function when the optional is absent.

Outside the current public helper subset:

- `value.or_error(message)` is not a current public helper. Use `match` when absence must become a new error.

Good examples:

- missing environment variable;
- item not found in a collection;
- optional field in a save file.

## `result<T,E>`

`result<T,E>` means: an operation can succeed with `T` or fail with `E`.

```zt
func parse_port(value: text) -> result<int, text>
    if value == ""
        return error("port is empty")
    end

    return success(8080)
end
```

Handle it explicitly:

```zt
match parse_port("8080")
    case success(port) ->
        return port
    case error(message) ->
        panic(message)
end
```

Use `result<T,E>` when failure is expected and the caller should decide what to do.

Small helpers:

- `value.is_success()` checks if a result succeeded.
- `value.is_error()` checks if a result failed.
- `value.or_wrap(context)` preserves success and adds text context to `core.Error` on failure.

Outside the current public helper subset:

- `value.or_panic(message)` is not a current public helper. Use `match`, `?`, or an explicit `panic`.

Good examples:

- reading a file;
- parsing user input;
- opening a network connection.

## `lazy<T>`

`lazy<T>` means: the value is created now, but evaluated later.

Current alpha surface is explicit and specialized through `std.lazy` helpers.
In `0.4.2-beta.rc1`, the executable checker accepts only
`lazy<int>`, `lazy<float>`, `lazy<bool>`, and `lazy<text>`.
Other payloads fail during `zt check` with post-RC guidance.

```zt
namespace app.main

import std.lazy as lazy

public var hits: int = 0

func tick() -> int
    hits = hits + 1
    return 41
end

public func main() -> int
    const later: lazy<int> = lazy.once_int(func() => tick())

    if hits != 0
        panic("lazy evaluated too early")
    end

    const value: int = lazy.force_int(later)

    if value != 41
        panic("lazy returned wrong value")
    end

    return 0
end
```

Important rule:

- executable `std.lazy` helpers currently cover `int`, `float`, `bool`, and `text`;
- fully generic `lazy<T>` and lazy iterators remain future work;
- each lazy value is one-shot;
- forcing the same value twice is a runtime contract error.

Philosophy:

- laziness is explicit;
- evaluation order is visible;
- the language does not make everything lazy by surprise.

## `any Trait`

`any Trait` means: dynamic dispatch through a trait.

Use it when different concrete types must be handled through the same trait value.

```zt
trait Shape
    func area() -> int
end

struct Circle
    radius: int
end

apply Shape to Circle
    func area() -> int
        return 3 * self.radius * self.radius
    end
end

struct Rect
    w: int
    h: int
end

apply Shape to Rect
    func area() -> int
        return self.w * self.h
    end
end

func total(a: any Shape, b: any Shape) -> int
    return a.area() + b.area()
end
```

Use `any` only when the heterogeneity matters.

If all values have the same concrete type, prefer the concrete type. This keeps code simpler and easier to optimize.

## `mut`

`mut` marks a method that can change `self`.

```zt
struct Counter
    value: int
end

apply Counter
    mut func bump()
        self.value = self.value + 1
        return
    end
end
```

Calling a mutating method needs a mutable receiver:

```zt
var counter: Counter = Counter(value: 0)
counter.bump()
```

`public var` does not remove the need for `mut`.

They answer different questions:

| Concept | Question it answers |
| --- | --- |
| `public var` | Can another namespace read this namespace state? |
| `mut func` | Can this method change its receiver? |
| `var` local | Can this local binding be reassigned? |

This keeps mutation visible at the point where it matters.

## Struct

```zt
public struct User
    id: int
    name: text
end
```

Construction:

```zt
const user: User = User(id: 1, name: "Ada")
```

## Enum

```zt
public enum Direction
    North
    South
    East
    West
end
```

## Trait and Apply

```zt
public trait Named
    func name() -> text
end

apply Named to User
    func name() -> text
        return self.name
    end
end
```
