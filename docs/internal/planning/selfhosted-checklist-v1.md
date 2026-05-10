# Zenith Self-Hosted Checklist v1

> Status: draft for discussion
> Created: 2026-04-29
> Derived from: language-readiness-roadmap.md
> Rule: do not mark implementation items until the matching roadmap topic has a recorded decision.

This checklist turns self-hosting decisions into small implementation steps.

## How To Use

For every topic:

1. Discuss the topic with Raillen.
2. Record the final decision in `language-readiness-roadmap.md`.
3. If accepted, create or update the related decision/spec file.
4. Implement the smallest useful slice.
5. Add tests.
6. Update docs.
7. Record evidence.

Do not skip the decision step.

## Global Gates

- [ ] Decision recorded before implementation.
- [ ] `python build.py` passes after implementation.
- [ ] Focused behavior tests pass.
- [ ] LSP smoke runs when syntax/semantics change.
- [ ] Docs updated when public behavior changes.
- [ ] Roadmap status updated after the slice closes.
- [ ] Checklist item marked only with evidence.

## Phase 0 - Decision Protocol

- [x] V8.00 - Decide whether every accepted/rejected language feature needs a decision record.
- [x] V8.01 - Decide whether roadmap/checklist v8 replace v7 as active source.
- [x] V8.02 - Decide when prototypes are mandatory before acceptance.
- [x] V8.03 - Add decision-log convention to `language-readiness-roadmap.md`.
- [x] V8.04 - Update `docs/internal/planning/README.md` after active-source decision.

Gate:

- [x] v8 workflow is accepted or explicitly revised.

## Phase 1 - Self-Hosting SH1 Dogfood

- [x] SH1.01 - Decide location: `compiler-selfhost/`, `packages/compiler_zt/`, or another path.
- [x] SH1.02 - Decide SH1 scope: lexer only, or lexer plus declaration parser.
- [x] SH1.03 - Decide golden fixture format.
- [x] SH1.04 - Decide EOF/whitespace/comment token stream policy.
- [x] SH1.05 - Decide token kind naming and C-to-Zenith mapping policy.
- [x] SH1.06 - Decide SH1 parity asset location.
- [x] SH1.07 - Decide package shape.
- [x] SH1.08 - Decide SH1 execution flow.
- [ ] SH1.09 - Create project scaffold.
- [ ] SH1.10 - Define token model.
- [ ] SH1.11 - Add token kind mapping artifact.
- [ ] SH1.12 - Implement lexer MVP.
- [ ] SH1.13 - Add golden fixtures.
- [ ] SH1.14 - Compare C lexer output and Zenith lexer output.
- [ ] SH1.15 - Define parser subset.
- [ ] SH1.16 - Implement declaration parser if accepted after lexer parity.
- [ ] SH1.17 - Write self-hosting gap report.

Gate:

- [ ] SH1 can run from one command.
- [ ] Output is tested against fixtures.
- [ ] Gaps are documented.

## Phase 2 - Syntax and Grammar Freeze

- [ ] GR.01 - Decide which syntax is stable for self-hosted parser work.
- [ ] GR.02 - Mark experimental syntax separately.
- [ ] GR.03 - Decide deprecation window for replaced syntax.
- [ ] GR.04 - Update grammar/spec docs.
- [ ] GR.05 - Add parser fixtures linked to grammar sections.
- [ ] GR.06 - Update LSP/TextMate grammar if syntax changes.

Gate:

- [ ] Stable syntax list exists.
- [ ] Parser, LSP, docs, and examples agree.

## Phase 3 - Compiler Data Model

- [ ] DM.01 - Decide AST strategy: copy C AST or design Zenith-native AST.
- [x] DM.02 - Define token/span/source-file structs.
- [ ] DM.03 - Define diagnostic structs.
- [ ] DM.04 - Define symbol table shape.
- [ ] DM.05 - Define type representation shape.
- [ ] DM.06 - Decide AST -> HIR -> ZIR boundaries for self-hosting.
- [ ] DM.07 - Add small examples for each model.

Gate:

- [ ] SH1 data model is documented.
- [ ] No parser work depends on undocumented shapes.

## Phase 4 - Core Stdlib for Compiler Work

