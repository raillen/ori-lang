# Zenith Post-v1 Implementation Plan

> Audience: contributor, maintainer, language designer
> Status: accepted implementation evidence
> Surface: spec
> Source of truth: implementation order evidence; `final-language-contract.md` prevails for current status
> Last updated: 2026-05-03
> Decision session: 2026-05-01 (item-by-item review with language designer)

This document records the accepted, rejected, and deferred decisions from the
post-v1 design session. It also defines the implementation order and phasing.

Post-v1 is the language-closure window. By the end of this plan, Zenith must have
its user-facing syntax, idioms, semantics, ZIR contract, compiler invariants, and
runtime model closed enough that later work can focus on mature concerns: LSP,
editor tooling, Zig/LLVM/WASM backends, package registry, and ecosystem growth.

Upstream: `final-language-contract.md`, `post-v1-surface-contract.md`, `post-v1-completeness-discussion.md`.

---

## Decision Summary

### Language Surface

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 1 | Full type inference | **Rejected** | Explicit `: type` stays everywhere. No `const x = 42`. |
| 2 | Generic argument inference | **Accepted** | Arg-position only. No return-context inference. No partial inference. |
| 3 | Callable types as first-class values | **Accepted** | `func(int) -> void` as type syntax. Structural matching. |
| 4 | Struct type omission `{ fields }` | **Rejected** | Violates 3 of 4 core philosophy rules (words > symbols, one form, explicit behavior). |
| 5 | Mutable closure capture v2 | **Exploration** | Current `capture` keyword sufficient. Revisit if v1 feedback demands it. |
| 6 | Nested functions | **Accepted** | `func` inside `func`. Captures parent scope (immutable). Sugar for `const name = func(...)`. |
| 7a | Pattern matching - destructuring in `const` | **Accepted** | `const (x, name) = get_pair()` |
| 7b | Pattern matching - multi-value match | **Accepted** | `match (status, code):` |
| 7c | Pattern matching - guard clauses | **Accepted** | `case x if x > 0:` |
| 8 | Operator overloading | **Accepted (Level 2)** | `Comparable` (`<`,`>`,`<=`,`>=`) + `Addable`/`Subtractable` (`+`,`-`). No full overloading. |
| 9 | Pipe operator | **Accepted** | `\|>` operator. Left-to-right function composition. |
| 10 | Variadic parameters | **Rejected** | Use explicit `list<T>` argument. |
| 11 | Selective imports | **Rejected** | Qualified imports only. Always know where symbols come from. |
| 12 | `unless` keyword | **Rejected** | `if not` is already clear. Two forms for same thing. |
| 13 | Ternary expressions | **Accepted** | `if cond then a else b` single-line form. |
| 14 | Math operators (`**`, `//`) | **Rejected** | Use `math.pow()` and regular division. |
| 15 | C-style `for` loops | **Rejected** | `for x in range()` already covers this. |
| 16 | Named tuple fields | **Rejected** | If you need names, use struct. Tuples are positional. |
| 17 | `group` alias for `tuple` | **Removed** | `tuple` is the sole canonical name. |
| 18 | Self shorthand | **Accepted** | `@field` as sugar for `self.field`. `self.field` remains valid. Both coexist. |
| 19 | Wildcard imports | **Rejected** | Violates qualified-imports-only rule. |
| 20 | `size_of`/`type_name` improvements | **Accepted** | Real implementations replacing current placeholders. |

### Concurrency

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 21a | Phase 2 - Transferable predicate | **Accepted** | Checker diagnostics for boundary violations. |
| 21b | Phase 3 - Jobs | **Accepted** | `jobs.spawn(fn, data)` / `jobs.join(job)?`. Copy-based. Thread pool. |
| 21c | Phase 4 - Channels | **Accepted** | `channels.create<T>()` / `send` / `receive`. Producer-consumer. |
| 21d | Phase 5 - Shared state | **Accepted** | `Shared<T>` (mutex-based) + `atomic<T>` (primitives). |

### Runtime / Memory

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 22 | Cycle collection strategy | **ORC (direct)** | Skip `weak<T>` stopgap. Build move analysis + cycle detector directly. |
| 23 | Generic collections | **Accepted** | `map<K,V>` and `list<T>` for all types. Type-erased internals. |
| 24 | Componentized runtime | **Accepted (before ORC)** | Split `zenith_rt.c` into modules first, then build ORC in modular structure. |
| 25 | `std.unsafe` module | **Accepted** | Escape hatch for raw pointer ops, FFI edge cases. |
| 26 | `std.mem.Allocator` | **Exploration** | Not urgent. Revisit after ORC + Borealis grows. |
| 27 | Hot-reload runtime | **Exploration** | Complex, low priority. Nice-to-have. |
| - | `own`/`view`/`edit` manual memory | **Accepted (in `std.mem`)** | Optional API for advanced users. Library-level, not language keywords. |

