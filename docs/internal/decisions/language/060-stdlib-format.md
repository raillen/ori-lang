# Decision 060 - Stdlib Format Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / formatting
- Scope: presentation formatting, number/date/datetime/bytes/radix formatting, explicit locale, pattern family split and typed bytes style

## Summary

`std.format` is the pure presentation module.

It returns `text` and does not perform IO, parsing or serialization.

It complements `fmt` and `TextRepresentable<T>` rather than replacing them.

Only the bytes-style family is promoted to a dedicated public type in this cut.

## Decision

Canonical import:

```zt
import std.format as format
```

Accepted principal types:

- `format.BytesStyle`

Accepted first-wave API direction:

- `format.number(...)`
- `format.percent(...)`
- `format.date(...)`
- `format.datetime(...)`
- `format.date_pattern(...)`
- `format.datetime_pattern(...)`
- `format.bytes(...)`
- `format.hex(...)`
- `format.bin(...)`

Deferred:

- `format.duration(...)` until `std.time`
- `format.currency(...)`
- `std.format.table`

Examples:

```zt
const total_text: text = format.number(total, decimals: 2)
const percent_text: text = format.percent(rate, decimals: 1)

const date_text: text = format.date(today, style: "iso")
const dt_text: text = format.datetime(now, style: "short", locale: "pt-BR")

const date_pattern_text = format.date_pattern(today, "yyyy-mm-dd")
const dt_pattern_text = format.datetime_pattern(now, "yyyy-mm-dd HH:mm:ss")

const size_text: text = format.bytes(size)
const hex_text: text = format.hex(color)
const bin_text: text = format.bin(flags)
```

Normative rules for this cut:

- `std.format` is pure
- formatting returns `text`
- locale is explicit when used
- locale is not implicitly taken from the operating system in the MVP design
- `fmt` remains responsible only for interpolation
- default value representation still comes from `TextRepresentable<T>`
- `std.format` exists for presentation-oriented alternatives
- table formatting is deferred to a separate future module or submodule such as `std.format.table`
- padding helpers such as `pad_left` and `pad_right` belong in `std.text`, not in `std.format`
- only the bytes-style family is promoted to a dedicated public type in this cut
- date and datetime style families remain untyped surface decisions for now

## Number And Percent

Accepted direction:

```zt
format.number(value)
format.number(value, decimals: 2)

format.percent(value)
format.percent(value, decimals: 1)
```

Semantics:

- `format.number` is presentation-only formatting
- `format.percent(0.25)` is intended to mean `25%`
- number and percent formatting should stay deterministic by default

## Date And Datetime Families

Two accepted families exist:

### Style family

```zt
format.date(value, style: "iso")
format.datetime(value, style: "short", locale: "pt-BR")
```

### Pattern family

```zt
format.date_pattern(value, "yyyy-mm-dd")
format.datetime_pattern(value, "yyyy-mm-dd HH:mm:ss")
```

Rationale for the split:

- style-based formatting is the simple, common path
- pattern-based formatting is the advanced path
- splitting the families avoids one overloaded API with mutually exclusive configuration styles

The exact invalid-pattern error policy remains implementation work, but the family split is accepted.

## Bytes

Accepted direction:

```zt
format.bytes(value)
format.bytes(value, style: format.BytesStyle)
format.bytes(value, style: format.BytesStyle, decimals: int)
```

Accepted `format.BytesStyle` value set:

- `binary`
- `decimal`

Rules:

- bytes formatting is part of the first-wave module
- `format.BytesStyle` is the accepted public style type for byte formatting
- style controls a coherent bundle of base and units
- the design does not expose base and units as fully independent free parameters in the MVP
- `decimals` may refine the display precision
- the exact member-access spelling for style values follows Zenith general enum rules and is not redefined here
- the exact default style is implementation/documentation work, but the typed style-based shape is accepted

## Hex And Binary

Accepted direction:

```zt
format.hex(value)
format.bin(value)
```

Semantics:

- `format.hex` and `format.bin` are presentation helpers for integer-like values
- they return `text` for display and reporting purposes
- prefix policy, casing policy and width options remain implementation details to be finalized later

## Rationale

Keeping `std.format` pure and presentation-oriented prevents it from becoming a grab bag of unrelated transformations.

Explicit locale matches Zenith explicitness philosophy.

Separating style-based and pattern-based date formatting keeps the common path simple without blocking advanced formatting later.

Typing only `BytesStyle` now captures a small, closed style family without prematurely freezing the broader date and datetime style catalog.

Adding `hex` and `bin` keeps radix-oriented presentation in the module where users will expect display helpers to live.

## Non-Canonical Forms

Making `std.format` responsible for parsing:

```zt
format.parse_int(value)
```

Using operating-system locale implicitly:

```zt
format.date(today)
-- silently depends on host locale
```

Exposing bytes `base` and unit family as unconstrained independent parameters from the start:

```zt
format.bytes(size, base: 1024, units: "decimal")
```

Putting table layout in the first-wave module:

```zt
format.table(rows)
```

Putting generic text-padding helpers in `std.format`.

Promoting all style families to dedicated public types before the date and datetime style catalog is settled.

## Out of Scope

This decision does not yet define:

- `format.duration`
- `format.currency`
- table layout APIs
- pattern-language validation details
- locale catalog shape
- OS-locale bridging
- finalized prefix/casing/width options for `hex` and `bin`
- the final typed surface for date and datetime style families
