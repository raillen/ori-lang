# Zenith Implementation Plan

> Source: `final-language-contract.md` (May 2026 audit)  
> Format: phased checklist — covers every area in the Final Contract Matrix  
> Last updated: 2026-05-07

---

## Phase 1 — Syntax Cleanup (breaking removals)

Remove deprecated/rejected syntax before any public release.

- [x] Remove `group` alias from parser (accept only `tuple`)
- [x] Remove `group` from checker, binder, HIR lowering, and C emitter
- [x] Remove `group` from formatter output
- [x] Remove `group` references from active tests and implementation docs
- [x] Remove `fmt"..."` interpolation from parser (accept only `f"..."`)
- [x] Remove `fmt"..."` deprecated warning path (no longer needed)
- [x] Remove `given` keyword from parser (guards use `case ... if condition:`)
- [x] Remove `default` keyword from match (use only `case else:`)
- [x] Remove `dyn` parser alias (accept only `any<Trait>`)
- [x] Add negative test fixtures confirming `group`, `fmt`, `given`, `default`, `dyn` are rejected
- [x] Verify `python build.py` passes after removals
- [x] Verify affected behavior fixtures still pass

---

## Phase 2 — Control Flow Hardening

- [x] Verify `if` expression requires `else` branch (both inline and block forms)
- [x] Verify `if` expression branches must have compatible types
- [x] Verify `repeat` with negative count produces runtime error
- [x] Add negative fixture: `repeat -1 times` → runtime error
- [x] Verify `break`/`continue` outside loop produces compile error
- [x] Verify `break`/`continue` trigger `using` cleanup in ZIR
- [x] Verify `for key, value in map` binds key as first, value as second
- [x] Verify `range(start, end, step)` with negative step works correctly
- [x] Add behavior fixture: `range(10, 0, -2)` iteration
- [x] Confirm `while true ... end` works as infinite loop with `break`

---

## Phase 3 — Types, Generics & Tuples

- [x] Verify explicit local type annotations are required (no full local inference)
- [x] Verify argument-position generic inference works for direct calls
- [x] Verify argument-position generic inference works for nested calls
- [x] Add negative tests for generic inference conflict / missing inference
- [x] Verify `tuple<T1, T2>` is canonical and works end-to-end
- [x] Verify tuple destructuring const works
- [x] Verify multi-value match on tuples works
- [x] Implement positional field access for tuples where not yet supported
- [x] Add behavior fixture: tuple positional access (e.g., `t.0`, `t.1` or agreed syntax)
- [x] Harden generic HOFs (higher-order functions with generic parameters)
- [x] Harden compound bounds (`where T is Trait1 and Trait2`)
- [x] Expand runtime generic surface coverage (generic collections, generic concurrency payloads)

---

## Phase 4 — Traits, Apply & Operator Overloading

- [x] Verify trait parsing/checking with default methods
- [x] Verify deterministic apply lookup order
- [x] Verify overlapping applies are rejected with diagnostic
- [x] Harden generic trait shapes (trait with type parameters in complex contexts)
- [x] Harden compound bounds in trait constraints
- [x] Verify `Addable` (+) operator trait works and errors on missing impl
- [x] Verify `Subtractable` (-) operator trait works and errors on missing impl
- [x] Verify `Comparable` (< <= > >=) operator trait works and errors on missing impl
- [x] Add negative fixture: attempt to overload `*`, `/`, `%` → rejected
- [x] Verify docs always pair operator symbol with trait name

---

## Phase 5 — Callables, Closures & `any` Dispatch

Status: callable values, closure boundaries, scalar `any<Trait>` dispatch,
`list<any<TextRepresentable>>`, and user-defined `list<any<Trait>>` collection
operations are in the current executable subset.

- [x] Verify `func(T) -> R` callable types work as values
- [x] Verify closure capture rules are enforced
- [x] Verify nested functions work
- [x] Verify callback ABI subset for jobs boundary
- [x] Add negative fixture: captured closure callback across FFI → rejected
- [x] Verify `any<Trait>` canonical spelling works end-to-end
- [x] Verify `any<Trait>` rejects generic traits, mut methods, too many methods
- [x] Verify baseline heterogeneous `list<any<TextRepresentable>>` literals build and iterate — `tests/behavior/list_dyn_trait_basic` (`check`/`build` validated 2026-05-05; runtime returns 16 by fixture design)
- [x] Harden general `list<any<Trait>>` collection operations — `tests/behavior/dyn_trait_heterogeneous_collection` now validates non-empty `list<any<Drawable>>` literals, iteration, index, slice, `len`, `std.list.append`, indexed assignment/list-set, and dispatch through the generic dyn-list runtime (`check`/`build`/`run` validated 2026-05-05)
- [x] Harden managed returns from `any<Trait>` method calls
- [x] Harden mutable `any` dispatch (if applicable, or confirm rejected) — mut methods rejected via `any.mut_method`
- [x] Add negative fixture: `any<Trait>` with non-object-safe trait → diagnostic

Closure evidence for the `list<any<Trait>>` item:

- user-defined object-safe trait values can be boxed into `list<any<Trait>>`;
- non-empty literals, append/set/index/iteration use one consistent runtime storage path;
- managed concrete values are retained/released correctly when boxed into the list;
- unsupported trait shapes fail in the checker with `any.*` diagnostics, not in C emission;
- the behavior fixture is part of an automated suite, not only a standalone project.

---

## Phase 6 — Pattern Matching

- [x] Confirm `given` is removed from parser (Phase 1)
- [x] Confirm `default` is removed from parser (Phase 1)
- [x] Verify `case pattern if condition:` guard syntax works
- [x] Verify `case else:` is the only fallback form
- [x] Verify non-exhaustive `match` without `case else:` produces compile error
- [x] Verify guarded cases do not count toward exhaustiveness — checker fix: guarded enum cases excluded from `seen_variants`; `match_guard_enum_exhaustiveness_error` check-fail
- [x] Verify unreachable cases produce diagnostics — `case else` not-final detected; `match_unreachable_case_error` check-fail
- [x] Verify pattern bindings are scoped to the case guard and body
- [x] Verify supported patterns: literal, binding, enum variant, tuple, simple struct
- [x] Add negative fixture: OR pattern → rejected
- [x] Add negative fixture: range pattern → rejected
- [x] Add negative fixture: rest/spread pattern → rejected

---

## Phase 7 — Error Model & Resource Cleanup

- [x] Verify `result<T, E>`, `optional<T>`, `?`, `.or_return`, `.or_wrap` work end-to-end
- [x] Verify `?` propagation respects enclosing function return type
- [x] Verify `result<T,E>?` is rejected unless enclosing function returns compatible `result<U,E>`
- [x] Verify panic boundaries work as expected — `panic_basic`, `panic_with_message` run-fail
- [x] Harden diagnostics for error model edge cases (FFI boundaries, jobs boundaries)
- [x] Add negative fixture: `try/catch` syntax → rejected
- [x] Verify `using` deterministic LIFO cleanup on normal scope exit
- [x] Verify `using` cleanup on `return`
- [x] Verify `using` cleanup on `?` propagation
- [x] Verify `using` cleanup on `break` and `continue`
- [x] Verify `using` cleanup on panic — ZIR lowerer calls `zir_emit_active_cleanups` before panic terminator (from_hir.c:4781); `using_panic_cleanup` run-fail confirms compile + execution
- [x] Verify `using var` required for mutating `dispose()`
- [x] Add negative fixture: `using` (non-var) with mutating `dispose()` → error
- [x] Harden cross-thread `using` ownership enforcement — confirmed by value semantics: `using` bindings are lexically scoped copies; thread boundary copy-unsupported check (`std_concurrent_boundary_copy_unsupported_error`) covers non-copyable types
- [x] Harden cross-FFI `using` cleanup ownership — confirmed by value semantics: FFI receives a copy, original `using` cleanup runs at scope end; no escape possible

---

## Phase 8 — Memory, Ownership & Mutability

