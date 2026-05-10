# Zenith Formatter Model Spec

- Status: canonical closure spec
- Date: 2026-04-18
- Scope: mandatory source formatting for reading-first Zenith code

## Purpose

The formatter is part of the language, not optional polish.

Zenith's reading-first design depends on stable layout for blocks, `where`, named arguments, attrs, imports, match cases and long declarations.

## Command

Canonical command:

```text
zt fmt
```

Required modes:

- format project
- format a specific file
- check formatting without writing

## Core Rules

- indentation uses 4 spaces
- formatting is deterministic
- formatting is idempotent
- broad style configuration is not MVP
- comments are preserved when possible
- formatter must not change semantics
- generated C is not formatted by the Zenith formatter

## File Shape

Canonical order:

1. leading comments if any
2. `namespace`
3. imports
4. declarations

Imports are sorted lexically by namespace path unless later style rules define grouping.

## Blocks

All block forms align their closing `end` with the opening keyword.

Canonical:

```zt
if ready
    start()
else if waiting
    queue()
else
    stop()
end
```

## Function Signatures

Short signatures may stay on one line.

Long signatures use multiline parameters:

```zt
func create_user(
    name: text where validate.not_empty(it),
    age: int where validate.between(it, min: 0, max: 130),
    email: text where validate.email(it)
) -> result<User, UserError>
    ...
end
```

Formatter must prefer multiline when:

- there are multiple parameters with `where`
- line length would exceed the configured hard target
- named defaults make the signature visually dense

## Named Arguments

Long named calls use one argument per line:

```zt
const user: User = create_user(
    name: "Julia",
    age: 18,
    email: "julia@example.com"
)?
```

## Structs And Enums

Struct fields use one field per line.

Inline enum form is allowed only when short and visually clear.

Long or payload enums use block form.

## Match

Each `case` is indented one level under its `match` header. `end` closes the outer `match` and aligns with the `match` keyword.

Canonical:

```zt
match result
    case success(value) ->
        return value
    case error(message) ->
        return 0
end
```

Case body uses `->` (not `:`) and is indented under its `case`. Arms do not emit their own `end`; only the outer `match` closes with `end`.

## Attributes

One `attr` per line.

No blank line between an attribute and its declaration.

## Comments

Line comments use `--`.

Block comments use `--- ... ---`.

Formatter preserves comments, but long public documentation belongs in ZDoc.

## Definition Of Done

Formatter is complete only when:

- `zt fmt` exists
- formatter is idempotent
- golden tests cover namespace/imports, where, named args, structs, enums, match, attrs and comments
- parser can consume formatter output without semantic change
- conformance can require formatted canonical examples

## Idempotence Gate

Idempotence is enforced by an automated gate, not only by review:

- runner: `tests/formatter/run_formatter_idempotence.py`
- property: `fmt(fmt(x)) == fmt(x)` for every case under `tests/formatter/cases/`
- integrated into `run_suite.py` as part of the `pr_gate` tooling suite
- failure mode: non-zero exit with a unified diff of `fmt^1` vs `fmt^2`
- status (2026-04-26): all 10 canonical cases converge; no `XFAIL` bucket remains
