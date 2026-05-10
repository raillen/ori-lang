# Tier 7 Documentation Reset Plan

> Audience: maintainer, docs writer
> Status: draft
> Surface: internal
> Source of truth: no

## Goal

Create one coherent documentation system for the implemented Zenith language.

Tier 7 is not a small polish pass. It is a documentation reset:

- preserve implementation-critical docs;
- archive stale and duplicate material;
- reconcile old decisions with current compiler behavior;
- write a complete canonical language specification;
- rebuild public docs from that source.

The content model should follow the useful shape of `lume-language-spec.html`:
start with the language purpose, then explain syntax, semantics, memory/runtime
model, errors, modules, stdlib, and full programs in one readable flow.

## Non-Destructive Rule

Do not delete docs first.

Use this order:

1. inventory;
2. classify;
3. archive;
4. rewrite canonical docs;
5. validate examples;
6. remove obsolete public entry points only after replacements exist.

## Source Of Truth

When docs disagree, use this order:

1. current compiler/parser/typechecker/runtime behavior;
2. passing behavior tests and fixtures;
3. `docs/spec/language/final-language-contract.md`;
4. topic-specific current files under `docs/spec/language/`;
5. `docs/reference/`;
6. current `docs/public/` after rewrite;
7. historical `docs/internal/decisions/language/`;
8. old plans, reports, translations, and archived docs.

Historical decisions explain why the language changed. They do not override
the current implementation.

## Keep

Keep these until the reset is complete:

- `docs/spec/language/v1-surface-contract.md`;
- `docs/spec/language/implementation-status.md`;
- `docs/spec/language/compiler-model.md`;
- `docs/spec/language/runtime-model.md`;
- `docs/spec/language/stdlib-model.md`;
- `docs/spec/language/project-model.md`;
- `docs/spec/language/diagnostic-code-catalog.md`;
- `docs/spec/language/decision-conflict-audit.md`;
- code-local maps such as `compiler/*_MAP.md`;
- runtime, stdlib, test, tool, and package docs used by maintainers.

These can be rewritten later, but they must not disappear before their content
is represented in the new canonical docs.

## Rewrite

Rewrite these surfaces from the new canonical spec:

- language reference;
- surface syntax reference;
- public tutorial;
- public cookbook;
- stdlib reference;
- coming-from-X guides;
- quick examples;
- translated docs, after the current public docs are stable.

The final language contract is canonical. Future public English docs and
translations are best-effort teaching layers and must not define extra semantics.

## Archive Candidates

Archive stale material before deletion:

- old roadmaps and checklists after their implementation facts are captured;
- old reports and audit files;
- duplicate public docs across languages;
- stale examples that no longer compile;
- old HTML monoliths that are not the canonical spec;
- decisions superseded by current spec, unless kept as historical context.

Suggested destination:

`docs/internal/archive/tier7-doc-reset/`

## Decision Reconciliation

Maintain `docs/internal/decisions/language/INDEX.md`.

Group decisions by topic:

- core syntax;
- modules and visibility;
- type system;
- structs, enums, traits, and composition;
- generics and constraints;
- optional/result/error flow;
- memory and runtime;
- stdlib;
- diagnostics;
- tooling;
- superseded decisions.

Each decision gets one status:

- `Current`: still describes current language behavior;
- `Superseded`: replaced by a newer spec or implementation;
- `Historical`: useful context, not current behavior;
- `Implementation-only`: internal architecture, not surface language contract.

## Known Incongruences To Resolve

Check and resolve these before public rewrite:

- `dyn` versus canonical `any<Trait>`;
- removed sub-integer plan versus current `u8`, `u16`, `u32`, `u64`;
- legacy names such as `uint8` versus canonical `u8`;
- `fmt "..."` versus canonical `f"..."`;
- old `assert` wording versus current `check`;
- `case default` versus current match fallback spelling;
- global `size_of` versus current stdlib/debug API;
- decisions that describe future or postponed features as if they shipped.

Use compiler behavior and tests as evidence before changing public docs.

## Canonical Language Spec Outline

Create a complete spec with this structure:

1. Why Zenith exists
2. Design principles
3. Program structure
4. Comments and documentation comments
5. Modules, imports, visibility, and namespaces
6. Variables, constants, and mutability
7. Primitive types
8. Text, bytes, lists, maps, and tuples
9. Optional and result values
10. Expressions and precedence
11. Functions, named parameters, and defaults
12. Generics and constraints
13. Structs and composition
14. Enums and pattern matching
15. Traits and `any<Trait>`
16. Control flow
17. Error handling without exceptions
18. Absence without null
19. Ownership, value semantics, and runtime model
20. Attributes and tests
21. FFI and C target boundaries
22. Standard library overview
23. Diagnostics model
24. Complete program examples
25. Compatibility and future reserved areas

## Semantic Guides

Write user-facing guides for concepts that differ from mainstream languages:

- composition instead of inheritance;
- traits instead of class hierarchies;
- enums and match instead of subclass trees;
- optional instead of null;
- result instead of exceptions;
- explicit imports and modules instead of implicit globals;
- value semantics and ownership expectations.

## Comparison Guides

Create short guides for users coming from:

- Python;
- TypeScript;
- Go;
- Rust;
- C;
- C# or Java.

Each guide should answer:

- what feels familiar;
- what is deliberately different;
- how to translate common patterns;
- what not to copy from the source language.

## Validation Gate

Add a docs check that catches stale public syntax.

Initial banned or context-sensitive terms:

- `dyn` outside historical decision context;
- `fmt "` outside migration notes;
- `assert` as a current language feature;
- `case default` if no longer canonical;
- `uint8`, `uint16`, `uint32`, `uint64` as preferred names;
- global `size_of` if only stdlib/debug API is current.

Examples in public docs should either:

- pass `zt check`; or
- be marked as pseudocode.

## Exit Criteria

Tier 7 is complete when:

- one canonical language spec exists;
- decisions have a status index;
- known syntax conflicts are resolved or explicitly marked historical;
- public docs are rebuilt from the canonical spec;
- translated public docs are either current and validated or explicitly post-RC;
- stale duplicate docs are archived or removed;
- examples are checked or marked as pseudocode;
- docs validation runs in the normal quality gate.
