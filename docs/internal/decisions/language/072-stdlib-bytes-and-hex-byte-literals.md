# Decision 072 - Stdlib Bytes And Hex Byte Literals

- Status: accepted
- Date: 2026-04-17
- Type: language / stdlib / runtime
- Scope: `std.bytes`, byte construction, hex byte literals and explicit UTF-8 conversion boundary

## Summary

Zenith adds `std.bytes` as the explicit helper module for the fundamental `bytes` type.

The MVP includes a readable hex byte literal form:

```zt
const magic: bytes = hex bytes "DE AD BE EF"
```

The literal is language syntax, not a `std.bytes` function, and does not require an import.

Text and bytes conversion remains explicit through UTF-8 APIs in `std.text`.

## Decision

Canonical import:

```zt
import std.bytes as bytes
```

Accepted first-wave `std.bytes` API direction:

- `empty`
- `from_list`
- `to_list`
- `join`
- `starts_with`
- `ends_with`
- `contains`

Accepted byte literal syntax:

```zt
const empty_data: bytes = hex bytes ""
const header: bytes = hex bytes "DE AD BE EF"
const spaced: bytes = hex bytes "DE_AD_BE_EF"
const lower: bytes = hex bytes "de ad be ef"
```

Multiline form is allowed when the literal is large:

```zt
const packet: bytes = hex bytes """
    DE AD BE EF
    00 01 02 03
"""
```

Normative rules for this cut:

- `std.bytes` is an explicit stdlib module
- `bytes` itself remains a fundamental core type
- byte literals use the canonical prefix `hex bytes`
- byte literals produce immutable `bytes`
- byte literals are compile-time values
- the content of a byte literal is hexadecimal data, not text data
- each byte is represented by exactly two hexadecimal digits after separators are ignored
- ASCII whitespace and `_` may be used as visual separators
- upper-case and lower-case hexadecimal digits are accepted
- `0x` prefixes inside byte literals are not canonical
- an odd number of hexadecimal digits is a compile-time error
- invalid hexadecimal characters are compile-time errors
- indexing a `bytes` value yields `u8`
- slicing a `bytes` value yields `bytes`
- `len(...)` applies to `bytes`
- implicit conversion between `text` and `bytes` is not allowed

## Stdlib Bytes API

Accepted signatures in direction form:

```zt
bytes.empty() -> bytes
bytes.from_list(values: list<u8>) -> bytes
bytes.to_list(data: bytes) -> list<u8>
bytes.join(parts: list<bytes>) -> bytes
bytes.starts_with(data: bytes, prefix: bytes) -> bool
bytes.ends_with(data: bytes, suffix: bytes) -> bool
bytes.contains(data: bytes, part: bytes) -> bool
```

Examples:

```zt
import std.bytes as bytes

const empty_data: bytes = bytes.empty()
const raw: bytes = bytes.from_list([1, 2, 3])
const copied: list<u8> = bytes.to_list(raw)
const merged: bytes = bytes.join([raw, hex bytes "04 05"])
const has_header: bool = bytes.starts_with(merged, hex bytes "01 02")
```

`bytes.from_list(...)` copies the list contents into an immutable `bytes` value.

`bytes.to_list(...)` returns a new list so later list mutation cannot mutate the original `bytes`.

## Text And Bytes Conversion

UTF-8 conversion belongs to `std.text`, because it is a text encoding boundary.

Accepted API direction:

```zt
text.to_utf8(value: text) -> bytes
text.from_utf8(data: bytes) -> result<text, text.Error>
```

Rules:

- `text.to_utf8(...)` is infallible because Zenith `text` is valid UTF-8
- `text.from_utf8(...)` is fallible because arbitrary bytes may not be valid UTF-8
- `text.Error` is introduced as the typed error family for text decoding failures
- no implicit `text -> bytes` conversion exists
- no implicit `bytes -> text` conversion exists

Example:

```zt
import std.text as text

const payload: bytes = text.to_utf8("Hello")
const decoded: text = text.from_utf8(payload)?
```

## Rationale

A dedicated `std.bytes` module gives binary APIs a clear home without pretending that `bytes` is just `list<u8>`.

The `hex bytes "..."` literal is explicit enough for readability and avoids overloading ordinary text literals with hidden encoding behavior.

Using whitespace and underscore as separators keeps long binary constants readable without introducing suffix-heavy or symbol-heavy syntax.

Keeping text encoding in `std.text` makes the boundary visible: bytes are raw data, text is valid UTF-8.

## Non-Canonical Forms

Implicit conversion from text to bytes:

```zt
const payload: bytes = "Hello"
```

Implicit conversion from bytes to text:

```zt
const label: text = payload
```

C-style escaped byte text as the canonical byte literal:

```zt
const payload: bytes = b"\xDE\xAD\xBE\xEF"
```

Per-byte `0x` prefixes inside the byte literal:

```zt
const payload: bytes = hex bytes "0xDE 0xAD 0xBE 0xEF"
```

Treating `bytes` as publicly equivalent to `list<u8>`:

```zt
const payload: list<u8> = hex bytes "DE AD"
```

## Out of Scope

This decision does not yet define:

- mutable byte buffers
- byte builders
- binary streams
- `bytes.from_hex(...)` runtime parsing
- `bytes.to_hex(...)` formatting
- base64 helpers
- compression helpers
- cryptographic hashing helpers
