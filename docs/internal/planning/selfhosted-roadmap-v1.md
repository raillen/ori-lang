# Zenith Self-Hosted Roadmap v1

> Status: draft for discussion
> Created: 2026-04-29
> Derived from: roadmap-v7.md, self-hosting discussion, and post-1.0 language/tooling gaps
> Rule: no topic becomes implementation work until Raillen makes the final decision.

This roadmap is a decision-first plan.

The goal is to prepare Zenith for a future self-hosted compiler without forcing
the language to grow too fast. Each topic must be discussed one by one before
implementation.

## How This Roadmap Works

Each topic follows this flow:

1. Present the topic.
2. Explain why it matters.
3. Explain what it enables.
4. List practical implementation directions.
5. List pros, cons, and risks.
6. Record Raillen's final decision.
7. Only then move to code, tests, docs, and release notes.

Status values:

- `discussion_pending`: not discussed yet.
- `deciding`: currently being discussed.
- `accepted`: approved for implementation.
- `rejected`: explicitly not entering the language/tooling.
- `deferred`: useful, but not now.
- `implemented`: code, tests, and docs are done.

## Principles

1. Self-hosting is a dogfood tool first, not a rewrite promise.
2. The C compiler remains the source of truth until the Zenith compiler can prove parity.
3. Every language feature must pay rent through a real compiler, stdlib, or app use case.
4. Prefer small, testable slices over large rewrites.
5. Keep syntax explicit and readable for neurodivergent readers.

## Phase 0 - Decision Protocol

Objective: create a safe path for discussing and approving v8 items.

| ID | Topic | Status |
| --- | --- | --- |
| V8.00 | Use decision records for public surface and relevant architecture decisions | accepted |
| V8.01 | Keep v7 active while v8 remains discussion-first | accepted |
| V8.02 | Require a small prototype before accepting high-risk features | accepted |
| V8.03 | Use topic-local decision blocks plus Decision Log summary rows | accepted |

Decision question:

- Should every accepted v8 topic create or update a `docs/internal/decisions/language/` file?

Decision:

- Option B accepted.
- A decision record is required when a v8 topic changes language syntax,
  semantics, public stdlib APIs, runtime behavior, backend contracts, package
  model, or self-hosting architecture.
- Small docs, tests, examples, and internal tooling changes do not require a
  decision record unless they reveal a public contract change.

Why:

- This keeps important decisions traceable without making every small task
  bureaucratic.

### V8.01 - Active Source Policy

Decision:

- Option B accepted.
- `roadmap-v7.md` and `checklist-v7.md` remain the active consolidated source.
- `language-readiness-roadmap.md` and `language-readiness-checklist.md` remain discussion-first documents for
  self-hosting topics that depend on language-readiness evidence.
- language-readiness can promote evidence into v1/post-v1 plans only through
  the owning contract documents.

Why:

- This avoids presenting undecided v8 topics as approved implementation work.
- It keeps the current consolidated plan stable while v8 matures.

### V8.02 - Prototype Policy

Decision:

- Option B accepted.
- Prototype is mandatory for high-risk v8 topics.
- High-risk means: syntax changes, memory model changes, runtime/concurrency,
  backend contracts, package model changes, and complex stdlib APIs such as
  HTTP/TLS or advanced regex.
- Low-risk docs, tests, tooling, and small helper APIs can proceed with normal
  review and tests.

Why:

- This protects Zenith from accepting features that look good in discussion but
  hurt readability, tooling, diagnostics, or implementation simplicity.

### V8.03 - Decision Log Convention

Decision:

- Option B accepted.
- Each decided topic gets a short `Decision`, `Why`, and `Impact` block near
  the topic.
- Each decided topic also gets one summary row in the roadmap `Decision Log`.
- Large public-surface decisions may also create or update a
  `docs/internal/decisions/language/` file, following V8.00.

Why:

- This keeps the local reasoning near the topic while preserving a quick
  project-level decision index.

Impact:

- Future v8 discussions should update both the topic section and the decision
  log when a decision closes.

## Phase 1 - Self-Hosting SH1 Dogfood

Objective: prove Zenith can write compiler-shaped code before porting the compiler.

