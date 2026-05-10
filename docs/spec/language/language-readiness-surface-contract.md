# Zenith Language Readiness Surface Contract

> Audience: contributor, maintainer
> Status: reconciliation contract
> Surface: spec
> Source of truth: no; derived from v1 and post-v1 contracts
> Created: 2026-05-01

This document keeps the old v8 language-readiness track in sync with the final
v1 and post-v1 contracts.

Precedence rule:

1. `v1-surface-contract.md` defines what blocks Zenith v1.
2. `post-v1-surface-contract.md` defines accepted/deferred direction after v1.
3. Historical language-readiness planning has been removed from the active
   tree. Current follow-up decisions live in `post-v1-remaining-language-work.md`.

If this document disagrees with v1 or post-v1, v1/post-v1 win.

## Current Role

Language readiness is not a new version line.

It is an internal reconciliation layer for:

- stale docs versus implemented behavior;
- accepted v1 work that still needs execution evidence;
- post-v1 ideas that must not be mistaken for v1 blockers;
- self-hosting prerequisites that depend on stable language behavior.

## v1-Owned Items

These items are v1 scope when listed as required or shipped by
`v1-surface-contract.md`.

| Area | Readiness Topic | Canonical Owner |
|------|-----------------|-----------------|
| Collections | `list<T>`, `map<K,V>`, `set<T>` v1 executable surface | `v1-surface-contract.md` |
| Text/bytes | v1 `text` and `bytes` APIs | `v1-surface-contract.md` |
| I/O | `std.fs`, `std.fs.path`, `std.io`, `std.os.process` v1 surface | `v1-surface-contract.md` |
| Diagnostics | Structured diagnostics and renderer behavior shipped for v1 | `v1-surface-contract.md` |
| Regex | v1 `std.regex` and `try_*` variants | `v1-surface-contract.md` |
| Tests/perf | test runner and perf gates required by v1 plan | `v1-implementation-plan.md` |

Readiness documents may contain discussion notes for these areas, but they must
not reopen the final v1 decisions without a new explicit decision.

## Post-v1-Owned Items

These items are post-v1 unless promoted by a later decision.

| Area | Readiness Topic | Canonical Owner |
|------|-----------------|-----------------|
| Backends | LLVM, WASM, Cranelift, Zig, C3 | `post-v1-surface-contract.md` |
| Runtime | ORC, cycle detector, `std.unsafe`, `std.mem` advanced APIs | `post-v1-surface-contract.md` |
| Language | callable type syntax, nested functions, pipe, guards, `@field` | `post-v1-surface-contract.md` |
| Concurrency | jobs, channels, `Shared<T>`, `atomic<T>` | `post-v1-surface-contract.md` |
| FFI | callbacks and ABI annotations | `post-v1-surface-contract.md` |
| Tooling | web playground, ZPM registry web, mature LSP beyond v1 | `post-v1-surface-contract.md` |

Readiness documents may track prerequisites and evidence for these areas, but
must label them as post-v1.

## Self-Hosting Boundary

Self-hosting is not the owner of language semantics.

Self-hosting work may request:

- stable grammar;
- stable AST/source-span model;
- parity fixtures;
- readable diagnostics;
- reliable test runner.

It must not force a feature into v1 if the feature is post-v1 or rejected by
the canonical contracts.

## Backend Boundary

Current backend policy:

```text
C      = stable backend, compatibility layer, behavior oracle
ZIR    = rigid backend contract
LLVM   = strategic native release backend, post-v1
WASM   = sandbox/web backend, post-v1
Cranelift = fast native backend spike, post-v1 exploration
Zig    = textual systems backend exploration
C3     = textual C-like backend exploration
```

C remains the oracle until another backend passes backend conformance.

## Maintenance Rule

When a readiness item changes:

1. Check v1 and post-v1 contracts first.
2. Update the owning contract if scope changed.
3. Update readiness roadmap/checklist only as tracking/evidence.
4. Keep old v8 wording historical, not normative.
