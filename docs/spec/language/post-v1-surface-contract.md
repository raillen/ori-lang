# Zenith Post-v1 Surface Contract

> Audience: contributor, maintainer, language designer
> Status: closure evidence with follow-up decisions
> Surface: spec
> Source of truth: topic evidence; superseded in conflicts by `final-language-contract.md`
> Last updated: 2026-05-03

This document records the accepted post-v1 surface - features, runtime capabilities,
backend targets, and tooling that are explicitly **not** part of v1 but represent the
accepted direction for the language after v1 ships.

Post-v1 is also the language-closure contract: syntax, idiom, semantics, ZIR, compiler invariants, and runtime behavior must be settled here before mature tooling, Zig/LLVM/WASM backends, package registry, optional dependencies, and ecosystem policy become active work.

Wave 7.2 syntax/keyword freeze output is recorded in `post-v1-syntax-freeze.md`.
Wave 7.4 `any` migration policy is closed in `post-v1-any-migration.md`.
Wave 7.5 `any` dispatch stabilization subset is recorded in `post-v1-any-dispatch-stabilization.md`.
Wave 7.7 executable monomorphization closure is recorded in `post-v1-monomorphization-closure.md`.
Wave 7.8 monomorphization controls closure is recorded in `post-v1-monomorphization-controls.md`.
Wave 7.9 callable/closure ABI closure is recorded in `post-v1-callable-closure-abi.md`.
Wave 7.10 `using` cleanup semantics closure is recorded in `post-v1-using-cleanup-semantics.md`.
Wave 7.11 concurrency semantics closure is recorded in `post-v1-concurrency-semantics-closure.md`.
Wave 7.12 error model closure is recorded in `post-v1-error-model-closure.md`.
Wave 7.13 pattern matching closure is recorded in `post-v1-pattern-matching-closure.md`.
Wave 7.14 ZIR consolidation closure is recorded in `post-v1-zir-consolidation.md`.
Wave 7.15 backend conformance closure is recorded in `post-v1-backend-conformance-suite.md`.
Wave 7.16 source mapping closure is recorded in `post-v1-source-mapping-contract.md`.
Wave 7.17 diagnostic contract closure is recorded in `post-v1-diagnostic-contract.md`.
Wave 7.18 runtime ABI and ownership closure is recorded in `post-v1-runtime-abi-ownership-audit.md`.
Wave 7.19 optimization boundary closure is recorded in `post-v1-optimization-boundary.md`.
Wave 7.20 final language closure review is recorded in `post-v1-final-language-closure-review.md`.

Current follow-up decisions in `post-v1-remaining-language-work.md` prevail over older v1 and post-v1 entries when they conflict.

Upstream sources: `v1-surface-contract.md`, `MVP_OUT_OF_SCOPE.md`, `runtime-model.md`,
`concurrency.md`, `ffi.md`, `compiler-model.md`, `backend-scalability-risk-model.md`,
`stdlib-model.md`, decisions 091-094.

---


## Closure Rule

If a topic changes user syntax, accepted idiom, language semantics, ZIR shape, compiler invariants, runtime ownership, cleanup, concurrency boundaries, FFI ABI, or backend-visible representation, it belongs in post-v1 closure.

If a topic only concerns editor UX, packaging, registry, distribution, alternative backend implementation, optimization quality, or ecosystem growth, it belongs after the post-v1 closure gate.

## Status Definitions

- **Accepted Direction**: design intent is clear; spec-level contract exists or is sketched.
- **Exploration**: interest confirmed; no binding spec yet; requires design session.
- **Conditional**: depends on v1 feedback, ecosystem maturity, or prerequisite work.
- **Rejected**: explicitly excluded from the language roadmap. Will not enter post-v1.

---

## 1. Language Surface

### 1.1 Accepted Direction

