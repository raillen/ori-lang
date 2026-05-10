# Decision 050 - Core And Stdlib Boundary

- Status: accepted
- Date: 2026-04-17
- Type: language / stdlib / architecture
- Scope: implicit core, explicit stdlib imports, testing boundary, formatting boundary and textual IO placement

## Summary

Zenith has a small implicit core and an explicit `std.*` library.

The user does not import `core`.

The user imports standard library modules explicitly.

`fmt "..."` is language/core syntax.

Textual IO, filesystem, formatting, validation, bytes helpers, process APIs and complementary test helpers belong to stdlib modules.

## Decision

Normative rules for this cut:

- `core.*` is implicit
- users do not write `import core...`
- `std.*` is not implicit
- users import stdlib modules explicitly
- core contains fundamental types, literals, operators, control flow, core traits and minimal builtins
- stdlib contains textual IO, filesystem, time, formatting, validation, JSON, bytes helpers, OS, process and complementary testing APIs
- `fmt "..."` is language/core syntax
- `format.*` belongs to stdlib
- `validate.*` belongs to stdlib
- `io.*` belongs to stdlib
- `check(...)` belongs to the language testing model, not to a general stdlib module
- `std.test` is complementary and provides helpers such as `fail(...)` and `skip(...)`
- no global/core `print` exists outside the explicit `std.io` module surface
- IO output functions do not add newline automatically
- newline must be explicit in the text
- `io.write` writes `text`
- `io.print` writes `TextRepresentable<Item>` values
- formatting parameters do not belong on `io.write`
- formatting belongs in `fmt` or `std.format`

## Core

Core includes fundamental types:

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
list<Item>
map<Key, Value>
optional<Item>
result<Success, Error>
```

Core includes literals and fundamental values:

```zt
true
false
none
success(...)
error(...)
hex bytes "..."
```

Core includes traits:

```zt
Equatable<Item>
Hashable<Item>
Comparable<Item>
TextRepresentable<Item>
```

Core includes ordering and minimal builtins/syntax:

```zt
Order
len(value)
panic(message)
fmt "..."
check(condition)
```

## Stdlib

Stdlib modules are explicit:

```zt
import std.io as io
import std.bytes as bytes
import std.format as format
import std.validate as validate
```

Example:

```zt
const message: text = fmt "Total: {format.number(total, decimals: 2)}"
io.write(message)
```

Stdlib contains APIs such as:

- `std.io`
- `std.bytes`
- `std.format`
- `std.validate`
- `std.text`
- `std.fs`
- `std.fs.path`
- `std.time`
- `std.json`
- `std.os`
- `std.os.process`
- `std.test`

The exact stdlib API is decided separately.

## Textual IO

Textual IO module direction:

```zt
import std.io as io

io.write("Hello")
io.print(42)
io.print(fmt "HP: {hp}\n")
```

Semantics:

- `io.write(value: text)` writes text exactly as provided
- `io.write` does not add newline
- `io.write` does not format arbitrary values
- `io.print<Item>(value: Item)` writes a `TextRepresentable<Item>`
- `io.print` does not add newline
- newline must be explicit
- formatting is done before calling IO output

## Tests

Test declaration belongs to the language through `attr test`.

The primary assertion form is `check(...)`.

Complementary helpers come from `std.test`.

Example direction:

```zt
import std.test as test

attr test
func calculates_total()
    if calculate_total(10, 3) != 30
        test.fail("expected 30")
    end
end
```

## Format And Validate

`fmt` is syntax:

```zt
const message: text = fmt "HP: {hp}"
```

Advanced formatting belongs to stdlib:

```zt
import std.format as format

const message: text = fmt "Total: {format.number(total, decimals: 2)}"
```

Validation predicates belong to stdlib:

```zt
import std.validate as validate

struct User
    age: int where validate.between(it, min: 0, max: 130)
end
```

## Rationale

A small core keeps the language predictable and makes hidden imports easier to avoid.

Keeping textual IO in `std.io` instead of the core preserves explicitness at the side-effect boundary.

Keeping `check(...)` in the language testing model while leaving `fail(...)` and `skip(...)` in `std.test` avoids splitting test declaration and assertion semantics across competing mechanisms.

## Non-Canonical Forms

Implicit stdlib use is not canonical:

```zt
io.write("Hello")
```

without:

```zt
import std.io as io
```

Global print is not core:

```zt
print("Hello")
```

Automatic newline is not assumed:

```zt
io.print("Hello")
-- does not add newline
```

Formatting parameters on `io.write` are not canonical:

```zt
io.write("Hello", color: "red")
```

## Out of Scope

This decision does not yet define:

- the exact implementation of `std.io`
- binary IO APIs
- any future richer prelude policy