### FFI

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 28 | FFI Phase 3 - Callbacks | **Accepted** | Function pointers in `extern c` declarations. Critical for C ecosystem. |
| 29 | FFI Phase 4 - ABI annotations | **Accepted** | `attr abi("stdcall")` + symbol renaming. |

### Backend Targets

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 30 | LLVM backend | **Accepted after language closure** | LLVM C API. Keep C backend for bootstrapping; do not start before ZIR/backend conformance gate. |
| 31 | WASM backend | **Accepted after language closure** | Via LLVM or direct lowering after runtime portability and backend conformance are stable. |
| 32 | JavaScript backend | **Rejected** | WASM covers web use case. JS backend = massive effort, limited value. |
| 33 | Zig backend | **Exploration after language closure** | Textual C-like backend candidate; must not reshape Zenith semantics. |
| 34 | Cranelift backend | **Exploration after ZIR gate** | Native backend spike only after backend conformance suite exists. |
| 35 | C3 backend | **Exploration after language closure** | Textual C-like backend candidate. Lower priority than Zig due ecosystem maturity risk. |

### Standard Library

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 36 | Core stdlib expansion items | **Accepted before ecosystem** | Generic HOFs, std.net, std.time, streams, async IO via jobs/channels, TLS, WebSocket, lazy iterators, generic lazy. Registry and optional deps move to ecosystem wave. |

### Compiler

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 37 | Compiler closure features | **Accepted before backend/tooling** | ZIR consolidation, verifier, backend conformance suite, monomorphization controls, exhaustive match, source mapping, diagnostics, optimization pass boundaries. |

### Tooling

| # | Feature | Decision | Notes |
|---|---------|----------|-------|
| 38 | Tooling/ecosystem items | **Accepted after language closure** | VSCode extension, LSP stable, web playground, ZPM registry. |

---

## Implementation Order

Ordered by dependency chain and strategic value.


### Post-v1 Closure Gate

Before any mature backend/tooling/ecosystem phase becomes active, the following must be true:

| Area | Closure requirement |
|------|---------------------|
| Syntax | Accepted and rejected syntax is final; no unresolved shorthand, operator, import, inference, or keyword question remains open. |
| Idiom | Canonical style is documented: explicit types, qualified imports, `result`/`optional`, jobs/channels, `any`, traits/apply, resource cleanup. |
| Semantics | Evaluation order, ownership/ARC/ORC, closure capture, generics, trait constraints, `any`, errors, concurrency boundaries, and cleanup are deterministic and tested. |
| ZIR | ZIR has a verifier, canonical type model, explicit ownership/runtime ops, golden fixtures, and a backend conformance suite. |
| Compiler | Parser, binder, checker, HIR, ZIR lowering, C emitter, diagnostics, and test matrix agree on every accepted feature. |
| Runtime | Componentized runtime, memory model, concurrency handles, FFI ABI, collections, lazy, net/time, and cleanup paths are stable under the C oracle. |
| Documentation | `language-reference.md`, spec files, decisions, examples, and diagnostics use the same terminology, especially `any` instead of user-facing `dyn`. |

If an item affects user syntax, language semantics, ZIR shape, runtime ownership, or backend-visible ABI, it belongs before this gate. If it only improves developer experience, alternative backend output, packaging, registry, distribution, or editor support, it belongs after this gate.


### Wave 1 - Foundation (Immediate Post-v1)

These have no external dependencies and unblock later waves.

| Priority | Item | Dependency | Estimated effort | Implementation status |
|----------|------|------------|------------------|-----------------------|
| 1.1 | Componentized runtime | None | 2-3 weeks | Done |
| 1.2 | `size_of`/`type_name` real implementations | None | 1 week | Done |
| 1.3 | Ternary expressions (`if cond then a else b`) | None | 1 week | Done |
| 1.4 | `tuple` canonical naming (remove `group`) | None | 1 day | Done |
| 1.5 | `@field` self shorthand | None | 1-2 weeks | Done |
| 1.6 | Nested functions | None (sugar for closures) | 1-2 weeks | Done |

Wave 1 progress:

