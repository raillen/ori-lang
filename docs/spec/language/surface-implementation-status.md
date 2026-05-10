# Zenith Surface Implementation Status

- Status: current compiler cut snapshot
- Date: 2026-05-10
- Documentation cleanup review: 2026-05-09
- Labels: `Spec`, `Parsed`, `Semantic`, `Lowered`, `Emitted`, `Runtime`, `Executable`, `Conformant`, `Deferred`, `Risk`, `Rejected`

## Language Surface

| Feature | Status | Notes |
| --- | --- | --- |
| Namespaces/imports/multifile | `Conformant` | covered in `multifile_*` behavior projects |
| Functions/control flow (`if/while/for/repeat/match`) | `Conformant` | covered in `control_flow_*` |
| Structs/traits/apply/methods | `Conformant` | covered in `structs_*` and `methods_*` |
| Collections (`list`, `map`, index/slice/len/get`) | `Conformant` | covered in `list_*` and `map_*`; safe structural `set<Struct>` and `map<Struct,V>` key paths are covered by `set_struct_key_basic` and `map_struct_key_basic` |
| `optional<T>` / `result<T,E>` | `Conformant` | includes `optional_match_value`, `result_question_basic`, `optional_result_helpers_pass` and `optional_result_helpers_absence_error` |
| `?` propagation (`result` + `optional`) | `Conformant` | covered in `result_question_basic` and `optional_question_basic` |
| `f"..."` / `fmt "..."` interpolation | `Conformant` | covered in `fmt_interpolation_basic` + type error case; `f"..."` is canonical per `language-reference.md`, `fmt "..."` is accepted as deprecated alias |
| `panic(...)`, `todo(...)`, `unreachable(...)` and `check(...)` | `Conformant` | covered in `panic_*`, `todo_builtin_fail`, `unreachable_builtin_fail` and `check_intrinsic_*` |
| `core.Error(...)` qualified | `Conformant` | covered in `core_error_construction` |
| Unsigned aliases (`u8/u16/u32/u64`) | `Conformant` | covered in `u_alias_basic` |
| Namespace `public var` (read public, write owner namespace) | `Conformant` | covered in `public_var_module`, `public_var_module_state`, `public_var_cross_namespace_write_error` |
| `std.random` public state (`seeded`, `last_seed`, `draw_count`, `stats`) | `Conformant` | covered in `std_random_basic`, `std_random_state_observability`, `std_random_between_branches`, `std_random_cross_namespace_write_error` |
| Closures v1 (`func ... end`, immutable capture) | `Conformant` | covered in `closure_capture_basic` and `closure_mut_capture_error`; mutable capture remains deferred |
| Lambdas v1 + primitive/text HOFs (`func(...) => expr`) | `Conformant` | covered in `lambda_hof_basic`, `list_hof_*_basic`, and `list_reduce_value_hof_basic`; HOF subset includes `std.collections.map_int/filter_int/reduce_int` and same-type `std.list` HOFs for primitive/text lists |
| Explicit lazy v1 (`lazy<int/float/bool/text>`) | `Conformant` | covered in `lazy_primitive_text_basic`, `lazy_explicit_order_basic`, and `lazy_reuse_error`; generic lazy and lazy iterators remain future work |
| Typed concurrency facades | `Conformant` | `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>`, and `Atomic<int>` are executable; wider payloads remain gated by diagnostics |
| Resource cleanup (`using`) | `Conformant` | deterministic cleanup is covered across scope exit, return, `?`, panic, loop control, `Disposable.dispose()`, and FFI callback body execution |
| `std.mem` ownership intent facade | `Conformant` | `mem.own/view/edit` execute for the safe Appendix B subset; unsupported nested mutable managed shapes fail at check time |
| FFI callbacks with user data | `Conformant` | top-level function refs can cross as immediate C callbacks, including explicit `user_data` parameters; covered by `extern_c_callback_basic` and `extern_c_callback_user_data_basic` |
| FFI C-repr structs | `Conformant` | `attr repr("c")` structs with primitive/nested C-repr fields can cross `extern c` by value; covered by `extern_c_struct_arg_basic`, `extern_c_struct_return_basic`, and negative layout diagnostics |
| FFI extern const | `Conformant` | read-only C globals can be declared as `extern c const` for primitive and C-repr types; covered by `extern_c_const_basic`, `extern_c_const_struct_basic`, and `extern_c_const_managed_error` |
| FFI target attrs | `Conformant` | `attr target("windows"|"unix"|"linux"|"macos"|"any")` selects extern items before binding/check/lowering; covered by `extern_c_target_const_basic` and `extern_c_target_unsupported_error` |

## Tooling And Runtime

| Area | Status | Notes |
| --- | --- | --- |
| Single-file execution (`zt run file.zt`) | `Conformant` | `check`, `build`, `run`, `emit-c` work on standalone `.zt` files without a project; synthetic manifest, stdlib auto-load, namespace-path validation skipped |
| `zt fmt` / `zt fmt --check` | `Conformant` | gate project `tooling_gate_smoke` |
| `zt fmt` idempotence (`fmt(fmt(x)) == fmt(x)`) | `Conformant` | gate runner `tests/formatter/run_formatter_idempotence.py`, integrado ao `pr_gate` tooling; cobre os 9 casos em `tests/formatter/cases/` |
| `zt doc check` | `Conformant` | gate project `tooling_gate_smoke` |
| Runtime contracts (`where`) | `Conformant` | positive and negative behavior tests |
| Runtime diagnostic codes | `Conformant` | canonical public-RC contract is `docs/spec/language/diagnostic-code-catalog.md` plus `docs/spec/language/diagnostics-model.md` |
| Perf nightly gate | `Conformant` | `reports/perf/summary-nightly.json` status `pass` |
| `std.console` interactive helpers | `Conformant` | Phase 5D: `console.write_line`, `console.error_line`, `console.pause`, `console.prompt`, and `console.confirm(default_value: ...)`; `std.io` remains the stream layer |

## Open Items

| Item | Label | Notes |
| --- | --- | --- |
| Default runtime thread-safety boundary | `Risk` | default runtime path is single-isolate with non-atomic ARC; cross-thread work must stay behind isolate/message-passing boundaries; typed jobs/channels are executable for `int` and `text`, while shared/atomic remain restricted to `int` |
| Cycle ownership strategy | `Risk` | no new public cycle-forming API is introduced in 0.4.2-beta.rc1; `std.orc.collect_cycles()` stays a diagnostic/runtime hook until real cycle-forming APIs exist |
| Advanced memory resources (`mem.Temp`, `mem.Pool<T>`) | `Deferred` | reserved as explicit library values, not syntax; public API waits for real allocator pressure and deterministic `using` fixtures |
| Terminal controls in `std.console` | `Conformant` | Phase 5D includes detection, size, clear screen, foreground colors, basic styles, reset, and non-blocking key read; cursor movement remains future work |
