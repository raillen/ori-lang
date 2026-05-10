# Zenith Post-v1 Closure Matrix

> Audience: maintainer, language designer, compiler/runtime implementer
> Status: active closure tracker
> Surface: spec
> Last updated: 2026-05-03

This document implements Wave 7.1 from `post-v1-implementation-plan.md`.
It classifies every post-v1 closure topic before implementation continues.

## Status Legend

| Status | Meaning |
|--------|---------|
| Defined | Decision exists; only consistency audit remains. |
| Audit-only | Implementation mostly exists; verify docs, tests, diagnostics, and edge cases. |
| Needs design | Requires design session and numbered decision before implementation. |
| Needs implementation | Direction is clear; compiler/runtime/std work remains. |
| Blocked | Cannot proceed until listed prerequisite closes. |

## Priority Legend

| Priority | Meaning |
|----------|---------|
| P0 | Blocks most of Wave 7 or language closure. |
| P1 | Blocks major feature family or Wave 8. |
| P2 | Important closure audit, but not first-order blocker. |
| P3 | Post-closure or late policy topic. |

## Closure Matrix

| ID | Priority | Area | Topic | Current state | Required output | Blocks |
|----|----------|------|-------|---------------|-----------------|--------|
| 7.1.1 | P0 | Syntax | Explicit local types | Defined:<br>full local inference rejected.<br>`const x = 42` invalid. | Audit:<br>language reference<br>examples<br>parser diagnostics | Syntax freeze |
| 7.1.2 | P0 | Syntax | Generic argument inference | Defined:<br>argument-position only.<br>No return-context.<br>No partial inference. | Audit:<br>implementation<br>docs<br>diagnostics<br>generic call tests | Monomorphization UX |
| 7.1.3 | P0 | Syntax | Struct type omission | Defined as rejected.<br>Bare `{ fields }` not accepted. | Remove stale references:<br>accepted/deferred shorthand claims. | Syntax freeze |
| 7.1.4 | P0 | Syntax | Enum dot shorthand | Defined by Wave 7.2 freeze. | Audit parser/docs/diagnostics consistency. | Pattern matching closure |
| 7.1.5 | P1 | Syntax | Closure return inference | Defined by Wave 7.2 freeze. | Audit checker diagnostics and callable typing parity. | Callable closure semantics |
| 7.1.6 | P1 | Syntax | Single-expression closures | Defined by Wave 7.2 freeze. | Audit parser/lowering/tests for shorthand closure forms. | Callable syntax audit |
| 7.1.7 | P0 | Syntax | Contextual keywords | Defined by final freeze:<br>`then` contextual; `given` removed from final syntax. | Audit identifier-vs-keyword edge cases in parser/tests. | Syntax diagnostics audit |
| 7.1.8 | P1 | Syntax | Match `else` vs `default` | Defined by Wave 7.2 freeze:<br>`else` canonical. | Audit stale docs/tests and diagnostics. | Pattern matching closure |
| 7.1.9 | P0 | Idiom | Canonical style | Defined by `post-v1-idiom-pass.md`. | Audit canonical idiom usage across docs/examples/diagnostics. | Final closure review |
| 7.1.10 | P0 | Dynamic dispatch | `any` migration | Defined by `post-v1-any-migration.md`.<br>`any` is canonical surface spelling. | Audit:<br>docs/tests/diagnostics stay `any`-first.<br>Legacy `dyn` remains deprecated alias only. | `any` backend stabilization |
| 7.1.11 | P0 | Dynamic dispatch | `any` safety | Defined for current subset by `post-v1-any-dispatch-stabilization.md`. | Audit subset boundaries in checker/runtime:<br>non-generic traits<br>non-mut methods<br>copyable signatures<br>FFI/concurrency constraints | Streams<br>advanced trait coherence |
| 7.1.12 | P0 | Traits | Trait coherence | Defined by `post-v1-trait-stability.md`.<br>Method lookup order specified.<br>Overlapping apply rejected.<br>Defaults and Transferable semantics stable. | Audit:<br>checker implementation<br>diagnostics consistency<br>fixture coverage | Operator traits<br>`any`<br>monomorphization |
| 7.1.13 | P1 | Traits | Operator trait scope | Mostly defined:<br>Level 2 only. | Audit implementation/docs.<br>Ensure arbitrary overload rejected. | Syntax freeze<br>diagnostics |
| 7.1.14 | P0 | Generics | Executable monomorphization | Defined by `post-v1-monomorphization-closure.md`.<br>Instance identity, lowering model, and failure contract are explicit for executable subset. | Audit:<br>checker/emitter consistency<br>generic inference diagnostics<br>fixture coverage | Generic HOFs<br>streams<br>lazy<br>typed concurrency payloads |
| 7.1.15 | P1 | Generics | Monomorphization controls | Defined by `post-v1-monomorphization-controls.md`.<br>Canonical keys, dedup/cache, recursion/capacity guards, and limit diagnostics are specified. | Audit:<br>control enforcement paths<br>limit diagnostics<br>project-model docs/tests | Backend scalability |
| 7.1.16 | P1 | Callable | Callable type syntax | Defined:<br>`func(T) -> U`. | Audit docs/tests.<br>Audit unsupported escape positions. | Callable ABI |
| 7.1.17 | P0 | Callable | Closure/callable ABI | Defined by `post-v1-callable-closure-abi.md`.<br>Stored callable ABI, closure/runtime shape, extern callback boundary, and jobs callback rules are explicit. | Audit:<br>callable diagnostics coverage<br>FFI/jobs callable boundary checks<br>runtime ABI consistency | FFI<br>jobs<br>runtime ABI |
| 7.1.18 | P0 | Resource cleanup | `using` semantics | Defined by `post-v1-using-cleanup-semantics.md`.<br>Deterministic cleanup under return/`?`/panic/loop-control is specified. | Audit:<br>HIR/ZIR cleanup lowering<br>control-flow fixtures<br>boundary notes in runtime ABI docs | Runtime ABI<br>diagnostics |
| 7.1.19 | P0 | Concurrency | Jobs/channels semantics | Defined by `post-v1-concurrency-semantics-closure.md`.<br>Capacity, close, backpressure, cancellation, and panic boundary policy are explicit. | Audit current C oracle subset and expand fixtures as payload support grows. | Async IO route<br>streams |
| 7.1.20 | P0 | Concurrency | Non-`int` payload strategy | Defined by `post-v1-concurrency-semantics-closure.md`.<br>Typed monomorphized runtime storage or capability diagnostics. | Implement backend wrappers when expanding executable payload support. | Generic channels<br>shared<br>jobs |
| 7.1.21 | P1 | Errors | Error model closure | Defined by `post-v1-error-model-closure.md`.<br>`result`, `optional`, `?`, helpers, panic, and FFI/jobs boundaries are closed. | Audit diagnostics and fixture coverage. | Diagnostics<br>concurrency |
| 7.1.22 | P1 | Pattern matching | Exhaustiveness | Defined by `post-v1-pattern-matching-closure.md`.<br>Enum exhaustiveness and guard semantics are closed. | Audit exhaustive diagnostics and guard fixtures. | Backend conformance |
| 7.1.23 | P1 | Pattern matching | Payload/destructuring semantics | Defined by `post-v1-pattern-matching-closure.md`.<br>Enum payload, optional, and multi-value semantics are closed. | Audit payload binding and multi-value diagnostics. | Syntax freeze |
| 7.1.24 | P0 | ZIR | ZIR verifier | Defined by `post-v1-zir-consolidation.md`.<br>Verifier invariants and failure classes are specified. | Implement/harden verifier checks incrementally. | Backend conformance<br>Zig/LLVM/WASM |
| 7.1.25 | P0 | ZIR | Canonical backend type model | Defined by `post-v1-zir-consolidation.md`.<br>Backend-visible type shapes and runtime ops are documented. | Audit type spelling and ownership op lowering. | Runtime ABI<br>backend targets |
| 7.1.26 | P0 | ZIR | Generic representation | Defined by `post-v1-zir-consolidation.md`.<br>Backend consumes instantiated concrete ZIR for executable generics. | Audit generic fixture dumps as emit-zir matures. | Monomorphization<br>conformance |
| 7.1.27 | P1 | Compiler | Source mapping contract | Defined by `post-v1-source-mapping-contract.md`.<br>Source span preservation, `#line`, ZIR spans, and debug-info expectations are specified. | Audit generated source and diagnostics spans. | LSP<br>tooling<br>backend targets |
| 7.1.28 | P1 | Compiler | Diagnostic contract | Defined by `post-v1-diagnostic-contract.md`.<br>Stable codes, ACTION/WHY/NEXT, spans, and negative fixture coverage are specified. | Audit invalid fixtures and CI output consistency. | Tooling<br>language reference |
| 7.1.29 | P0 | Runtime | Runtime ABI | Defined by `post-v1-runtime-abi-ownership-audit.md`.<br>Values, closures, ARC/ORC, collections, sum types, jobs/channels, FFI, net/time are audited. | Track concrete runtime ABI gaps as implementation issues. | Backends<br>FFI<br>conformance |
| 7.1.30 | P0 | Runtime | Ownership/cleanup ABI | Defined by `post-v1-runtime-abi-ownership-audit.md`.<br>Retain/release, move/sink, cleanup paths, and compiler/runtime agreement are explicit. | Audit ownership hardening and cleanup fixtures. | `using`<br>ORC<br>FFI |
| 7.1.31 | P1 | Stdlib boundary | Language foundation vs package | Partially defined. | Decide which APIs stay stdlib<br>before ecosystem exists. | Wave 8<br>package model |
| 7.1.32 | P1 | IO/dataflow | Stream design | Route defined:<br>generic monomorphized first. | Design `Stream<T>` after:<br>traits<br>monomorphization<br>`any` safety | Async IO<br>WebSocket<br>lazy iterators |
| 7.1.33 | P1 | IO/dataflow | Async IO route | Route defined:<br>jobs + channels.<br>No `async/await`. | Define:<br>cancellation<br>backpressure<br>errors<br>completion values | WebSocket<br>net expansion |
| 7.1.34 | P2 | IO/dataflow | WebSocket scope | Route defined.<br>Scope not closed. | Decide:<br>blocking client first?<br>server support?<br>message/error API | Wave 8 WebSocket |
| 7.1.35 | P2 | IO/dataflow | TLS ownership | Not defined. | Decide:<br>stdlib TLS binding<br>host-provided TLS<br>or package-first | Secure sockets<br>`wss` |
| 7.1.36 | P0 | Backend | Backend conformance suite | Defined by `post-v1-backend-conformance-suite.md`.<br>C oracle, mandatory fixture classes, diagnostics, and accepted variance are specified. | Build runner automation for future backends. | Zig/LLVM/WASM activation |
| 7.1.37 | P2 | Backend | Optimization boundary | Defined by `post-v1-optimization-boundary.md`.<br>Semantic ZIR passes and backend-specific optimization boundaries are explicit. | Keep optimization behind conformance gates. | Backends<br>performance docs |
| 7.1.38 | P2 | Versioning | Edition/deprecation model | Not defined. | Decide:<br>editions<br>or semver + `zt migrate`<br>deprecation diagnostics | Tooling<br>`dyn` migration |

