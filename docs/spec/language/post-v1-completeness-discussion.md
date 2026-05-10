# Zenith Post-v1 Completeness Discussion

> Audience: contributor, maintainer, language designer
> Status: draft
> Surface: spec
> Last updated: 2026-05-03

This document is the companion to `post-v1-surface-contract.md`. While the contract
says **what** is post-v1, this document discusses **why**, **when**, **how**, and
**what risks** each area carries. It serves as the starting point for future design
sessions.

> 2026-05-02 roadmap correction: this document is discussion, not final status.
> The current source of truth rejects full local type inference and struct type omission, accepts argument-position generic inference, and treats post-v1 as the final language/ZIR/compiler/runtime closure window before mature tooling, Zig/LLVM/WASM backends, and ecosystem work.
> Wave 7.2 syntax/keyword freeze is now recorded in `post-v1-syntax-freeze.md`.
> Wave 7.3 idiom consolidation is tracked in `post-v1-idiom-pass.md`.
> Wave 7.4 `any` migration policy is closed in `post-v1-any-migration.md`.
> Wave 7.5 current `any` dispatch stabilization is recorded in `post-v1-any-dispatch-stabilization.md`.


## 0. Post-v1 Closure Doctrine

Post-v1 is not a general backlog. It is the point where Zenith must stop accumulating unresolved language questions and close the language itself.

By the end of post-v1, the following must be defined and internally consistent:

- **Syntax:** final accepted/rejected syntax, keywords, contextual keywords, operators, literals, declarations, imports, attributes, and shorthand forms.
- **Idioma:** canonical Zenith style for error handling, resource cleanup, traits, `any`, generic APIs, concurrency, modules, and stdlib design.
- **Semantics:** evaluation order, initialization, ownership/ARC/ORC, cleanup, closures, captures, generic instantiation, trait resolution, dynamic dispatch via `any`, errors, pattern matching, and concurrency boundaries.
- **ZIR:** canonical IR types, explicit control flow, ownership/runtime operations, verifier invariants, textual dump expectations, and backend conformance fixtures.
- **Compiler:** parser, binder, checker, HIR, ZIR lowering, emitter, diagnostics, hardening tests, and negative fixtures aligned with the surface contract.
- **Runtime:** componentized C oracle runtime, ABI, memory model, collections, concurrency, FFI, lazy, net/time foundations, cleanup, and platform boundaries stable enough for alternative backends.

After this closure gate, work should shift to mature-language concerns: LSP, editor integrations, formatter/migrator, Zig/LLVM/WASM backends, package registry, optional dependencies, distribution, and ecosystem policy.

### Closure questions and current answers

The following topics were high-risk enough to require explicit post-v1 decisions before the language could be considered closed:

1. **`any` migration (closed in Wave 7.4):** `post-v1-any-migration.md` defines canonical `any`, deprecated `dyn` alias behavior, and diagnostic/tooling surface policy.
2. **`any` safety (current subset closed in Wave 7.5):** `post-v1-any-dispatch-stabilization.md` closes the current subset; advanced-shape policy remains open (managed returns, broader trait shapes, cross-boundary guarantees).
3. **Trait coherence (closed in Wave 7.6):** `post-v1-trait-stability.md` defines method lookup, defaults, overlapping apply policy, operator traits, and `Transferable`.
4. **Generic monomorphization (closed in Waves 7.7 and 7.8):** `post-v1-monomorphization-closure.md` and `post-v1-monomorphization-controls.md` define executable instantiation, inference boundaries, and controls.
5. **Closure/callable ABI (closed in Wave 7.9):** `post-v1-callable-closure-abi.md` defines storage, FFI, and jobs boundary rules.
6. **Resource cleanup (closed in Wave 7.10):** `post-v1-using-cleanup-semantics.md` defines deterministic cleanup under return, `?`, panic, and loop control.
7. **Concurrency semantics (closed in Wave 7.11):** `post-v1-concurrency-semantics-closure.md` defines channels, jobs, cancellation policy, panic boundary, `Transferable`, and typed payload strategy.
8. **Error model completeness (closed in Wave 7.12):** `post-v1-error-model-closure.md` defines `result`, `optional`, `?`, helpers, panic boundaries, and interop.
9. **Pattern matching (closed in Wave 7.13):** `post-v1-pattern-matching-closure.md` defines exhaustiveness, guards, payload binding, optional matching, multi-value matching, and diagnostics.
10. **ZIR contract (closed in Wave 7.14):** `post-v1-zir-consolidation.md` defines verifier rules, ownership/runtime ops, call ABI representation, generic representation, and textual stability.
11. **Backend, source, diagnostics, runtime, and optimization contracts (closed in Waves 7.15-7.19):** conformance, source mapping, diagnostics, runtime ABI, and optimization boundaries are now explicit.
12. **Remaining ecosystem questions:** Stdlib boundary, editions/deprecation, streams/async IO APIs, WebSocket, and TLS remain future design sessions after the language closure gate.