- 1.1 `Componentized runtime` is done. First cache-correctness slice:
  `zenith_collections_rt.c` is now part of runtime object staleness checks, matching
  the unity include in `runtime/c/zenith_rt.c`.
  Second slice: `runtime/c/zenith_rt_manifest.h` is now the canonical runtime
  source/dependency manifest used by the driver, reducing drift between runtime modules
  and `.ztc-tmp/runtime/zenith_rt.o` invalidation.
  Third slice: `tests/hardening/test_runtime_manifest.py` now verifies that every
  `.c` component included by the unity runtime is listed in the manifest and exists.
  Fourth slice: memory pool / validation helpers moved from `zenith_rt.c` into
  `runtime/c/zenith_rt_memory.c`, still compiled through the unity runtime.
  Fifth slice: memory helper declarations are now explicit in `runtime/c/zenith_rt.h`,
  so extracted runtime components keep a visible ABI/API contract.
  Sixth slice: path helpers `zt_path_*` moved from `zenith_rt.c` into
  `runtime/c/zenith_rt_path.c`, keeping the same public declarations in `zenith_rt.h`.
  Seventh slice: `std.math`, safe integer math helpers `zt_*_i64`, and
  `zt_validate_between_i64` moved from `zenith_rt.c` into `runtime/c/zenith_rt_math.c`,
  keeping the same public declarations in `zenith_rt.h`.
  Eighth slice: host API, formatting/scalar/random/encoding helpers, and dynamic dispatch
  moved into focused runtime components. `zenith_rt.c` now acts as a thin unity source
  plus shared low-level glue.
  Ninth slice: low-level core glue moved into `runtime/c/zenith_rt_core.c`, and path
  private helpers moved next to the public path implementation.
  Completion validation:
  `python tests/hardening/test_runtime_manifest.py`,
  alternate driver `check zenith.ztproj --all --ci`,
  direct runtime C tests for text, host fs guardrails, and arithmetic overflow,
  plus behavior builds/runs for `std_format_basic`, `std_encoding_hash_basic`,
  `std_random_basic`, `std_fs_aliases_basic`, and `std_os_process_capture_basic`.
  legacy `dyn_dispatch_basic` still fails before runtime linkage because the generated C assigns
  concrete `Circle`/`Rect` values directly to `zt_dyn_value *`; this is a backend `any`/dynamic-dispatch
  issue, not a runtime componentization blocker.
- 1.2 `size_of`/`type_name` real implementations are implemented for the current C backend path.
  `std.debug.size_of(value)` and `std.debug.type_name(value)` are now compiler-lowered
  for typed values. The older global `type_name(value)` remains accepted as compatibility
  surface, but the public stdlib contract should teach `std.debug.type_name(value)`. Validation:
  `python build.py`, `zt.exe check tests/behavior/std_debug_basic/zenith.ztproj --all --ci`,
  `zt.exe build tests/behavior/std_debug_basic/zenith.ztproj -o tests/behavior/std_debug_basic/build/std-debug-basic.exe --ci --native-raw`,
  and running `tests/behavior/std_debug_basic/build/std-debug-basic.exe`.
- 1.3 `if cond then a else b` is implemented. Parser, AST, semantic checker,
  HIR, ZIR, C emitter, formatter, and LSP paths already support `ZT_AST_IF_EXPR` /
  `ZT_HIR_IF_EXPR` / `ZIR_EXPR_IF`. Validation:
  `zt check tests/behavior/syntax_coherence_core/zenith.ztproj --all --ci`,
  `zt build tests/behavior/syntax_coherence_core/zenith.ztproj -o tests/behavior/syntax_coherence_core/build/syntax-coherence-core-ifexpr.exe --ci --native-raw`,
  and running `tests/behavior/syntax_coherence_core/build/syntax-coherence-core-ifexpr.exe`.
- 1.4 `group` alias was removed from the active language surface; `tuple` is now
  the only canonical naming form. Validation focuses on rejected-syntax fixtures
  and tuple behavior fixtures (`group_removed_error`,
  `tuple_generated_struct_callbacks`).
- 1.5 `@field` self shorthand is implemented as parser sugar for `self.field`.
  It works for field reads and assignments inside `apply` methods, while `self.field`
  remains valid. Validation:
  `zt check tests/behavior/self_field_shorthand/zenith.ztproj --all --ci`,
  `zt build tests/behavior/self_field_shorthand/zenith.ztproj -o tests/behavior/self_field_shorthand/build/self-field-shorthand.exe --ci --native-raw`,
  running `tests/behavior/self_field_shorthand/build/self-field-shorthand.exe` with exit `10`,
  regression `methods_inherent_apply` with exit `7`,
  `zt test tests/behavior/std_test_attr_pass_skip/zenith.ztproj --ci`,
  and invalid check `self_field_shorthand_outside_apply_error`.
