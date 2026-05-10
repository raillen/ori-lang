# Decision 036 - Text Concatenation And Interpolation

- Status: accepted
- Date: 2026-04-17
- Type: language / syntax / semantics
- Scope: `text`, concatenation, interpolation, formatting, escapes, `TextRepresentable`

## Summary

Zenith uses `text` as the canonical UTF-8 text type.

Text concatenation uses `+`, but only for `text + text`.

Text interpolation uses the explicit `fmt "..."` prefix.

The future `format` namespace remains available for stdlib formatting APIs such as `format.number(...)`, `format.date(...)` and `format.currency(...)`.

## Decision

Canonical concatenation:

```zt
const full_name: text = first_name + " " + last_name
```

Canonical interpolation:

```zt
const message: text = fmt "Player {player.name} has {player.hp} HP"
```

Canonical multiline interpolation:

```zt
const message: text = fmt """
Player: {player.name}
HP: {player.hp}
"""
```

Normative rules for this cut:

- the canonical text type is `text`
- `text` is UTF-8
- ordinary text literals do not interpolate
- interpolation requires `fmt`
- `+` concatenates only `text + text`
- `+` does not perform implicit conversion to `text`
- expressions inside `{...}` in `fmt` are ordinary Zenith expressions
- function calls are allowed inside interpolation expressions
- arithmetic and other ordinary expressions are allowed inside interpolation expressions
- each interpolated expression must implement `TextRepresentable<T>`
- contextual conversion to `text` through `TextRepresentable<T>` exists inside `fmt`
- outside `fmt`, conversion to `text` remains explicit
- postfix `?` inside interpolation expressions is not part of the MVP
- advanced numeric/date/currency formatting uses future stdlib functions, not a mini format-spec language
- `format` is reserved as the likely future stdlib namespace for formatting APIs

## Concatenation

Valid:

```zt
const full_name: text = first_name + " " + last_name
```

Valid when the function returns `text`:

```zt
const message: text = "Hello, " + get_name()
```

Invalid:

```zt
const message: text = "HP: " + hp
```

because `hp` is not `text`.

Explicit conversion is required outside `fmt`:

```zt
const message: text = "HP: " + to_text(hp)
```

or interpolation should be used:

```zt
const message: text = fmt "HP: {hp}"
```

## Interpolation

`fmt` marks a text literal as interpolated:

```zt
const message: text = fmt "Hello, {name}"
```

Expressions inside braces are ordinary Zenith expressions:

```zt
const message: text = fmt "Total: {price * quantity}"
```

Function calls are allowed:

```zt
const message: text = fmt "Name: {normalize_name(user.name)}"
```

Field access is allowed:

```zt
const message: text = fmt "{player.name}: {player.hp} HP"
```

Evaluation order is left to right following Decision 035:

```zt
const message: text = fmt "{a()} {b()} {c()}"
```

Order:

1. `a()`
2. `b()`
3. `c()`

## TextRepresentable

Inside `fmt`, each expression is converted using `TextRepresentable<T>`.

This is valid because `int` implements `TextRepresentable<int>`:

```zt
const hp: int = 100
const message: text = fmt "HP: {hp}"
```

This is invalid unless `User` implements `TextRepresentable<User>`:

```zt
const message: text = fmt "User: {user}"
```

The explicit trait requirement keeps formatting extensible without allowing arbitrary implicit conversion everywhere.

## Advanced Formatting

Zenith does not define a compact mini-language such as:

```zt
fmt "Price: {price:.2}"
```

in the MVP.

Instead, advanced formatting uses explicit function calls inside interpolation:

```zt
const message: text = fmt "Total: {format.number(total, decimals: 2)}"
```

Future examples:

```zt
const date_text: text = format.date(today, pattern: "yyyy-mm-dd")
const message: text = fmt "Today: {date_text}"
```

```zt
const message: text = fmt "Price: {format.currency(price, locale: "pt-BR")}"
```

Parser implementation may require escaping nested quotes in some forms:

```zt
const message: text = fmt "Today: {format.date(today, pattern: \"yyyy-mm-dd\")}"
```

The design goal is that formatting behavior stays visible as ordinary function calls rather than being hidden in symbolic format specs.

## Multiline Text

Ordinary multiline text uses triple quotes:

```zt
const page: text = """
Hello
World
"""
```

Interpolated multiline text uses `fmt` plus triple quotes:

```zt
const page: text = fmt """
Hello, {name}

HP: {hp}
"""
```

No automatic indentation trimming is part of the MVP.

The literal content is preserved according to the lexer rules.

## Escapes

The MVP supports common escapes in ordinary and interpolated text:

- `\n`
- `\r`
- `\t`
- `\"`
- `\\`

Inside `fmt`, literal braces are escaped:

- `\{`
- `\}`

Unicode escapes such as `\u{...}` may be added later. UTF-8 source text is already allowed directly.

## Performance

`text + text` produces a new `text`.

```zt
const full: text = a + b + c
```

is semantically left-associative according to ordinary expression rules:

```zt
const temp: text = a + b
const full: text = temp + c
```

The compiler/runtime may optimize repeated concatenation into a builder internally, but the observable result must remain the same.

For many pieces or mixed value formatting, `fmt` is preferred:

```zt
const message: text = fmt "{name} has {hp} HP"
```

## Non-Canonical Forms

Implicit conversion in concatenation is invalid:

```zt
const message: text = "HP: " + hp
```

Ordinary strings do not interpolate:

```zt
const message: text = "HP: {hp}"
```

This is literal text containing braces, not interpolation.

Symbol-prefixed interpolation is not canonical:

```zt
const message: text = $"HP: {hp}"
```

The `format "..."` prefix is not canonical because `format` is reserved for future stdlib formatting APIs:

```zt
const message: text = format "HP: {hp}"
```

Mini format specs are not part of the MVP:

```zt
const message: text = fmt "Total: {total:.2}"
```

Postfix `?` inside interpolation is not part of the MVP:

```zt
const message: text = fmt "User: {load_user(id)?}"
```

Use an explicit statement first:

```zt
const user: User = load_user(id)?
const message: text = fmt "User: {user}"
```

## Diagnostics

Expected diagnostic directions:

```text
operator + cannot concatenate text and int
use fmt interpolation or convert the value to text explicitly
```

```text
type User does not implement TextRepresentable<User>
cannot interpolate value of type User
```

```text
ordinary text literals do not interpolate
use fmt "..." for interpolation
```

```text
? inside interpolation is not supported in this implementation cut
move the expression into a const or var declaration
```

## Rationale

Using `+` for `text + text` preserves familiarity for the simplest case.

Rejecting implicit conversion in `+` keeps ordinary expressions predictable.

Using `fmt` makes interpolation visible without using a small symbolic prefix such as `$`.

Using `TextRepresentable<T>` inside `fmt` gives a controlled and contextual conversion path.

Leaving `format` available for stdlib avoids conflict between syntax and future formatting APIs.

Avoiding mini format specs keeps formatting readable and compositional through ordinary function calls.

## Out of Scope

This decision does not yet define:

- final `format.*` stdlib APIs
- locale-aware formatting
- date/time formatting types
- currency formatting
- pluralization
- raw string literals
- indentation trimming for multiline text
- Unicode escape syntax
- compile-time validation of all format expressions