Upstream sources: all documents referenced in the surface contract, plus
`backend-scalability-risk-model.md`, `cognitive-accessibility.md`,
`legibility-evaluation.md`.

---

## 1. Type System Completeness

### 1.1 Explicit Type Policy

**Closure decision:** Full local type inference is rejected. `const x = 42` remains outside the language. Variable declarations keep explicit `: type` annotations.

**Reason:** Zenith prioritizes explicit behavior, predictable diagnostics, and low ambiguity over minimal syntax. Type information should remain visible at declaration boundaries.

**Post-v1 work:**

- ensure parser diagnostics for missing `: type` are stable and helpful;
- ensure examples and docs do not imply local inference;
- keep function signatures, struct fields, public vars, and local declarations explicit;
- remove stale discussion that treats local type inference as an accepted future feature.

### 1.2 Generic Argument Inference

**Closure decision:** Accepted only for argument-position inference. No return-context inference. No partial inference unless a later explicit decision changes this.

**Required closure details:**

- all generic parameters must be inferable from provided arguments;
- ambiguity produces a stable diagnostic;
- trait constraints are checked after inference;
- no inference from assignment target or return type;
- backend-emittable generic calls require monomorphization support.

### 1.3 Callable Types As First-Class Values

**Closure decision:** `func(T) -> U` is the canonical callable type syntax with structural matching.

**Required closure details:**

- storage rules for callable values;
- capture rules for immutable and `capture` state;
- whether callable values can enter lists/maps/results/options;
- ABI distinction between Zenith closures, runtime callback closures, and raw `extern c` function pointers;
- diagnostics for escaping unsupported closures.

### 1.4 Struct Literal Shorthand

**Closure decision:** Struct type omission with bare `{ fields }` is rejected. Constructors remain explicit.

**Reason:** It weakens explicit behavior and creates ambiguity pressure with maps, blocks, and expected-type propagation.

**Post-v1 work:**

- ensure `language-reference.md`, examples, and diagnostics agree that explicit `StructName(...)` construction is canonical;
- keep enum dot shorthand and other shorthand proposals out unless separately accepted by numbered decision;
- reject stale docs that describe `{ fields }` as accepted.

### 1.5 Type System Closure Checklist

Before the language is closed, verify:

- generic constraints syntax and semantics are final;
- `where` and `is` roles are unambiguous;
- `Transferable` is consistently documented as a predicate/core trait;
- `any` is the only user-facing dynamic dispatch term;
- operator traits are limited to the accepted Level 2 surface;
- tuple semantics are final (`group` removed);
- optional/result/lazy/list/map generic behavior is consistent with monomorphization.

## 2. Concurrency Completeness

### 2.1 Phase 2 - Transferable Predicate

**Current state:** Groundwork in checker. No surface diagnostic for non-transferable boundary violations.

**Why next:**
- Foundation for all later concurrency work.
- Users cannot get meaningful errors today when passing non-transferable values.

**Risks:**
- Low. This is internal plumbing with surface diagnostics.

**Recommended approach:**
- Add `transferable` predicate to the checker type system.
- Emit `concurrency.not_transferable` diagnostic when a non-transferable value reaches a boundary.
- Cover all shapes listed in `concurrency.md` Section "Transferable shapes".

**Prerequisites:** None beyond current groundwork.