- 1.6 `Nested functions` is implemented as local immutable callable bindings lowered
  through the closure path. A nested `func` can capture parent scope immutably, can be
  called by local name after declaration, and rejects mutation of captured parent vars.
  Validation:
  `zt check tests/behavior/nested_function_basic/zenith.ztproj --all --ci`,
  `zt fmt tests/behavior/nested_function_basic/zenith.ztproj --check`,
  `zt build tests/behavior/nested_function_basic/zenith.ztproj -o tests/behavior/nested_function_basic/build/nested-function-basic.exe --ci --native-raw`,
  running `tests/behavior/nested_function_basic/build/nested-function-basic.exe` with exit `4`,
  invalid check `nested_function_mut_capture_error`,
  regression `lambda_hof_basic`, and root `zt check zenith.ztproj --all --ci`.

### Wave 2 - Type System + Pattern Matching

| Priority | Item | Dependency | Estimated effort | Implementation status |
|----------|------|------------|------------------|-----------------------|
| 2.1 | Callable types (`func(T) -> U` as type) | None | 3-4 weeks | Done |
| 2.2 | Generic argument inference (arg-position) | None | 3-4 weeks | Done |
| 2.3 | Pattern matching: guard clauses (`if`) | None | 2 weeks | Done |
| 2.4 | Pattern matching: destructuring in `const` | Tuple maturity | 2 weeks | Done |
| 2.5 | Pattern matching: multi-value match | 2.4 | 2 weeks | Done |
| 2.6 | Operator overloading (`Comparable` + `Addable`/`Subtractable`) | Trait maturity | 3-4 weeks | Done |
| 2.7 | Pipe operator (`\|>`) | 2.1 (callable types) | 1-2 weeks | Done |

Wave 2 progress:

- 2.1 `Callable types` is already implemented and validated in the current
  pipeline. The compiler accepts `func(T, ...) -> R` type syntax, resolves
  callable values structurally, supports local callable bindings and callable
  calls, and rejects v1 escape positions such as containers, struct fields, and
  public vars. Validation:
  `zt check tests/behavior/callable_basic/zenith.ztproj --all --ci`,
  `zt build tests/behavior/callable_basic/zenith.ztproj -o tests/behavior/callable_basic/build/callable-basic.exe --ci --native-raw`,
  running `tests/behavior/callable_basic/build/callable-basic.exe` with exit `7`,
  invalid checks for `callable_invalid_func_ref_error`,
  `callable_signature_mismatch_error`, `callable_escape_container_error`,
  `callable_escape_struct_field_error`, and `callable_escape_public_var_error`,
  plus regression `closure_capture_basic`.
- 2.2 `Generic argument inference` is implemented in semantic checking for
  generic function calls. The checker infers all type params from supplied
  positional/named argument types, rejects return-only inference with a
  "provide explicit `<T>`" diagnostic, and rejects conflicting argument
  evidence. Validation:
  `zt check tests/behavior/generic_arg_inference_basic/zenith.ztproj --all --ci`,
  invalid checks for `generic_arg_inference_missing_error` and
  `generic_arg_inference_conflict_error`. The C backend now supports an
  executable monomorphization subset for direct and nested generic calls,
  validated with `generic_monomorphization_nested_call` (`check`, `build`,
  runtime exit `9`) including composed inference (`list<T>`) and transitively
  specialized calls across different generic parameter names (`T -> U`).
  Full closure contract is now recorded by Wave 7.7 and 7.8 artifacts.
- 2.3 `Pattern matching guard clauses` is implemented with `if`
  after case patterns. Guards are bound in the case scope, checked as `bool`,
  preserved through HIR, lowered into ZIR as `pattern and guard`, and formatted
  by `zt fmt`. Validation:
  `zt check tests/behavior/match_guard_basic/zenith.ztproj --all --ci`,
  invalid check `match_guard_non_bool_error`,
  `zt build tests/behavior/match_guard_basic/zenith.ztproj -o tests/behavior/match_guard_basic/build/match-guard-basic.exe --ci --native-raw`,
  and running `tests/behavior/match_guard_basic/build/match-guard-basic.exe`
  with exit `7`.
