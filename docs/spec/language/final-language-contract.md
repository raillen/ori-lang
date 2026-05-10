# Zenith Final Language Contract

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: final contract  
> Surface: normative consolidation  
> Source of truth: yes  
> Last updated: 2026-05-04

This document is the compact current contract for Zenith after the post-v1 closure pass and the final language contract audit (May 3–4, 2026).

Use it to answer: what is final, what executes today, what gap remains, and where the detailed canonical evidence lives.

## Status Labels

| Label | Meaning |
|---|---|
| Final Contract | Approved language shape. This is the design target unless a newer explicit decision supersedes it. |
| Current Executable Subset | What the current C backend/runtime can check, build, and run today. |
| Historical | Old rationale or planning context. Not authoritative for current behavior. |
| Migration Context | Old spelling or behavior retained intentionally for compatibility/deprecation. |
| Future Implementation | Accepted direction, but not implemented in the current executable subset. |
| Open Discussion | Not finalized. Requires a design session before implementation. |

## Final Contract Matrix

| Area | Final decision | Current implementation | Gap | Canonical doc |
|---|---|---|---|---|
| Syntax freeze | Accepted/rejected syntax is frozen for current closure: explicit types, qualified imports, `using`, `if cond then a else b`, `case pattern if guard:`, `|>`, `@field`, `..` slice, `f"..."` interpolation, no `unless`, no C-style `for`, no variadics, no bare `{ fields }`, no `group` (removed), no `fmt"..."` (removed), no `given` (removed — use `case ... if guard:`). | Parser/checker/formatter cover the current executable syntax set. | Symbol-heavy features need teaching and formatting guidance before public docs are rebuilt. | `post-v1-syntax-freeze.md`, `post-v1-implementation-plan.md` |
| Types and generics | Explicit local type annotations remain required. Argument-position generic inference is accepted. Return-context/full local inference remains rejected. | Generic argument inference and executable C-backend monomorphization subset are implemented for direct/nested generic calls. | Broader generic HOFs, richer compound bounds, and more runtime generic surfaces remain future work. | `post-v1-monomorphization-closure.md`, `post-v1-monomorphization-controls.md`, `post-v1-remaining-language-work.md` |
| Tuples | `tuple<T1,T2,...>` is the sole canonical name. `group` has been removed entirely (language is pre-commercial; no backward compat concern). | Tuple literals, `tuple<...>`, generated C structs, destructuring const, and multi-value match are implemented. | Positional field access remains a separate surface item. | `language-reference.md`, `post-v1-implementation-plan.md` |
| Traits and apply | Traits/apply are the behavior composition model. Default methods and deterministic apply lookup are accepted. Overlapping applies are rejected. | Trait parsing/checking, default methods, method lookup, core traits, and overlap diagnostics are implemented for current subset. | Richer generic trait shapes and compound bounds require incremental hardening. | `post-v1-trait-stability.md`, `post-v1-remaining-language-work.md` |
| Operator overloading Level 2 | Implemented and final only as restricted operator traits: `Addable` for `+`, `Subtractable` for `-`, `Comparable` for `<`, `<=`, `>`, `>=`. Broad function/method/operator overloading remains rejected. | Checker recognizes `Addable`, `Subtractable`, `Comparable`; HIR lowers supported operators to trait method calls; behavior fixtures cover positive and missing-trait errors. | No multiplication/division/modulo/bitwise operator traits. Any expansion needs a new explicit decision because it increases hidden-behavior risk. | `post-v1-trait-stability.md`, `post-v1-implementation-plan.md`, `docs/internal/decisions/language/042-overload-lambdas-and-macros.md` |
| Callables and closures | `func(T) -> R` callable types are accepted. Closures use explicit callable syntax and defined capture rules. Stored callable/closure ABI is defined. | Callable values, local bindings, closure capture subset, nested functions, callback ABI subset, and jobs callback boundary are implemented. | Captured closure callbacks across FFI remain rejected. Closure docs are evidence artifacts; this contract is the reading entry point. | `post-v1-callable-closure-abi.md`, `language-reference.md`, `post-v1-implementation-plan.md` |
| `any` dispatch | Canonical spelling is `any<Trait>`. `dyn` is only migration context/deprecated parser alias. `any` is object-safe trait dispatch, not dynamic field access or universal reflection. | Parser/checker/diagnostics/LSP/emitter use canonical `any` behavior while preserving deprecated `dyn` compatibility. Heterogeneous `list<any<TextRepresentable>>` and user-defined `list<any<Trait>>` baselines are validated for literal, iteration, index, slice, `len`, `std.list.append`, indexed assignment/list-set, and vtable dispatch. | Broader trait shapes, managed-return edge cases beyond the validated subset, mutable scalar `any`, cross-thread guarantees, and generic trait object shapes need future hardening. | `post-v1-any-migration.md`, `post-v1-any-dispatch-stabilization.md`, `post-v1-remaining-language-work.md` |
| Pattern matching | `match value ... end` with `case pattern if condition:` guards. `case else:` is the sole fallback (no `default`). `given` is removed. Supported patterns: literal, binding, enum variant, tuple, simple struct. OR patterns, range patterns, rest/spread patterns, and complex nested patterns are excluded. Every `match` must be exhaustive via complete coverage or `case else`. Guarded cases do not count as exhaustive. Cases are evaluated in order; unreachable cases produce diagnostics. Pattern bindings are scoped to that case's guard and body. | Implemented and covered by behavior fixtures for guards, tuple destructuring, multi-value match, optional/enum patterns. | Further pattern expansion must preserve exhaustiveness and diagnostics contracts. | `post-v1-pattern-matching-closure.md`, `language-reference.md` |
| Error model | `result`, `optional`, `?`, `.or_return`, `.or_wrap`, and panic boundaries are final. No `try/catch`; no `async/await` syntax. | Current checker/lowering enforce compatible propagation and boundary behavior for implemented subset. | Additional diagnostics hardening and FFI/jobs edge cases remain incremental work. | `post-v1-error-model-closure.md`, `diagnostic-code-catalog.md` |
| Resource cleanup | `using` is the public cleanup construct. Cleanup is deterministic and LIFO across return, `?`, panic, and loop-control exits. | Implemented for current C backend scope. | Cross-thread/cross-FFI cleanup ownership must stay explicit as APIs expand. | `post-v1-using-cleanup-semantics.md`, `runtime-model.md` |
| Memory and ownership | Zenith keeps value/managed semantics without ownership keywords. ORC hooks and `std.mem` intent APIs are library-level. | ARC/ORC last-use moves, stable ORC hooks, generic collection subsets, `std.unsafe`, concrete `std.mem` text/list helpers, and compiler-known `mem.own/view/edit` for the finalized Appendix B safe subset are implemented. | Full cycle collection becomes meaningful only when cycle-forming public APIs exist. Enums, optional/result payloads, nested mutable managed values, tuple/struct set keys, managed map values, and allocator resources remain tracked in Appendix B. | `post-v1-runtime-abi-ownership-audit.md`, `post-v1-remaining-language-work.md`, `runtime-model.md`, `implementation-plan.md` |
| Concurrency | Final user direction is typed jobs/channels/shared/atomic using explicit handles, `Transferable`, jobs/channels for async IO, no hidden scheduler, no `async/await`. | Current executable subset has `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>`, and `Atomic<int>`. Type-specialized `_int`/`_text` APIs remain concrete backend/runtime anchors for the current C oracle. | Keep public teaching on typed facades where available; treat specialized runtime names as backend evidence. Wider runtime payload storage, capacity/backpressure, cancellation, and richer panic capture remain future implementation. | `post-v1-concurrency-semantics-closure.md`, `post-v1-remaining-language-work.md`, `stdlib/std/jobs.zt`, `stdlib/std/channels.zt`, `stdlib/std/atomic.zt` |
| FFI | `extern c` is explicit. Callbacks and ABI annotations are accepted. User structs crossing FFI need explicit C representation. Managed values cross only through supported ABI shapes. | Top-level primitive callbacks, immediate C invocation, `attr name`, and `attr abi("cdecl"|"stdcall")` are implemented. | Captured callbacks, managed values, extern vars, varargs, and conditional externs require gated future work as specified. | `post-v1-callable-closure-abi.md`, `post-v1-remaining-language-work.md` |
| Runtime ABI and ZIR | C backend remains the oracle. ZIR/runtime ABI contracts are defined before alternate backends. | Verifier, source mapping contracts, runtime ABI audit, C oracle conformance contract, and closure fixtures exist. | Automate backend conformance runner before Zig/LLVM/WASM activation. Expand golden ZIR fixtures. | `post-v1-zir-consolidation.md`, `post-v1-backend-conformance-suite.md`, `post-v1-source-mapping-contract.md` |
| Standard library boundary | Zenith uses foundation stdlib plus official packages. Protocol foundations may live in stdlib; frameworks/domain libraries live in packages. | Current executable subset includes core stdlib plus implemented time/net/lazy/HOF/memory/concurrency foundations. | HTTP/TLS/WebSocket/server APIs, generic streams/sinks, generic lazy, cross-type HOFs, and package graduation policy implementation remain future work. | `post-v1-remaining-language-work.md`, `stdlib-model.md` |
| Tooling boundary | Tooling is LSP-first and external. No in-process compiler plugins. `zt bench` and `zt migrate` are accepted directions. | Current CLI/LSP/formatter diagnostics exist; docs syntax lint exists. | Public docs are being reset; mature LSP, marketplace extension, web playground, migrator polish, and registry are future work. | `post-v1-remaining-language-work.md`, `tooling-model.md` |

