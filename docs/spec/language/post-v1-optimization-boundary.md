# Zenith Wave 7.19: Optimization Boundary Definition

> Audience: compiler/backend implementer, language designer  
> Status: design session artifact  
> Surface: compiler contract  
> Last updated: 2026-05-03

## Purpose

This document closes Wave 7.19 by defining which optimizations belong to semantic ZIR passes and which belong to backend-specific implementation.

## Scope

This closure covers:
- semantic-preserving compiler passes;
- backend-specific optimization freedom;
- ownership/cleanup constraints;
- source mapping constraints;
- conformance expectations.

## Decisions

### O1: Semantic First

No optimization may change observable Zenith behavior.
Observable behavior includes:
- return value and exit code;
- panic/contract failure behavior;
- cleanup timing when visible through user cleanup calls;
- copy-on-write isolation;
- diagnostic/source mapping for invalid programs.

### O2: ZIR-Level Passes

ZIR-level passes may perform target-independent simplifications only when proven semantic-preserving:
- constant folding for pure scalar expressions;
- dead temporary removal after ownership analysis;
- unreachable block cleanup after diagnostics are preserved;
- redundant retain/release elimination when ownership proof is local and sound;
- monomorphized instance deduplication through canonical keys.

### O3: Backend-Specific Passes

Backends may perform target-specific optimizations after conformance:
- instruction selection;
- native compiler flags;
- inlining;
- allocation strategy;
- platform runtime specialization;
- vectorization or low-level numeric optimization.

These must not leak into language semantics.

### O4: Ownership Barrier

Optimization must respect ownership and cleanup barriers:
- `using` cleanup boundaries;
- sink parameter moves;
- managed value retain/release pairs;
- FFI calls;
- runtime calls with side effects;
- channel/job/shared/atomic operations.

### O5: Source Mapping Barrier

Optimization must preserve usable source mapping.
Generated temporaries may move, but diagnostics and debug-facing spans must remain tied to user source constructs.

### O6: Activation Rule

Optimization quality is not a language-closure blocker.
Optimization passes become active only after the backend conformance suite is green for the affected feature area.

## Validation Envelope

Minimum validation set:
- backend conformance suite before/after optimization;
- ownership hardening tests;
- cleanup fixtures;
- source mapping diagnostic smoke tests;
- performance tests only after semantic conformance is stable.

## Closure Result

Wave 7.19 is closed as an optimization boundary contract.
Semantic ZIR passes are allowed but constrained; backend-specific optimization remains downstream of conformance.

## Relationship To Other Documents

- `post-v1-zir-consolidation.md`
- `post-v1-backend-conformance-suite.md`
- `post-v1-source-mapping-contract.md`
- `post-v1-runtime-abi-ownership-audit.md`
