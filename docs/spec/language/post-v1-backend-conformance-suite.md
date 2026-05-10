# Zenith Wave 7.15: Backend Conformance Suite

> Audience: backend implementer, compiler maintainer, release engineer  
> Status: design session artifact  
> Surface: compiler/runtime contract  
> Last updated: 2026-05-03

## Purpose

This document defines the Wave 7.15 backend conformance suite that every future backend must pass before it is considered real.

## Scope

This closure covers:
- C backend as oracle;
- behavior fixture selection;
- negative diagnostic expectations;
- runtime ABI equivalence;
- accepted backend variance;
- activation gates for Zig, LLVM, WASM, or future targets.

## Decisions

### B1: C Oracle

The C backend is the canonical executable oracle for language behavior until another backend passes this suite.

Alternative backends may differ in generated code shape, optimization quality, and platform linkage, but observable Zenith behavior must match the C oracle.

### B2: Fixture Classes

Backend conformance includes:
- parser/check-only invalid fixtures;
- buildable positive fixtures;
- executable fixtures with expected exit code;
- runtime contract failure fixtures where applicable;
- textual/golden ZIR fixtures for IR stability;
- generated source mapping smoke checks.

### B3: Mandatory Feature Coverage

Minimum backend feature coverage:
- project model, namespace/imports, public declarations;
- scalar expressions and control flow;
- structs, enums, traits, apply, method dispatch;
- collections and managed value semantics;
- optional/result and `?` propagation;
- pattern matching;
- generic monomorphization;
- callable/closure subset;
- `any<Trait>` subset;
- `using` cleanup;
- jobs/channels current executable subset;
- FFI extern C subset;
- stdlib modules already accepted as compiler/runtime surface.

### B4: Diagnostics

Diagnostic text may have harmless formatting variance, but each backend pipeline must preserve:
- stable diagnostic code;
- source span;
- stage classification where available;
- actionable ACTION/WHY/NEXT content for closure features.

### B5: Accepted Variance

Accepted variance:
- different generated helper names;
- different temporary variable ordering when behavior and source spans are stable;
- different optimization choices;
- platform-specific runtime implementation details behind the same ABI.

Rejected variance:
- different program exit code;
- different language-visible panic/contract behavior;
- accepting invalid fixtures;
- rejecting valid closure fixtures;
- changing ownership/cleanup timing in a visible way.

## Validation Envelope

Minimum validation set:
- `tests/behavior/MATRIX.md` positive and invalid rows relevant to closure features;
- `docs/spec/language/conformance-matrix.md` as prior RC baseline;
- hardening tests for ORC, concurrency transferability, and monomorphization limits;
- ZIR verifier tests.

## Closure Result

Wave 7.15 is closed as a backend activation gate.
No non-C backend should be marked conformant until it can run this suite and document accepted variance.

## Relationship To Other Documents

- `docs/spec/language/conformance-matrix.md`
- `tests/behavior/MATRIX.md`
- `post-v1-zir-consolidation.md`
- `post-v1-runtime-abi-ownership-audit.md`
