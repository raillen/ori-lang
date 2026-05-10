# Language Decisions Index

> Audience: maintainer, language designer
> Status: draft
> Surface: internal
> Source of truth: no

## How To Read This Index

This index organizes historical language decisions by current usefulness.

Use `docs/spec/language/` for current rules. Use this directory to understand why
the language moved in a certain direction.

## Status Values

| Status | Meaning |
| --- | --- |
| `Current` | Still describes current language behavior or accepted direction. |
| `Superseded` | Replaced by newer spec, implementation, or decision. |
| `Historical` | Useful context, not current contract. |
| `Implementation-only` | Internal architecture, not public language surface. |

## Current Authority Rule

When this index, an old decision, and current implementation disagree:

1. compiler/parser/typechecker/runtime behavior wins;
2. passing behavior tests win over stale prose;
3. `docs/spec/language/final-language-contract.md` wins over old decisions;
4. `docs/spec/language/zenith-language-spec.md` and topic-specific current specs win
   over superseded surface docs;
5. old decisions remain historical context.

## High-Impact Supersession Notes

| Topic | Current Rule |
| --- | --- |
| Dynamic dispatch spelling | Use `any<Trait>`, not public `dyn<Trait>`. |
| Match fallback | Use canonical `case else:` where supported; legacy `case default` is transitional/historical. |
| Interpolation | Use `f"..."`; `fmt "..."` is a deprecated/migration spelling. |
| Generic constraints | Prefer inline `<T: Trait>`; use `given` for complex constraints. |
| Sub-integer types | `u8`, `u16`, `u32`, `u64` remain shipped public surface types. |

## By Topic

### Core Syntax And Program Shape

| Decision | Status | Notes |
| --- | --- | --- |
| 001 namespace declaration | Current | Namespace model remains core surface. |
| 002 import declarations | Current | Import model remains core surface. |
| 003 functions and blocks | Current | Block/function structure remains active. |
| 004 variables and mutability | Current | Mutability model remains active. |
| 005 scope and visibility | Current | Visibility model remains active, with later refinements. |
| 007 expressions and precedence | Current | Precedence model remains active unless contradicted by syntax coherence changes. |
| 012 syntax sugar policy | Historical | Older syntax policy; use current spec for final spellings. |
| 017 control flow | Current | Current in concept; examples need syntax audit. |
| 022 entrypoint and program model | Current | Entrypoint model remains active. |
| 036 text concatenation and interpolation | Superseded | Concept remains; `fmt "..."` spelling superseded by `f"..."`. |
| 043 attributes and annotations | Current | Attribute model remains active. |
| 044 comments and zdoc boundary | Current | Comments and zdoc boundary remain active. |
| 045 formatter and style | Current | Formatter/style policy remains active. |
| 092 single-file execution | Current | Accepted user workflow. |
| 093 language design session v7 | Superseded | Largely replaced by Decision 094. |
| 094 syntax coherence refinements | Current | Main current syntax cleanup decision. |
| 095 `group` tuple alias | Current | Post-v1 alias: `group<...>` resolves to the canonical tuple type. |

### Types, Values, And Semantics

| Decision | Status | Notes |
| --- | --- | --- |
| 006 user-visible types | Current | Base type set remains active, but sub-integer removal notes are not current. |
| 008 text, lists, and maps | Current | Collection concepts remain active. |
| 009 optional, result, and error flow | Current | No-null/no-exception direction remains active. |
| 018 literals, text indexing, and slices | Current | Current in concept; verify examples. |
| 020 construction and initialization | Current | Construction model remains active. |
| 021 field and collection mutation | Current | Mutation model remains active. |
| 024 numeric conversions, overflow, unsigned integers | Current | Numeric conversion model remains active. |
| 034 value semantics and ownership | Current | Core semantic direction. |
| 035 evaluation order and temporaries | Current | Core semantic direction. |
| 040 equality, hashing, and ordering | Current | Core protocol/semantic direction. |
| 041 no null and optional absence | Current | Core language philosophy and behavior. |
| 071 bytes and binary foundation | Current | Binary/text split remains active. |
| 072 stdlib bytes and hex byte literals | Current | Byte literal and stdlib direction. |
| 082 syntax accessibility ergonomics | Current | Current readability/accessibility policy. |

### Structs, Traits, Enums, Composition

| Decision | Status | Notes |
| --- | --- | --- |
| 010 structs, traits, apply, enums, and match | Superseded | Concepts remain, but `case default` and older match spelling are superseded. |
| 014 contracts, refinement, and field invariants | Current | Contract/invariant direction remains active. |
| 019 user-defined generics | Current | User generic model remains active. |
| 023 generic constraints | Superseded | Older `where T is Trait` surface superseded by `<T: Trait>`/`given`. |
| 027 core prelude traits and operator semantics | Current | Prelude/operator trait model remains active. |
| 028 generic monomorphization and call inference | Implementation-only | Backend/typechecker strategy; public guide should summarize, not expose internals. |
| 029 executable enums with payload | Current | Current in concept; match fallback examples need syntax cleanup. |
| 030 question propagation for result and optional | Current | `?` propagation remains active. |
| 032 generic collections and update block | Current | Collection/generic direction remains active. |
| 088 dyn dispatch minimum subset | Superseded | Dynamic dispatch semantics useful, but public spelling superseded by `any`. |
| 089 callable delegates v1 | Current | Callable/delegate surface remains active. |
| 090 closures v1 | Current | Closure surface remains active, with Decision 094 refinements. |