- [ ] STD.01 - Audit `std.text` against lexer/parser needs.
- [ ] STD.02 - Audit `std.bytes` against byte scanning needs.
- [ ] STD.03 - Audit `std.list` for compiler workloads.
- [ ] STD.04 - Decide `std.map` generic expansion scope.
- [ ] STD.05 - Decide `std.set` compiler-facing APIs.
- [ ] STD.06 - Decide `std.fs` path and directory APIs.
- [ ] STD.07 - Decide `std.os.process` tooling APIs.
- [ ] STD.08 - Decide `std.time` perf helper APIs.
- [ ] STD.09 - Implement accepted APIs with tests.
- [ ] STD.10 - Update stdlib docs and zdoc.

Gate:

- [ ] SH1 does not require ad hoc stdlib workarounds.

## Phase 5 - Regex and Text Processing

- [ ] TXT.01 - Decide whether `std.regex` captures enter v8.
- [ ] TXT.02 - Decide regex flags and Unicode policy.
- [ ] TXT.03 - Decide replacement-with-captures API.
- [ ] TXT.04 - Decide scanner/parser helper library scope.
- [ ] TXT.05 - Decide whether compiler lexer may depend on regex.
- [ ] TXT.06 - Implement accepted regex/text APIs.
- [ ] TXT.07 - Add positive and negative tests.

Gate:

- [ ] Regex scope is explicit: accepted, deferred, or rejected.

## Phase 6 - Networking, HTTP, and Package Infrastructure

- [ ] NET.01 - Decide blocking-first `std.net` policy.
- [ ] NET.02 - Decide `std.http` client MVP.
- [ ] NET.03 - Decide TLS strategy.
- [ ] NET.04 - Decide JSON + HTTP examples.
- [ ] NET.05 - Decide ZPM registry download/install path.
- [ ] NET.06 - Decide package cache/offline behavior.
- [ ] NET.07 - Implement accepted networking slice.
- [ ] NET.08 - Add security and failure-mode tests.

Gate:

- [ ] No network API ships without failure behavior docs.

## Phase 7 - Error, Result, and Diagnostic Ergonomics

- [ ] ERR.01 - Review current `result<T,E>` in compiler-shaped examples.
- [ ] ERR.02 - Decide diagnostic builder location.
- [ ] ERR.03 - Decide multi-error collection API.
- [ ] ERR.04 - Decide if helper syntax beyond `?` is needed.
- [ ] ERR.05 - Implement accepted diagnostic APIs.
- [ ] ERR.06 - Add examples with multiple diagnostics.

Gate:

- [ ] Compiler-shaped code can report multiple readable errors.

## Phase 8 - Memory, Ownership, and Allocators

- [ ] MEM.01 - Measure SH1 allocation/performance with current ARC.
- [ ] MEM.02 - Decide whether arenas are needed.
- [ ] MEM.03 - Decide whether arena experiment is internal only.
- [ ] MEM.04 - Decide `std.mem.Allocator` API status.
- [ ] MEM.05 - Decide `std.unsafe` and raw pointer policy.
- [ ] MEM.06 - Implement only accepted internal/library experiments first.

Gate:

- [ ] No allocator/lifetime feature enters without measurement.

## Phase 9 - Functions, Closures, and Callbacks

- [ ] FN.01 - Decide first-class function types.
- [ ] FN.02 - Decide lambda syntax.
- [ ] FN.03 - Decide callback use in stdlib/compiler helpers.
- [ ] FN.04 - Decide closure capture limits.
- [ ] FN.05 - Prototype accepted function feature if needed.
- [ ] FN.06 - Add readability review before implementation.

Gate:

- [ ] Any function feature has a real compiler or stdlib use case.

## Phase 10 - Concurrency and Async

- [ ] CON.01 - Decide blocking-first policy.
- [ ] CON.02 - Decide `task` status.
- [ ] CON.03 - Decide `channel` status.
- [ ] CON.04 - Decide `async/await` status.
- [ ] CON.05 - Decide `Shared<T>` status.
- [ ] CON.06 - Keep deferred items visibly deferred if not accepted.

Gate:

- [ ] No async syntax enters before runtime and teaching model are clear.

## Phase 11 - Tooling and Developer Experience