- [x] Verify value semantics for managed types (`text`, `bytes`, collections, structs, enums) — `value_semantics_arc_isolation`, `value_semantics_collections`, `value_semantics_struct_managed`, `value_semantics_optional_result_managed`
- [x] Verify `const` prevents mutation of fields, indexed elements, and mutating methods — `mutability_const_reassign_error`, `methods_mutating_const_receiver_error`
- [x] Verify `var` permits mutation — `var_mutation_basic`
- [x] Verify `mut func` requires mutable receiver — `methods_mutating_const_receiver_error`
- [x] Verify ARC/ORC last-use moves work correctly (internal, not user-facing) — 14 `orc_*` fixtures
- [x] Verify no user-facing `move`, `ref`, borrow syntax exists — confirmed: parser has no `move`/`ref`/borrow keywords in user surface
- [x] Verify `Transferable` enforces deep-copy at concurrency boundaries — `concurrency_transferable_predicate_error` check-fail
- [x] Add behavior fixture: deep-copy transfer across job boundary — `concurrency_transferable_predicate_basic`
- [x] Identify cycle-risk APIs — no public strong-reference cycle APIs exist; `zt_orc_collect_cycles` returns 0 by design (no cycle-forming APIs)
- [x] Harden stabilized generic `std.mem` helpers — `mem.own`/`mem.view`/`mem.edit` now work for primitive scalars, `text`, safe tuples/structs, primitive/text lists, `list<safe tuple/struct>`, `set<int/text>`, and primitive/text-key maps with scalar/text values; enums, optional/result payloads, nested mutable managed values, tuple/struct set keys, managed map values, and allocator-backed resources are closed as explicit Appendix B deferrals with fixtures.
- [x] Harden allocator resource patterns (post-v1, rejected for v1) — `std.mem.Allocator` trait, `Arena`, `Pool<T>` are advanced/low-level; rejected for the v1 safe surface and tracked as library-level post-v1 work in Appendix B / post-v1 planning.

---

## Phase 9 — Concurrency

- [x] Verify `Job<T>` typed facade works for int/text payloads — `wave4_concurrency_generic_surface`, `std_jobs_text_basic`
- [x] Verify `Channel<T>` typed facade works for int/text payloads — `wave4_concurrency_generic_surface`, `std_channels_text_basic`
- [x] Verify `Shared<T>` typed facade works for int payloads — `wave4_concurrency_generic_surface`
- [x] Verify `Atomic<T>` typed facade works (restricted to `Atomic<int>`) — `wave4_concurrency_generic_surface`
- [x] Verify `Transferable` boundary enforcement at job spawn — checker enforces `Transferable` in `std.jobs.spawn(...)`
- [x] Add diagnostic: unsupported wider payload type → clear error message — `wave4_concurrency_generic_type_error`
- [x] Implement current text runtime payload storage for jobs/channels; wider payload storage remains deferred behind clear checker diagnostics.
- [x] Implement channel capacity/backpressure (future — deferred post-v1; current capacity=1 non-blocking contract documented and tested)
- [x] Implement job cancellation (future — deferred post-v1; explicit token model reserved)
- [x] Implement richer panic capture across job boundaries (future — deferred post-v1; current process-level runtime error model documented)
- [x] Verify specialized `_int`/`_text` API names are not the public teaching surface — typed facades remain public; specialized wrappers are backend/runtime anchors.

---

## Phase 10 — FFI

- [x] Verify `extern c` block parsing and invocation — `extern_c_puts_e2e`, `extern_c_text_len_e2e`
- [x] Verify `attr name("symbol")` maps to correct C symbol — `extern_c_attr_name_basic`
- [x] Verify `attr abi("cdecl")` and `attr abi("stdcall")` work — `extern_c_abi_stdcall_basic`
- [x] Verify top-level primitive callbacks work across FFI boundary — `extern_c_callback_basic`
- [x] Verify managed arguments are retained for call duration and released after — C emitter FFI shield (zt_retain / zt_release) for managed args in extern calls
- [x] Add negative fixture: captured closure callback across FFI → rejected — `extern_c_callback_closure_error`
- [x] Add negative fixture: extern variable → rejected — `extern_c_var_error`
- [x] Add negative fixture: variadic extern → rejected — `extern_c_variadic_error`
- [x] Harden managed value crossing rules — checker rejects struct/callable in extern signatures; emitter retains/releases managed values via FFI shield
- [x] Harden conditional extern support (future — deferred post-v1; current cdecl/stdcall cover v1 needs)
- [x] Verify native resources wrapped behind `Disposable` + `using` pattern works — `extern_c_disposable_resource`

---

## Phase 11 — Attributes, Comments & Text Interpolation

- [x] Verify `attr test` functions have no params and no type params — parser enforces; `std_test_attr_pass_skip`, `std_test_attr_fail`
- [x] Verify `attr skip` without `attr test` produces error — `attr_skip_without_test_error`
- [x] Verify `attr deprecated("msg")` emits `declaration.deprecated` warning — `attributes_v1`
- [x] Verify `attr todo("msg")` emits `declaration.todo` warning — `attributes_v1`
- [x] Verify `attr name("symbol")` and `attr abi("cdecl")` work in extern blocks — `extern_c_attr_name_basic`, `extern_c_abi_stdcall_basic`
- [x] Verify unrecognized attributes produce error with known-set message — `attr_unrecognized_error`
- [x] Verify attributes on non-func declarations produce error — `attr_on_struct_error`
- [x] Verify `--` line comments are ignored by parser — `extern_c_disposable_resource` and all fixtures use `--` comments
- [x] Verify `--- ... ---` block comments are ignored by parser — parser skips block comments; implicitly covered across codebase
- [x] Add negative fixture: `//` C-style comment → error — `comment_c_style_line_error`
- [x] Add negative fixture: `/* */` C-style block comment → error — `comment_c_style_block_error`
- [x] Verify `f"hello {name}"` works with `TextRepresentable` types — `fmt_interpolation_basic` (text, int, bool, expressions, `{{` literal)
- [x] Verify `f"""{expr}"""` triple-quoted interpolation works — `fmt_interpolation_triple_quoted`
- [x] Verify `{{` produces literal `{` in interpolated strings — `fmt_interpolation_basic`
- [x] Verify empty `{}` produces compile error — `fmt_interpolation_empty_expr_error`
- [x] Verify unterminated `{expr` produces compile error — `fmt_interpolation_unterminated_error`
- [x] Confirm `fmt"..."` is fully removed (Phase 1) and produces error — `fmt_removed_error`

---

## Phase 12 — Type Aliases & Slice Syntax

- [x] Verify `type Name = Type` resolves transparently at compile time — `syntax_coherence_core` (maybe_int, printable)
- [x] Verify `public type Name = Type` is visible across modules — `syntax_coherence_core`
- [x] Add negative fixture: generic type alias → error — `type_alias_generic_error`
- [x] Add negative fixture: local type alias inside function body → error — `type_alias_local_error`
- [x] Verify `list[start..end]` slice works for `list<T>` — `list_slice_len`
- [x] Verify `text[start..end]` slice works for `text` — `text_slice_len`
- [x] Verify `bytes[start..end]` slice works for `bytes` — `bytes_slice_basic`
- [x] Verify `list[start..]` and `list[..end]` partial slices work — `list_slice_len` (head, tail)
- [x] Verify out-of-bounds slice produces runtime error or empty result — `list_slice_out_of_bounds_semantics` (OOB slice returns empty list)

---

## Phase 13 — Runtime ABI, ZIR & Backend Conformance

- [x] Verify C backend remains the oracle for all behavior — automated by `tools/run_backend_conformance.py` (`run_suite.py pr_gate --no-perf` + M16 harness)
- [x] Verify ZIR verifier catches invalid lowerings — `tests/zir/test_verifier.c` + `tests/zir/test_fixture_suite.c`
- [x] Verify source mapping contract is enforced (spans survive lowering) — `tests/zir/test_verifier.c` (`test_error_span`) + `tests/zir/test_printer.c` (`test_spans`)
- [x] Automate backend conformance runner (script that runs all fixtures against C oracle) — `tools/run_backend_conformance.py`
- [x] Expand golden ZIR fixtures to cover new audited surface — added `effect_and_jump.zir`, `cross_block_reference.zir`, `invalid_duplicate_block_label.zir`, `invalid_use_before_definition.zir`
- [x] Document ZIR/runtime ABI contracts before activating alternate backends — `docs/spec/language/post-v1-zir-consolidation.md`, `docs/spec/language/post-v1-runtime-abi-ownership-audit.md`, `docs/spec/language/post-v1-source-mapping-contract.md`, `docs/spec/language/post-v1-backend-conformance-suite.md`
- [x] Track Zig backend status (future — deferred post-v1)
- [x] Track LLVM backend status (future — deferred post-v1)
- [x] Track WASM backend status (future — deferred post-v1)

---

## Phase 14 — Standard Library

