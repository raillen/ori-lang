# Zenith Wave 7.13: Pattern Matching Closure

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: audit artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

This document closes Wave 7.13 pattern matching semantics for exhaustiveness, guards, destructuring, enum payloads, multi-value matches, and diagnostics.

## Scope

This closure covers:
- scalar/control-flow `match`;
- enum variant matching and payload binding;
- optional value matching;
- guards with `if`;
- multi-value match expressions;
- non-exhaustive diagnostics.

## Decisions

### P1: Canonical Syntax

`else` is the canonical default arm.
`default` is not the post-v1 canonical spelling.

Guards use `if`:
- `case pattern if condition`

The guard expression must be `bool`.

### P2: Exhaustiveness

Enum matches without `else` must cover all enum variants.
Non-exhaustive enum matches are semantic errors.

Matches over scalar values may use `else` to make fallback behavior explicit.

### P3: Enum Payloads

Payload patterns bind variant fields in source order.
Bindings are local to the arm body and must not leak to sibling arms or the outer scope.

### P4: Optional Matching

Optional matching is modeled as matching `some(value)` or `none`.
This is the canonical alternative to rejected `?.` safe navigation.

### P5: Multi-Value Matching

Multi-value matching matches tuple-shaped values positionally.
All arm patterns must be compatible with the matched arity and element types.

### P6: Diagnostics

Pattern diagnostics must identify:
- non-exhaustive enum variants;
- guard expression type mismatch;
- multi-value arity/type mismatch;
- unreachable or duplicate arms where the checker can prove them.

## Validation Envelope

Minimum validation set:
- `tests/behavior/control_flow_match`
- `tests/behavior/enum_match`
- `tests/behavior/enum_match_non_exhaustive_error`
- `tests/behavior/match_guard_basic`
- `tests/behavior/match_guard_non_bool_error`
- `tests/behavior/multivalue_match_basic`
- `tests/behavior/multivalue_match_type_error`
- `tests/behavior/optional_match_value`

## Closure Result

Wave 7.13 is closed for the implemented pattern-matching subset.
Further expansion must preserve these invariants and add fixtures before being treated as conformant.

## Relationship To Other Documents

- `post-v1-syntax-freeze.md`
- `docs/internal/decisions/language/010-structs-traits-apply-enums-and-match.md`
- `post-v1-error-model-closure.md`