**Estimated complexity:** Medium.

### 2.2 Phase 3 - Jobs

**Current state:** Not started.

**Why:**
- First real user-facing concurrency primitive.
- Enables CPU-bound parallelism (navmesh building, asset processing, batch computation).
- Copy-based - no new ownership model needed.

**Risks:**
- Error propagation across job boundary: `jobs.join(job)?` must carry `result<T,E>`.
- Cancellation semantics: what happens when the spawning scope exits?
- Resource cleanup: `using` blocks inside a job must still run.

**Open questions:**
1. Does `jobs.spawn` accept any callable, or only named functions?
2. Is there a job pool, or each spawn creates a new OS thread?
3. How does panic in a job propagate? Fatal to the parent isolate?

**Recommended approach:**
- Start with named functions only (no closures with mutable capture across boundary).
- Thread pool managed by runtime (not 1:1 with OS threads).
- Panic in a job returns `error(...)` to the `join` call, not fatal to parent.

**Estimated complexity:** High.

### 2.3 Phase 4 - Channels

**Current state:** Not started.

**Why:**
- Producer/consumer patterns, streaming data between workers.
- Required for non-trivial concurrent applications.

**Risks:**
- Deadlock potential with unbounded channels.
- Backpressure model: bounded vs unbounded.
- Channel close semantics.

**Open questions:**
1. Bounded or unbounded channels by default?
2. `channels.receive` blocking vs `try_receive` non-blocking?
3. Can channels carry `any<Trait>` or only transferable shapes?

**Estimated complexity:** High.

### 2.4 Phase 5 - Shared State

**Current state:** Not started.

**Why:**
- Narrow use cases where copy-based transfer is too expensive (e.g., shared counters, configuration).
- `Shared<T>` and `atomic<T>` complete the concurrency model.

**Risks:**
- Breaks the "no shared mutable state" simplicity that makes Zenith easy.
- Must be visibly opt-in and documented as an advanced feature.
- `Shared<T>` needs a lock - mutex or reader-writer?

**Recommended approach:**
- `Shared<T>` uses an internal mutex. Access via `.read(func(value: T) -> R)` and `.write(func(value: mut T) -> R)`.
- `atomic<T>` for `int`, `float`, `bool` only - no managed types.
- Surface explicitly warns: "you probably want jobs + channels first."

**Estimated complexity:** High.

---

## 3. Runtime Completeness

### 3.1 Cycle Collection

**Current state:** RC cycles are a documented leak risk. No collector.

**Why critical:**
- Any graph-like data structure (trees with parent pointers, observer patterns, UI component hierarchies) can cycle.
- Borealis game engine will need this for entity/component graphs.

**Options (mutually exclusive starting points):**

| Option | Pros | Cons |
|--------|------|------|
| `weak<T>` | Explicit, zero overhead on non-weak paths | User must choose where to break cycles |
| Trial deletion (Lins/Bacon) | Automatic, no user annotation | Complex implementation, scan pauses |
| Arenas | Bulk deallocation, simple | Lifetime scoping is manual |
| Constrained ownership graphs | Prevents cycles by construction | Limits expressiveness |

**Recommended approach:**
- Ship `weak<T>` first - explicit, low risk, covers the majority of practical cases.
- Investigate trial deletion as a post-`weak<T>` addition for APIs where cycle location is not obvious.
- Arenas are orthogonal and can land independently for performance use cases.

**Prerequisites:** Stable ARC paths, FFI shielding compatible with weak references.

**Estimated complexity:** High (for any option).

### 3.2 Generic Collection Runtime

**Current state:** `map<text,text>` only. `list<int>`, `list<text>`, `list<float>` exist. No generic instantiation at runtime level.

**Why:**
- Users cannot create `map<int, list<text>>` or `map<text, MyStruct>`.
- Blocks real-world application development.

**Risks:**
- Monomorphization of runtime C code for every map/list instantiation.
- Code bloat if not controlled.

**Recommended approach:**
- Type-erased internal representation with thin monomorphized wrappers for type safety.
- Instance cache + dedup per `backend-scalability-risk-model.md`.

**Estimated complexity:** High.