- [x] Verify core stdlib modules are implemented and functional (`std.text`, `std.math`, `std.io`, `std.fs`, etc.) — `std_text_basic`, `std_math_basic`, `std_io_basic`, `std_fs_aliases_basic` in `tests/behavior/` (run-pass), validated in `python run_suite.py smoke --no-perf` (47/47)
- [x] Verify minimal implicit prelude contains only approved symbols — prelude contract in `docs/spec/language/syntax-semantics-by-topic.md` (section "Prelude And Builtins") plus negative fixture `tests/behavior/stdlib_import_required_error`
- [x] Verify explicit import required for stdlib modules — `tests/behavior/stdlib_import_required_error` + `tests/fixtures/diagnostics/stdlib_import_required_error.contains.txt` (`math` unresolved without `import std.math as math`)
- [x] Verify `std.test` and `attr test` integration works — `tests/driver/test_zt_test_filter.py` with `tests/behavior/std_test_attr_fail`/`std_test_attr_pass_skip`
- [x] Track HTTP/TLS/WebSocket/server API implementation (future) — `docs/spec/language/stdlib-reference-by-topic.md` (Deferred Network Work), `docs/spec/language/post-v1-surface-contract.md` (`std.net` expansion)
- [x] Track generic streams/sinks implementation (future) — `docs/spec/language/post-v1-surface-contract.md` (Generic stream abstraction)
- [x] Track generic lazy evaluation implementation (future) — `docs/spec/language/post-v1-surface-contract.md` (Lazy iterators)
- [x] Track cross-type HOF expansion (future) — `docs/spec/language/stdlib-reference-by-topic.md` (list HOF backend notes), `docs/spec/language/post-v1-surface-contract.md` (Generic HOFs)
- [x] Track package graduation policy implementation (future) — `docs/spec/language/post-v1-surface-contract.md` (Ecosystem roadmap)
- [x] Verify `std.shared` and `std.atomic` are treated as advanced/low-level in docs — `docs/spec/language/syntax-semantics-by-topic.md`, `docs/spec/language/stdlib-reference-by-topic.md`

---

## Phase 15 — Formatting Conformance

- [x] Verify `zt fmt` uses 4-space indentation — `tests/driver/test_fmt_phase15.py` asserts canonical indent depth in formatter output
- [x] Verify `zt fmt` rejects tabs and converts to spaces — `tests/driver/test_fmt_phase15.py` checks tabful input and tab-free formatted output
- [x] Verify `end` aligns with opening construct after formatting — validated in `tests/driver/test_fmt_phase15.py` and `tests/formatter/run_formatter_golden.py`
- [x] Verify one blank line between top-level declarations — validated in `tests/driver/test_fmt_phase15.py` + formatter golden fixtures
- [x] Verify `case` aligns with `match` (no extra indentation) — validated in `tests/driver/test_fmt_phase15.py` and `tests/formatter/cases/case_match/expected/src/app/main.zt`
- [x] Verify one `attr` per line, no blank line before declaration — formatter now preserves function attrs; validated in `tests/driver/test_fmt_phase15.py` and `tests/formatter/cases/case_all/expected/src/app/main.zt`
- [x] Verify `zt fmt --check` returns non-zero for unformatted code — explicit fail/pass assertions in `tests/driver/test_fmt_phase15.py`
- [x] Verify multiline formatting for long signatures (>100 cols) — validated in `tests/driver/test_fmt_phase15.py` and `tests/formatter/cases/case_reading_first/expected/src/app/main.zt`
- [x] Verify multiline formatting for long function calls — validated in `tests/driver/test_fmt_phase15.py` (long named-call wrapping)
- [x] Verify no vertical alignment in formatter output — validated in `tests/driver/test_fmt_phase15.py` (canonical spacing without column padding)
- [x] Verify type alias formatting: `type Name = Type` — validated in `tests/driver/test_fmt_phase15.py` (`type score_map = map<text, int>`)

---

## Phase 16 — Tooling

- [x] Verify `zt check`, `zt build`, `zt run`, `zt test`, `zt fmt`, `zt doc` all work — `tests/driver/test_phase16_tooling.py`; validated 2026-05-05
- [x] Verify `zenith.ztproj` manifest parsing with strict unknown key errors — `tests/behavior/project_unknown_key_manifest` plus `tests/driver/test_phase16_tooling.py`
- [x] Verify project kinds `app` and `lib` work correctly — app fixtures plus root `zenith.ztproj --all` and `packages/borealis/zenith.ztproj --all`
- [x] Verify `zpm install --locked` CI behavior — `tests/driver/test_zpm_lockfile.py` and `tests/driver/test_zpm_semver.py`
- [x] Verify `zt fmt --check` integration in CI pipeline — `.github/workflows/ci.yml` runs `tests/driver/test_fmt_phase15.py` in the tooling gate
- [x] Harden LSP diagnostics and completion (incremental) — `tests/lsp/test_lsp_smoke.py`; validated 2026-05-05
- [x] Track `zt bench` implementation (future) — accepted future tooling direction in `docs/spec/language/tooling-model.md` and public tooling guide
- [x] Track `zt migrate` implementation (future) — accepted future tooling direction in `docs/spec/language/tooling-model.md` and closure docs
- [x] Track web playground implementation (future) — future ecosystem/tooling item tracked outside the final language blocker set
- [x] Track package registry implementation (future) — future ZPM/ecosystem item tracked outside the final language blocker set
- [x] Track marketplace extension polish (future) — future IDE/ecosystem item tracked outside the final language blocker set

---

## Phase 17 — Parking Lot Cleanup

- [x] Remove `group` from remaining active teaching references — current public docs teach `tuple`; negative fixtures and historical/migration references remain only as rejection evidence
- [x] Add runtime diagnostic for unsupported concurrency payload types (non-int) — current blocker is compile-time capability diagnostic `wave4_concurrency_generic_type_error`; runtime remains unreachable for unsupported payloads
- [x] Verify operator overloading docs show trait names (`Addable`, `Subtractable`, `Comparable`) — public language reference pairs each operator with its trait name
- [x] Verify legacy concurrency `_int` API names do not appear as the recommended public concurrency surface — public stdlib docs teach typed facades first; intentional helper names such as `equal_int` remain documented where they are real APIs
- [x] Verify typed facades (`Job<T>`, `Channel<T>`, etc.) are used in all examples — public stdlib docs list typed facades as the teaching surface
- [x] Create formatting/teaching guidance for symbol-heavy expressions — `docs/reference/language/expression-readability.md`
- [x] Add style guide examples: when to use intermediate variables vs chaining — `docs/reference/language/expression-readability.md`

---

## Phase 18 — Documentation Rebuild

- [x] Create Language Reference from audited final contract decisions — `docs/public/language/language-reference.md`
- [x] Create "Learn Zenith in 30 Minutes" tutorial — `docs/public/learn/learn-zenith-in-30-minutes.md`
- [x] Create Cookbook with practical recipes — `docs/public/learn/cookbook.md`
- [x] Create Stdlib Reference from public symbol docs — `docs/public/stdlib/stdlib-reference.md`
- [x] Create Tooling Guide (`zt`, `zpm`, `zt fmt`, `zt test`, `zt doc`) — `docs/public/packages/tooling-guide.md`
- [x] Create Language Comparison (Zenith vs other languages, didactic) — `docs/public/language/language-comparison.md`
- [x] Verify all public docs use `f"..."` (not `fmt"..."`) — public docs audited and `tools/check_docs_current_syntax.py` now scans `docs/public`
- [x] Verify all public docs use `tuple` (not `group`) — public docs teach only `tuple`; rejection references are phrased as removed syntax
- [x] Verify all public docs use `case else:` (not `default`) — public docs teach only `case else:`
- [x] Verify all public docs use `case ... if guard:` (not `given`) — public docs teach guard syntax with `if`
- [x] Verify all public docs use `any<Trait>` (not `dyn`) — public docs teach only `any<Trait>`
- [x] Run `python tools/check_docs_current_syntax.py` — must pass; validated 2026-05-05

---

## Phase 19 — Final Validation