| ID | Topic | Status |
| --- | --- | --- |
| SH1.01 | Create `packages/compiler_zt/` project for SH1 | accepted |
| SH1.02 | Start SH1 with lexer parity only | accepted |
| SH1.03 | Use JSONL plus human-readable text for lexer golden tests | accepted |
| SH1.04 | Include EOF, skip whitespace, skip comments by default | accepted |
| SH1.05 | Use Zenith-native token names with official C-to-Zenith mapping | accepted |
| SH1.06 | Store SH1 parity assets under `tests/selfhost/lexer/` | accepted |
| SH1.07 | Create executable package with library-shaped modules | accepted |
| SH1.08 | Use Python parity runner plus simple package main for smoke | accepted |
| SH1.09 | Implement parser subset for declarations | discussion_pending |
| SH1.10 | Define AST representation that is natural in Zenith | discussion_pending |
| SH1.11 | Add `zt-self parse <file.zt>` experimental CLI | discussion_pending |
| SH1.12 | Write a self-hosting gap report after SH1 | discussion_pending |

Decision questions:

- Should SH1 live inside the main repo or as a package under `packages/`?
- Should the first target be lexer parity only, or lexer plus declaration parser?

Decision for SH1.01:

- Option B accepted.
- SH1 will live at `packages/compiler_zt/`.
- The package is experimental, but intentionally uses the normal Zenith package
  layout to dogfood package structure, module naming, stdlib, tests, and tooling.
- It is not the official compiler until explicit parity gates are met.

Why:

- Self-hosting should test the real package workflow instead of living as an
  isolated one-off experiment.

Impact:

- SH1 scaffolding should create a normal package project under
  `packages/compiler_zt/`.
- Any package-system gaps found during SH1 become v8 evidence, not ad hoc hacks.

Decision for SH1.02:

- Option A accepted.
- SH1 starts with lexer parity only.
- The first milestone is a Zenith lexer that reads `.zt` files and emits token
  kind, lexeme, line, column, and byte span.
- Golden tests compare the Zenith lexer against the current C lexer.
- Declaration parser work starts only after lexer parity is green.

Why:

- Lexer parity is the smallest real self-hosting slice.
- It gives objective evidence before adding AST and parser complexity.

Impact:

- SH1 implementation should not begin with parser work.
- Any missing stdlib APIs discovered while writing the lexer should be recorded
  as v8 evidence.

Decision for SH1.03:

- Option C accepted.
- Lexer golden tests use two artifacts:
  - `.tokens.jsonl` as the machine-readable contract.
  - `.tokens.txt` as the human-readable review view.
- JSONL is authoritative for automated comparison between the C lexer and the
  Zenith lexer.
- Text snapshots exist to make diffs easier to inspect.

Why:

- JSONL is stable and script-friendly.
- Text snapshots reduce cognitive load during review.

Impact:

- The golden runner must fail on JSONL mismatch.
- Text snapshots may be generated from the same token stream to avoid drift.

Decision for SH1.04:

- EOF A accepted: the SH1 compiler lexer includes EOF.
- Whitespace A accepted: whitespace is skipped in the default compiler token
  stream.
- Comments C accepted: comments are skipped by default, but a future
  `include_trivia` mode may expose comments and whitespace for formatter,
  documentation, and LSP use.
- Golden parity for SH1 follows the default compiler mode.

Why:

- EOF makes the future parser simpler and more explicit.
- Skipping whitespace keeps the compiler token stream small.
- Deferring trivia keeps formatter/docs options open without polluting SH1.

Impact:

- SH1 JSONL goldens include an EOF token.
- SH1 JSONL goldens do not include whitespace or comments by default.

Decision for SH1.05:

- Option C accepted.
- `packages/compiler_zt/` uses Zenith-native `TokenKind` names.
- Golden tests use an official C-to-Zenith token kind mapping.
- The mapping is part of the SH1 parity contract.

Why:

- The self-hosted code should read naturally in Zenith.
- Parity with the C lexer still needs an explicit, testable bridge.

Impact:

- Token names in Zenith should avoid C prefixes and abbreviations.
- Tests need a stable mapping artifact, for example
  `tests/selfhost/lexer/token_kind_map.json`.

Decision for SH1.06:

- Option B accepted.
- SH1 parity assets live under `tests/selfhost/lexer/`.
- This includes fixtures, JSONL goldens, text snapshots, and
  `token_kind_map.json`.
- `packages/compiler_zt/` contains the implementation, not the parity contract.

Why:

- Lexer parity is a repo-level integration contract between the C compiler and
  the Zenith implementation.
- Keeping parity assets under `tests/` makes that purpose explicit.

Impact:

- Test runners should read fixtures and expected outputs from
  `tests/selfhost/lexer/`.
- Package-local tests may exist later, but they are not the SH1 parity source of
  truth.

Decision for SH1.07:

- Option B accepted.
- `packages/compiler_zt/` is an executable package with library-shaped modules.
- It has a small experimental `main` entrypoint for SH1.
- Lexer, token, source, and diagnostic code should be organized as reusable
  modules.
- A separate CLI can be split later if needed.

Why:

- SH1 needs to be runnable while still producing reusable compiler modules.
- This keeps the first package practical without over-architecting the CLI.

Impact:

- The package should be runnable through the normal project flow.
- Module naming should stay clean enough to become library code later.

Decision for SH1.08:

- Accepted.
- SH1 uses a Python runner for parity tests.
- `packages/compiler_zt/` also has a simple `main()` for manual smoke.
- No new `zt selfhost` command enters the driver yet.
- A future CLI command can be discussed after lexer parity is green.

Why:

- The parity runner needs reliable fixture handling, C-vs-Zenith comparison, and
  snapshot generation.
- A package `main()` keeps dogfood visible without forcing the main CLI to own an
  experimental workflow too early.

Impact:

- Add self-hosting test orchestration under `tools/` or `tests/`.
- Keep `zt` command surface unchanged during SH1.

Why it matters:

- A lexer/parser dogfood slice reveals missing language features with low risk.
- It avoids rewriting binder, checker, and backend before Zenith proves it can model compiler data well.

Implementation directions:

- Start with token kinds, spans, and diagnostics.
- Compare outputs through fixtures, not manual inspection.
- Keep the existing C compiler authoritative.

Pros:

- Real dogfood.
- Low blast radius.
- Produces concrete evidence for future features.

Cons:

- Duplicates compiler logic temporarily.
- May expose missing stdlib APIs before they are ready.

## Phase 2 - Syntax and Grammar Freeze

Objective: stabilize the syntax enough that a self-hosted parser is not chasing moving target rules.

| ID | Topic | Status |
| --- | --- | --- |
| GR.01 | Publish a current grammar/spec for stable syntax | discussion_pending |
| GR.02 | Mark experimental syntax separately from stable syntax | discussion_pending |
| GR.03 | Define migration policy for syntax changes before 1.0 | discussion_pending |
| GR.04 | Decide whether deprecated syntax emits warning for one release | discussion_pending |

Decision questions:

- Which syntax is stable enough for a self-hosted parser?
- How long should old syntax remain accepted after replacement?

Why it matters:

- A self-hosted parser needs stable contracts.
- Tooling, docs, LSP, TextMate grammar, and examples must agree.

Implementation directions:

- Treat grammar as a contract tested by fixtures.
- Link grammar items to parser tests.
- Keep examples small and direct.

## Phase 3 - Compiler Data Model

Objective: decide the data shapes Zenith needs for compiler work.

| ID | Topic | Status |
| --- | --- | --- |
| DM.01 | Official AST model for self-hosting | discussion_pending |
| DM.02 | Complete SourceSpan model from SH1 | accepted |
| DM.03 | Diagnostic model with code, message, span, action, why, next | discussion_pending |
| DM.04 | Symbol table shape | discussion_pending |
| DM.05 | Type representation shape | discussion_pending |
| DM.06 | IR boundary: AST -> HIR -> ZIR in Zenith | discussion_pending |

Decision questions:

- Should the self-hosted AST copy the C AST, or use a cleaner Zenith-native model?
- Should diagnostics be structs from the start, instead of formatted strings?

Decision for DM.02:

- Option B accepted.
- SH1 tokens use a complete `SourceSpan` model.
- Tokens include `kind`, `lexeme`, and `span`.
- Byte offsets are 0-based and end-exclusive.
- Lines and columns are 1-based for human-facing messages.
- `file_id` may start as `0` during SH1 and become real when source-file
  indexing lands.