- 2.4 `Pattern matching: destructuring in const` is implemented for tuple
  destructuring with `const (a, b) = expr`. The parser accepts local
  destructuring const declarations, the checker requires a tuple initializer
  with matching arity, bindings are immutable and typed from tuple elements,
  formatter preserves the surface syntax, and HIR lowers to a hidden tuple temp
  plus immutable field bindings. Validation:
  `zt check tests/behavior/const_destructuring_basic/zenith.ztproj --all --ci`,
  invalid checks for `const_destructuring_non_tuple_error` and
  `const_destructuring_arity_error`,
  `zt fmt tests/behavior/const_destructuring_basic/zenith.ztproj --check`,
  `zt build tests/behavior/const_destructuring_basic/zenith.ztproj -o tests/behavior/const_destructuring_basic/build/const-destructuring-basic.exe --ci --native-raw`,
  and running `tests/behavior/const_destructuring_basic/build/const-destructuring-basic.exe`
  with exit `7`.
- 2.5 `Pattern matching: multi-value match` is implemented through tuple
  subjects and tuple case patterns. The parser accepts the committed
  `match (a, b):` form while the formatter keeps the existing canonical match
  layout. ZIR lowers tuple patterns to per-field comparisons (`item0`,
  `item1`, ...), avoiding whole-struct tuple equality in the C backend.
  Validation:
  `zt check tests/behavior/multivalue_match_basic/zenith.ztproj --all --ci`,
  invalid check `multivalue_match_type_error`,
  `zt fmt tests/behavior/multivalue_match_basic/zenith.ztproj --check`,
  `zt build tests/behavior/multivalue_match_basic/zenith.ztproj -o tests/behavior/multivalue_match_basic/build/multivalue-match-basic.exe --ci --native-raw`,
  and running `tests/behavior/multivalue_match_basic/build/multivalue-match-basic.exe`
  with exit `7`.
- 2.6 `Operator overloading Level 2` is implemented for the accepted core
  trait surface only: `Comparable` maps `<`, `<=`, `>`, `>=`;
  `Addable` maps `+`; and `Subtractable` maps `-`. The checker accepts
  compatible trait-backed operands and rejects arbitrary user operators.
  HIR lowering rewrites supported overloaded operators to trait method calls,
  and the C backend now emits vtable wrappers that support trait methods with
  regular parameters. Validation:
  `zt check tests/behavior/operator_overloading_level2_basic/zenith.ztproj --all --ci`,
  invalid check `operator_overloading_missing_trait_error`,
  `zt fmt tests/behavior/operator_overloading_level2_basic/zenith.ztproj --check`,
  `zt build tests/behavior/operator_overloading_level2_basic/zenith.ztproj -o tests/behavior/operator_overloading_level2_basic/build/operator-overloading-level2-basic.exe --ci --native-raw`,
  and running `tests/behavior/operator_overloading_level2_basic/build/operator-overloading-level2-basic.exe`
  with exit `7`.
- 2.7 `Pipe operator` is implemented as left-to-right call sugar. The parser
  accepts `value |> transform` and `value |> transform(extra)`, the formatter
  preserves `|>`, the checker validates the right side as a callable target,
  and HIR lowering rewrites the pipe into a normal call with the left value as
  the first positional argument. Validation:
  `zt check tests/behavior/pipe_operator_basic/zenith.ztproj --all --ci`,
  invalid check `pipe_operator_non_callable_error`,
  `zt fmt tests/behavior/pipe_operator_basic/zenith.ztproj --check`,
  `zt build tests/behavior/pipe_operator_basic/zenith.ztproj -o tests/behavior/pipe_operator_basic/build/pipe-operator-basic.exe --ci --native-raw`,
  and running `tests/behavior/pipe_operator_basic/build/pipe-operator-basic.exe`
  with exit `7`.

### Wave 3 - Runtime + Memory

| Priority | Item | Dependency | Estimated effort | Implementation status |
|----------|------|------------|------------------|-----------------------|
| 3.1 | ORC - move analysis (last-use optimization) | Wave 1.1 (modular runtime) | 4-6 weeks | Done |
| 3.2 | ORC - cycle detector (trial deletion) | 3.1 | 4-6 weeks | Done |
| 3.3 | Generic collections (`map<K,V>`, `list<T>` for all types) | 3.1 | 4-6 weeks | Done |
| 3.4 | `std.unsafe` module | None | 2-3 weeks | Done |
| 3.5 | `own`/`view`/`edit` in `std.mem` (optional) | 3.1 + 3.2 | 3-4 weeks | Done |

Wave 3 progress:

- 3.1 `ORC - move analysis` is complete. First C backend slice:
  managed local assignment now detects a final use of a managed local in
  `dest = source` and emits a move (`dest = source; source = NULL;`) instead
  of retain/copy. If `source` is used later on any reachable path, including a
  loop backedge, the previous retain path is kept.
  Validation:
  temporary driver compile under `.ztc-tmp`, `python build.py`,
  `zt check` / `zt fmt --check` / native build / executable run for
  `orc_last_use_move_basic`, `orc_last_use_no_move_after_alias`, and
  `orc_last_use_loop_backedge_no_move`, plus the branch precision fixture
  `orc_last_use_branch_sibling_move`.
  Generated C evidence confirms `moved = original; original = NULL;` in the
  move fixture and `zt_retain(original)` in the later-use and loop-backedge
  fixtures. It also confirms move remains allowed when the only later source
  uses are on sibling paths. The guard is automated by
  `tests/hardening/test_orc_last_use_move.py`.
- 3.1 managed field move is implemented for direct struct-field reads:
  `dest = box.value` may become `dest = box.value; box.value = NULL;` when
  `box.value` and the whole `box` have no later relevant use. The field
  liveness is field-aware, so `box.marker` can still be read after moving
  `box.value`. Validation: `orc_field_last_use_move`,
  `orc_field_no_move_after_object_use`, `orc_field_move_other_field_later_use`,
  and `tests/hardening/test_orc_last_use_move.py`.
- 3.1 sink parameter transfer is implemented for a narrow safe pattern:
  a managed parameter consumed once by `local = param` with no later reachable
  use. The caller passes an owned argument for sink parameters. If the caller
  still uses the argument, it passes a retained owner. If the argument is a
  final use in a simple assignment call, standalone effect call, or return-call
  cleanup path, the caller moves it and emits `source = NULL` after the call.
  Duplicate-source calls are guarded so only one sink argument consumes the
  original owner; the other sink argument receives a retained owner. Validation:
  `orc_sink_param_owned_transfer_basic`, `orc_sink_param_last_use_arg_move`,
  `orc_sink_param_return_move`, `orc_sink_param_effect_move`,
  `orc_sink_param_duplicate_source`, and `tests/hardening/test_orc_last_use_move.py`.
- 3.1 list item move is implemented for `list<text>` index reads when the
  source list has no later reachable use. The runtime now exposes
  `zt_list_text_take(list, index)`, which transfers the element without retain
  for unique lists and falls back to retaining when aliases keep the list
  shared. The C emitter lowers `dest = names[i]` to `zt_list_text_take` only
  when the list is last-use; later list uses keep the existing retaining
  `zt_list_text_get` path. Validation:
  `orc_list_text_item_last_use_move`,
  `orc_list_text_item_no_move_after_list_use`, native builds/runs for both
  fixtures, and `tests/hardening/test_orc_last_use_move.py`.
- 3.2 `ORC - cycle detector` now has a stable runtime/stdlib collection hook:
  `zt_orc_collect_cycles()` / `std.orc.collect_cycles()`. For the current
  public heap surface there are no user-visible APIs that construct strong
  managed reference cycles, so the collector returns `0` collected cycles while
  exposing the integration point and hardening coverage needed before future
  cycle-forming heap kinds are enabled. ORC introspection helpers were added
  for `text` and `list<text>` reference counts and uniqueness checks.
  Validation: `wave3_runtime_memory_surface` and
  `tests/hardening/test_wave3_runtime_memory.py`.
- 3.3 generic collections are complete for the current C backend surface:
  specialized `list<float>`, `list<bool>`, integer-width lists, `list<text>`,
  `map<text,text>`, generated map helpers such as `map<int,text>`, and existing
  std map/list APIs are covered. Validation:
  `list_float_primitive_storage`, `list_primitive_numeric_matrix`, `map_basic`,
  `map_int_text_basic`, `map_len_basic`, `map_safe_get`,
  `map_value_api_basic`, and `tests/hardening/test_wave3_runtime_memory.py`.
- 3.4 `std.unsafe` is implemented as an explicit stdlib module backed by
  runtime escape hatches for heap kind inspection and manual retain on `text`
  and `list<text>`. The module is narrow by design and remains qualified-only.
  Validation: `wave3_runtime_memory_surface` and hardening symbol checks.
- 3.5 `std.mem` is implemented as an optional library-level API with
  `own_text`/`view_text`/`edit_text` and corresponding `list<text>` helpers.
  `own`/`edit` deep-copy, while `view` retains and returns a shared managed
  value. This avoids language-level ownership keywords while giving advanced
  users explicit memory intent. Validation: `wave3_runtime_memory_surface` and
  hardening symbol checks.

### Wave 4 - Concurrency

