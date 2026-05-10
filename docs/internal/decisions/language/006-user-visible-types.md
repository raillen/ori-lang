# Decision 006 - User Visible Types

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: builtin types, generic surface types, numeric widths, binary type surface and user-facing naming

## Summary

Zenith Next exposes a small builtin type surface with readable canonical names.

`text` remains the canonical textual type name in user code.

`bytes` is part of the fundamental user-visible type surface for binary data.

`void` exists as a user-visible type, but functions that do not return a value omit the return clause instead of writing `-> void`.

Explicit numeric widths are available, but they are opt-in and are not the default everyday style.

## Decision

The builtin scalar types are:

```zt
bool
int
int8
int16
int32
int64
u8
u16
u32
u64
float
float32
float64
text
bytes
void
```

The builtin generic surface types are:

```zt
list<T>
map<K, V>
optional<T>
result<Success, Error>
```

Normative rules for this cut:

- `text` is the canonical textual type name in user code
- `string` is not part of the MVP type surface
- `bytes` is part of the builtin user-visible type surface
- `void` is user-visible as a type, but `void` functions omit the return clause in canonical syntax
- `none` is not the canonical type name for empty return
- `int` always means `int64`, independent of target platform
- `float` always means `float64`, independent of target platform
- `int8`, `int16`, `int32` and `int64` are available when a width-specific signed integer type is needed
- `u8`, `u16`, `u32` and `u64` are available when a width-specific unsigned integer type is needed
- plain `uint` is not part of the MVP type surface
- `float32` and `float64` are available when a width-specific floating-point type is needed
- `float16` and `float8` are not part of the MVP type surface
- explicit-width numeric types are opt-in and should be used when layout, interop, storage or numeric contracts require them
- user-defined nominal types continue to use `PascalCase`
- numeric conversion and overflow rules are defined separately by Decision 024
- binary-data behavior for `bytes` is defined separately by Decision 071

## Rationale

Keeping `text` aligns the new syntax with the language's existing conceptual base and avoids unnecessary renaming churn.

Making `bytes` fundamental reflects the fact that binary data is a core runtime concern for files, processes, interop and networking.

Making `void` user-visible preserves a useful semantic type for APIs such as `result<void, text>`, FFI boundaries and future type-level constructs, while still allowing ordinary side-effect functions to stay visually light by omitting `-> void`.

Fixed meanings for `int` and `float` avoid platform-dependent surprises and improve portability.

Explicit widths are useful, but they carry cognitive cost if overused. The language therefore exposes them without making them the default style.

The documentation and style guide should be explicit that smaller widths do not automatically imply faster code. Their primary value is in memory layout, binary formats, FFI, packed collections and numeric contracts.

## Canonical Examples

Default everyday numeric types:

```zt
const retry_count: int = 0
const average_score: float = 0.0
```

Width-specific numeric types when there is a clear reason:

```zt
const pixel_count: int32 = 1024
const red: u8 = 255
const sample_value: float32 = 0.25
```

Text and binary surface types:

```zt
public func render_name(user: user_types.User) -> text
    return text.trim(user.name)
end

public func digest(data: bytes) -> bytes
    ...
end
```

Void as a visible type in a compound type:

```zt
public func flush_output() -> result<void, text>
    ...
end
```

## Documentation Guidance

The style guide for Zenith Next should teach widths this way:

- use `int` and `float` by default
- use width-specific numeric types only when the code has a concrete reason
- do not claim that smaller widths are automatically faster
- explain width-specific choices at API and data-structure boundaries

Recommended documentation support:

- a table listing each numeric type, width and common use cases
- a short guide named like "When To Use `int` And `float`"
- a short guide named like "When To Use Explicit Numeric Widths"
- examples from FFI, serialization, image data, audio buffers and dense collections

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `string` instead of `text`:

```zt
const display_name: string = "Zenith"
```

using `none` as the empty-return type:

```zt
func log_message(message: text) -> none
    text.write_line(message)
end
```

using width-specific types as the default style without a reason:

```zt
const retry_count: int32 = 0
const average_score: float32 = 0.0
```

depending on platform-sized `int`:

```zt
// non-canonical idea: int width varies by target
```

using plain `uint` as if it were a canonical builtin type:

```zt
const byte_count: uint = 4
```

## Out of Scope

This decision does not yet define:

- decimal or arbitrary-precision numeric types
- `grid<T>` and other specialized collection families
- platform-sized aliases such as `usize`