Why:

- Compiler, LSP, diagnostics, formatter, and future source maps all need precise
  spans.
- Starting with complete spans avoids an early migration from a weaker token
  shape.

Impact:

- Golden tests should include token kind, lexeme, byte span, line, and column.
- Any text/bytes helper needed for accurate spans becomes part of the SH1
  stdlib evidence.

Why it matters:

- Bad compiler data shapes make every later phase harder.
- A good model also improves LSP, docs, and formatter tooling.

Implementation directions:

- Start with explicit structs and enums.
- Avoid clever generic abstractions until repeated patterns are real.
- Keep source spans first-class.

## Phase 4 - Core Stdlib for Compiler Work

Objective: strengthen the stdlib surfaces needed by real compiler code.

| ID | Topic | Status |
| --- | --- | --- |
| STD.01 | `std.text` compiler-grade helpers | discussion_pending |
| STD.02 | `std.bytes` byte scanning helpers | discussion_pending |
| STD.03 | `std.list` performance and ergonomics audit | discussion_pending |
| STD.04 | `std.map` beyond `map<text,text>` | discussion_pending |
| STD.05 | `std.set` for token/symbol collections | discussion_pending |
| STD.06 | `std.fs` path normalization and directory walking | discussion_pending |
| STD.07 | `std.os.process` command execution for tooling | discussion_pending |
| STD.08 | `std.time` measurement helpers for perf tests | discussion_pending |

Decision questions:

- Which stdlib APIs are required before SH1?
- Which APIs can stay internal until proven?

Why it matters:

- Compiler code stresses text, bytes, maps, lists, paths, and diagnostics.
- Missing stdlib pieces force workarounds that can distort the language design.

Implementation directions:

- Add only APIs used by SH1, tests, docs, or tooling.
- Keep names explicit.
- Benchmark only after correctness is covered.

## Phase 5 - Regex, Parsing Helpers, and Text Processing

Objective: decide how much text-processing power belongs in stable stdlib.

| ID | Topic | Status |
| --- | --- | --- |
| TXT.01 | `std.regex` captures/groups | discussion_pending |
| TXT.02 | `std.regex` flags and Unicode behavior | discussion_pending |
| TXT.03 | `std.regex` replacement with captures | discussion_pending |
| TXT.04 | Scanner/parser helper library for tooling | discussion_pending |
| TXT.05 | Decide whether compiler lexer should use regex or manual scanning | discussion_pending |

Decision questions:

- Should regex become richer before self-hosting, or stay small?
- Should compiler code depend on regex at all?

Why it matters:

- Regex is useful for tooling and docs, but compiler lexers are often clearer and faster with manual scanners.

Implementation directions:

- Recommended default for compiler lexer: manual scanner.
- Recommended stdlib path: add captures only if real tools need them.

Pros:

- Better tooling and text utilities.
- Easier user scripts.

Cons:

- Unicode and capture semantics can become complex quickly.
- Regex-heavy compiler code can hurt readability.

## Phase 6 - Networking, HTTP, and Package Infrastructure

Objective: decide which network APIs are needed for ZPM, docs tooling, and future package registry work.

| ID | Topic | Status |
| --- | --- | --- |
| NET.01 | `std.net` blocking TCP basics | discussion_pending |
| NET.02 | `std.http` client GET/POST basics | discussion_pending |
| NET.03 | TLS strategy for HTTPS | discussion_pending |
| NET.04 | JSON + HTTP integration examples | discussion_pending |
| NET.05 | ZPM registry download/install path | discussion_pending |
| NET.06 | Offline/cache policy for packages | discussion_pending |

Decision questions:

- Should Zenith ship blocking `std.http` before async exists?
- Should HTTPS rely on platform libraries, bundled library, or external tool integration?

Why it matters:

- Package management, docs tooling, and real apps need HTTP.
- Network APIs are security-sensitive and platform-sensitive.

Implementation directions:

- Start blocking and explicit if accepted.
- Keep TLS decision separate.
- Do not promise async until the concurrency model is approved.

Pros:

- Makes ZPM and practical tools more useful.
- Enables examples beyond local files.

