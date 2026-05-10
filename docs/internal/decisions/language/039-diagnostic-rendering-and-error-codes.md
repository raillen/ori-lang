# Decision 039 - Diagnostic Rendering And Error Codes

- Status: accepted
- Date: 2026-04-17
- Type: tooling / UX / diagnostics
- Scope: terminal diagnostics, stable diagnostic codes, warning policy, ignored result/optional, future JSON/LSP output

## Summary

Zenith diagnostics use a detailed, accessible terminal format by default.

The format favors plain labels, stable codes, small code excerpts and actionable help.

A short format may be added later for CI and scripts.

Ignoring `result` values is an error. Ignoring `optional` values as standalone statements is also an error in the MVP, because both cases hide meaningful control-flow intent.

## Decision

Canonical terminal diagnostic:

```text
error[type.mismatch]
Expected int, but found text.

where
  src/app/main.zt:8:21

code
  8 | const age: int = "18"
    |                  ^^^^
    |                  this is text

help
  Convert the value explicitly or change the variable type.
```

Normative rules for this cut:

- detailed diagnostics are the default terminal format
- short one-line diagnostics may be added later for CI/scripts
- diagnostics must not depend on color for meaning
- the header uses `severity[stable.code]`
- section labels are plain words such as `where`, `code`, `help`, `note`
- diagnostic codes must be stable enough for tests, docs and IDEs
- `help` appears only when it is useful
- warnings are conservative, sparse and actionable
- errors in cascade should be reduced when possible
- runtime diagnostics follow the same visual philosophy
- diagnostics should be generated from a structured model that can later render as JSON/LSP
- ignoring `result` is an error
- ignoring `optional` as a standalone statement is an error in the MVP

## Header

The diagnostic header is:

```text
error[type.mismatch]
```

or:

```text
warning[unused.value]
```

or:

```text
note[generic.candidate]
```

This is preferred over visually heavy banners or box drawing.

The header must be readable without terminal color.

## Sections

Canonical section labels:

- `where`
- `code`
- `help`
- `note`
- `opened here`
- `related`
- `value`
- `expected`
- `found`
- `what happened`

Labels are lowercase plain words for visual stability.

## Detailed Format

Type mismatch:

```text
error[type.mismatch]
Expected int, but found text.

where
  src/app/main.zt:8:21

code
  8 | const age: int = "18"
    |                  ^^^^
    |                  this is text

help
  Convert the value explicitly or change the variable type.
```

Missing `end`:

```text
error[syntax.expected_end]
Expected end to close this func block.

where
  src/app/main.zt:14:1

opened here
  src/app/main.zt:3:1

help
  Add end after the function body.
```

Generic inference failure:

```text
error[generic.inference_failed]
Cannot infer generic parameter Item for call to empty.

where
  src/app/main.zt:7:28

help
  Write the type explicitly, for example: empty<int>()
```

## Short Format

A future short format may be:

```text
src/app/main.zt:8:21 error[type.mismatch] Expected int, but found text.
```

This is intended for CI, scripts and compact logs.

Detailed format remains the default for interactive development.

## Stable Code Categories

Initial code category direction:

- `syntax.*`
- `name.*`
- `type.*`
- `trait.*`
- `generic.*`
- `mutability.*`
- `result.*`
- `optional.*`
- `control_flow.*`
- `project.*`
- `runtime.*`
- `internal.*`

Example codes:

- `syntax.expected_token`
- `syntax.expected_end`
- `name.unresolved`
- `name.duplicate`
- `type.mismatch`
- `type.not_callable`
- `trait.missing_impl`
- `generic.inference_failed`
- `mutability.const_update`
- `mutability.deep_index_update`
- `result.ignored`
- `optional.ignored`
- `project.import_missing`
- `project.import_cycle`
- `runtime.contract`
- `runtime.bounds`
- `runtime.map_key`

The exact catalog can grow, but codes should not churn casually.

## Help Text

`help` is optional.

Good help:

```text
help
  Use fmt "..." for interpolation.
```

Bad help:

```text
help
  Fix the error.
```

A help message should appear only when it gives a concrete next step.

## Warnings

Warnings must be:

- sparse
- actionable
- low-noise
- likely to prevent bugs
- not based only on subjective style

Initial warning candidates:

- unused local `const` or `var`
- unreachable code
- deprecated symbol in the future
- computed value ignored when the type is harmless but likely accidental

Warnings should not enforce broad style preferences such as function length in the compiler core.

Style-only checks belong to optional lint tooling if they exist.

## Ignored Result

A standalone `result<T, E>` expression is an error.

Invalid:

```zt
load_user(id)
```

if `load_user(id)` returns `result<User, text>`.

Diagnostic:

```text
error[result.ignored]
This result value must be handled.

where
  src/app/main.zt:12:5

code
  12 | load_user(id)
     | ^^^^^^^^^^^^^

help
  Use ?, match, attempt/rescue, or assign it to a named value.
```

Rationale:

- `result` represents expected recoverable failure
- ignoring it discards the failure path
- this hides control flow and violates explicitness

## Ignored Optional

A standalone `optional<T>` expression is an error in the MVP.

Invalid:

```zt
find_user(id)
```

if `find_user(id)` returns `optional<User>`.

Diagnostic:

```text
error[optional.ignored]
This optional value must be handled.

where
  src/app/main.zt:12:5

code
  12 | find_user(id)
     | ^^^^^^^^^^^^^

help
  Use match, ?, or assign it to a named value.
```

Rationale:

- `optional` represents meaningful absence
- ignoring it usually means the program forgot a branch
- explicit handling keeps reader intent clear

## Runtime Diagnostics

Runtime contract failure:

```text
error[runtime.contract]
Contract failed.

what happened
  Player.hp must satisfy: it >= 0

value
  -1

where
  src/app/player.zt:3:13

code
  3 | hp: int where it >= 0
    |               ^^^^^^^

help
  Ensure hp is not negative before constructing Player.
```

Bounds failure:

```text
error[runtime.bounds]
List index out of bounds.

where
  src/app/main.zt:10:25

value
  index: 99
  length: 3
```

Missing map key:

```text
error[runtime.map_key]
Map key was not found.

where
  src/app/main.zt:10:25
```

Runtime diagnostics should include source location and relevant values when available.

## Cascade Control

The compiler should avoid flooding the user.

Guidelines:

- prefer one precise root error over many derived errors
- suppress type errors caused only by unresolved names
- recover at block boundaries when possible
- attach related spans as notes instead of separate unrelated errors
- limit repeated identical diagnostics
- keep parser recovery conservative when continuation would mislead

## JSON/LSP Future

Diagnostics should be stored in a structured internal representation.

Required future mapping fields:

- severity
- code
- message
- file
- span
- related spans
- notes
- help
- machine-readable metadata

Terminal rendering, JSON output and LSP diagnostics should be different views of the same data.

## Non-Canonical Forms

Overly terse output is not canonical:

```text
type error
```

Color-only meaning is not canonical:

```text
red text without severity/code in plain text
```

Internal implementation leaks are not canonical:

```text
HIR lowering failed: invalid node tag 37
```

If an internal failure reaches the user, it should be reported as a compiler bug with a human-readable message.

Noisy subjective warnings are not compiler-core diagnostics:

```text
warning[style.function_too_long]
This function is too long.
```

## Rationale

Diagnostics are part of Zenith's accessibility model.

A clear diagnostic lowers cognitive load, teaches the language and makes tooling more trustworthy.

Stable codes support tests, documentation, search, ZDoc linking and IDE features.

Detailed-by-default output serves local development, while a future short mode can serve automation.

## Out of Scope

This decision does not yet define:

- exact CLI flags for diagnostic modes
- JSON schema
- LSP mapping implementation
- localization
- full diagnostic code catalog
- warning configuration file
- quick-fix protocol
- ZDoc diagnostic link format