### 3.3 Componentized Runtime Build

**Current state:** Monolithic `zenith_rt.c`.

**Why:**
- Compile times grow linearly with runtime size.
- Modularity enables conditional inclusion (no net runtime for CLI tools).
- Enables separate compilation units.

**Recommended approach:**
- Unity Build as the default (single TU, include-based).
- Optional split build for development (`-DZENITH_SPLIT_BUILD`).
- Stable `zenith_rt.h` facade that doesn't change between modes.

**Estimated complexity:** Medium.

---

## 4. FFI Completeness

### 4.1 Callbacks (FFI Phase 3)

**Current state:** `extern c` declarations cannot include function pointer parameters.

**Why:**
- Required for C library interop (event handlers, comparison functions in `qsort`, iterator callbacks).
- Blocks broader ecosystem integration.

**Risks:**
- Callback lifetime: who owns the closure? When is it safe to release?
- Re-entrancy: C callback calling back into Zenith managed code.
- GC/ARC interaction during callback execution.

**Open questions:**
1. Surface syntax: `func(callback: extern func(int) -> int)` or separate type?
2. Can closures with captures be passed as C callbacks? (Requires trampoline.)
3. What happens if C stores the callback pointer beyond the call duration?

**Prerequisites:**
- First-class callable types (1.3) should land first for consistent syntax.
- Runtime must support trampoline generation for closure-to-C-function-pointer conversion.

**Estimated complexity:** Very High.

### 4.2 ABI Annotations (FFI Phase 4)

**Current state:** No `__stdcall`/`__cdecl` control.

**Why:**
- Required for Windows API interop.
- Required for embedded/OS-level programming.

**Risks:**
- Platform-specific surface leaking into the language.
- Testing matrix explodes.

**Recommended approach:**
- Attribute-based: `attr abi("stdcall")` before an `extern c` block.
- Not a keyword - keeps core syntax clean.

**Estimated complexity:** Medium.

---

## 5. Backend Targets After Closure

Backend targets are accepted strategic directions, but they are not active post-v1 closure work. They become mature-language work only after syntax, semantics, ZIR, compiler invariants, and runtime ABI are closed.

### 5.1 Closure prerequisite

Before Zig, LLVM, WASM, Cranelift, or any other backend becomes active, Zenith needs:

- a stable ZIR verifier;
- canonical backend-visible type representations;
- explicit ZIR ownership/runtime operations;
- stable closure/function/FFI ABI representation;
- a backend conformance suite using the C backend as behavior oracle;
- source-span and diagnostic mapping expectations;
- runtime ABI docs for ARC/ORC, collections, options/results, lazy, jobs/channels, net/time, and FFI.

### 5.2 LLVM backend

**Why valuable after closure:**

- Production-quality optimizations (O2/O3/LTO) without depending on C compiler quality.
- Debug info (DWARF/PDB) integration for proper IDE debugging.
- Opens path to WASM, ARM, and other targets.

**Constraint:** LLVM must implement Zenith semantics as defined by closed ZIR. LLVM must not become a second semantic source of truth.

### 5.3 WASM backend

**Why valuable after closure:**

- Web playground.
- Browser-based tooling and education.
- Serverless/edge deployment.

**Constraint:** WASM requires a settled runtime portability story. ARC/ORC, IO, host APIs, and memory layout must be represented without changing language semantics.

### 5.4 Cranelift backend

**Why valuable after closure:**

- Fast native backend spike for dev-mode compile speed.
- Useful for validating the ZIR-to-native lowering model before full LLVM investment.

**Constraint:** Cranelift remains a spike path until it passes the same backend conformance suite as C/LLVM.

### 5.5 Zig backend

**Why valuable after closure:**

- Textual backend close to C but with stronger structure, explicit allocation, and better cross-compilation ergonomics.
- Good candidate for a safer generated systems target.

**Constraint:** Zig is a target language, not a design authority. Zig semantics must not reshape Zenith semantics.

### 5.6 C3 backend

**Why valuable after closure:**

- C-like textual backend candidate with a more modern systems-language surface than C.

**Constraint:** Lower priority than Zig/LLVM unless C3 demonstrates clear toolchain and maintenance advantages.