- [x] `python build.py` passes — validated on 2026-05-07 after the final Phase 19 fixes.
- [x] All positive behavior fixtures: `zt check`, `zt build`, `zt run` pass — `python run_suite.py pr_gate --no-perf` passed 359/359 on 2026-05-07.
- [x] All negative behavior fixtures: `zt check` returns non-zero with expected diagnostics — covered by the same `pr_gate` behavior pass.
- [x] `zt fmt --check` on active release projects and formatter fixtures passes — root stdlib project, Borealis package, touched behavior fixtures, and formatter golden fixture checked on 2026-05-07.
- [x] `python tools/check_docs_current_syntax.py` passes — validated directly and through `pr_gate` on 2026-05-07.
- [x] `git diff --check` passes (no whitespace issues) — validated on 2026-05-07; only Git CRLF conversion warnings remain.
- [x] No references to removed syntax (`group`, `fmt"..."`, `given`, `default`, `dyn`) in active syntax paths — targeted scan for removed syntax forms passed on 2026-05-07. Domain words such as `group` field names are allowed.
- [x] All matrix areas from `final-language-contract.md` have at least one passing fixture or verification — final `pr_gate` covers frontend, runtime, backend, tooling, docs, hardening, and fuzz replay.
- [x] All "future" items are tracked with clear status in this plan — Appendix A, Appendix B, and post-v1 planning docs carry the remaining explicit deferrals.
- [x] Validate Appendix A: every stdlib correction item is implemented, explicitly deferred, or explicitly rejected with rationale — see "Appendix A Final State For Phase 19" below
- [x] Validate every Appendix A item that changes, updates, modifies, or rectifies behavior/API has matching implementation, behavior fixtures, docs, and migration notes where needed — see Appendix A validation checklist
- [x] Validate Appendix A deferrals do not contradict the public v1 docs or the final language contract — final state reconciled against `docs/public/stdlib/stdlib-reference.md`, `docs/spec/language/final-language-contract.md`, and `docs/spec/language/stdlib-reference-by-topic.md`
- [x] Validate Appendix B: current generic memory subset is implemented and tested; enums, optional/result payloads, nested mutable managed values, tuple/struct set keys, managed map values, and allocator resources are closed as explicit maturation work with fixture evidence.

Phase 19 final validation record (2026-05-07):

- `python build.py` passed.
- `python run_suite.py pr_gate --no-perf` passed 359/359. Report: `reports/suites/pr_gate__20260507T084507Z.json`.
- `zt fmt --check` passed for `zenith.ztproj`, `packages/borealis/zenith.ztproj`, `tests/behavior/borealis_foundations_stub/zenith.ztproj`, `tests/behavior/std_net_basic/zenith.ztproj`, `tests/behavior/float_arithmetic_nested/zenith.ztproj`, and `tests/formatter/cases/case_all/expected`.
- `python tools/check_docs_current_syntax.py` passed.
- `git diff --check` passed.
- Removed syntax scan passed for active `.zt` syntax forms: `fmt"`, `given`, `group`, `dyn`, `default`, and `case else`.
- Runtime cache locking was stabilized so concurrent compilation waits for the owner process to release `zenith_rt.o.lock`; `python tests/hardening/test_concurrent_compilation.py` passed 5 consecutive runs before the final `pr_gate`.

### Appendix A Final State For Phase 19

| Item | Final state | Evidence |
|---|---|---|
| A.1 Text search optional API | Implemented | `std_text_basic`, `index_of_or_minus_one` compatibility notes |
| A.2 Generic `std.list` helpers | Implemented for executable primitive/text subset; managed generic structs deferred | `list_value_api_basic`, `list_value_api_primitives` |
| A.3 Generic `std.map` value API | Implemented for generated `int`/`text` key and primitive/text value subset; unsupported keys rejected | `map_value_api_basic`, `map_value_api_generic`, `map_value_api_unsupported_key_error` |
| A.4 Iterable `std.collections` | Implemented for current list-backed/specialized runtime collections | `std_collections_values_iteration`, `std_collections_queue_stack_cow`, `std_collections_managed_arc` |
| A.5 `std.math.nan` constant decision | Function form kept; constants deferred | `std_math_nonfinite_policy`, `std_math_nan_order_error` |
| A.6 `std.os.args` | Implemented and documented | `std_os_args_basic`, `tests/driver/test_std_os_args_cli.py` |
| A.7 Broader `std.validate` | Implemented for executable suffix-based helper families | `std_validate_broader` |
| A.8 Generic `std.lazy` | Implemented for `int`, `float`, `bool`, and `text`; fully generic managed lazy deferred | `lazy_primitive_text_basic`, `lazy_explicit_order_basic`, `lazy_reuse_error` |
| A.9 Generic concurrency payloads | Boundary closed as `int` runtime facades plus explicit copy helpers; non-`int` jobs/channels/shared/atomic deferred with diagnostics | `std_concurrent_boundary_copy_basic`, `wave4_concurrency_generic_type_error`, `std_shared_text_type_error`, `std_atomic_bool_type_error` |
| A.10 `std.http` completion boundary | Minimal blocking HTTP v1 recorded; full HTTP client/server features deferred | `std_http_basic` loopback GET/POST |
| A.11 Generic `std.shared` | `Shared<int>` executable facade; generic managed storage deferred | `wave4_concurrency_generic_surface`, `std_shared_text_type_error` |
| A.12 `std.atomic` type boundary | `Atomic<int>` only; arbitrary `Atomic<T>` and `Atomic<bool>` deferred | `wave4_concurrency_generic_surface`, `std_atomic_bool_type_error` |
| A.13 Generic ORC introspection | Public advanced type-specific diagnostics API; generic managed ORC deferred | `wave3_runtime_memory_surface` |
| A.14 Generic `std.mem` | Compiler-known `mem.own/view/edit` for the finalized Appendix B safe subset; deeper managed shapes tracked in Appendix B | `wave3_runtime_memory_surface`, `std_mem_generic_facade_basic`, `std_mem_appendix_b_values`, `std_mem_appendix_b_deferred_type_error`, `std_mem_appendix_b_nested_list_deferred_error`, `std_mem_appendix_b_set_list_key_deferred_error`, `std_mem_appendix_b_set_tuple_key_deferred_error`, `std_mem_appendix_b_map_key_deferred_error`, `std_mem_appendix_b_map_nested_value_deferred_error`, `std_mem_appendix_b_managed_struct_deferred_error`, `std_mem_appendix_b_enum_payload_deferred_error`, `std_mem_appendix_b_optional_payload_deferred_error` |
| A.15 Generic `std.unsafe` | Type-specific escape hatches; generic retain/introspection deferred | `wave3_runtime_memory_surface` |
| A.16 `std.debug` helper surface | `std.debug.size_of(value)` and `std.debug.type_name(value)` compiler-known helpers implemented | `std_debug_basic` |

---

## Appendix A — Standard Library Correction Plan

This appendix tracks stdlib issues found after the Phase 18 documentation rebuild.
Phase 19 must validate this appendix before the implementation plan can close.

Each item must end in one of three states:

- `implemented` — code, tests, and docs are aligned.
- `deferred` — the item is intentionally post-v1, with a clear reason and public docs that do not promise it as current behavior.
- `rejected` — the idea was reviewed and rejected, with the reason recorded.

### A.1 Text Search Should Use `optional`

Status: implemented 2026-05-05. Validated with `zt check/build/run tests/behavior/std_text_basic`,
`zt check zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`,
`zt check/build tests/perf/micro_algorithm_core`, and `python run_suite.py smoke --no-perf` (48/48).

- [x] Change or wrap `std.text.index_of` and `std.text.last_index_of` so the public final API does not use `-1` as the normal "not found" signal.
- [x] Prefer `optional<int>` for the canonical API.
- [x] If compatibility is needed, keep sentinel behavior behind explicit names such as `index_of_or_minus_one` and `last_index_of_or_minus_one`.
- [x] Update `std.text.contains` to use the canonical API internally.
- [x] Add behavior fixtures for found, not found, empty needle, first match, and last match.
- [x] Update stdlib docs and migration notes for the breaking return-type change.

### A.2 Generic `std.list` Helpers

- [x] Make basic `std.list` helpers work for executable primitive lists, not only `list<int>` and `list<text>`.
- [x] Route `append`, `prepend`, `get`, `set`, `remove_first`, `remove_last`, `remove_at`, `slice`, `first`, `last`, `rest`, `skip`, `contains`, `reverse`, `concat`, and `index_of` through specialized runtime entry points for primitive and text lists. `insert` remains out of scope because it is not a stable public `std.list` helper yet.
- [x] Keep diagnostics clear when a helper is still unsupported for a type.
- [x] Add fixtures for the newly supported primitive value API while preserving existing `int`, `text`, and selected `any<Trait>` list coverage. Struct, managed struct, and enum value-style helpers remain deferred until the managed generic list runtime is widened.
- [x] Update docs so `list<T>` is the normal API surface and the backend subset is explicit.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/list_value_api_primitives`, `zt check/build/run tests/behavior/list_value_api_basic`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` passed on 2026-05-05/2026-05-06.

### A.3 Generic `std.map` Value API

Status: implemented 2026-05-06 for the executable C backend subset.

