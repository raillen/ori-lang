# Ori Language Specification — Chapter 04: Type System

> Status: normative
> Audience: compiler implementers, language designers

---

## Overview

Ori is statically typed. Every binding, parameter, and return position has a
type known at compile time. Type annotations are always explicit; there is no
global type inference for binding declarations.

---

## Primitive Types

| Type | Description | Size |
|---|---|---|
| `bool` | Boolean: `true` or `false` | 1 byte |
| `int` | Signed 64-bit integer (default) | 8 bytes |
| `int8` | Signed 8-bit integer | 1 byte |
| `int16` | Signed 16-bit integer | 2 bytes |
| `int32` | Signed 32-bit integer | 4 bytes |
| `int64` | Alias for `int` | 8 bytes |
| `u8` | Unsigned 8-bit integer | 1 byte |
| `u16` | Unsigned 16-bit integer | 2 bytes |
| `u32` | Unsigned 32-bit integer | 4 bytes |
| `u64` | Unsigned 64-bit integer | 8 bytes |
| `float` | IEEE 754 64-bit float (default) | 8 bytes |
| `float32` | IEEE 754 32-bit float | 4 bytes |
| `float64` | Alias for `float` | 8 bytes |
| `string` | Immutable, valid UTF-8 text | Managed |
| `bytes` | Raw binary data | Managed |
| `void` | No useful value (return type only) | 0 bytes |

Primitive types are value types: they are copied on assignment.

`string` and `bytes` are immutable managed values with reference counting.
Assigning a `string` copies the reference, not the content.

---

## Compound Types

### Struct

A product type. All fields must be named.

```ori
struct Point
    x: int
    y: int
end

const p: Point = Point(x: 0, y: 0)
```

Structs are value types. Assigning a struct copies all fields.
Fields that are managed types (`string`, `bytes`, collections) copy their
references.

**Field contracts** constrain the valid range of a field value:

```ori
struct Rectangle
    width: int  if it > 0
    height: int if it > 0
end
```

`it` is the contextual keyword that refers to the field value being validated.
Contracts are checked at construction time and on mutation. A violation is a
runtime panic (`contract.field_violation`).

### Enum

A sum type. Each variant is a named case.

```ori
enum Direction
    North
    South
    East
    West
end

enum Shape
    Circle(radius: float)
    Rectangle(width: float, height: float)
    Point
end
```

Variants may be:
- **Unit**: no payload (`North`)
- **Named variant**: all fields must have explicit names (`Circle(radius: float)`)

Positional (unnamed) enum payload is not allowed in Ori. All variant fields
must be named. This is required by the reading-first philosophy: `Circle(float)`
does not tell the reader what the float represents.

Enums are value types.

### Tuple

An ordered product of named positional values.

```ori
const pair: tuple<int, string> = tuple(1, "one")
```

Access by index:

```ori
const n: int    = pair.0
const s: string = pair.1
```

---

## Generic Collection Types

These types are built into the language and require no import.

| Type | Description |
|---|---|
| `list<T>` | Ordered, resizable sequence |
| `map<K, V>` | Key-value mapping, keys must implement `Hashable` |
| `set<T>` | Unordered unique values, elements must implement `Hashable` |
| `optional<T>` | A value that may be absent |
| `result<T, E>` | A value that represents success or failure |
| `range<T>` | An inclusive range of ordered values |
| `lazy<T>` | A value computed at most once, on first access |
| `any<Trait>` | Dynamic dispatch over a trait |

---

## Optional

`optional<T>` represents a value that may be absent. There is no `null`.

```ori
const name: optional<string> = some("Ada")
const empty: optional<string> = none
```

Constructors: `some(value)` and `none`.

Operations:

```ori
value.or(fallback)         -- unwrap or use fallback
value.or_return(expr)      -- unwrap or return expr from enclosing function
```

Pattern matching over `optional<T>`:

```ori
match maybe_name
case some(name):
    io.print(name)
case none:
    io.print("not found")
end
```

Binding shorthand:

```ori
if some(name) = maybe_name
    io.print(name)
end
```

---

## Result

`result<T, E>` represents an operation that may succeed or fail.

```ori
const ok: result<int, string>  = success(42)
const bad: result<int, string> = error("something went wrong")
```

Constructors: `success(value)` and `error(value)`.

Operations:

```ori
value.or_wrap("context message")    -- keep success, add context to error
```

Pattern matching:

```ori
match load_config(path)
case success(config):
    use_config(config)
case error(msg):
    io.print(f"failed: {msg}")
end
```