## 6. Stdlib Completeness

### 6.1 Priority Ordering

Post-v1 stdlib expansion should follow user demand, but it must not outrun the language/runtime foundations. Current corrected priority:

1. **Generic HOFs** - executable primitive/text same-type subset is implemented; full `map<T,U>` waits for monomorphization.
2. **`std.time` MVP** - implemented for `Instant`, `Duration`, unix conversion, arithmetic, and sleep; calendar/date APIs can wait.
3. **`std.net` blocking TCP client** - implemented as the first network foundation; TLS, UDP, and server APIs remain separate future work.
4. **Generic monomorphization + trait/`any` stabilization** - required before real generic streams, generic lazy, and erased stream adapters.
5. **Generic stream abstraction** - dataflow primitive for IO, lazy iterators, WebSocket, and future network APIs. Prefer generic monomorphized `Stream<T>` first; add `any`-erased adapters only after `any` dispatch is stable.
6. **Async IO via jobs + channels** - Zenith rejected `async/await` keywords. The intended route is background jobs doing blocking IO and channels carrying data, completion, errors, cancellation, and backpressure signals.
7. **WebSocket** - protocol layer over `std.net` + streams/channels. Start blocking/core first, then expose concurrent helpers with jobs/channels.
8. **TLS / secure sockets** - after stream/error/certificate model is clear.
9. **Package registry and optional dependencies** - intentionally late ecosystem work; avoid locking package metadata before the language/runtime model stabilizes.

#### Async IO route correction

Async IO in Zenith should not introduce `async`/`await` syntax or colored functions. The concurrency model already provides the mental model: jobs are independent work and channels are pipes. For IO this means:

- a job owns a blocking `std.net.Connection` or file handle;
- the job sends `bytes`, messages, progress, or `result<T,E>` through a `Channel<T>`;
- callers coordinate with `join`, `receive`, `close`, explicit timeout values, and explicit cancellation messages;
- no implicit sharing of managed state crosses the boundary; values must satisfy `Transferable`.

This keeps async IO aligned with the language directives: explicit control flow, no hidden scheduler syntax, no implicit shared mutable state, and no `async/await` keywords.

#### WebSocket route

WebSocket should be implemented as an ordinary protocol library, not as syntax:

1. blocking handshake and frame codec over `std.net`;
2. `Message` enum for `text`, `binary`, `ping`, `pong`, `close`;
3. channel-based helpers for concurrent read/write loops;
4. TLS-backed `wss://` only after secure sockets are designed;
5. all errors are `result<T, websocket.Error>` and all cleanup uses normal resource rules.

#### Streams: generic monomorphized vs `any Stream`

The best first design is **generic monomorphized streams**. They preserve static types, avoid vtable overhead, work with future HOF/lazy APIs, and fit the current backend direction.

`any Stream` should be a later adapter, not the core model. Current `any` support is still constrained: generic traits are not `any`-safe, mutable methods are not `any`-safe, and backend coverage for heterogeneous user-trait collections needs stabilization. A type-erased stream can still be useful for plugin-like heterogeneous pipelines, but only after `any` dispatch is stable and either generic traits become supported or the erased stream trait is intentionally non-generic, such as `ByteStream`/`TextStream`.

### 6.2 Stdlib vs Package Decision Framework

Not everything belongs in stdlib. Decision criteria:

| Criterion | stdlib | Package |
|-----------|--------|---------|
| Needed by >50% of programs? | Yes | No |
| Stable API unlikely to change? | Yes | No |
| Platform-specific? | Only if abstracted | Yes |
| Requires C bindings? | Only via `extern host` | Yes, via `extern c` |
| Can be updated independently? | No (ships with compiler) | Yes |

Candidates for **package, not stdlib:**
- `std.crypto` -> `zenith-crypto` package
- `std.image` -> `zenith-image` package
- `std.db` -> `zenith-db` package
- `std.xml` -> `zenith-xml` package

---

## 7. Compiler Completeness

### 7.1 Incremental Compilation

**Why:**
- Build times will grow with project size.
- Edit-compile-run cycle must stay under 2 seconds for moderate projects.