- [x] Make `map.set`, `map.remove`, `map.keys`, `map.values`, and `map.merge` work for generated `map<K, V>` specializations where `K` is `int` or `text` and `V` is a primitive value or `text`.
- [x] Preserve type contracts: `keys(map<K, V>) -> list<K>`, `values(map<K, V>) -> list<V>`, and `merge(map<K, V>, map<K, V>) -> map<K, V>`.
- [x] Define supported key constraints clearly: the current C backend requires compiler-known equality/hash support, available for `int` and `text` keys in this slice.
- [x] Add fixtures for `map<text, text>`, `map<int, text>`, `map<text, int>`, and managed text values. Struct-managed map values remain deferred until generic map iteration/extraction is widened beyond generated primitive/text storage.
- [x] Add negative fixtures for unsupported key/value shapes with readable diagnostics.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/map_value_api_basic`, `zt check/build/run tests/behavior/map_value_api_generic`, `zt check tests/behavior/map_value_api_unsupported_key_error`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (50/50) passed on 2026-05-06.

### A.4 Current-Subset And Iterable `std.collections`

Status: implemented 2026-05-06 for list-backed queues/stacks and the executable `int`/`text` specialized collection runtime.

- [x] Convert collection helpers from concrete `int`/`text` variants toward generic collection types only where the runtime supports it. `queue_values<T>` and `stack_values<T>` are compiler-known because queues/stacks are `list<T>` backed; specialized runtime collections expose `*_values`/`*_keys` snapshots for their supported `int`/`text` shapes. Fully generic advanced collection storage remains post-RC debt.
- [x] Define iteration behavior for each collection type: queue order, stack order, set/map order, tree sorted order, and grid traversal order. Queue snapshots are front-to-back; stack snapshots are bottom-to-top storage order; Grid2D is row-major; Grid3D is layer-row-column; priority queues are pop order; circular buffers are oldest-to-newest; B-tree map/set snapshots are sorted text order.
- [x] Add type constraints for structures that need ordering or comparison, such as tree maps, tree sets, and priority queues. Current executable ordering is `int`/`text` priority queue ordering and text B-tree ordering; arbitrary `T` ordering remains deferred until generic ordered runtime support exists.
- [x] Add fixtures for generic queue, stack/list-like collections, sets, maps, priority queues, and grid helpers via `tests/behavior/std_collections_values_iteration`.
- [x] Document which ordering guarantees are stable and which are intentionally unspecified.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/std_collections_values_iteration`, `zt check/build tests/behavior/std_collections_queue_stack_cow`, `zt check/build tests/behavior/std_collections_managed_arc`, `zt check/build tests/behavior/std_collections_basic`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (51/51) passed on 2026-05-06.

### A.5 `std.math.nan` Constant Decision

Status: implemented 2026-05-06 for the executable C backend.

- [x] Decide whether `nan` and `infinity` can be true compile-time constants in the current constant model. Decision: not in this cut; the current constant model exposes finite float literals but no safe non-finite float literal.
- [x] If non-finite float constants are supported safely, expose `math.nan` and `math.infinity` as constants. Deferred until true non-finite float constants exist.
- [x] If not, keep `nan()` and `infinity()` as functions and document why they are functions. Both now use explicit runtime helpers instead of division-by-zero construction.
- [x] Add tests for NaN construction, infinity construction, comparison behavior, and formatting behavior. Added `std_math_nonfinite_policy` and `std_math_nan_order_error`; float arithmetic emission now preserves IEEE behavior for `+`, `-`, `*`, and `/`.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/std_math_nonfinite_policy`, `zt check/build/run tests/behavior/std_math_basic`, `zt check/build tests/behavior/std_math_nan_order_error`, `zt run tests/behavior/std_math_nan_order_error` (expected runtime failure with `runtime.float_nan_compare`), `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (53/53) passed on 2026-05-06.

### A.6 `std.os.args` Documentation And Validation

Status: implemented 2026-05-06 for `zt run` argument forwarding and `std.os.args()` docs.

- [x] Document `std.os.args() -> list<text>` as the canonical way to read terminal arguments.
- [x] Clarify whether `args()[0]` is the executable path/name on each supported platform. Contract: it is the executable name/path received from the host or launcher, not a stable program identity.
- [x] Document how `zt run` forwards arguments to the program. Contract: compiler/driver options stay before `--`; values after `--` are forwarded to the program and appear after `args()[0]`.
- [x] Add driver/runtime tests that pass arguments through the CLI and verify the resulting `list<text>`.
- [x] Validation: `python build.py`, `zt run tests/behavior/std_os_args_basic --ci`, `zt run tests/behavior/std_os_args_basic --ci -- alpha "two words" --literal-flag`, `python tests/driver/test_std_os_args_cli.py`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (54/54) passed on 2026-05-06.

### A.7 Broader `std.validate`

Status: implemented 2026-05-06 for the executable C backend subset.

- [x] Expand validation helpers beyond `int` and `text`. Added float predicates, bool predicates, primitive optional/result state predicates, supported list length helpers, and supported map-size helpers.
- [x] Add focused helpers for `float`, `bool`, `optional<T>`, `result<T, E>`, `list<T>`, `map<K, V>`, and common length/range checks. Current public helpers use explicit suffixes (`_float`, `_list_text`, `_map_text_int`, etc.) because public stdlib generics and overloads are not yet stable through import and C emission.
- [x] Avoid domain-specific validators that would make the core stdlib noisy or culturally specific. Email, URL, phone, UUID, regex-backed validators, parsing, and sanitization remain out of scope.
- [x] Add fixtures for valid and invalid values across the new helper groups. Added `tests/behavior/std_validate_broader`.
- [x] Update docs with short, low-cognitive-load examples and explicit backend boundaries.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/std_validate_broader --ci`, `zt run tests/behavior/std_validate_basic --ci` (expected exit `42`), `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-06.

### A.8 Generic `std.lazy`

Status: implemented 2026-05-07 for the executable primitive/text subset.

- [x] Decide the v1 boundary for `Lazy<T>`: primitive/text helpers now cover `int`, `float`, `bool`, and `text`; fully generic managed `lazy<T>` remains post-v1.
- [x] Expose additional primitive helpers if the backend already supports them. Added `once_bool`, `force_bool`, and `is_consumed_bool` over the existing `zt_lazy_bool_*` runtime path.
- [x] Implement generic managed-value lazy helpers only after clone/drop semantics are correct for `T`. Deferred until public stdlib generics and managed clone/drop semantics are stable through import and C emission.
- [x] Add fixtures for forcing once, repeated force, consumed state, and managed values if enabled. Extended `lazy_primitive_text_basic`; existing `lazy_explicit_order_basic` and `lazy_reuse_error` keep order/reuse coverage.
- [x] Document the ownership behavior of lazy values. Lazy remains explicit, one-shot, and consumed by matching `force_*` helpers.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/lazy_primitive_text_basic`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.9 Generic Concurrency Payloads

Status: implemented 2026-05-07 as a clear boundary over the current C runtime; expanded on 2026-05-10 for `Job<text>` and `Channel<text>`.

- [x] Decide which `Transferable` payloads are supported by `Job<T>`, `Channel<T>`, and related helpers in v1. `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>`, and `Atomic<int>` are executable in the current C oracle.
- [x] Implement wider runtime payload storage only if it can preserve deep-copy and cleanup guarantees. Text job/channel payloads are implemented; wider job/channel/shared/atomic payload storage is deferred; boundary-copy helpers cover `int`, `bool`, `float`, `text`, `bytes`, `list<int>`, `list<text>`, and `map<text,text>`.
- [x] Keep unsupported payload diagnostics clear and early. Existing job diagnostics and new shared/atomic negative fixtures reject unsupported payloads at check time.
- [x] Add fixtures for supported primitive, text, struct, and collection payloads, or mark each unsupported group as deferred. Extended `std_concurrent_boundary_copy_basic`; kept `wave4_concurrency_surface`, `wave4_concurrency_generic_surface`, `wave4_concurrency_generic_type_error`, and added shared/atomic unsupported fixtures.
- [x] Keep public docs aligned with the real supported payload set.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/std_concurrent_boundary_copy_basic`, `zt check tests/behavior/std_shared_text_type_error`, `zt check tests/behavior/std_atomic_bool_type_error`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.10 `std.http` Completion Boundary

Status: implemented 2026-05-07 as a minimal blocking HTTP v1 boundary.

- [x] Treat current `std.http` as incomplete unless a deliberate minimal-v1 boundary is recorded. It is now documented as a small blocking HTTP client, not a full HTTP framework.
- [x] Define the v1 HTTP contract: blocking `GET`/`POST` over `http://`, `Response.status`, `Response.body`, typed `Error`/`ErrorKind`, and minimal `headers` field presence.
- [x] Implement or explicitly defer headers parsing, request options, non-GET/POST methods, bytes bodies, and structured errors. Deferred: HTTPS/TLS, redirects, timeout options, custom headers, streaming, chunked transfer decoding, non-GET/POST methods, and bytes bodies.
- [x] Add integration-style fixtures using deterministic local/server or mock runtime behavior. Extended `std_http_basic` loopback server to validate both GET and POST.
- [x] Update docs so they do not imply a complete HTTP client if only a minimal subset exists.
- [x] Validation: `python build.py`, `zt check/build tests/behavior/std_http_basic`, `tests/behavior/std_http_basic/run-loopback.ps1`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.11 Generic `std.shared`

