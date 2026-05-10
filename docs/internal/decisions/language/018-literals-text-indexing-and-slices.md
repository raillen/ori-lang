# Decision 018 - Literals Text Indexing And Slices

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: numeric literals, text literals, escapes, multiline text, indexing, slice syntax

## Summary

Zenith Next uses 0-based indexing for sequence access in user code.

Slice syntax uses inclusive end bounds with omitted-bound forms such as `value[start..]`, `value[..end]` and `value[..]`.

`text` is UTF-8 and text indexing returns `text`, not a separate `char` type.

The MVP supports decimal, hexadecimal and binary integer literals, decimal floating-point literals, numeric underscore separators, basic escapes, triple-quoted multiline text and explicit hex byte literals.

## Decision

The canonical literal and access forms in this cut are:

- decimal integer: `0`, `42`, `1_000_000`
- decimal float: `0.0`, `3.14`, `1.5e3`, `2.0e-4`
- hexadecimal integer: `0xFF`
- binary integer: `0b1010`
- text literal: `"Zenith"`
- multiline text literal: `"""..."""`
- byte literal: `hex bytes "DE AD BE EF"`
- indexing: `value[index]`
- slice: `value[start..end]`
- slice to end: `value[start..]`
- slice from start: `value[..end]`
- full slice: `value[..]`

Normative rules for this cut:

- sequence indexing in user code is 0-based
- `text[index]` and `list[index]` use 0-based indices
- `map[key]` remains key-based lookup and is not affected by index base rules
- indexing `text` yields `text`
- the MVP does not define a separate `char` type
- `text` is UTF-8 in the language surface
- `len(text)` counts code points in the MVP model
- text indexing and slicing operate on code points, not raw bytes
- slice syntax uses inclusive end bounds
- omitted slice start means the first valid sequence index
- omitted slice end means the last valid sequence index
- `value[..]` means the whole sequence or text value
- slice bounds, when present, must have type `int`
- indexing a `text` or `list` out of bounds is a runtime error
- a slice bound outside the valid range is a runtime error when written explicitly
- after omitted-bound normalization, a slice whose start is greater than its end yields an empty value of the same sequence kind
- text literals use double quotes
- single-quoted text literals are not part of the canonical syntax
- multiline text literals use triple double quotes
- multiline text literals preserve line breaks
- multiline text literals remove common indentation from the literal body
- the MVP escape set is `\\`, `\"`, `\n`, `\r`, `\t`, `\0` and `\u{...}`
- decimal numeric literals may use `_` as a visual separator between digits
- decimal floating-point literals may use scientific notation
- hexadecimal literals use the `0x` prefix
- binary literals use the `0b` prefix
- octal literals are not part of the MVP
- literal suffix syntax is not part of the MVP
- unary minus is an operator applied to a literal expression, not part of the literal token itself
- byte literals use the `hex bytes` prefix and produce immutable `bytes`
- byte literal content uses two hexadecimal digits per byte after whitespace and `_` separators are ignored
- byte literals do not use per-byte `0x` prefixes

## Canonical Examples

0-based indexing:

```zt
const first_letter: text = name[0]
const third_port: int = ports[2]
```

Inclusive slice:

```zt
const middle: text = name[2..4]
const tail: text = name[2..]
const prefix: text = name[..4]
const copy: text = name[..]
```

List slice:

```zt
const head: list<int> = values[..2]
const rest: list<int> = values[3..]
```

Decimal and scientific notation:

```zt
const retries: int = 1_000
const ratio: float = 3.5e-2
```

Hexadecimal and binary literals:

```zt
const mask: int = 0xFF
const flags: int = 0b1010_0001
```

Byte literal:

```zt
const header: bytes = hex bytes "DE AD BE EF"
```

Escaped text literal:

```zt
const message: text = "Line 1\nLine 2\tTabbed"
```

Multiline text literal:

```zt
const banner: text = """
    Hello
    Zenith
"""
```

## Rationale

Using 0-based indexing aligns the language with common expectations in modern programming environments and makes integration with many host APIs and native targets more direct.

Keeping inclusive slice ends preserves readability for human counting while omitted bounds remove the need for sentinel values such as `-1` in source code.

Treating text as UTF-8 while indexing by code point gives the language a user-facing text model that is meaningfully better than byte-oriented indexing.

Triple-quoted multiline text is easier to read than heredoc markers in the MVP and introduces less syntax surface than a dedicated text-block keyword.

Allowing decimal separators, scientific notation, hexadecimal and binary integer literals covers the practical numeric cases without introducing suffix-heavy syntax.

Using `hex bytes "..."` makes binary constants explicit without making ordinary text literals carry hidden encoding behavior.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

1-based indexing in user code:

```zt
const first_letter: text = name[1]
```

using a sentinel end bound in source syntax:

```zt
const tail: text = name[2..-1]
```

single-quoted text literals:

```zt
const name: text = 'Zenith'
```

octal literal syntax:

```zt
const mode: int = 0o755
```

literal suffix syntax:

```zt
const count: int32 = 10i32
```

C-style byte escape literals:

```zt
const header: bytes = b"\\xDE\\xAD\\xBE\\xEF"
```

heredoc syntax as the canonical multiline form:

```zt
const banner: text = <<<TEXT
Hello
Zenith
TEXT
```

## Out of Scope

This decision does not yet define:

- interpolation syntax
- raw string literals
- escape handling for invalid Unicode scalar values
- exact normalization behavior for newline conventions in multiline text
- implementation details of lowering UTF-8 code-point indexing into each backend
- future reconsideration of octal or literal suffix syntax
