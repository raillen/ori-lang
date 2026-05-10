# Decision 071 - Bytes And Binary Foundation

- Status: accepted
- Date: 2026-04-17
- Type: language / stdlib / runtime
- Scope: fundamental binary type, byte scalar naming, immutability, indexing, slicing and deferred encoding boundary
- Amended by: Decision 072 for byte literals, `std.bytes` construction helpers and UTF-8 conversion APIs

## Summary

Zenith introduces `bytes` as a fundamental user-visible binary type.

The element type for binary data is `u8`.

`bytes` is immutable in the MVP and belongs to the language/runtime foundation rather than to a late library convenience layer.

This decision also amends the earlier unsigned-width spelling so the canonical family becomes `u8`, `u16`, `u32` and `u64` instead of `uint8`, `uint16`, `uint32` and `uint64`.

## Decision

Accepted fundamental binary types in this cut:

```zt
u8
u16
u32
u64
bytes
```

Normative rules for this cut:

- `bytes` is a fundamental user-visible type
- `bytes` is not modeled as just `list<u8>` in the public language surface
- `bytes` is immutable in the MVP
- the element type of `bytes` is `u8`
- indexing a `bytes` value yields `u8`
- slicing a `bytes` value yields `bytes`
- `len(...)` applies to `bytes`
- equality on `bytes` is expected as part of ordinary core equality support
- literal syntax for `bytes` is defined by Decision 072 as `hex bytes "..."`
- implicit conversion between `text` and `bytes` is not part of this cut
- explicit text/bytes conversion is defined by Decision 072 through UTF-8 APIs in `std.text`
- the canonical unsigned-width type family is `u8`, `u16`, `u32`, `u64`
- previous `uint8`, `uint16`, `uint32`, `uint64` spellings are superseded by this decision

## Why `bytes` Is Fundamental

`bytes` is treated as a foundational type because it is needed by multiple parts of the language and stdlib architecture:

- binary file IO
- process capture and stdin/stdout extensions
- networking
- TLS
- interop and packed binary data

Treating binary data as a first-class type avoids pretending that it is just a generic list when the runtime and APIs need more specific semantics.

## Why `u8`

`u8` is accepted as the byte-sized scalar because it is the most natural companion to `bytes` and is easier to read in binary-oriented APIs than a longer `uint8` spelling.

This shorter family is now the canonical unsigned naming direction for Zenith.

## Construction And Conversion Boundary

Decision 072 later defines byte literal syntax, `std.bytes` constructor helpers and explicit UTF-8 conversion APIs.

This foundation decision by itself does not define:

- broader text encoding APIs beyond UTF-8
- implicit `text` to `bytes` conversion
- implicit `bytes` to `text` conversion
- mutable byte-buffer builders

Those belong to the next binary-data discussion layer.

## Rationale

A dedicated immutable `bytes` type keeps binary APIs honest and gives the runtime a clear optimization target.

Using `u8` as the byte element type aligns the scalar naming with practical binary work and avoids maintaining two parallel unsigned naming families.

This foundation was intentionally separated from the later literal and encoding decision so the binary type model stayed clear before the construction surface was chosen.

## Non-Canonical Forms

Treating binary data as ordinary generic lists in the public surface:

```zt
const payload: list<int> = ...
```

Using `uint8` / `uint16` / `uint32` / `uint64` as the canonical unsigned spelling after this decision.

Making `bytes` mutable in the MVP without a separate mutation model.

Introducing implicit text/binary conversion.

## Out of Scope

This decision does not yet define:

- byte literal syntax beyond the `hex bytes "..."` form accepted by Decision 072
- mutable byte buffers
- text encoding modules or policies
- binary IO APIs
- binary network APIs
- helper APIs for `bytes` beyond the first-wave `std.bytes` set accepted by Decision 072