**Risks:**
- Namespace-level granularity may not be fine enough.
- Cache invalidation for cross-namespace dependencies is tricky.
- Generic instantiation complicates caching (same generic, different instantiation in different files).

**Recommended approach:**
- File-level hashing for change detection.
- Namespace-level rebuild granularity.
- Full rebuild when a public struct/trait/enum signature changes.

**Estimated complexity:** High.

### 7.2 Optimization Passes

**Current state:** All optimization delegated to the C compiler.

**Why:**
- Language-specific optimizations (dead code elimination, constant propagation on `where` contracts, closure inlining) cannot be done by the C compiler.
- LLVM backend makes this less urgent but doesn't eliminate the need entirely.

**Recommended approach:**
- ZIR-level optimization passes before backend lowering.
- Start with: dead code elimination, constant folding, contract propagation.
- Measure before optimizing - profile-guided priorities.

**Estimated complexity:** Medium per pass, High total.

---

## 8. Cognitive Accessibility Post-v1

Per `cognitive-accessibility.md`, post-v1 features must pass the same accessibility bar:

- **Type inference** must not produce confusing error messages at a distance.
- **Concurrency** must have clear mental models - "jobs are independent copies, channels are pipes."
- **Callbacks in FFI** must have clear ownership language - "you lend, C borrows."
- **New syntax** must pass legibility evaluation before acceptance.
- **Diagnostics** for post-v1 features must use the ACTION / WHY / NEXT format.

Every post-v1 feature gets a legibility review before its decision is accepted.

---

## 9. Versioning And Compatibility

### 9.1 Stability Promise

After v1 ships, Zenith must:

- Not break existing v1 programs without a deprecation cycle.
- Provide `zt migrate` for automated syntax updates.
- Use edition-based compatibility if necessary (Rust-style).

### 9.2 Deprecation Model

1. Feature marked `@deprecated("use X instead")` in one release.
2. Warning emitted for one release cycle.
3. Removed in the next major version.
4. `zt migrate` handles automated conversion.

### 9.3 Semantic Versioning

Post-v1 releases follow semver:

- **v1.x.y**: backward-compatible additions and fixes.
- **v2.0.0**: breaking changes that survived a deprecation cycle.

---

## 10. Open Questions That Must Close During Post-v1

These questions are not optional future ecosystem questions. They are closure blockers for the language/ZIR/compiler/runtime contract. Each requires a dedicated design session and numbered decision before post-v1 can be considered complete.


### Closure status matrix

Wave 7.1 operational tracker: `post-v1-closure-matrix.md`.

Not every topic needs the same amount of discussion. Current classification:

| Topic | Current state | Required action |
|-------|---------------|-----------------|
| Explicit local types | Defined: full local inference is rejected | Audit docs/examples/diagnostics |
| Generic argument inference | Defined: argument-position only, no return-context, no partial inference | Audit implementation + docs |
| Callable type syntax | Defined: `func(T) -> U` | Audit storage/escape/ABI rules |
| Struct type omission | Defined in current contract as rejected | Audit stale decision notes that treated it as deferred/accepted |
| Syntax/keyword freeze | Defined by `post-v1-syntax-freeze.md` | Audit parser/tests/docs/diagnostics parity |
| `any` migration | Defined by `post-v1-any-migration.md` | Audit docs/tests/diagnostics for `any`-first surface and deprecated `dyn` alias guidance |
| `any` safety | Defined for current subset by `post-v1-any-dispatch-stabilization.md` | Audit boundaries + open advanced-shape policy in later waves |
| Trait coherence | Defined by `post-v1-trait-stability.md` | Audit implementation/docs/diagnostics |
| Generic monomorphization | Defined by `post-v1-monomorphization-closure.md` and `post-v1-monomorphization-controls.md` | Audit implementation/docs/diagnostics |
| Closure/callable ABI | Defined by `post-v1-callable-closure-abi.md` | Audit callable diagnostics and boundary coverage |
| Resource cleanup | Defined by `post-v1-using-cleanup-semantics.md` | Audit HIR/ZIR lowering and control-flow fixtures |
| Concurrency semantics | Defined by `post-v1-concurrency-semantics-closure.md` | Audit current C oracle subset and expand typed runtime support behind monomorphized wrappers |
| Error model | Defined by `post-v1-error-model-closure.md` | Audit diagnostics and fixture coverage |
| Pattern matching | Defined by `post-v1-pattern-matching-closure.md` | Audit exhaustiveness, guard, payload, and multi-value fixtures |
| ZIR contract | Defined by `post-v1-zir-consolidation.md` | Harden verifier and golden fixture coverage incrementally |
| Runtime ABI | Defined by `post-v1-runtime-abi-ownership-audit.md` | Track concrete runtime ABI gaps as implementation issues |
| Streams/Async/WebSocket/TLS | Routes partially defined | Dedicated IO/dataflow design sessions after closure prerequisites |
| Backend conformance | Defined by `post-v1-backend-conformance-suite.md` | Build runner automation before Zig/LLVM/WASM activation |
| Edition/deprecation | Not defined | Design session before `zt migrate`/stable tooling |
| Stdlib boundary | Partially defined | Design session before ecosystem/package registry |


