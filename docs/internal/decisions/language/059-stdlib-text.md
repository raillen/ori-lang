# Decision 059 - Stdlib Text Module

- Status: accepted
- Date: 2026-04-22
- Type: stdlib / text
- Scope: current alpha-safe `std.text` surface

## Summary

`std.text` is the pure, locale-neutral text helper module for the current alpha cut.

In this cut, the module exposes only the subset that is implemented safely with the current runtime and backend behavior.

Broader helpers such as replace, split/join, casing and padding remain accepted direction for future expansion, but they are not part of the delivered alpha surface today.

## Decision

Canonical import:

```zt
import std.text as text
```

Current alpha-safe surface:

- `trim`
- `trim_start`
- `trim_end`
- `contains`
- `starts_with`
- `ends_with`
- `has_prefix`
- `has_suffix`
- `has_whitespace`
- `index_of`
- `last_index_of`
- `is_empty`
- `is_digits`
- `limit`
- `to_utf8`
- `from_utf8`

Examples:

```zt
const clean: text = text.trim(name)
const prefixed: bool = text.starts_with(name, "Dr.")
const same_prefixed: bool = text.has_prefix(name, "Dr.")
const spaced: bool = text.has_whitespace(full_name)

const bytes_value: bytes = text.to_utf8("Zenith")
const decoded: result<text, text> = text.from_utf8(bytes_value)
```

Normative rules for this cut:

- `std.text` is pure
- operations are locale-neutral by default
- text operations use code-point semantics in the MVP model
- `len(...)` remains in the core language and is not duplicated here
- core `len(text)` counts code points in the MVP model
- text slicing remains in the core language and is not duplicated here
- `to_utf8(...)` is infallible because Zenith `text` is valid UTF-8
- `from_utf8(...)` currently returns `result<text, text>` in the alpha runtime cut
- there is no implicit conversion between `text` and `bytes`

## Rationale

The old first-wave draft promised a wider surface than the backend could safely deliver.

For the current alpha, it is better to publish a smaller module that is honest and tested than to keep aspirational helpers in the public contract.

This keeps the module useful now while leaving room for a later expansion pass after ownership and codegen edges are better covered.

## Deferred Expansion

These helpers remain future expansion direction, not current alpha guarantee:

- `replace_all`
- `replace_first`
- `split`
- `split_lines`
- `join`
- `to_lower`
- `to_upper`
- `capitalize`
- `title_case`
- `pad_left`
- `pad_right`
- `mask`
- `truncate`
- typed `text.Error`

## Out of Scope

This decision still does not define:

- regex
- accent removal
- emoji detection
- grapheme-aware reverse
- locale-aware sorting/casing
- encodings beyond UTF-8
- a user-visible `char` type