## Resolved Parking Lot (May 2026 audit)

All previously open parking lot items have been resolved:

| Topic | Resolution |
|---|---|
| Symbol budget pressure | All current symbols confirmed final: `?`, `|>`, `..`, `<T>`, `{}`. `|>` kept because `.` method chaining confuses with field access and module paths. `..` kept for slice syntax. `{}` overload kept for map/set/struct. Resolved via teaching/formatting guidance, not restriction. |
| `tuple` versus `group` | `group` removed entirely. `tuple` is the sole canonical name. |
| Operator overloading tension | Level 2 fixed. No expansion without new explicit decision. Docs must show trait name alongside operator. |
| Runtime/backend subset naming | `_int` APIs are backend-only. Public docs teach typed facades only. Compiler should emit diagnostic for unsupported payloads. |
| Public docs reset | Rebuild from scratch. Planned structure: Language Reference, Learn Zenith in 30 Minutes, Cookbook, Stdlib Reference, Tooling Guide, Language Comparison. Separate incremental project: book on the language creation journey. |

---

## Audited Language Surface Contracts (May 2026)

The following sections record normative decisions confirmed during the final language contract audit.

### Control Flow

**Branching:**

- `if cond ... end` — statement with `bool` condition.
- `else if cond` — canonical chaining (not `elif`).
- `else` — fallback branch.
- `if cond then expr else expr` — inline expression form; `else` mandatory; both branches must have compatible types.
- `if cond ... else ... end` — block expression form; same rules.
- `unless` — rejected.

