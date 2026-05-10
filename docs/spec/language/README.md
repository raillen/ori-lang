# Zenith Language Specs

- Status: canonical consolidated specs
- Date: 2026-05-03
- Source: accepted decisions in `docs/internal/decisions/language/`

## Purpose

This directory contains the readable canonical specs for Zenith Next.

Decision documents remain the historical source for rationale, tradeoffs and conversation context.

These spec files are the implementation-facing reference used after the design consolidation. When older v1, post-v1, or closure documents conflict with the current consolidated state, `final-language-contract.md` prevails first, then the latest topic-specific closure artifact, then `post-v1-remaining-language-work.md`.

## Project context (AI-assisted)

- These specs are part of an early AI-assisted language development initiative.
- The repository is used as a study case, training ground, and validation target.
- Final architecture and release decisions remain human-owned.

## Canonical Documents

- `final-language-contract.md`: compact final/current/future contract and the
  required entry point before reading older closure artifacts.
- `syntax-semantics-by-topic.md`: topical companion reference that separates
  canonical syntax and semantics for each language area.
- `stdlib-reference-by-topic.md`: topical standard library reference covering
  modules, functions, helpers, constants and observable namespace state.
- `zenith-language-spec.md`: primary current language specification for syntax,
  semantics, mental model, examples and compatibility notes.
- `language-reference.md`: implementation-facing unified reference that supports
  the primary spec.
- `lambdas-hof-guidelines.md`: expression lambda syntax guidance and first HOF usage rules.
- `lazy.md`: explicit `lazy<T>` rules, one-shot consumption and no implicit lazy evaluation.
- `stdlib-model.md`: standard library architecture, module policy, error policy and safe API rules.
- `runtime-model.md`: C runtime, managed values, value semantics, cleanup, panic and contracts.
- `backend-scalability-risk-model.md`: RC cycles, monomorphization, stack/heap policy and backend scalability gates.
- `diagnostics-model.md`: structured diagnostics, stable codes and terminal rendering.
- `diagnostic-code-catalog.md`: initial stable code catalog for renderer/tests/tooling alignment.
- `formatter-model.md`: mandatory canonical formatting rules.
- `project-model.md`: `zenith.ztproj`, app/lib projects, file layout, ZDoc layout and ZPM package model.
- `lockfile-schema.md`: initial `zenith.lock` schema for reproducible dependency resolution.
- `compiler-model.md`: compiler pipeline, IR boundaries, C backend, runtime and artifact modes.
- `tooling-model.md`: `zt`, `zpm`, diagnostics, formatter, tests and documentation tooling.
- `implementation-status.md`: status vocabulary and closure rules for implementation tracking.
- `conformance-matrix.md`: final conformance snapshot by layer/feature/risk for M32.
- `decision-conflict-audit.md`: reconciled conflicts between historical decisions and canonical specs.
- `post-v1-surface-contract.md`: accepted post-v1 surface evidence for features, runtime, backends, stdlib and tooling.
- `post-v1-remaining-language-work.md`: current follow-up decisions and accepted gaps after the final contract.
- `post-v1-completeness-discussion.md`: why, when, how and risks for each post-v1 area; starting point for future design sessions.
- `post-v1-implementation-plan.md`: accepted/rejected decisions from 2026-05-01 session and wave-based implementation evidence; not the active public release checklist.
- `language-readiness-surface-contract.md`: historical reconciliation boundary for the renamed language-readiness track.

## Supporting Documents

- `legibility-evaluation.md`: protocol for validating reading-first legibility, metrics, tasks, approval criteria and release gates.
- `cognitive-accessibility.md`: design principles and tooling proposals for ADHD, Dyslexia, Autism and neurodiversity.

## Closure Evidence Documents

The `post-v1-*closure*.md`, `post-v1-*stabilization*.md`, and Wave 7 artifacts are evidence documents.

Read `final-language-contract.md` first. Use the closure artifacts only for detailed rationale, validation envelopes, and implementation constraints.

## Superseded Surface Documents

These files are historical support material after the Tier 7 documentation
reset:

- `surface-syntax.md`
- `closures.md`
- `dyn-dispatch.md`
- `callables.md`

Do not use them as the first source for current public syntax.

## Reading Order

1. `final-language-contract.md`
2. `syntax-semantics-by-topic.md`
3. `stdlib-reference-by-topic.md`
4. `zenith-language-spec.md`
5. `language-reference.md`
6. `post-v1-remaining-language-work.md`
7. `post-v1-implementation-plan.md` for historical implementation evidence
8. `post-v1-final-language-closure-review.md`
9. topic-specific Wave 7 closure artifact only when deeper evidence is needed
10. `implementation-status.md`
11. `stdlib-model.md`
12. `runtime-model.md`
13. `compiler-model.md`
14. `diagnostics-model.md`
15. `diagnostic-code-catalog.md`
16. `formatter-model.md`
17. `project-model.md`
18. `tooling-model.md`
19. `conformance-matrix.md`
20. `decision-conflict-audit.md`
21. `legibility-evaluation.md`
22. `cognitive-accessibility.md`
23. `v1-surface-contract.md` for historical baseline only
24. `language-readiness-surface-contract.md` for historical reconciliation context only

## Relationship To Decisions

The specs consolidate Decisions 001-086.

If a spec and an older decision conflict, the newer accepted decision and this consolidated spec take precedence. `final-language-contract.md` takes precedence over older v1/post-v1 wording and summarizes which gaps remain only executable subsets.

Older decisions are not deleted because they preserve why the rule exists.

## Relationship To Implementation

Implementation status is tracked separately in `surface-implementation-status.md` and summarized in `final-language-contract.md`.

The compiler may lag behind this spec during implementation milestones.

When that happens, roadmap/checklist items should describe the gap explicitly instead of weakening the spec.
