# Decision 038 - Diagnostics And Warning Philosophy

- Status: accepted
- Date: 2026-04-17
- Type: language / tooling / UX
- Scope: compiler diagnostics, warnings, runtime panic messages, accessibility, LSP/IDE output

## Summary

Zenith diagnostics must follow the same philosophy as the language: reading-first, explicit, predictable and accessible.

Errors, warnings and runtime failure messages should be clear enough for beginners, precise enough for experienced programmers and structured enough for tools.

Diagnostics are part of the language experience, not secondary polish.

## Decision

Canonical diagnostic qualities:

- explicit
- human-readable
- actionable
- visually stable
- low noise
- structured for IDE/LSP
- consistent across compiler stages
- aligned with TDAH/dyslexia-friendly design

Diagnostics should explain:

- what happened
- where it happened
- why it is invalid
- how to fix it when a clear fix exists

## Canonical Shape

Preferred terminal shape:

```text
error[type.mismatch]:
  Expected int, but found text.

where:
  src/app/main.zt:8:21

code:
  const age: int = "18"
                   ^^^^

help:
  Convert the value explicitly or change the variable type.
```

Preferred runtime shape:

```text
error[runtime.contract]:
  Contract failed.

what happened:
  Player.hp must satisfy: it >= 0

value:
  -1

where:
  src/app/player.zt:3:13

help:
  Ensure hp is not negative before constructing Player.
```

The exact renderer may differ between CLI, IDE and tests, but the information model should remain stable.

## Diagnostic Information Model

Each diagnostic should carry:

- severity
- stable code
- stage/source
- primary message
- source location when available
- highlighted span when available
- optional related spans
- optional notes
- optional help/suggestion
- machine-readable metadata when useful

Severity levels:

- `error`
- `warning`
- `note`
- `help`

Example stable codes:

- `syntax.expected_token`
- `name.unresolved`
- `type.mismatch`
- `mutability.const_update`
- `generic.inference_failed`
- `runtime.contract`
- `runtime.bounds`
- `runtime.map_key`

## Language And Tone

Diagnostics should use plain language.

Prefer:

```text
Expected int, but found text.
```

Avoid:

```text
Type unification failed for node 0x7ff...
```

Prefer direct action:

```text
Use fmt "..." for interpolation.
```

Avoid vague advice:

```text
Invalid expression.
```

Messages should avoid blame.

Prefer:

```text
This value cannot be assigned to hp because hp requires it >= 0.
```

Avoid:

```text
You passed an invalid value.
```

## Accessibility Rules

Diagnostics should be designed for scanning.

Rules:

- short headings
- stable labels
- no dense symbolic output when words are clearer
- avoid overly long paragraphs
- keep code excerpts small
- point to the exact span
- use consistent indentation
- avoid color as the only carrier of meaning
- avoid excessive jargon
- avoid multiple unrelated errors when one root cause explains them
- suppress cascaded errors when possible

## Warnings

Warnings must be sparse and actionable.

A warning should exist only when:

- behavior is likely unintended
- the fix is clear
- the warning does not require subjective style preference alone
- it helps prevent bugs or confusion

Warnings should not become background noise.

Example:

```text
warning[unused.value]:
  This value is computed but never used.

where:
  src/app/main.zt:12:5

help:
  Remove it or assign it to a named const/var.
```

Future tooling may allow warning configuration, but default warnings should remain conservative.

## Parser Diagnostics

Parser diagnostics should state what was expected and what was found.

Example:

```text
error[syntax.expected_end]:
  Expected end to close this func block.

where:
  src/app/main.zt:14:1

opened here:
  src/app/main.zt:3:1

help:
  Add end after the function body.
```

This is especially important because explicit `end` is part of Zenith's visual stability philosophy.

## Type Diagnostics

Type diagnostics should name both expected and actual types.

Example:

```text
error[type.mismatch]:
  Expected list<int>, but found list<text>.

where:
  src/app/main.zt:9:24
```

If generics are involved, diagnostics should name unresolved parameters:

```text
error[generic.inference_failed]:
  Cannot infer generic parameter Item for call to empty.

where:
  src/app/main.zt:7:28

help:
  Write the type explicitly, for example: empty<int>()
```

## Mutability Diagnostics

Mutability diagnostics should explain observable mutation.

Example:

```text
error[mutability.const_update]:
  Cannot update item of const list<Player>.

where:
  src/app/main.zt:10:5

help:
  Use var if the list should be changed.
```

Deep mutation diagnostic:

```text
error[mutability.deep_index_update]:
  Deep mutation through indexed access is not supported in this implementation cut.

where:
  src/app/main.zt:10:5

help:
  Use read-modify-write or update players[0] as player ... end.
```

## Runtime Diagnostics

Runtime diagnostics should be as explicit as compiler diagnostics when possible.

Contract failure:

```text
error[runtime.contract]:
  Contract failed.

what happened:
  Player.hp must satisfy: it >= 0

value:
  -1
```

Bounds failure:

```text
error[runtime.bounds]:
  List index out of bounds.

index:
  99

length:
  3
```

Missing map key:

```text
error[runtime.map_key]:
  Map key was not found.
```

Runtime messages should include source location when the compiler can provide it.

## Error Recovery

The compiler should continue after an error when doing so improves feedback without producing misleading cascades.

Guidelines:

- recover from syntax errors at block boundaries when possible
- avoid emitting many type errors caused by one missing declaration
- prefer one precise root error over many speculative errors
- keep related notes attached to the primary diagnostic

## IDE/LSP Requirements

Diagnostics should be structured so the IDE can show:

- squiggles
- hover explanation
- quick fixes
- related locations
- severity icons
- stable diagnostic codes
- documentation links

Human output and machine output should come from the same underlying diagnostic model.

## Non-Canonical Forms

Overly terse diagnostics are not acceptable:

```text
type error
```

Implementation-leaking diagnostics are not acceptable for normal users:

```text
HIR lowering failed: invalid node tag 37
```

If an internal diagnostic reaches the user, it should still be wrapped in a human-readable message and treated as a compiler bug when appropriate.

Warnings based only on subjective style are not canonical in the compiler core:

```text
warning: this function is too long
```

Such checks belong to optional lint tooling if they exist.

## Rationale

Zenith's accessibility goals require diagnostics to reduce cognitive load.

A good diagnostic is not just a failure report. It is part of the teaching loop.

Clear diagnostics also make the compiler easier to test, the IDE easier to build and the language easier to learn.

## Out of Scope

This decision does not yet define:

- exact CLI renderer implementation
- JSON diagnostic schema
- LSP mapping details
- localization/translation strategy
- warning configuration file format
- quick-fix implementation
- documentation link syntax for diagnostics
- complete diagnostic code catalog