**Loops:**

- `while cond ... end` — conditional loop, `bool` condition.
- `for item in collection ... end` — iteration over `list<T>`, `map<K,V>`, `set<T>`, `text`.
- `for item, second in collection` — second binding: index (`int`) for list/set/text; value (`V`) for map.
- `repeat N times ... end` — fixed-count; count must be integral; evaluated once; `0` = zero iterations; negative = runtime error.
- `while true ... end` — canonical infinite loop (no `loop` keyword).
- Loops are statements, not expressions.

**Loop control:**

- `break` — exits the nearest enclosing loop.
- `continue` — skips to next iteration of the nearest enclosing loop.
- Both valid only inside loops. Both trigger `using` cleanup before jump. No labeled loops in v1.

**Return:**

- `return` — returns void. `return expr` — returns value; type must match function declaration.
- Void functions may omit `return` at the end.

**`range()`:**

- `range(start, end)` — step defaults to 1.
- `range(start, end, step)` — explicit step, may be negative.
- Implemented as `zt_builtin_range2` / `zt_builtin_range3`. No `0..10` range syntax in v1.

**Rejected for v1:** `unless`, `elif`, `loop` keyword, `repeat until`/`do...while`, labeled loops/`break label`, range syntax `0..10`, loops as expressions, C-style `for`, `then` in if-statement (only valid in if-expression).