### Runtime, Backend, FFI, Concurrency

| Decision | Status | Notes |
| --- | --- | --- |
| 003-zir structured internals | Implementation-only | Internal IR direction. |
| 011 extern C and extern host | Current | FFI model remains active. |
| 031 runtime where contracts | Current | Runtime contract direction remains active. |
| 037 panic, fatal errors, and attempt | Current | Runtime failure model remains active. |
| 047 C target and interop boundary | Current | C backend/interop boundary remains active. |
| 053 build pipeline and artifact modes | Implementation-only | Build/backend architecture. |
| 073 general streams policy | Current | Runtime/stdlib IO policy. |
| 074 network blocking and timeout policy | Current | Runtime/stdlib network policy. |
| 075 network socket and address model | Current | Network model. |
| 078 backend scalability and runtime risk policy | Implementation-only | Backend risk policy; examples may need syntax cleanup. |
| 079 memory and dispatch architecture | Current | Runtime model current; `dyn` wording needs `any` rewrite in public docs. |
| 080 modding, UI, and tooling architecture | Implementation-only | Future architecture direction. |
| 085 core and platform layering contract | Current | Runtime/stdlib layering. |
| 087 concurrency workers and transfer boundaries | Current | Accepted concurrency boundary direction. |
| 091 defer concurrency full surface | Historical | Defers full concurrency; not shipped surface. |
| 096 generic runtime capability gates | Implementation-only | First `0.4.2-beta.rc1` gate for materialized generic runtime capabilities. |

### Project, Packaging, Tooling, Docs

| Decision | Status | Notes |
| --- | --- | --- |
| 015 zenith.ztproj and package model | Current | Project model remains active. |
| 016 zdoc and doc linking | Current | ZDoc model remains active, with later refinements. |
| 025 process model and CLI arguments | Current | Process/CLI argument model remains active. |
| 038 diagnostics and warning philosophy | Current | Diagnostic philosophy remains active. |
| 039 diagnostic rendering and error codes | Current | Diagnostic rendering/code model remains active. |
| 048 tests in language | Current | Language test model remains active. |
| 049 build conditionals and feature flags | Current | Conditional build model remains active. |
| 051 zenith.ztproj project manifest | Current | Manifest model remains active. |
| 052 file layout and namespace path mapping | Current | Namespace/file mapping remains active. |
| 054 CLI conceptual model | Current | CLI model remains active. |
| 055 zdoc final model | Current | Current ZDoc model. |
| 056 zpm package and library model | Current | Package/library model remains active. |
| 076 project root cutover and legacy archive | Historical | Repository migration history. |
| 077 language coherence closure | Current | Closure/completeness policy remains active. |

### Standard Library

| Decision | Status | Notes |
| --- | --- | --- |
| 050 core stdlib boundary | Current | Stdlib boundary remains active. |
| 057 main entrypoint typed error results | Current | Entrypoint/result direction remains active. |
| 058 stdlib io | Current | API must be verified against implementation. |
| 059 stdlib text | Current | API must be verified against implementation. |
| 060 stdlib format | Current | API must be verified against implementation. |
| 061 stdlib validate | Current | API must be verified against implementation. |
| 062 stdlib math | Current | API must be verified against implementation. |
| 063 stdlib time | Current | API must be verified against implementation. |
| 064 stdlib fs | Current | API must be verified against implementation. |
| 065 stdlib fs path | Current | API must be verified against implementation. |
| 066 stdlib json | Current | API must be verified against implementation. |
| 067 stdlib os | Current | API must be verified against implementation. |
| 068 stdlib os process | Current | API must be verified against implementation. |
| 069 stdlib test | Current | API must be verified against implementation. |
| 070 prelude and stdlib architecture | Current | Stdlib architecture remains active. |
| 081 stdlib collections and random | Current | API must be verified against implementation. |
| 083 stdlib net refinements | Current | API must be verified against implementation. |
| 084 net error catalog | Current | API must be verified against implementation. |

### Philosophy And Product Direction

| Decision | Status | Notes |
| --- | --- | --- |
| 033 language philosophy and manifesto | Current | Public philosophy input. |
| 042 overload, lambdas, and macros | Historical | Deferred or rejected ideas; do not expose as current surface. |
| 086 namespace public var and controlled mutation | Current | Public state/mutation direction remains active. |

## Next Index Work

1. Verify each `Current` stdlib decision against implemented modules.
2. Move any decision with stale examples to `Current` plus syntax cleanup notes,
   or `Superseded` if the rule changed.
3. Add links from superseded decisions to the exact replacement spec section.
4. Update `docs/internal/decisions/language/README.md` to point here after review.