---

## Range

`range<T>` is an inclusive range with a start and end value.

```ori
const r: range<int> = 0..9
```

The range `a..b` includes both `a` and `b`.
- If `a <= b`: ascending (0, 1, 2, ..., 9)
- If `a > b`: descending (9, 8, 7, ..., 0)
- If `a == b`: single element

Properties:

```ori
r.start       -- int: first value
r.end         -- int: last value
r.length()    -- int: number of elements (abs(end - start) + 1)
r.contains(v) -- bool: true if v is in the range
```

---

## Lazy

`lazy<T>` computes its value at most once, the first time it is accessed.

```ori
const expensive: lazy<int> = lazy.once(do() => compute_heavy_value())
const value: int = lazy.force(expensive)
```

A `lazy<T>` is consumed once. Accessing it a second time returns the cached value.

---

## Dynamic Dispatch (`any<Trait>`)

`any<Trait>` holds a value of any type that implements `Trait`, selected at runtime.

```ori
const shape: any<Drawable> = Circle(radius: 10.0)
shape.draw()
```

Rules:
- `any<Trait>` values have heap-allocated vtable dispatch.
- Prefer generics for performance-sensitive paths.
- `==` on `any<Trait>` is a compile error.
- Passing `any<Trait>` across FFI requires explicit ABI annotation.

---

## Func Types (Callable)

A function type describes the signature of a callable value:

```ori
const double: func(int) -> int = do(x: int) => x * 2
var handler: func(string) -> bool
```

Syntax: `func(ParamType, ...) -> ReturnType`

A callable with no return value: `func(string)` (void return implied).

---

## Type Aliases

`alias` gives a name to an existing type. It does not create a new type.

```ori
alias UserId   = int
alias UserMap  = map<string, User>
alias Callback = func(string) -> bool
```

Aliases are transparent: `UserId` and `int` are interchangeable everywhere.

---

## `success()` — Void Result

When a function returns `result<void, E>`, `success()` with no arguments is valid:

```ori
func ping() -> result<void, string>
    send_packet()?
    return success()
end

func start() -> result<void, string>
    ping()?
    return success()
end
```

This is the exact analogue of `return` with no value in a `void` function.
The `void` value is implicit. `success()` with no args is a compile error
when the expected type is not `result<void, _>`.

---

## Structural Equality (`==`)

All types support `==` and `!=` by default using structural equality:

| Type | `==` behavior |
|---|---|
| `bool`, `int`, `float`, `string`, `bytes` | Value equality |
| `list<T>` | Same length and each element `==` in order |
| `map<K, V>` | Same key-value pairs (order irrelevant) |
| `set<T>` | Same elements (order irrelevant) |
| `optional<T>` | `none == none`; `some(a) == some(b)` iff `a == b` |
| `result<T, E>` | `success(a) == success(b)` iff `a == b`; same for `error` |
| `tuple<...>` | Field-by-field in declaration order |
| `struct` | Field-by-field (deep structural equality) |
| `any<Trait>` | **Compile error.** Equality on dynamic dispatch is not defined. |
| `func(...)` | **Compile error.** Function values are not comparable. |

**Override with `Equatable`:** implement `Equatable for T` to provide custom equality:

```ori
implement Equatable for User
    func equals(other: User) -> bool
        return self.id == other.id
    end
end
```

When `Equatable` is implemented, `==` and `!=` use `equals()`.

**Structs with incomparable fields:** if a struct contains a `func` or `any<Trait>`
field, using `==` on that struct is a compile error.

---

## Subtyping and Conversion

Ori does not have implicit type coercion. All conversions are explicit.

**Integer widening** is not implicit. Use the conversion functions:

```ori
const n: int  = 42
const b: u8   = u8(n)         -- explicit narrowing (runtime check)
const w: int64 = int64(n)     -- explicit widening
```

**String conversion:** any type implementing `Displayable` can be converted:

```ori
const s: string = string(42)
const t: string = string(3.14)
```

**Type checking at runtime** (for `any<Trait>`):

```ori
if shape is Circle
    -- shape is accessible as Circle in this block
end
```

---

## Type Compatibility Rules

1. A `result<T, E>` is compatible with `result<T, F>` only if `E == F`.
2. A `list<T>` is compatible with `list<U>` only if `T == U` (no covariance).
3. Generic type parameters are invariant by default.
4. An `any<Trait>` accepts any concrete type implementing `Trait`.
5. A `func(T) -> R` is compatible with `func(T) -> R` only when signatures match exactly.
