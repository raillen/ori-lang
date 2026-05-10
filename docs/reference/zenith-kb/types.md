# Zenith Type System

## Primitive Types

| Type | Description | Notes |
|------|-------------|-------|
| `bool` | Boolean | `true` / `false` |
| `int` | Signed 64-bit integer | Platform-independent, always int64 |
| `int8` | Signed 8-bit | Opt-in width-specific |
| `int16` | Signed 16-bit | Opt-in width-specific |
| `int32` | Signed 32-bit | Opt-in width-specific |
| `int64` | Signed 64-bit | Same as `int` |
| `u8` | Unsigned 8-bit | Opt-in width-specific |
| `u16` | Unsigned 16-bit | Opt-in width-specific |
| `u32` | Unsigned 32-bit | Opt-in width-specific |
| `u64` | Unsigned 64-bit | Opt-in width-specific |
| `float` | 64-bit floating point | Platform-independent, always float64 |
| `float32` | 32-bit floating point | Opt-in width-specific |
| `float64` | 64-bit floating point | Same as `float` |
| `text` | UTF-8 string | Canonical text type; `string` is NOT a type |
| `bytes` | Immutable binary data | Hex literals: `hex bytes "DE AD"` |
| `void` | No-value type | Only in `result<void, E>`, NOT for locals/fields/params |

NOT in MVP: `any`, `null`, `char`, `uint` (standalone), `grid`

## Generic Types

| Type | Description |
|------|-------------|
| `list<T>` | Ordered collection, zero-indexed |
| `map<K, V>` | Key-value collection, K must be Hashable + Equatable |
| `optional<T>` | Presence or absence (`none`) |
| `result<T, E>` | Success or error |

### Generic Type Syntax

```zt
list<int>
map<text, int>
optional<User>
result<File, io.Error>
list<list<int>>           -- nested generics
list<any<Widget>>          -- dynamic dispatch (fat pointer)
result<void, text>        -- void in success position
```

## Core Traits (Implicit, No Import Needed)

| Trait | Purpose | Required For |
|-------|---------|--------------|
| `Equatable<T>` | `==`, `!=` | Map keys, `contains`, comparisons |
| `Hashable<T>` | Hashing | Map keys |
| `Comparable<T>` | Ordering | `<`, `<=`, `>`, `>=` |
| `TextRepresentable<T>` | Text conversion | `fmt` interpolation |

## Constraint Syntax

```zt
-- Compile-time type constraint
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable<Item>
    ...
end

-- Multiple constraints
func process<T>(value: T)
where T is Equatable<T>
where T is TextRepresentable<T>
    ...
end
```

## Dynamic Dispatch with `any`

```zt
-- Heterogeneous collection of different types conforming to Widget
var ui_nodes: list<any<Widget>> = []
ui_nodes.append(Button())
ui_nodes.append(Label())
```

Rules:
- `any Trait` creates fat pointers (vtable + data pointer)
- Used when monomorphization is impossible (heterogeneous collections)
- Monomorphization is the default for generic functions/structs

## Structs

```zt
struct Player
    name: text
    hp: int where it >= 0
    position: Vec2
end
```

Rules:
- Value semantics (copied on assignment)
- Construction: `Player(name: "Julia", hp: 100)`
- Fields explicitly typed
- Field defaults allowed when compiler supports
- `where` on fields = runtime contract

## Enums

```zt
-- Payload enum
enum LoadUserError
    NotFound(id: int)
    InvalidName(message: text)
end

-- Unit enum
enum Direction = North, South, East, West
```

Rules:
- Enums may have payloads
- Block and inline forms
- Construction: `LoadUserError.NotFound(id: 10)`
- Match must be exhaustive unless an intentional `case else` fallback exists

## Type Conversion

All numeric conversions are EXPLICIT:

```zt
int(value)
float(value)
int64(value)
to_text(value)
```

Rules:
- No implicit conversion between numeric types
- Numeric conversion calls use the target numeric type as the function name
- `to_int` and `to_float` are not the current surface syntax
- `to_text(value)` is separate and requires `TextRepresentable<T>`
- Contextual literal fitting allowed
- Mixed numeric ops require explicit conversion
- Integer overflow checked by default
- Division by zero = runtime error
- Float to int truncates toward zero

## Text/Bytes Conversion

```zt
const data: bytes = text.to_utf8("Hello")
const decoded: text = text.from_utf8(data)?
```

No implicit conversion between `text` and `bytes`.

## Type Rules Summary

- `void` is restricted to `result<void, E>` and similar no-value positions
- `void` is NOT valid for: locals, fields, parameters, lists, maps, optionals
- `text` is the canonical text type; `string` does not exist
- `int` and `float` are platform-independent (always 64-bit)
- Plain `uint` is NOT in MVP (use `u8`/`u16`/`u32`/`u64`)
- `bytes` is a fundamental binary type (immutable)
- `grid` is NOT a core MVP type
