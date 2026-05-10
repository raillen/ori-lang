# Decision 024 - Numeric Conversions Overflow And Unsigned Integers

- Status: accepted
- Date: 2026-04-16
- Type: language / semantics
- Scope: unsigned integers, conversion rules, overflow policy and mixed numeric expressions

## Summary

Zenith Next keeps the everyday numeric surface small, but still exposes explicit-width unsigned integers when layout or interop require them.

The canonical unsigned family is `u8`, `u16`, `u32` and `u64`.

Numeric conversions are explicit, not implicit, except for contextual numeric literals.

Integer arithmetic is checked by default and does not silently wrap in the MVP.

## Decision

Zenith Next adds the following unsigned integer types to the user-visible numeric surface:

```zt
u8
u16
u32
u64
```

The canonical explicit conversion form is a target-type call:

```zt
const port: u16 = u16(8080)
const total: int64 = int64(count32)
const opacity: float32 = float32(value)
```

Normative rules for this cut:

- `u8`, `u16`, `u32` and `u64` are part of the MVP numeric type surface
- plain `uint` is not part of the MVP surface
- signed and unsigned integer types are distinct and are not implicitly interchangeable
- there is no implicit numeric conversion between different integer widths, signedness families or float families
- there is no implicit conversion between integer and floating-point types
- the only implicit numeric adaptation in this cut is contextual literal fitting when a numeric literal is directly checked against an expected numeric type and the value is representable there
- mixed numeric expressions require operands of the same numeric type after any explicit conversion
- explicit numeric conversion uses the canonical form `TargetType(expression)`
- in this cut, the canonical target types for explicit numeric conversion are builtin numeric types
- if a conversion target is obviously invalid at compile time, semantic analysis must reject it
- if a runtime numeric value cannot be represented in the target type, the explicit conversion fails as a runtime numeric conversion error
- conversion from floating-point to integer requires the source value to be finite, integral and representable in the target integer type
- conversion from integer to floating-point is explicit and uses the target floating-point representation
- conversion from one floating-point width to another is explicit and uses the target floating-point representation
- integer `+`, `-`, `*`, `/` and `%` are checked operations in the MVP
- integer overflow and underflow are runtime numeric errors in the MVP
- integer division by zero and remainder by zero are runtime numeric errors in the MVP
- unsigned integer arithmetic follows the same checked policy and does not wrap by default
- `float32` and `float64` are ordinary floating-point types in the language model and do not become trapping arithmetic by default in the MVP

## Canonical Examples

Unsigned integer for binary or packed data:

```zt
const red: u8 = 255
const packet_size: u16 = 512
```

Explicit width change:

```zt
const count32: int32 = 120
const count64: int64 = int64(count32)
```

Explicit signed-to-unsigned conversion:

```zt
const retries: int = 3
const retry_byte: u8 = u8(retries)
```

Explicit float narrowing:

```zt
const ratio64: float = 0.5
const ratio32: float32 = float32(ratio64)
```

## Rationale

Exposing explicit-width unsigned integers is useful for FFI, binary formats, image data, audio buffers and dense storage without forcing a platform-sized `uint` into the surface language.

Rejecting implicit numeric conversions keeps the language readable and avoids silent changes of width, signedness or floating-point family in the middle of expressions.

Checked integer arithmetic matches the fail-fast direction already chosen for contracts and runtime errors. Silent wrapping would be shorter to implement in some backends, but it would be harder to read and easier to misuse.

Allowing only contextual literal fitting as the implicit numeric adaptation keeps common code ergonomic without opening the door to broad hidden promotions.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using a platform-sized plain `uint`:

```zt
const count: uint = 10
```

relying on implicit widening between integer widths:

```zt
const count32: int32 = 120
const count64: int64 = count32
```

mixing signed and unsigned integers without explicit conversion:

```zt
const a: int32 = 10
const b: u32 = 20
const c: int32 = a + b
```

assuming that integer overflow wraps by default:

```zt
const next: u8 = 255 + 1
```

using symbolic cast syntax as the canonical conversion form:

```zt
const total: int64 = count32 as int64
```

## Out of Scope

This decision does not yet define:

- saturating arithmetic helpers
- explicit wrapping arithmetic helpers
- decimal or arbitrary-precision numeric families
- platform-sized aliases such as `usize`
- numeric literal suffix syntax
- fine-grained floating-point exception handling