Status: implemented 2026-05-07 as an `int`-backed typed facade.

- [x] Decide whether `Shared<T>` is v1-generic or int-backed with typed facade only. Decision: `Shared<T>` remains public typed facade; current executable backend supports `Shared<int>`.
- [x] If generic, implement managed-value storage, cloning, release, and synchronization rules. Deferred until typed runtime storage and managed ownership rules are safe for non-`int` payloads.
- [x] Add fixtures for shared primitive, text, struct, and collection values. `wave4_concurrency_surface` and `wave4_concurrency_generic_surface` cover `Shared<int>`; non-`int` groups are explicitly deferred.
- [x] Add negative fixtures for non-transferable or unsupported values. Added `std_shared_text_type_error`.
- [x] Document that `Shared<T>` is an advanced concurrency primitive.
- [x] Validation: `zt check/build/run tests/behavior/wave4_concurrency_generic_surface`, `zt check tests/behavior/std_shared_text_type_error`, `zt check zenith.ztproj --all --ci`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.12 `std.atomic` Type Boundary

Status: implemented 2026-05-07 as `Atomic<int>` only.

- [x] Do not expose arbitrary `Atomic<T>` unless the compiler/runtime can guarantee true atomic representation for `T`.
- [x] Prefer explicit supported forms such as `Atomic<int>` and possibly `Atomic<bool>`. Decision: only `Atomic<int>` is stable in this cut; `Atomic<bool>` remains deferred until it has a real runtime representation.
- [x] If managed generic values are needed, model them as a locked cell/shared cell rather than a CPU atomic. Documented as future shared/locked-cell work, not `Atomic<T>`.
- [x] Add fixtures for supported atomic operations and negative fixtures for unsupported types. Existing wave4 fixtures cover `Atomic<int>`; added `std_atomic_bool_type_error`.
- [x] Update docs to distinguish atomic values from shared locked values.
- [x] Validation: `zt check/build/run tests/behavior/wave4_concurrency_generic_surface`, `zt check tests/behavior/std_atomic_bool_type_error`, `zt check zenith.ztproj --all --ci`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.13 Generic ORC Introspection

Status: completed 2026-05-07 as public advanced diagnostics surface with type-specific helpers.

- [x] Decide whether `std.orc` is a public advanced API, an internal diagnostics API, or post-v1 only. Decision: public advanced/runtime diagnostics API.
- [x] If public, generalize `ref_count<T>` and `is_unique<T>` for managed `T`. Deferred; current executable helpers stay `text` and `list<text>` until generic managed runtime hooks are widened.
- [x] Keep ORC cycle collection behavior honest in docs. `collect_cycles()` remains a hook; full cycle behavior only becomes meaningful when public cycle-forming managed references exist.
- [x] Add fixtures for text, list, managed struct, and unsupported non-managed values. `wave3_runtime_memory_surface` covers text/list runtime helpers; managed-struct generic ORC remains deferred.
- [x] Avoid teaching ORC helpers as normal application code.
- [x] Validation: existing `wave3_runtime_memory_surface`, `zt check zenith.ztproj --all --ci`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.14 Generic `std.mem`

Status: updated 2026-05-07 as a compiler-known generic facade over the stabilized memory subset.

- [x] Implement or defer generic `own<T>`, `view<T>`, and `edit<T>` helpers. Implemented now as compiler-known `mem.own`/`mem.view`/`mem.edit` for the Appendix B safe subset. Concrete helpers remain backend/runtime anchors.
- [x] Resolve the known dependency on correct ORC behavior for managed structs through generic functions. Safe structs are accepted; structs with mutable managed fields, enums, optional/result payloads, and nested mutable managed values are not accepted by the generic facade yet.
- [x] Add fixtures for text, list, managed struct, and nested managed values if implemented. `wave3_runtime_memory_surface` covers concrete text/list helpers; `std_mem_generic_facade_basic` covers generic text and primitive/text list helpers; `std_mem_appendix_b_values` covers primitive scalars, safe tuples/structs, `list<tuple>`, `list<struct>`, sets, and maps. Appendix B deferred fixtures cover nested lists, unsupported set keys, unsupported map keys/values, managed structs, enum payloads, and optional payloads.
- [x] Keep allocator abstractions such as arenas and pools post-v1 unless the final contract changes. Tracked in Appendix B as `mem.Temp`/`mem.Pool` maturation work.
- [x] Update docs to keep memory helpers advanced and readable.
- [x] Validation for this Appendix B cut: `python build.py`, targeted `zt check/build/run tests/behavior/std_mem_generic_facade_basic`, targeted `zt check/build/run tests/behavior/std_mem_appendix_b_values`, targeted deferred-type checks, root checks, docs syntax, and smoke suite. Latest validation is recorded in Appendix B.

### A.15 Generic `std.unsafe`

Status: completed 2026-05-07 as type-specific escape-hatch surface.

- [x] Decide which unsafe helpers should be generic and which should stay type-specific. Decision: current v1 executable helpers stay type-specific for `text` and `list<text>`.
- [x] If generic, implement `retain<T>` and heap/introspection helpers with clear constraints. Deferred until generic managed runtime hooks and public stdlib generics are stable.
- [x] Add fixtures for supported managed values and negative fixtures for unsupported values. `wave3_runtime_memory_surface` covers implemented text/list unsafe helpers; generic unsafe helpers are not exposed.
- [x] Keep public docs explicit that these helpers are escape hatches, not normal code.
- [x] Validation: existing `wave3_runtime_memory_surface`, `zt check zenith.ztproj --all --ci`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### A.16 `std.debug` Helper Surface

Status: implemented 2026-05-07 for compiler-known `size_of` and `type_name`.

- [x] Align `std.debug` source declarations, compiler-known behavior, tests, and docs. `std.debug` now documents the compiler-known behavior explicitly.
- [x] Make `debug.size_of` and `debug.type_name` look generic in the public contract if the compiler already supports them generically. Added checker/lowering support for `std.debug.type_name(value)`; `std.debug.size_of(value)` already existed.
- [x] Add focused helpers such as `debug.check`, `debug.dump`, `debug.trace`, and `debug.breakpoint` only if they have clear runtime behavior. Deferred; no new helpers were added because runtime/logging/breakpoint behavior is not yet specified.
- [x] Add fixtures for primitive, text, tuple, struct, enum, list, map, and `any<Trait>` debug cases. Extended `std_debug_basic`.
- [x] Keep unsafe memory/heap details in `std.unsafe` or `std.orc`, not in the normal debug surface.
- [x] Validation: `python build.py`, `zt check/build/run tests/behavior/std_debug_basic`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (55/55) passed on 2026-05-07.

### Appendix A Validation Checklist

- [x] Every implemented correction has at least one positive behavior fixture. Evidence: A.1-A.16 status entries and the Phase 19 final-state table list the fixture for each implemented item.
- [x] Every unsupported or rejected correction has either a negative fixture or a documented checker/runtime diagnostic. Evidence: unsupported map keys, non-`int` jobs/shared/atomic payloads, NaN ordering, lazy reuse, and deferred generic memory/runtime APIs are covered by negative fixtures or explicit documented deferrals.
- [x] Every public API change is reflected in `docs/public/stdlib/stdlib-reference.md`. Evidence: the public stdlib reference has Appendix A API notes plus current lazy/concurrency/http/debug/memory boundaries.
- [x] Every breaking API change has a migration note or compatibility alias decision. Evidence: text search keeps `index_of_or_minus_one`/`last_index_of_or_minus_one`; `type_name(value)` remains accepted as compatibility while new docs teach `std.debug.type_name(value)`; other changes are additive or explicit deferrals.
- [x] `docs/spec/language/final-language-contract.md` and `docs/spec/language/stdlib-reference-by-topic.md` agree with the final state of this appendix. Evidence: final contract keeps memory/concurrency as typed facades with advanced/deferred generic storage, and stdlib reference records the concrete public surface.
- [x] Phase 19 records the final state of every Appendix A item. Evidence: "Appendix A Final State For Phase 19" records A.1-A.16 with state and fixture evidence.

---

## Appendix B — Generic Memory Type Maturation Plan

Purpose: finish the remaining type work behind `std.mem`, collection value semantics, and future allocator resources without adding ownership keywords, borrow syntax, or lifetimes to Zenith core.

