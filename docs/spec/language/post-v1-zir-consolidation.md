# Zenith Wave 7.14: ZIR Consolidation

> Audience: compiler/runtime implementer, backend implementer  
> Status: implementation design artifact  
> Surface: compiler contract  
> Last updated: 2026-05-03

## Purpose

This document closes the Wave 7.14 ZIR consolidation contract: canonical type model, ownership/runtime operations, verifier invariants, golden fixtures, and textual dump stability.

## Scope

This closure covers:
- in-memory ZIR as the canonical compiler IR;
- textual ZIR as debug/golden artifact;
- backend-visible type shapes;
- ownership and runtime operation representation;
- verifier invariants;
- conformance fixture expectations.

## Decisions

### Z1: Canonical Representation

The canonical ZIR representation is the structured in-memory model in `compiler/zir/model.h`.
Textual `.zir` is secondary and exists for debugging, fixtures, and golden tests.

This follows `docs/internal/decisions/language/003-zir-structured-internals.md`.

### Z2: Type Model

Backend-visible ZIR types must preserve:
- primitive scalar widths and signedness;
- `text`, `bytes`, `list`, `map`, `set`, `tuple`;
- `optional<T>` and `result<T,E>`;
- user structs/enums with qualified names;
- callable and closure ABI shapes;
- `any<Trait>` as canonical spelling;
- generic monomorphized instance identity.

### Z3: Ownership and Runtime Ops

ZIR must make runtime-sensitive operations visible enough for verifier and backend agreement:
- retain/release/drop for managed values;
- move/sink boundaries;
- cleanup stack entry/exit effects;
- runtime calls for collections, jobs, channels, lazy, net/time, and FFI.

### Z4: Verifier Invariants

The verifier must reject:
- unknown types and target-only leaked type strings;
- references to undeclared locals/functions where resolvable;
- invalid terminator targets;
- malformed metadata/source spans;
- runtime operations with incompatible argument/return shapes;
- non-canonical dynamic dispatch spelling in user-visible dumps.

### Z5: Textual Stability

Textual ZIR dumps must be stable enough for golden fixtures:
- deterministic function/block ordering;
- deterministic type spelling;
- stable source span form;
- no backend-specific C names unless the operation is explicitly an extern/runtime call.

### Z6: Generic Representation

Executable generics are represented after monomorphization for backend consumption.
Future template-level ZIR may exist for analysis, but C oracle conformance uses instantiated concrete ZIR.

## Validation Envelope

Minimum validation set:
- `python build.py`
- `tests/zir/test_verifier.c`
- existing `zt emit-zir` or verifier smoke paths used by the driver
- generic monomorphization fixtures from Wave 7.7
- `any` dispatch fixtures from Wave 7.5
- cleanup fixtures from Wave 7.10

## Closure Result

Wave 7.14 is closed as a compiler contract.
Full implementation remains incremental, but no core ZIR architecture question remains open for backend planning.

## Relationship To Other Documents

- `docs/internal/decisions/language/003-zir-structured-internals.md`
- `compiler/zir/ZIR_MODEL_MAP.md`
- `compiler/zir/ZIR_VERIFIER_MAP.md`
- `post-v1-backend-conformance-suite.md`
- `post-v1-runtime-abi-ownership-audit.md`