Cons:

- TLS is hard to own correctly.
- Cross-platform behavior can be uneven.

## Phase 7 - Error, Result, and Diagnostic Ergonomics

Objective: make large Zenith programs readable without hiding failure paths.

| ID | Topic | Status |
| --- | --- | --- |
| ERR.01 | Review `result<T,E>` ergonomics in compiler-shaped code | discussion_pending |
| ERR.02 | Standard diagnostic builder API | discussion_pending |
| ERR.03 | Error collections and multi-error reporting | discussion_pending |
| ERR.04 | Decide whether `try`-style helper syntax is needed | discussion_pending |

Decision questions:

- Is current `?` propagation enough for compiler code?
- Do we need a diagnostic builder in stdlib or compiler package?

Why it matters:

- Compiler code often needs to collect multiple errors, not stop at the first one.
- Error flow must remain explicit and easy to scan.

## Phase 8 - Memory, Ownership, and Allocators

Objective: decide whether self-hosting proves the need for allocator or unsafe features.

| ID | Topic | Status |
| --- | --- | --- |
| MEM.01 | Measure ARC overhead in SH1 | discussion_pending |
| MEM.02 | Arena allocation for compiler phases | discussion_pending |
| MEM.03 | Debug allocator | discussion_pending |
| MEM.04 | `std.mem.Allocator` API | discussion_pending |
| MEM.05 | `std.unsafe` and raw pointer policy | discussion_pending |

Decision questions:

- Can SH1 run well with normal ARC and current collections?
- If not, should arenas be library-only, compiler-internal, or language-level?

Why it matters:

- Compilers allocate many short-lived objects.
- Allocators can improve performance but add cognitive cost.

Implementation directions:

- Measure before adding language features.
- Prefer internal/compiler package experiments first.

## Phase 9 - Functions, Closures, and Callbacks

Objective: decide whether compiler and stdlib work need richer function values.

| ID | Topic | Status |
| --- | --- | --- |
| FN.01 | First-class function types | discussion_pending |
| FN.02 | Lambda expressions | discussion_pending |
| FN.03 | Callbacks for iteration APIs | discussion_pending |
| FN.04 | Closure capture limits | discussion_pending |

Decision questions:

- Does SH1 need first-class functions, or can named functions keep code clearer?
- Would lambdas improve readability or add visual noise?

Why it matters:

- Parser combinators and collection helpers often want callbacks.
- Zenith currently favors explicit named code.

## Phase 10 - Concurrency and Async

Objective: decide whether concurrency belongs before self-hosting or remains post-self-hosting.

| ID | Topic | Status |
| --- | --- | --- |
| CON.01 | Blocking-first policy for stdlib | discussion_pending |
| CON.02 | `task` model | discussion_pending |
| CON.03 | `channel` model | discussion_pending |
| CON.04 | `async/await` syntax | discussion_pending |
| CON.05 | `Shared<T>` and thread-safety policy | discussion_pending |

Decision questions:

- Is concurrency required for the compiler, ZPM, or LSP now?
- Should host runtimes handle concurrency until the language has stronger evidence?

Why it matters:

- Async affects syntax, stdlib, runtime, diagnostics, and teaching material.

## Phase 11 - Tooling and Developer Experience

Objective: make the self-hosting path easy to run, test, and debug.

| ID | Topic | Status |
| --- | --- | --- |
| TOOL.01 | `zt selfhost` or `zt dev selfhost` command | discussion_pending |
| TOOL.02 | Golden test runner for compiler artifacts | discussion_pending |
| TOOL.03 | Snapshot update workflow | discussion_pending |
| TOOL.04 | Debug maps for generated C | discussion_pending |
| TOOL.05 | VSCode tasks for compiler dogfood | discussion_pending |
| TOOL.06 | LSP awareness of self-hosted package code | discussion_pending |

Decision questions:

- Should self-hosting tools live in `zt`, `zpm`, or scripts first?
- Should debugger planning start now or after SH1?

## Phase 12 - Package and Module System

Objective: decide which package features are needed before real self-hosting and ecosystem work.