| Feature | Description | Prerequisite | Source |
|---------|-------------|--------------|--------|
| Generic argument inference | `foo(42)` instead of `foo<int>(42)` - arg-position only, no return-context or partial inference | Checker constraint matching | v7 Dialogo Futuro, 2026-05-01 session |
| First-class callable types | `func(int) -> void` as type syntax. Structural matching. | None | 2026-05-01 session |
| Nested functions | `func` inside `func`. Captures parent scope (immutable). Sugar for `const name = func(...)`. | None | 2026-05-01 session |
| Pattern matching: destructuring | `const (x, name) = get_pair()` | Tuple maturity | 2026-05-01 session |
| Pattern matching: multi-value | `match (status, code):` | Destructuring | 2026-05-01 session |
| Pattern matching: guard clauses | `case x if x > 0:` | None | 2026-05-01 session |
| Operator overloading (Level 2) | `Comparable` (`<`,`>`,`<=`,`>=`) + `Addable`/`Subtractable` (`+`,`-`). No full overloading. | Trait maturity | 2026-05-01 session |
| Pipe operator | `value \|> transform` - classic functional pipe | Callable types | 2026-05-01 session |
| Ternary expressions | `if cond then a else b` - single-line form | None | 2026-05-01 session |
| `tuple` naming | `tuple` is canonical. `group` is removed from the active surface. | None | 2026-05-01 session |
| `@field` self shorthand | `@x` as sugar for `self.x`. Both forms valid. Only inside `apply` blocks. | None | 2026-05-01 session |
| `size_of`/`type_name` improvements | Real implementations replacing current placeholders | None | 2026-05-01 session |

### 1.2 Exploration

| Feature | Open Question | Source |
|---------|---------------|--------|
| Mutable closure capture v2 | Current `capture` sufficient? Revisit if v1 feedback demands more. | Decision 090 |

### 1.3 Rejected (Not Post-v1 Either)

| Feature | Reason | Source |
|---------|--------|--------|
| `char` type | `text` with helpers suffices; adding `char` fragments the string model | v7 Rejected |
| `?.` safe navigation | Dense symbol; use `match` with `case some(x)` | v7 Rejected |
| `??` null coalescing | Use `.or_return` / `.or_wrap` / explicit match | v7 Rejected |
| Implicit return | Explicitness is a core value | v7 Rejected |
| `try/catch` | `result<T,E>` + `?` is the error model | v7 Rejected |
| `async/await` as keywords | Concurrency uses workers/jobs/channels, not colored functions | v7 Rejected |
| `owned<T>` / `borrow<T>` / lifetimes | Language-level ownership rejected; optional `std.mem` API instead | v7 Rejected, 2026-05-01 session |
| Macros | Not in language philosophy | v7 Rejected |
| Method/function overloading | Explicit naming preferred | v7 Rejected |
| `uint` as standalone type | Use `u8`-`u64` aliases | v7 Rejected |
| Rest operator (`...`) | Not syntax | v7 Rejected |
| Postfix guard `return x if cond` | Not syntax | language-reference.md |
| `unpack` destructuring on `const` | Not syntax | language-reference.md |
| Full type inference | Explicit `: type` is a core value; no `const x = 42` | 2026-05-01 session |
| Struct type omission `{ fields }` | Violates words > symbols, one form, explicit behavior (3 of 4 rules) | 2026-05-01 session |
| Variadic parameters | Use explicit `list<T>` argument | 2026-05-01 session |
| Selective imports | Qualified imports only; always know where symbols come from | 2026-05-01 session |
| `unless` keyword | `if not` is already clear; two forms for same thing | 2026-05-01 session |
| Math operators (`**`, `//`) | Use `math.pow()` and regular division | 2026-05-01 session |
| C-style `for` loops | `for x in range()` covers this | 2026-05-01 session |
| Named tuple fields | If you need names, use struct; tuples are positional | 2026-05-01 session |
| Wildcard imports | Violates qualified-imports-only rule | 2026-05-01 session |
| JavaScript backend | WASM covers web use case; JS backend = massive effort, limited value | 2026-05-01 session |