| Priority | Item | Dependency | Estimated effort | Implementation status |
|----------|------|------------|------------------|-----------------------|
| 4.1 | Phase 2 - Transferable predicate | None | 2-3 weeks | Done |
| 4.2 | Phase 3 - Jobs | 4.1 + Wave 3 (ORC) | 4-6 weeks | Done (`int` runtime + typed handle facade) |
| 4.3 | Phase 4 - Channels | 4.2 | 4-6 weeks | Done (`int` runtime + typed handle facade) |
| 4.4 | Phase 5 - Shared state (`Shared<T>`, `atomic<T>`) | 4.3 | 3-4 weeks | Done (`int` runtime + typed handle facade) |

### Wave 5 - FFI Expansion

| Priority | Item | Dependency | Estimated effort | Implementation status |
|----------|------|------------|------------------|-----------------------|
| 5.1 | FFI Phase 3 - Callbacks | 2.1 (callable types) | 4-6 weeks | Done (top-level primitive callbacks; immediate C invocation) |
| 5.2 | FFI Phase 4 - ABI annotations | 5.1 | 2-3 weeks | Done (`attr name`, `attr abi("cdecl"|"stdcall")`) |

### Wave 6 - Standard Library Expansion (executable C-backend foundation)

Wave 6 is closed for the practical executable C-backend subset. Items that require unresolved language/runtime foundations move into later closure or IO/dataflow waves.

| Priority | Item | Implementation status |
|----------|------|-----------------------|
| 6.1 | Generic HOFs (`map`, `filter`, `find`, `sort_by`) | Done for executable C-backend primitive/text lists as same-type HOFs; cross-type `map<T,U>` and generic `reduce<T>` move to closure monomorphization |
| 6.2 | `std.time` expansion (dates, timestamps, duration) | Done for executable C-backend MVP (`Instant`, `Duration`, unix ms/s conversion, arithmetic, sleep) |
| 6.3 | Generic lazy / lazy iterators | Done for executable C-backend monomorphic lazy values (`int`, `float`, `text`); full generic lazy and lazy iterators move to closure/IO work |
| 6.4 | `std.net` expansion | Done for executable C-backend blocking TCP client surface (`connect`, `read_some`, `write_all`, `close`, `is_closed`); TLS/UDP/server APIs move to later IO/network waves |

### Wave 7 - Language, ZIR, Compiler, Runtime Closure

This wave is the mandatory closure gate for the language itself. It must finish before advanced IO/dataflow, Zig, LLVM, WASM, mature tooling, registry, optional dependencies, or ecosystem work becomes active.

