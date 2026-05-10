# Zenith Wave 7.17: Diagnostic Contract Pass

> Audience: compiler maintainer, tooling implementer, language designer  
> Status: audit artifact  
> Surface: diagnostics contract  
> Last updated: 2026-05-03

## Purpose

This document closes Wave 7.17 diagnostic contract expectations for language-closure features.

## Scope

This closure covers:
- stable diagnostic codes;
- ACTION/WHY/NEXT style;
- source spans;
- severity/stage consistency;
- closure feature negative fixtures.

## Decisions

### D1: Stable Codes

Every language-closure diagnostic must have a stable code.
Codes should identify the semantic area first, such as:
- `type.*`;
- `callable.*`;
- `concurrency.*`;
- `any.*`;
- `control_flow.*`;
- `mutability.*`.

### D2: ACTION/WHY/NEXT

Human-facing diagnostics should answer:
- ACTION: what failed;
- WHY: why the compiler rejects it;
- NEXT: what the user should change or rerun.

The compact `--ci` form may preserve the same content as fields/fragments.

### D3: Span First

Diagnostics should point to the smallest useful source span.
If the exact token is unavailable, point to the nearest expression/declaration that caused the error.

### D4: Closure Negative Fixtures

Each closed feature with rejection rules must have at least one negative fixture or hardening assertion.
Required areas:
- syntax freeze rejected forms;
- generic inference failure/conflict;
- monomorphization limits;
- callable escape and signature errors;
- `any` safety errors;
- concurrency transferability errors;
- optional/result propagation misuse;
- pattern non-exhaustiveness and guard type errors;
- mutability and ownership errors.

### D5: Tooling Consumption

Diagnostics should remain machine-consumable for LSP and CI:
- code and span must be parseable;
- severity must be stable;
- `--ci` output must avoid localization-only ambiguity.

## Validation Envelope

Minimum validation set:
- invalid rows in `tests/behavior/MATRIX.md`;
- callable negative fixtures;
- generic negative fixtures;
- concurrency transferability negative fixtures;
- pattern matching negative fixtures;
- optional/result negative fixtures.

## Closure Result

Wave 7.17 is closed as a diagnostic contract.
Remaining work is coverage expansion and consistency fixes found by audits.

## Relationship To Other Documents

- `post-v1-source-mapping-contract.md`
- `post-v1-backend-conformance-suite.md`
- `compiler/semantic/diagnostics/diagnostics.c`