---

## 2. Concurrency

### 2.1 Accepted Direction (Phased)

| Phase | Surface | Status | Source |
|-------|---------|--------|--------|
| Phase 2 | Checker understands `Transferable` boundaries and emits diagnostics for invalid crossings | Done for current C-backend surface; closure audit remains | concurrency.md |
| Phase 3 | `jobs.spawn(fn[, value])` / `jobs.join(job)` typed facade over current runtime handles | Done for current C-backend surface; non-`int` runtime payload strategy remains closure work | concurrency.md |
| Phase 4 | `channels.create<T>` / `send` / `receive` / `close` typed facade over current runtime handles | Done for current C-backend surface; capacity/backpressure/cancellation semantics remain closure work | concurrency.md |
| Phase 5 | `Shared<T>` and `Atomic<T>` typed facades, with `Atomic<int>` currently supported | Done for current C-backend surface; full generic shared/atomic payloads remain closure work | concurrency.md |

### 2.2 Explicitly Not Planned

- Raw thread handles
- Mutex/condvar-first programming
- Implicit cross-thread sharing of managed values
- Reintroduction of `global`
- `async/await` colored function model

---

## 3. Runtime

### 3.1 Accepted Direction

| Feature | Description | Prerequisite | Source |
|---------|-------------|--------------|--------|
| ORC (Ownership-aware Reference Counting) | Skip `weak<T>` -> build move analysis + cycle detector directly | Componentized runtime | 2026-05-01 session |
| Thread-safe reference counting | Atomic ARC for `Shared<T>` wrapper | Concurrency Phase 5 | runtime-model.md |
| Generic transferable-copy | `std.concurrent.copy<T>` replacing per-type helpers | `transferable` predicate in checker | concurrency.md |
| `map` specializations | Beyond `map<text,text>` - generic key/value | C backend generic map | MVP_OUT_OF_SCOPE |
| `list<float>` specialization | Runtime C specialization | C backend templates | MVP_OUT_OF_SCOPE |
| `optional<float>`, `optional<bool>` specializations | Stack-optimized layout | Runtime maturity | MVP_OUT_OF_SCOPE |
| Struct runtime generic field access | Reflection-like field iteration | Design session | MVP_OUT_OF_SCOPE |
| Stack-optimized `result<void,E>` | Success path zero-alloc | Runtime maturity | runtime-model.md |
| Componentized runtime | Split `zenith_rt.c` into modules with Unity Build - **before ORC** | None | 2026-05-01 session |
| `std.unsafe` module | Escape hatch for raw pointer ops, FFI edge cases | None | 2026-05-01 session |
| `own`/`view`/`edit` in `std.mem` | Optional manual memory API for advanced users. Library-level, not keywords. | ORC | 2026-05-01 session |

### 3.2 Decided Follow-up

| Feature | Decision | Source |
|---------|---------------|--------|
| Custom allocator hooks | Advanced allocation is accepted as explicit library API, not language syntax. `mem.Temp` and `mem.Pool` are the preferred intent-oriented resources. | post-v1-remaining-language-work.md |
| Hot-reload runtime | Experimental opt-in tooling only; not a v1 language/runtime contract. | post-v1-remaining-language-work.md |

---

## 4. FFI

### 4.1 Accepted Direction (Phased)

| Phase | Surface | Status | Source |
|-------|---------|--------|--------|
| FFI 2 | Arity and invalid-return negative fixtures; explicit runtime helper matrix | Covered by current diagnostics/hardening where applicable | ffi.md |
| FFI 3 | Narrow callbacks / function pointers in `extern c` declarations | Done for top-level primitive/text/bytes callbacks; captured closure callbacks remain rejected | ffi.md |
| FFI 4 | ABI annotations (`cdecl`, `stdcall`) and symbol renaming (`name("other")`) | Done for current C backend | ffi.md |

### 4.2 Decided Follow-up

