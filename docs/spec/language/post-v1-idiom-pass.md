# Zenith Post-v1 Idiom Reference Pass (Wave 7.3)

> Audience: contributor, maintainer, language designer
> Status: closed
> Surface: spec
> Last updated: 2026-05-02

This document is Wave 7.3 consolidation artifact.
It defines canonical Zenith idioms that user-facing docs and examples must follow.

## Scope

Wave 7.3 covers idiom/reference consistency for:

- error handling;
- resource cleanup;
- traits and `any` usage;
- generic APIs;
- concurrency usage;
- modules and stdlib boundaries.

Syntax-level decisions remain in `post-v1-syntax-freeze.md`.

## 1) Errors

Canonical idiom:

- use `result<T, E>` for recoverable errors;
- use `optional<T>` for absence;
- use `?`, `.or_return(...)`, `.or_wrap(...)` for explicit propagation;
- keep panic boundaries explicit; do not encode recoverable flow as panic.

Avoid:

- `try/catch` style examples;
- exception-oriented control flow language;
- hidden error conversion rules not specified in checker diagnostics.

## 2) Resource Cleanup

Canonical idiom:

- use `using` for deterministic cleanup at source level;
- cleanup semantics must stay explicit at control-flow boundaries.

Wave 7 note:

- boundary behavior under `return`, `?`, panic, loops, jobs/channels, and FFI re-entrancy is finalized in Wave 7.10.

## 3) Traits And `any`

Canonical idiom:

- use traits for behavior contracts;
- use `apply Trait to Type` for implementation;
- use `any<Trait>` for heterogeneous dynamic dispatch at API boundaries.

Avoid:

- new user-facing `dyn` spelling;
- documentation that treats `dyn` as preferred syntax.

Wave 7 note:

- migration policy for legacy `dyn` handling is finalized in Wave 7.4;
- exact any-safe trait shape and ABI boundaries are finalized in Wave 7.11 + 7.17.

## 4) Generics

Canonical idiom:

- prefer explicit generic signatures and explicit local types;
- allow argument-position generic inference where supported;
- keep generic API contracts explicit in function signatures and constraints.

Avoid:

- examples relying on full local type inference (`const x = 42`);
- examples implying return-context generic inference or partial inference.

Wave 7 note:

- executable monomorphization behavior is finalized in Wave 7.7 and 7.8.

## 5) Concurrency

Canonical idiom:

- use jobs/channels/shared/atomic as explicit concurrency primitives;
- keep producer/consumer and cancellation semantics explicit in API design;
- treat async route as jobs + channels, not syntax-level `async/await`.

Avoid:

- docs implying hidden scheduler magic;
- docs implying syntax-level async functions.

Wave 7 note:

- close/capacity/backpressure/cancellation/panic semantics are finalized in Wave 7.11.

## 6) Modules And Stdlib

Canonical idiom:

- use qualified imports as default reading model;
- keep stdlib examples explicit and namespace-first;
- keep language foundation modules distinct from package-level ecosystem modules.

Avoid:

- selective import examples as canonical style;
- docs that blur language foundation vs package registry boundaries.

Wave 7 note:

- stdlib boundary finalization lands in Wave 7.1.31 matrix item and Wave 11 policy work.

## 7) Reference Consistency Checklist

Pass checklist for user-facing docs:

- no stale `dyn` preferred spelling;
- no historical default-case spelling as canonical fallback;
- no accepted claim for struct type omission shorthand `{ fields }`;
- no examples relying on rejected full local type inference;
- closure examples align with frozen closure shorthand + explicit binding type rules;
- examples and diagnostics use same canonical wording.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave status and sequencing.
- `post-v1-closure-matrix.md` - operational tracker.
- `post-v1-syntax-freeze.md` - frozen syntax/keyword decisions.
- `language-reference.md` - implementation-facing language reference.
- `post-v1-surface-contract.md` - canonical direction and ordering.
