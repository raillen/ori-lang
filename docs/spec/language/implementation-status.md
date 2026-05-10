# Zenith Implementation Status Model Spec

- Status: canonical closure spec
- Date: 2026-04-18
- Scope: how Zenith tracks implemented, partial and deferred language features

## Purpose

The implementation status model prevents ambiguity between accepted language design and compiler support.

A feature can be designed, parsed, semantically checked, emitted or fully conformance-tested. These are different statuses.

## Status Labels

Use these labels consistently:

- `Spec`: accepted language or tooling design exists
- `Parsed`: parser represents the syntax
- `Semantic`: binder/typechecker validate it
- `Lowered`: HIR/ZIR lowering supports it
- `Emitted`: backend emits target code
- `Runtime`: runtime behavior exists when needed
- `Executable`: behavior test proves observable execution
- `Conformant`: parser, semantic, behavior, diagnostic and runtime tests exist as applicable
- `Deferred`: accepted direction, intentionally not in current cut
- `Risk`: known architecture risk requiring an explicit mitigation or gate
- `Rejected`: explicitly non-canonical

## Feature Closure Rule

A feature is not "done" unless it is at least `Executable`.

A stable MVP feature should reach `Conformant`.

## Required Tracking

Implementation status must track:

- syntax features
- type system features
- runtime features
- stdlib modules
- diagnostics
- formatter behavior
- CLI commands
- ZDoc behavior
- package/ZPM behavior
- backend scalability risks
- runtime ownership risks

## Risk Tracking

A `Risk` item is not a language feature.

It marks an accepted design direction whose implementation can become unsafe, slow or misleading if no mitigation exists.

Required risk tracking examples:

- RC cycles without weak/handle/cycle policy
- monomorphization code bloat
- heap-first optional/result representation
- runtime `where` not enforced
- enum match without exhaustiveness diagnostics

Risk items must point to a roadmap/checklist mitigation before stable release.

## Current Status Document

The current surface implementation snapshot lives at:

```text
docs/spec/language/surface-implementation-status.md
```

That document records the actual compiler cut.

This spec defines the vocabulary and closure rules used by such status documents.

## Definition Of Done

The implementation status model is complete when:

- feature tables use these labels
- checklist items map to status transitions
- conformance matrix references the same labels
- no feature is described as complete only because it appears in a decision document
- architecture risks are tracked explicitly instead of hidden under Deferred