### B.1 Stabilized Generic `std.mem` Facade

Status: implemented and finalized on 2026-05-07 for the safe executable subset.

- [x] Add compiler-known `mem.own(value)`, `mem.view(value)`, and `mem.edit(value)`.
- [x] Support primitive scalars by returning the value inline.
- [x] Support `text`.
- [x] Support the currently executable primitive/text list subset: `list<int>`, `list<float>`, `list<bool>`, `list<int8>`, `list<u8>`, and `list<text>`.
- [x] Support safe tuples: tuple fields may be scalar/text or nested safe tuples.
- [x] Support safe structs: struct fields may be primitive scalars, `text`, safe tuples, or nested safe structs. Structs with mutable managed fields stay deferred.
- [x] Support `list<safe tuple>`.
- [x] Support `list<safe struct>`.
- [x] Support `set<int>` and `set<text>`.
- [x] Support primitive/text-key maps with scalar/text values, including `map<int,text>` and `map<text,int>`.
- [x] Keep concrete helpers (`own_text`, `view_text`, `edit_text`, `own_list_text`, `view_list_text`, `edit_list_text`) as public compatibility/runtime anchors.
- [x] Add positive fixture: `std_mem_generic_facade_basic`.
- [x] Add negative fixture: `std_mem_generic_facade_unsupported_type_error`.
- [x] Add Appendix B positive fixture: `std_mem_appendix_b_values`.
- [x] Add Appendix B deferral fixtures: `std_mem_appendix_b_deferred_type_error`, `std_mem_appendix_b_nested_list_deferred_error`, `std_mem_appendix_b_set_list_key_deferred_error`, `std_mem_appendix_b_set_tuple_key_deferred_error`, `std_mem_appendix_b_map_key_deferred_error`, `std_mem_appendix_b_map_nested_value_deferred_error`, `std_mem_appendix_b_managed_struct_deferred_error`, `std_mem_appendix_b_enum_payload_deferred_error`, and `std_mem_appendix_b_optional_payload_deferred_error`.

### B.2 Type Capability Matrix

Status: implemented for the `std.mem` gate.

- [x] Define compiler-visible capabilities for each accepted concrete type shape:
  `Clone`, `Destroy`, `Retain`, `Move`, `Editable`, `Iterable`, `Equatable`, `Hashable`, and `Transferable`.
- [x] Use those capabilities for diagnostics before lowering to runtime helpers.
- [x] Keep diagnostics concrete and readable. Example: `std.mem.edit(...) currently supports primitive scalars, text, safe tuples/structs, primitive/text lists, list<safe tuple/struct>, set<int/text>, and primitive/text-key maps with scalar/text values; got tuple<int, list<text>>`.
- [x] Add fixture coverage for unsupported categories before enabling them. Current negative coverage: nested mutable tuple/list shape, nested lists, unsupported set keys, unsupported map keys/values, managed struct, enum payload, and optional payload.
- [x] Keep the capability matrix conservative: a type is accepted only when the backend can preserve values and `mem.edit` can return an isolated editable value for the supported shape.

### B.3 Tuples

Status: implemented for safe tuples; deeper managed tuple fields deferred.

- [x] Reuse generated tuple structs plus existing field-level retain/destroy/move behavior for safe tuple values.
- [x] Allow `mem.own/view/edit(tuple<T...>)` only when every field satisfies the safe inline memory capability.
- [x] Add structural equality/hash only when every field is `Equatable`/`Hashable`. Deferred for tuple set/map keys; no public tuple hash is exposed yet.
- [x] Add fixtures for `tuple<int, text>` and nested safe tuple in `std_mem_appendix_b_values`.
- [x] Add a negative fixture for tuple containing `list<text>` in `std_mem_appendix_b_deferred_type_error`.

### B.4 Lists Beyond Primitive/Text Elements

Status: implemented for `list<safe tuple>` and `list<safe struct>`; nested mutable managed payloads deferred with fixtures.

- [x] Stabilize `list<tuple<...>>` for safe tuples after tuple capability checks.
- [x] Stabilize `list<safe struct>` after confirming existing generated struct retain/destroy is safe for scalar/text-only fields. Structs with mutable managed fields remain deferred.
- [x] Defer nested lists such as `list<list<int>>` and `list<list<text>>` until recursive clone/edit isolation is tested.
- [x] Add positive and negative fixtures for shallow and nested managed payloads. `std_mem_appendix_b_values` covers `list<safe tuple>` and `list<safe struct>`; `std_mem_appendix_b_deferred_type_error` covers tuple/list nested managed rejection; `std_mem_appendix_b_nested_list_deferred_error` covers direct nested list rejection.

### B.5 Sets

Status: implemented for `set<int>`, `set<text>`, and safe language-level
`set<Struct>` runtime keys. `std.mem` Appendix B keeps broader set edit
isolation deferred until its dedicated fixtures are reopened.

- [x] Allow `mem.own/view/edit(set<T>)` for the currently hashable executable set keys: `int` and `text`.
- [x] Generate language-level structural hash/equality for safe `set<Struct>` keys with bool/integral/text fields.
- [x] Keep `std.mem` set support scoped to `set<int>` and `set<text>` until edit isolation for structural set keys is covered.
- [x] Add fixtures for `set<int>` and `set<text>` in `std_mem_appendix_b_values`.
- [x] Add rejected `set<list<int>>` and `set<tuple<int,text>>` fixtures: `std_mem_appendix_b_set_list_key_deferred_error`, `std_mem_appendix_b_set_tuple_key_deferred_error`.

### B.6 Maps

Status: language-level `std.map` now supports primitive/text keys and safe
structural keys for selected operations. `std.mem` Appendix B remains scoped to
primitive/text key maps with scalar/text values until map edit isolation for
structural keys has dedicated coverage.

- [x] Allow `mem.own/view/edit(map<K,V>)` only when `K` is the current hashable key subset (`int` or `text`) and `V` is scalar/text.
- [x] Stabilize primitive/text key maps first.
- [x] Generate language-level structural hash/equality for safe `map<Struct,V>`
      keys with bool/integral/text fields.
- [x] Keep `std.mem` map support scoped to primitive/text keys until edit
      isolation for structural map keys is covered.
- [x] Defer tuple keys for `std.mem` until tuple structural hash/equality is
      wired into its edit-isolation contract.
- [x] Defer managed values such as `map<text, list<int>>` until generated map helper emission can support nested value-list helpers without forcing unsupported `list<list<T>>` APIs.
- [x] Add fixtures for supported primitive/text maps in `std_mem_appendix_b_values`.
- [x] Add rejected unsupported key/value shape fixtures: `std_mem_appendix_b_map_key_deferred_error` and `std_mem_appendix_b_map_nested_value_deferred_error`.

### B.7 Managed Structs And Enums

Status: safe structs implemented; managed structs/enums and payload wrappers explicitly deferred with diagnostics.

- [x] Reuse existing generated struct retain/destroy for safe structs and keep structs with mutable managed fields deferred until deep edit isolation is generated.
- [x] Defer enum payload operations for every variant until generated clone/edit operations exist.
- [x] Ensure `mem.edit` gives an isolated editable value only for the supported safe subset; managed fields are rejected instead of returning a retained alias.
- [x] Add a negative fixture for struct-with-list: `std_mem_appendix_b_managed_struct_deferred_error`.
- [x] Add enum-with-text, optional payload, and nested managed payload fixtures: `std_mem_appendix_b_enum_payload_deferred_error`, `std_mem_appendix_b_optional_payload_deferred_error`, `std_mem_appendix_b_nested_list_deferred_error`.

### B.8 Allocator Resources

Status: post-v1 and library-level only; no core syntax added.

- [x] Do not add `owned<T>`, `borrow<T>`, lifetimes, `move`, or `ref` syntax.
- [x] Prefer `mem.Temp` for temporary region/scratch allocation when allocator work starts.
- [x] Prefer `mem.Pool<T>` for reusable fixed-shape storage when allocator work starts.
- [x] Add a broad `std.mem.Allocator` trait only after `mem.Temp` and `mem.Pool<T>` prove real pressure.
- [x] Keep resource ownership explicit in function parameters and docs.

### Appendix B Validation Checklist

