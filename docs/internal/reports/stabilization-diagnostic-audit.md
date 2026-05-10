# Phase 0 Diagnostic Audit

> Audience: maintainer
> Date: 2026-04-28
> Scope: action-first diagnostics and raw-internal leakage

Phase 0 requires diagnostics to be action-first and readable.

## Current Contract

Canonical diagnostics are defined by:

- `docs/spec/language/diagnostics-model.md`;
- `docs/spec/language/diagnostic-code-catalog.md`;
- `docs/reference/zenith-kb/diagnostics.md`.

The expected detailed shape is:

- `ACTION`: what the user can do next;
- `WHY`: why the compiler is reporting it;
- `NEXT`: the next concrete command or edit;
- `where`: source location;
- `code`: source snippet when available;
- `help`: short fix guidance.

The CI renderer keeps stable one-line diagnostics for automation.

## Covered Areas

Existing behavior and driver tests cover:

- syntax errors;
- unresolved names and imports;
- type mismatches;
- invalid calls and arguments;
- mutability errors;
- noncanonical syntax errors;
- runtime contract failures;
- stdlib test failure helpers;
- formatter and doc tooling errors.

## Audit Result

No known P0/P1 diagnostic blocker is recorded for this cut.

Known acceptable warnings:

- generated C may trigger native unused-parameter warnings;
- `--native-raw` remains the escape hatch for raw compiler output;
- style/readability warnings can appear in fixture-heavy test programs.

## Release Rule

New diagnostics must:

- use a stable code when the class is public;
- explain the user action first;
- avoid exposing HIR/ZIR/C internals in normal user-facing output;
- include a negative test or fixture fragment when practical.