- [ ] TOOL.01 - Decide command shape: `zt selfhost`, `zt dev selfhost`, or script.
- [ ] TOOL.02 - Add golden test runner if accepted.
- [ ] TOOL.03 - Add snapshot update workflow if accepted.
- [ ] TOOL.04 - Decide debug map scope.
- [ ] TOOL.05 - Add VSCode tasks if accepted.
- [ ] TOOL.06 - Decide LSP treatment of self-hosted compiler project.

Gate:

- [ ] SH1 can be run and tested without hidden steps.

## Phase 12 - Package and Module System

- [ ] PKG.01 - Decide dependency aliases.
- [ ] PKG.02 - Decide version ranges.
- [ ] PKG.03 - Decide optional dependencies.
- [ ] PKG.04 - Decide feature flags.
- [ ] PKG.05 - Decide local path package ergonomics.
- [ ] PKG.06 - Decide workspace/multi-package manifest.
- [ ] PKG.07 - Implement only package features required by accepted layout.

Gate:

- [ ] Self-hosting project layout is supported without manual hacks.

## Phase 13 - Backend and Runtime Boundaries

- [ ] BCK.01 - Decide C backend as self-hosting bootstrap target.
- [ ] BCK.02 - Decide ZIR stability requirements.
- [ ] BCK.03 - Discuss LLVM backend and record decision.
- [ ] BCK.04 - Discuss WASM backend and record decision.
- [ ] BCK.05 - Define runtime API boundary needed by self-hosting.

Gate:

- [ ] Backend target for SH1 is explicit.

## Phase 14 - Documentation and Teaching

- [ ] DOC.01 - Decide self-hosting guide scope.
- [ ] DOC.02 - Decide compiler architecture guide scope.
- [ ] DOC.03 - Create stdlib maturity matrix if accepted.
- [ ] DOC.04 - Create accepted/rejected/deferred feature matrix if accepted.
- [ ] DOC.05 - Add small-step examples for accepted v8 features.

Gate:

- [ ] Public docs only promise implemented behavior.

## Evidence Log

| Date | Item | Evidence |
| --- | --- | --- |
| 2026-04-29 | v8 draft created | `language-readiness-roadmap.md` and `language-readiness-checklist.md` added. |
| 2026-04-29 | V8.00 | Option B accepted in `language-readiness-roadmap.md`: decision records are required for public surface and relevant architecture decisions. |
| 2026-04-29 | V8.01 | Option B accepted in `language-readiness-roadmap.md`: v7 remains active; v8 remains discussion-first. |
| 2026-04-29 | V8.02 | Option B accepted in `language-readiness-roadmap.md`: prototypes are mandatory for high-risk topics only. |
| 2026-04-29 | V8.03 | Option B accepted in `language-readiness-roadmap.md`: use topic-local decision blocks plus Decision Log rows. |
| 2026-04-29 | V8.04 | `docs/internal/planning/README.md` already lists v8 as discussion-first and keeps v7 as active. |
| 2026-04-29 | SH1.01 | Option B accepted in `language-readiness-roadmap.md`: SH1 lives at `packages/compiler_zt/`. |
| 2026-04-29 | SH1.02 | Option A accepted in `language-readiness-roadmap.md`: SH1 starts with lexer parity only. |
| 2026-04-29 | DM.02 | Option B accepted in `language-readiness-roadmap.md`: tokens use complete `SourceSpan` from SH1. |
| 2026-04-29 | SH1.03 | Option C accepted in `language-readiness-roadmap.md`: lexer goldens use authoritative JSONL plus generated text snapshots. |
| 2026-04-29 | SH1.04 | EOF A + Whitespace A + Comments C accepted in `language-readiness-roadmap.md`. |
| 2026-04-29 | SH1.05 | Option C accepted in `language-readiness-roadmap.md`: Zenith-native token names plus official C-to-Zenith mapping. |
| 2026-04-29 | SH1.06 | Option B accepted in `language-readiness-roadmap.md`: SH1 parity assets live under `tests/selfhost/lexer/`. |
| 2026-04-29 | SH1.07 | Option B accepted in `language-readiness-roadmap.md`: executable package with library-shaped modules. |
| 2026-04-29 | SH1.08 | Accepted in `language-readiness-roadmap.md`: Python parity runner plus simple package main, no new `zt selfhost` command yet. |