| ID | Topic | Status |
| --- | --- | --- |
| PKG.01 | Dependency aliases | discussion_pending |
| PKG.02 | Version ranges | discussion_pending |
| PKG.03 | Optional dependencies | discussion_pending |
| PKG.04 | Feature flags | discussion_pending |
| PKG.05 | Local path package ergonomics | discussion_pending |
| PKG.06 | Workspace/multi-package manifest | discussion_pending |

Decision questions:

- Should self-hosting be one project, multiple packages, or a workspace?
- Which ZPM features are required for that layout?

## Phase 13 - Backend and Runtime Boundaries

Objective: keep backend decisions explicit instead of drifting into accidental architecture.

| ID | Topic | Status |
| --- | --- | --- |
| BCK.01 | Keep C backend as bootstrap target | discussion_pending |
| BCK.02 | Define ZIR stability needed by self-hosting | discussion_pending |
| BCK.03 | LLVM backend discussion | discussion_pending |
| BCK.04 | WASM backend discussion | discussion_pending |
| BCK.05 | Runtime API boundaries | discussion_pending |

Decision questions:

- Should self-hosting target the current C backend only?
- What ZIR guarantees are needed before a Zenith-written frontend emits it?

## Phase 14 - Documentation and Teaching

Objective: keep v8 understandable as the language grows.

| ID | Topic | Status |
| --- | --- | --- |
| DOC.01 | Self-hosting guide | discussion_pending |
| DOC.02 | Compiler architecture guide | discussion_pending |
| DOC.03 | Stdlib maturity matrix | discussion_pending |
| DOC.04 | "Accepted / rejected / deferred" public feature matrix | discussion_pending |
| DOC.05 | Small-step examples for every accepted feature | discussion_pending |

Decision questions:

- Which parts of v8 should be public now?
- Which parts should stay internal until implemented?

## Suggested Discussion Order

1. Decision protocol.
2. SH1 scope and project location.
3. Syntax/grammar freeze.
4. Compiler data model.
5. Core stdlib needed for SH1.
6. Regex/text processing.
7. Networking/HTTP/ZPM.
8. Error and diagnostic ergonomics.
9. Memory/allocators.
10. Function values/callbacks.
11. Concurrency/async.
12. Tooling/debug maps.
13. Package/workspace model.
14. Backend/runtime boundaries.
15. Docs and teaching.

## Decision Log

| Date | Topic | Decision | Notes |
| --- | --- | --- | --- |
| 2026-04-29 | v8 format | pending | Roadmap created as discussion-first draft. |
| 2026-04-29 | V8.00 decision records | accepted | Option B: required for public surface and relevant architecture, not for tiny internal work. |
| 2026-04-29 | V8.01 active source policy | accepted | Option B: v7 remains active; v8 remains discussion-first. |
| 2026-04-29 | V8.02 prototype policy | accepted | Option B: prototypes required for high-risk topics only. |
| 2026-04-29 | V8.03 decision log convention | accepted | Option B: topic-local decision block plus Decision Log summary row. |
| 2026-04-29 | SH1.01 self-hosting location | accepted | Option B: SH1 lives at `packages/compiler_zt/` as an experimental package. |
| 2026-04-29 | SH1.02 SH1 scope | accepted | Option A: start with lexer parity only; parser waits until lexer parity is green. |
| 2026-04-29 | DM.02 token/span model | accepted | Option B: tokens include kind, lexeme, and complete SourceSpan from SH1. |
| 2026-04-29 | SH1.03 lexer golden format | accepted | Option C: JSONL is authoritative; text snapshot is generated for human review. |
| 2026-04-29 | SH1.04 lexer stream policy | accepted | Include EOF, skip whitespace, skip comments by default; future include_trivia may expose trivia. |
| 2026-04-29 | SH1.05 token kind names | accepted | Option C: Zenith-native names plus official C-to-Zenith mapping for parity tests. |
| 2026-04-29 | SH1.06 parity asset location | accepted | Option B: fixtures, goldens, snapshots, and token map live under `tests/selfhost/lexer/`. |
| 2026-04-29 | SH1.07 package shape | accepted | Option B: executable package with library-shaped modules. |
| 2026-04-29 | SH1.08 SH1 execution flow | accepted | Python parity runner plus simple package main; no new `zt selfhost` command yet. |