## Wave 7 Execution Order

| Order | Work | Reason |
|-------|------|--------|
| 1 | Syntax freeze audit | Prevent parser/checker churn. |
| 2 | `any` migration and safety | Blocks trait objects, streams, heterogeneous collections. |
| 3 | Trait coherence | Blocks stable `any`, operator traits, generic constraints. |
| 4 | Monomorphization design | Blocks generic streams, HOFs, lazy, typed concurrency payloads. |
| 5 | Callable/closure ABI | Blocks FFI/jobs/runtime callback consistency. |
| 6 | Resource cleanup + concurrency semantics | Blocks async-via-jobs/channels and runtime ABI closure. |
| 7 | ZIR contract + verifier | Blocks backend conformance and future backends. |
| 8 | Runtime ABI audit | Locks C oracle before backend targets. |
| 9 | Diagnostics/source mapping | Locks tooling-facing compiler behavior. |
| 10 | Final closure review | Confirms no nebulous language/runtime topic remains. |

## Immediate Wave 7.3 Inputs

Wave 7.2 freeze output now feeds Wave 7.3 idiom/reference pass:

- language-reference examples must only use frozen syntax forms;
- docs/tutorials must avoid rejected shorthand and rejected import/operator proposals;
- diagnostics fixtures must align with `else` fallback and contextual keywords;
- style guidance must show canonical closure forms and callable type annotations;
- remove stale `dyn`, `default`, or rejected shorthand claims from public docs.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave ordering and status.
- `post-v1-syntax-freeze.md` - frozen syntax and keyword decisions.
- `post-v1-idiom-pass.md` - Wave 7.3 idiom consolidation artifact.
- `post-v1-any-migration.md` - Wave 7.4 `any` migration closure policy.
- `post-v1-any-dispatch-stabilization.md` - Wave 7.5 `any` dispatch stabilization closure policy.
- `post-v1-trait-stability.md` - Wave 7.6 trait coherence, defaults, apply lookup, operator traits, and Transferable closure.
- `post-v1-monomorphization-closure.md` - Wave 7.7 executable monomorphization closure contract.
- `post-v1-monomorphization-controls.md` - Wave 7.8 monomorphization controls closure contract.
- `post-v1-callable-closure-abi.md` - Wave 7.9 callable and closure ABI closure contract.
- `post-v1-using-cleanup-semantics.md` - Wave 7.10 `using` cleanup semantics closure contract.
- `post-v1-concurrency-semantics-closure.md` - Wave 7.11 concurrency closure contract.
- `post-v1-error-model-closure.md` - Wave 7.12 error model closure contract.
- `post-v1-pattern-matching-closure.md` - Wave 7.13 pattern matching closure contract.
- `post-v1-zir-consolidation.md` - Wave 7.14 ZIR consolidation contract.
- `post-v1-backend-conformance-suite.md` - Wave 7.15 backend conformance contract.
- `post-v1-source-mapping-contract.md` - Wave 7.16 source mapping contract.
- `post-v1-diagnostic-contract.md` - Wave 7.17 diagnostic contract.
- `post-v1-runtime-abi-ownership-audit.md` - Wave 7.18 runtime ABI/ownership audit.
- `post-v1-optimization-boundary.md` - Wave 7.19 optimization boundary contract.
- `post-v1-final-language-closure-review.md` - Wave 7.20 final closure review.