| Feature | Decision | Source |
|---------|---------------|--------|
| User-defined struct as `extern c` argument | Allowed only with explicit C representation annotation and FFI-safe fields. | post-v1-remaining-language-work.md |
| `extern` variables | Read-only C globals may be `extern const`; mutable globals require unsafe gates. | post-v1-remaining-language-work.md |
| Variadic `extern` functions | Raw C varargs only behind unsafe gates; public Zenith APIs should expose typed wrappers. | post-v1-remaining-language-work.md |
| Conditional `extern` per target | Use declarative target/cfg attributes and inspectable package/project provider selection. | post-v1-remaining-language-work.md |

---

## 5. Backend Targets After Closure

### 5.1 Accepted Direction

Backend targets are accepted strategic directions, but they are not active implementation work until the post-v1 closure gate is complete. C remains the behavior oracle while future backends prove conformance.

| Target | Post-closure role | Prerequisite | Source |
|--------|-------------------|--------------|--------|
| Zig backend | Textual systems backend exploration; must not reshape Zenith semantics | Closed ZIR contract + backend conformance suite | 2026-05-01 backend discussion |
| LLVM backend | Strategic optimized native release backend | Closed ZIR contract + backend conformance suite | v7 Dialogo Futuro |
| WASM backend | Sandbox/web target for playground and web deployment | Runtime portability audit + LLVM or direct ZIR-to-WASM route | v8 FUT.12 |
| Cranelift backend | Fast native backend spike for dev-mode validation | Backend conformance suite | 2026-05-01 backend discussion |
| C3 backend | Lower-priority textual C-like experiment | Stable C oracle + toolchain maturity | 2026-05-01 backend discussion |

### 5.2 Closure Blockers

No backend target becomes active until these are settled:

- ZIR verifier and canonical backend-visible type model;
- ownership/runtime operation representation;
- closure/function/FFI ABI representation;
- source-span/debug-info expectations;
- runtime ABI documentation;
- backend conformance suite using C as behavior oracle.

---

## 6. Standard Library

### 6.1 Accepted Direction

| Module / Feature | Description | Source |
|------------------|-------------|--------|
| `std.net` expansion | Blocking TCP client is implemented; TLS, UDP, and server APIs remain future network work | v1 M12 decision |
| `std.time` expansion | MVP `Instant`/`Duration` surface is implemented; calendar/date APIs remain future work | MVP_OUT_OF_SCOPE |
| Generic HOFs | Same-type primitive/text HOFs are implemented; cross-type `map<T,U>` and generic `reduce<T>` require full monomorphization | v8 COL.06 |
| Generic stream abstraction | Composable IO/data pipelines; prefer monomorphized `Stream<T>` first, with `any`-erased adapters only after `any` dispatch is stable | stdlib-model.md |
| Async IO | No `async/await` keywords; use jobs + channels as the concurrency substrate for background blocking IO and completion/data delivery | stdlib-model.md, concurrency.md |
| TLS | `std.net.tls` or integrated secure socket layer after stream/error/certificate model is designed | stdlib-model.md |
| WebSocket | `std.net.websocket` over `std.net` + streams/channels; blocking core first, jobs/channels for concurrency | stdlib-model.md |
| `std.console` cursor movement | Terminal cursor positioning | surface-implementation-status.md |
| Lazy iterators | Composable lazy evaluation layered on streams/lazy values | stdlib-model.md |
| Generic lazy | `lazy<T>` for any `T`; requires executable generic monomorphization | lazy.md |

### 6.2 Decided Follow-up

| Module / Feature | Decision | Source |
|------------------|---------------|--------|
| `std.regex` expansion | Regex remains bounded stdlib utility; heavy engines belong in packages. | post-v1-remaining-language-work.md |
| `std.crypto` | Narrow crypto foundation may live in stdlib; high-level crypto belongs in packages. | post-v1-remaining-language-work.md |
| `std.image` | Belongs in official packages, not core stdlib. | post-v1-remaining-language-work.md |
| `std.db` | Database abstractions belong in packages, not core stdlib. | post-v1-remaining-language-work.md |