1. **Stream design:** Monomorphized `Stream<T>` first vs erased `ByteStream`/`TextStream`, and when `any` adapters become legal.
2. **Async IO route:** Jobs/channels remain the substrate; Wave 8 still needs concrete IO/dataflow APIs.
3. **WebSocket scope:** Blocking client first, or include server support in the first WebSocket wave?
4. **TLS ownership:** stdlib TLS binding, host-provided TLS, or package-first TLS?
5. **Edition/deprecation model:** Whether Zenith needs Rust-style editions or semver + `zt migrate` is sufficient after closure.
6. **Stdlib boundary:** Which modules are language foundations and which belong to packages after the registry exists.

---

## Relationship To Other Documents

- `post-v1-surface-contract.md` - the **what** (canonical list of features).
- This document - the **why, when, how, risks** (discussion and analysis).
- `post-v1-syntax-freeze.md` - Wave 7.2 syntax and keyword closure decisions.
- `post-v1-idiom-pass.md` - Wave 7.3 canonical idiom consolidation artifact.
- `post-v1-any-migration.md` - Wave 7.4 `any` migration closure policy.
- `post-v1-any-dispatch-stabilization.md` - Wave 7.5 `any` dispatch stabilization subset and validation.
- `post-v1-trait-stability.md` - Wave 7.6 trait coherence, defaults, apply lookup, operator traits, and Transferable closure.
- `post-v1-monomorphization-closure.md` - Wave 7.7 executable monomorphization closure contract.
- `post-v1-monomorphization-controls.md` - Wave 7.8 monomorphization controls closure contract.
- `post-v1-callable-closure-abi.md` - Wave 7.9 callable and closure ABI closure contract.
- `post-v1-using-cleanup-semantics.md` - Wave 7.10 `using` cleanup semantics closure contract.
- `post-v1-concurrency-semantics-closure.md` - Wave 7.11 concurrency closure contract.
- `post-v1-error-model-closure.md` - Wave 7.12 error model closure contract.
- `post-v1-pattern-matching-closure.md` - Wave 7.13 pattern matching closure contract.
- `post-v1-zir-consolidation.md` - Wave 7.14 ZIR consolidation contract.
- `post-v1-backend-conformance-suite.md` - Wave 7.15 backend conformance gate.
- `post-v1-source-mapping-contract.md` - Wave 7.16 source mapping contract.
- `post-v1-diagnostic-contract.md` - Wave 7.17 diagnostic contract.
- `post-v1-runtime-abi-ownership-audit.md` - Wave 7.18 runtime ABI and ownership audit.
- `post-v1-optimization-boundary.md` - Wave 7.19 optimization boundary.
- `post-v1-final-language-closure-review.md` - Wave 7.20 final language closure review.
- `v1-surface-contract.md` - current scope (not modified by this document).
- `backend-scalability-risk-model.md` - runtime/compiler risks relevant here.
- `cognitive-accessibility.md` - accessibility bar that applies to all post-v1 work.
- `legibility-evaluation.md` - evaluation protocol for syntax additions.