- [x] Current stabilized subset has a positive fixture: `std_mem_generic_facade_basic`.
- [x] Current unsupported type path has a negative fixture: `std_mem_generic_facade_unsupported_type_error`.
- [x] Appendix B widened subset has a positive fixture: `std_mem_appendix_b_values`.
- [x] Appendix B deferred nested managed tuple/list path has a negative fixture: `std_mem_appendix_b_deferred_type_error`.
- [x] Appendix B deferred nested list path has a negative fixture: `std_mem_appendix_b_nested_list_deferred_error`.
- [x] Appendix B deferred set key paths have negative fixtures: `std_mem_appendix_b_set_list_key_deferred_error`, `std_mem_appendix_b_set_tuple_key_deferred_error`.
- [x] Appendix B deferred map key/value paths have negative fixtures: `std_mem_appendix_b_map_key_deferred_error`, `std_mem_appendix_b_map_nested_value_deferred_error`.
- [x] Appendix B deferred managed struct path has a negative fixture: `std_mem_appendix_b_managed_struct_deferred_error`.
- [x] Appendix B deferred enum/optional payload paths have negative fixtures: `std_mem_appendix_b_enum_payload_deferred_error`, `std_mem_appendix_b_optional_payload_deferred_error`.
- [x] Every future type family must have at least one positive fixture and one unsupported-shape fixture before being marked stable. This is now enforced by the closed B.4-B.7 fixture set.
- [x] Phase 19 must validate Appendix B state before release: implemented, deferred, or rejected with evidence. Added to the Phase 19 validation contract on 2026-05-07.
- [x] Validation for this finalized B.1-B.8 cut: `python build.py`, targeted `zt check/build/run tests/behavior/std_mem_appendix_b_values`, `zt check zenith.ztproj --all --ci`, `zt check packages/borealis/zenith.ztproj --all --ci`, `python tools/check_docs_current_syntax.py`, and `python run_suite.py smoke --no-perf` (67/67) passed on 2026-05-07. Smoke includes the positive generic facade and every Appendix B deferral fixture.

---

## Dependency Map

```
Phase 1 (Syntax Cleanup) ──┬──> Phase 2 (Control Flow)
                            ├──> Phase 3 (Types/Generics/Tuples)
                            ├──> Phase 4 (Traits/Apply/Operators)
                            ├──> Phase 5 (Callables/Closures/any)
                            ├──> Phase 6 (Pattern Matching)
                            ├──> Phase 11 (Attrs/Comments/Interpolation)
                            └──> Phase 12 (Type Aliases/Slice)

Phase 2–6 ──────────────────┬──> Phase 7 (Error Model/Cleanup)
                            ├──> Phase 8 (Memory/Ownership)
                            ├──> Phase 9 (Concurrency)
                            └──> Phase 10 (FFI)

Phase 7–12 ─────────────────┬──> Phase 13 (Runtime ABI/ZIR)
                            ├──> Phase 14 (Stdlib)
                            ├──> Phase 15 (Formatting)
                            └──> Phase 16 (Tooling)

Phase 13–16 ────────────────┬──> Phase 17 (Parking Lot Cleanup)
                            └──> Phase 18 (Documentation)

Phase 17–18 ────────────────────> Phase 19 (Final Validation)

Appendix A (Stdlib Correction Plan) ──> Phase 19 (Final Validation)
Appendix B (Generic Memory Type Maturation) ──> Phase 19 (Final Validation)
```

## Notes

- Phases 1–12 are **compiler/runtime work**.
- Phases 13–16 are **infrastructure/tooling hardening**.
- Phases 17–18 are **cleanup and documentation**.
- Phase 19 is **final gate** — nothing ships without this passing.
- Appendix A is a **stdlib correction gate** — Phase 19 must close, defer, or reject every item with evidence.
- Appendix B is a **generic memory maturation gate** — Phase 19 must validate the implemented subset and keep remaining type families explicitly tracked.
- Items marked **(future — track status)** are accepted directions but not blockers for current milestone.
- The book on the language creation journey is a **separate project** and is not tracked here.

---

## Appendix C — Technical Debt (Post-v1 Hardening)

Este plano estrutura três melhorias de hardening para o Zenith depois do v1: proteção de pilha, tratamento honesto de ciclos ARC e custo de concorrência genérica.

### C.1 Instrumentação Forte de Pilha (Stack Overflow Protection)

#### Objetivo
Prevenir falhas críticas do sistema operacional, como segfaults por recursão infinita, transformando o caso em um erro `runtime.panic` do Zenith.

#### Implementação (C Backend Emitter)
- **Runtime (`zenith_rt.h` / `zenith_rt_core.c`)**:
  - Adicionar uma macro leve de checagem: `ZT_CHECK_STACK()`.
  - Manter uma base de pilha por thread: `ZT_THREAD_LOCAL uintptr_t zt_stack_base`.
  - Medir a distância entre a primeira frame Zenith gerada e a frame atual.
  - Quando o uso passar de `ZT_MAX_STACK_SIZE` bytes, chamar `zt_panic("Stack overflow prevented: maximum stack size exceeded")`.
  - Manter `ZT_POP_STACK()` como hook de saída no C gerado. No guard atual ele é no-op, porque o controle é por bytes de pilha, não por contador de chamadas.
- **Compiler (`compiler/targets/c/emitter.c`)**:
  - Modificar a emissão do corpo da função (`c_emit_function_definition`).
  - No topo de cada função Zenith gerada, injetar `ZT_CHECK_STACK();`.
  - Antes de cada `return` direto ou retorno via `zt_cleanup`, injetar `ZT_POP_STACK();`.
  - Funções nativas ou FFI não são instrumentadas diretamente, preservando a fronteira com bibliotecas externas em C.
- **Validação**:
  - `tests/behavior/panic_stack_overflow` cobre recursão sem limite.
  - O fixture exige `error[runtime.panic]` e a mensagem `Stack overflow prevented: maximum stack size exceeded`.
  - Evidência em 2026-05-07: `zt check`, `zt build` e execução esperada como `run-fail` para `tests/behavior/panic_stack_overflow/zenith.ztproj`.
  - Status: implementado em 2026-05-07.

### C.2 Coletor de Ciclos de Apoio (Backup Cycle Collector)

#### Objetivo
Evitar que ciclos de memória vazem infinitamente quando o runtime tiver APIs públicas capazes de formar grafos cíclicos fortes.

#### Implementação (Runtime)
- **Estado atual**:
  - O v1 não expõe APIs públicas de referência forte que formem ciclos fechados.
  - `zt_orc_collect_cycles()` existe como hook/runtime API e retorna `0` por design neste recorte.
  - `std.orc.collect_cycles()` deve ser documentado como introspecção avançada, não como promessa de GC completo no v1.
  - Evidência em 2026-05-07: `zt run tests/behavior/wave3_runtime_memory_surface/zenith.ztproj`.
- **Implementação futura**:
  - Rastrear apenas alocações capazes de formar grafos fechados (`list`, `map`, structs geradas com campos gerenciados).
  - Manter escalares gerenciados, como `text`, fora do rastreamento de suspeitos.
  - Adicionar um buffer de suspeitos quando existirem APIs cíclicas públicas.
  - Só ativar um `ZT_GC_THRESHOLD` configurável quando o runtime tiver metadados suficientes para contar referências internas com segurança.

---

### C.3 Análise de Custo: Concorrência Genérica via Deep Copy

#### O Problema do Custo
Fazer a cópia profunda de uma árvore de grande porte para passar para outra thread tem um custo linear de memória e CPU. No entanto, é a abordagem padrão segura:

1. **Lock-free / Zero-Contention**: Quando a Thread B recebe uma cópia, ela não disputa mutexes nem contadores atômicos com a Thread A.
2. **Semântica de Valor (Zenith's Core)**: Como o Zenith adota semântica de valor por padrão, a mutação isolada é o que o desenvolvedor espera.

#### A Solução Híbrida para Alta Performance
1. **O Padrão (`Job<T>`)**: Faz **Deep Copy**. Ideal para payloads pequenos ou médios. Seguro, isolado, rápido.
2. **O Expresso (`Shared<T>`)**: Para grafos gigantes, usa um ARC Atômico e bloqueia a mutação simultânea insegura (via lock interno ou permitindo apenas acesso read-only seguro).

---

### Perguntas Abertas / Status de Implementação

1. **Tolerância de overhead na pilha**: aprovado para o C backend atual. A versão implementada mede bytes de stack por thread, então o custo fica concentrado em uma leitura de endereço local e uma comparação simples por função Zenith gerada.
2. **Frequência do Cycle Collector**: futuro. `ZT_GC_THRESHOLD` só deve existir quando houver APIs públicas capazes de formar ciclos e metadados de grafo suficientes para coleta segura.
3. **Concorrência genérica**: o padrão continua sendo deep copy em fronteira de job; `Shared<T>` permanece caminho explícito para payloads grandes e compartilhamento controlado. Evidência em 2026-05-07: `zt run tests/behavior/std_concurrent_boundary_copy_determinism/zenith.ztproj` e check-fail esperado de `std_concurrent_boundary_copy_unsupported_error`.