| Priority | Item | Closure output | Discussion status |
|----------|------|----------------|-------------------|
| 7.1 | Closure topic matrix | `post-v1-closure-matrix.md` classifies each topic as:<br>Defined<br>Audit-only<br>Needs design<br>Needs implementation<br>Blocked | Done |
| 7.2 | Syntax and keyword freeze | `post-v1-syntax-freeze.md` defines accepted/rejected syntax list.<br>No open shorthand, import, operator, inference, or keyword question. | Done |
| 7.3 | Idiom and language-reference pass | `post-v1-idiom-pass.md` defines canonical idioms for:<br>errors<br>resources<br>traits/`any`<br>generics<br>concurrency<br>modules/stdlib | Done |
| 7.4 | `any` terminology migration | `post-v1-any-migration.md` closes naming policy:<br>surface syntax/docs/diagnostics use `any`<br>`dyn` accepted only as deprecated alias with warning | Done |
| 7.5 | `any` dispatch backend stabilization | `post-v1-any-dispatch-stabilization.md` records stabilized subset and validation for heterogeneous dispatch (`list<any Trait>` baseline). | Done |
| 7.6 | Trait stability pass | `post-v1-trait-stability.md` closes trait coherence, method lookup order, defaults, overlapping apply policy, and Transferable semantics. | Done |
| 7.7 | Generic monomorphization for executable generic functions | `post-v1-monomorphization-closure.md` defines instance identity, lowering model, failure contract, and validation envelope for executable C-backend monomorphization subset. | Done |
| 7.8 | Monomorphization controls | `post-v1-monomorphization-controls.md` closes canonical keys, instance cache/dedup, recursion/capacity guards, and `build.monomorphization_limit` enforcement contract. | Done |
| 7.9 | Callable/closure ABI closure | `post-v1-callable-closure-abi.md` defines stored callable ABI, closure lowering/runtime shape, extern callback constraints, and jobs callback boundary rules. | Done |
| 7.10 | Resource cleanup semantics | `post-v1-using-cleanup-semantics.md` defines deterministic `using` cleanup under return/`?`/panic/loop control, with current boundary notes for concurrency/FFI. | Done |
| 7.11 | Concurrency semantics closure | `post-v1-concurrency-semantics-closure.md` defines channel close/capacity/backpressure/cancellation, panic boundaries, `Transferable`, and non-`int` payload strategy. | Done |
| 7.12 | Error model closure | `post-v1-error-model-closure.md` audits and closes `result`, `optional`, `?`, `.or_return`, `.or_wrap`, panic boundaries, and FFI/jobs interop. | Done |
| 7.13 | Pattern matching closure | `post-v1-pattern-matching-closure.md` closes exhaustiveness, guards, destructuring, enum payloads, multi-value matches, and diagnostics for the implemented subset. | Done |
| 7.14 | ZIR consolidation | `post-v1-zir-consolidation.md` defines canonical type model, ownership/runtime ops, verifier invariants, golden fixtures, textual dump stability, and generic representation. | Done |
| 7.15 | Backend conformance suite | `post-v1-backend-conformance-suite.md` defines C oracle fixtures and variance rules every future backend must match before Zig/LLVM/WASM activation. | Done |
| 7.16 | Source mapping contract | `post-v1-source-mapping-contract.md` defines stable source spans, generated C `#line` expectations, ZIR spans, and future debug-info expectations. | Done |
| 7.17 | Diagnostic contract pass | `post-v1-diagnostic-contract.md` defines stable codes, ACTION/WHY/NEXT style, span expectations, and closure negative fixture coverage. | Done |
| 7.18 | Runtime ABI and ownership audit | `post-v1-runtime-abi-ownership-audit.md` audits function ABI, closure ABI, ARC/ORC, cleanup, FFI, concurrency, collections, lazy/net/time under C oracle. | Done |
| 7.19 | Optimization boundary definition | `post-v1-optimization-boundary.md` defines semantic-preserving ZIR passes versus backend-specific optimization boundaries. | Done |
| 7.20 | Final language closure review | `post-v1-final-language-closure-review.md` confirms no remaining language/ZIR/compiler/runtime nebulous topic lacks a decision artifact. | Done |

---

## Relationship To Other Documents

- `post-v1-idiom-pass.md` - Wave 7.3 canonical idiom consolidation artifact.
- `post-v1-any-migration.md` - Wave 7.4 `any` migration policy and deprecation behavior.
- `post-v1-any-dispatch-stabilization.md` - Wave 7.5 backend/runtime stabilization envelope for `any` dispatch.
- `post-v1-trait-stability.md` - Wave 7.6 trait coherence, defaults, apply lookup, operator traits, and Transferable closure.
- `post-v1-monomorphization-closure.md` - Wave 7.7 executable monomorphization closure contract.
- `post-v1-monomorphization-controls.md` - Wave 7.8 monomorphization control mechanisms and limits.
- `post-v1-callable-closure-abi.md` - Wave 7.9 callable and closure ABI closure contract.
- `post-v1-using-cleanup-semantics.md` - Wave 7.10 `using` cleanup semantics closure.
- `post-v1-concurrency-semantics-closure.md` - Wave 7.11 jobs/channels and concurrency boundary closure.
- `post-v1-error-model-closure.md` - Wave 7.12 optional/result, propagation, and panic boundary closure.
- `post-v1-pattern-matching-closure.md` - Wave 7.13 pattern matching closure.
- `post-v1-zir-consolidation.md` - Wave 7.14 ZIR consolidation contract.
- `post-v1-backend-conformance-suite.md` - Wave 7.15 backend conformance gate.
- `post-v1-source-mapping-contract.md` - Wave 7.16 source mapping contract.
- `post-v1-diagnostic-contract.md` - Wave 7.17 diagnostics contract.
- `post-v1-runtime-abi-ownership-audit.md` - Wave 7.18 runtime ABI and ownership audit.
- `post-v1-optimization-boundary.md` - Wave 7.19 optimization boundary.
- `post-v1-final-language-closure-review.md` - Wave 7.20 final language closure gate.
- `post-v1-syntax-freeze.md` - Wave 7.2 frozen syntax/keyword decisions.
- `post-v1-surface-contract.md` - must be updated to reflect rejections and changes from this session.
- `post-v1-completeness-discussion.md` - discussion remains valid; this document records final decisions.
- `post-v1-closure-matrix.md` - Wave 7.1 operational closure tracker.
- `v1-surface-contract.md` - not affected by this document.
- Each accepted feature requires a numbered decision in `docs/internal/decisions/language/` before implementation.