### Attributes

- Closed set in v1: `test`, `skip`, `deprecated`, `todo`, `name`, `abi`.
- `test`, `skip`, `deprecated`, `todo` — only on `func`.
- `name`, `abi` — only on `extern` func.
- `attr skip` requires `attr test`. `attr skip("message")` accepts optional string.
- `attr deprecated("message")` and `attr todo("message")` require a string argument.
- Unrecognized attributes are errors. Custom user attributes rejected in v1.
- Extension to other targets (structs, enums, traits) is future work.

### Comments

- `--` for line comments (until end of line).
- `--- ... ---` for block comments (no nesting in v1).
- Comments are ignored by the compiler. Comments do not generate documentation.
- `//`, `/* */`, `#`, `///`, `doc "..."` — all rejected.
- Public documentation belongs in ZDoc (`.zdoc`), not in source code.

### Text Interpolation

- `f"text {expr} text"` is the canonical final form.
- `fmt"..."` must be removed entirely (not deprecated — fully removed).
- `{expr}` uses `TextRepresentable` for conversion.
- `{{` for literal `{`. Works with triple-quoted: `f"""..."""`.
- No format specifiers (`:format`) in v1. Empty `{}` is an error. Unterminated `{expr` is an error.

### Type Aliases

- `type Name = Type` is final as a transparent alias, top-level only.
- May be `public`. Resolved to target type at compile time.
- Generic type aliases (`type Pair<T> = ...`) rejected in v1.
- Local type aliases (inside functions) rejected in v1.

### Formatting (`zt fmt`)

- `zt fmt` formats; `zt fmt --check` verifies. Both final.
- 4 spaces indentation. Tabs rejected. Target 100 columns.
- `end` aligns with the opening construct.
- One blank line between top-level declarations.
- Multiline for long signatures/calls/literals (one item per line).
- `case` aligns with `match` (no extra indentation).
- One `attr` per line, no blank line before the declaration.
- No vertical alignment. No per-project style config in v1.

**Naming conventions (guidance, not enforced by `zt fmt` in v1):**

| Element | Convention | Example |
|---|---|---|
| Types | `PascalCase` | `User`, `LoadResult` |
| Enum cases | `PascalCase` | `Success`, `NotFound` |
| Functions | `snake_case` | `load_user` |
| Variables/params/fields | `snake_case` | `user_id` |
| Namespaces | `snake_case` | `app.users` |
| Generic parameters | `PascalCase` descriptive | `Item`, `Key`, `Value` |
| Single-letter generics | Not canonical | `T`, `U`, `E` |

**Rejected for v1:** tabs, vertical alignment, import sorting, per-project style config, naming enforcement via formatter.

### Slice Syntax

- `list[start..end]` — slice with start and end indices.
- `list[start..]` — slice from start to the end.
- `list[..end]` — slice from beginning to end.
- Applies to `list<T>`, `text`, `bytes`.
- Implemented as `ZT_AST_SLICE_EXPR` using `..` (`ZT_TOKEN_DOTDOT`).

---

## Precedence

1. This `final-language-contract.md` is the compact normative index for current final/future/current-subset distinctions.
2. Detailed closure artifacts remain authoritative evidence for their specific topic.
3. `post-v1-remaining-language-work.md` tracks accepted gaps and future implementation.
4. Older decisions preserve rationale but lose conflicts to newer specs and this contract.
5. Public docs must not be rebuilt from old public material without checking this contract first.