---

## 7. Compiler

### 7.1 Accepted Direction

| Feature | Description | Source |
|---------|-------------|--------|
| Incremental compilation | File-level or module-level change detection | MVP_OUT_OF_SCOPE |
| Source maps / debug info | Beyond current `#line` directives - DWARF, PDB | MVP_OUT_OF_SCOPE |
| Monomorphization controls | Instance cache, dedup, recursive guard, build report | backend-scalability-risk-model.md |
| Exhaustive match diagnostics | Missing enum case compile-time error | backend-scalability-risk-model.md |
| Optimization passes | Beyond what the C compiler provides | MVP_OUT_OF_SCOPE |

### 7.2 Decided Follow-up

| Feature | Decision | Source |
|---------|---------------|--------|
| Plugin / extension system | External declarative tools only; no arbitrary in-process compiler plugins. | post-v1-remaining-language-work.md |
| Cross-compilation | Target triple model, sysroot management | compiler-model.md |
| Separate compilation units | Split C output for parallel compilation | MVP_OUT_OF_SCOPE |

---

## 8. Tooling

### 8.1 Accepted Direction

| Tool | Description | Source |
|------|-------------|--------|
| VSCode extension (Marketplace) | Syntax highlighting, diagnostics, go-to-definition | v1-surface-contract.md T.01 |
| LSP full production | Move from beta to stable | v1-surface-contract.md |
| Web playground | Browser-based Zenith REPL; deferred until backend/WASM route is ready | v7 Dialogo Futuro |
| ZPM registry web | Ecosystem item; intentionally late after language/runtime/package model stabilizes | v7 Dialogo Futuro |

### 8.2 Decided Follow-up

| Tool | Decision | Source |
|------|---------------|--------|
| Helix / Neovim / Zed configs | LSP-first policy; editor adapters are best-effort and do not define semantics. | post-v1-remaining-language-work.md |
| `zt bench` | Minimal stable runner belongs in core CLI; advanced analysis belongs in packages/tools. | post-v1-remaining-language-work.md |
| `zt migrate` | Handles mechanically safe migrations with dry-run/patch review. | post-v1-remaining-language-work.md |
| Borealis Studio integration | External tooling that consumes stable Zenith protocols; it does not define language semantics. | post-v1-remaining-language-work.md |

---

## Ordering Principles

Post-v1 work should follow these priorities:

1. **Foundation** - componentized runtime, ternary, `@field`, nested functions, builtin improvements.
2. **Type system** - callable types, generic inference, pattern matching, operator overloading, pipe.
3. **Runtime + memory** - ORC hooks/move analysis, generic collections, `std.unsafe`, `std.mem`.
4. **Concurrency** - jobs, channels, shared state, atomics, `Transferable`, and typed handle facades.
5. **FFI** - callbacks and ABI annotations.
6. **Stdlib foundation** - executable C-backend HOFs, `std.time`, lazy primitives/text, and blocking `std.net`.
7. **Language/ZIR/compiler/runtime closure** - decide and audit every remaining language contract topic.
8. **Core IO/dataflow stdlib** - streams, async-via-jobs/channels, lazy iterators, WebSocket, TLS where justified.
9. **Developer tooling** - LSP, VSCode, diagnostics, migration tooling.
10. **Backend targets** - Zig/LLVM/WASM/Cranelift/C3 only after conformance tests.
11. **Ecosystem** - package registry, optional deps, feature flags, and package graduation policy come last.

Full wave breakdown in `post-v1-implementation-plan.md`.

Each unresolved post-v1 closure item requires a design session and decision document before implementation begins. No new language/compiler feature should enter the compiler without a numbered decision in `docs/internal/decisions/language/`.

---

## Relationship To v1 Contract

Current post-v1 and remaining-work decisions prevail over the historical
`v1-surface-contract.md` whenever they intentionally supersede older v1 scope
or deferral language.

