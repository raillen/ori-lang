# Zenith Next Behavior Matrix

This matrix is the M16 executable coverage map for the current C backend cut.

For the canonical M32 layered/risk matrix, see docs/spec/language/conformance-matrix.md.

Legend:

- `valid`: project must build and the executable exit code is checked.
- `invalid`: project must fail verification/build and diagnostics are checked by fragment.
- `deferred`: accepted surface area not executable in this cut.

## Valid Projects

| Project | Feature | Expected exit |
| --- | --- | --- |
| `simple_app` | Project smoke, `emit-c` golden and integer return | `42` |
| `control_flow_while` | `while` lowering and C emission | `6` |
| `control_flow_repeat` | `repeat N times` lowering and C emission | `9` |
| `control_flow_match` | `match case/default` lowering and C emission | `7` |
| `enum_match` | Enum construction and payload match (`case Enum.Variant(...)`) | `0` |
| `control_flow_break_continue` | `break` and `continue` in loops | `8` |
| `control_flow_for_list` | `for item, index in list<int>` lowering and C emission | `13` |
| `control_flow_for_map` | `for key, value in map<text,text>` lowering and C emission | `6` |
| `functions_calls` | Direct calls, recursion and mixed return types | `6` |
| `functions_named_args` | Named arguments in declaration order | `6` |
| `functions_defaults` | Trailing default parameters | `18` |
| `structs_constructor` | Struct constructor | `42` |
| `structs_field_defaults` | Struct field defaults | `117` |
| `structs_field_read` | Struct field read | `11` |
| `structs_field_update` | Struct field update on `var` | `12` |
| `structs_with_expression` | `with` expression: derive struct value with partial overrides (`source with field: value`), preserving non-listed fields without mutating source | `0` |
| `methods_inherent` | Inherent method via `apply Type` | `7` |
| `methods_inherent_apply` | Inherent mutating method through method call syntax | `7` |
| `self_field_shorthand` | `@field` shorthand inside `apply` methods as sugar for `self.field`, including read and mutating assignment | `10` |
| `methods_mutating` | Receiver mutation through `mut func` method | `6` |
| `methods_trait_apply` | Trait method through `apply Trait to Type` | `8` |
| `list_basic` | `list<int>` literal, index and update | `18` |
| `list_text_basic` | `list<text>` literal, index and update | `0` |
| `list_struct_generic` | `list<Struct>` generic runtime path for plain structs: literal, `len`, index and update | `0` |
| `generic_helper_name_collision_safe` | Generic list/set/map helpers stay distinct for same simple struct name in different namespaces | `0` |
| `tuple_generated_struct_callbacks` | `tuple<text, int>` lowered to generated C struct and reused as `list<tuple<...>>` element with generic callbacks | `0` |
| `list_dyn_trait_basic` | `list<any<TextRepresentable>>` heterogenea com `for` + `to_text()` | `16` |
| `dyn_trait_heterogeneous_collection` | `list<any<Drawable>>` literal heterogeneo com `for`, slice, index, `std.list.append`, set e dispatch por vtable | `0` |
| `list_slice_len` | `list<int>` slice and `len(list)` | `37` |
| `text_slice_len` | `text` slice and `len(text)` | `8` |
| `text_utf8_index_slice` | `text` index/slice por code point com UTF-8 multi-byte | `21` |
| `std_text_basic` | `std.text` alpha-safe (`trim`, busca, predicados, `limit`, `to_utf8`) | `0` |
| `std_concurrent_boundary_copy_basic` | `std.concurrent` transfer helpers (`copy_int`, `copy_text`, `copy_bytes`, `copy_list_int`, `copy_list_text`, `copy_map_text_text`) | `0` |
| `std_jobs_text_basic` | `Job<text>` spawn/join with copied text payload and text result | `0` |
| `std_channels_text_basic` | `Channel<text>` create/send/receive/close with `optional<text>` result | `0` |
| `extern_c_callback_user_data_basic` | C callback with explicit `user_data` parameter, top-level function ref and `using` cleanup inside the callback | `0` |
| `extern_c_struct_arg_basic` | `attr repr("c")` struct passed by value into a linked C shim | `0` |
| `extern_c_struct_return_basic` | `attr repr("c")` struct returned by value from a linked C shim | `0` |
| `extern_c_const_basic` | `extern c const` scalar read from a linked C shim | `0` |
| `extern_c_const_struct_basic` | `extern c const` C-repr struct read from a linked C shim | `0` |
| `extern_c_target_const_basic` | `attr target(...)` selects the active extern const for the current platform | `0` |
| `map_basic` | `map<text,text>` literal, index and update | `0` |
| `map_empty_expected_type` | Empty map literal `{}` with expected `map<int,bool>` in init and function call | `0` |
| `map_int_text_basic` | `map<int,text>` literal, index, update and `len(map)` | `7` |
| `map_struct_expected_type` | Empty map literal `{}` with expected `map<int,Flag>` and generated `optional<Flag>` helper | `0` |
| `map_safe_get` | Safe lookup `map.get(key) -> optional<text>` sem panic em chave ausente | `15` |
| `list_safe_get` | Safe lookup `list.get(index) -> optional<int>` sem panic em indice ausente | `27` |
| `map_len_basic` | `len(map<text,text>)` | `2` |
| `value_semantics_collections` | Copy/mutate isolation for `list` and `map` via COW in runtime/backend | `131` |
| `value_semantics_struct_managed` | Copy/mutate isolation para struct com campos `list/map` via rebind COW | `131` |
| `value_semantics_arc_isolation` | Chain-copy isolation (`a -> b -> c`) for `list` and `map` under COW/RC | `158` |
| `orc_last_use_move_basic` | ORC last-use move for managed local assignment avoids retain and nulls the moved source in generated C | `7` |
| `orc_last_use_no_move_after_alias` | ORC keeps retain/copy when the managed source local is used after alias assignment | `7` |
| `orc_last_use_loop_backedge_no_move` | ORC blocks move when a managed source local is used again through a reachable loop backedge | `7` |
| `orc_last_use_branch_sibling_move` | ORC allows move in one branch when later source uses exist only on sibling paths | `7` |
| `orc_field_last_use_move` | ORC moves a managed struct field when the owning local has no later relevant use and nulls the source field | `7` |
| `orc_field_no_move_after_object_use` | ORC keeps retain/copy when the same managed field is used again through the source object | `7` |
| `orc_field_move_other_field_later_use` | ORC field liveness allows moving one managed field while a different field is read later | `7` |
| `orc_sink_param_owned_transfer_basic` | ORC sink parameter transfer passes a retained owner when the caller still uses the argument | `7` |
| `orc_sink_param_last_use_arg_move` | ORC sink parameter transfer moves a last-use managed argument without retain and nulls the caller source | `7` |
| `orc_sink_param_return_move` | ORC sink parameter transfer moves a final-use managed argument in a return call before cleanup | `7` |
| `orc_sink_param_effect_move` | ORC sink parameter transfer moves a final-use managed argument in a standalone effect call | `7` |
| `orc_sink_param_duplicate_source` | ORC sink parameter transfer moves only one occurrence when the same source feeds two sink parameters, retaining the second owner | `7` |
| `value_semantics_optional_result_managed` | `optional<list<int>>` creation/copy and `result<list<int>, text>` `?` with COW-safe list mutation | `0` |
| `optional_struct_qualified_managed` | `optional<mod.Struct>` com `struct` qualificada, retorno direto, atribuicao/call-site implicitos, campo opcional em outra `struct` e isolamento de `list<text>` | `0` |
| `optional_primitive_specialized` | `optional<float/bool/int8/int16/int32/int64/u8/u16/u32/u64>` present, none, `is_some`, `is_none` and `or` | `0` |
| `std_collections_managed_arc` | Copy/mutate isolation para `grid2d<text>`, `pqueue<text>`, `circbuf<text>`, `btreemap<text,text>`, `btreeset<text>` e `grid3d<text>` | `0` |
| `std_collections_queue_stack_cow` | `queue/stack` com retorno estruturado (`colecao + item`) e isolamento por copia em `dequeue/pop` | `0` |
| `std_collections_values_iteration` | `std.collections` snapshots iteraveis: `queue_values<T>`/`stack_values<T>`, grids em ordem dimensional, pqueue em ordem de pop, circbuf em FIFO, btree em ordem ordenada | `0` |
| `std_console_basic` | `std.console`: helpers de linha, deteccao, tamanho e leitura de tecla nao bloqueante sobre `std.io` | `0` |
| `std_debug_basic` | `std.debug.size_of` e `std.debug.type_name` para tipos basicos, tupla, struct, enum, list, map e `any<Trait>` | `0` |
| `optional_result_basic` | 
one`, `success(...)` and `error(...)` | `0` |
| `result_question_basic` | `result<T,E>` `?` propagation in const/var initialization | `0` |
| `optional_question_basic` | `optional<T>` `?` propagation with `none` short-circuit in const initialization | `0` |
| `optional_or_return_basic` | `optional<T>.or_return(value)` unwraps present values or returns from the enclosing function | `0` |
| `result_or_wrap_basic` | `result<T, core.Error>.or_wrap(context)` preserves success and adds `core.Error.context` on failure | `0` |
| `edge_boundaries_empty` | Edge values: `u8/u16/u32/u64` bounds, near `int` limits, and empty `text/list/map/bytes` invariants | `0` |
| `bytes_hex_literal` | `hex bytes "..."`, `len(bytes)`, byte indexing and byte slicing | `9` |
| `std_bytes_utf8` | `std.bytes.empty`, `std.text.to_utf8`, `std.text.from_utf8` and UTF-8 failure path | `14` |
| `std_bytes_ops` | `std.bytes.from_list`, `std.bytes.to_list`, `std.bytes.join`, `std.bytes.starts_with`, `std.bytes.ends_with` and `std.bytes.contains` | `7` |
| `std_validate_basic` | `std.validate` baseline predicates (`between`, `one_of`, `one_of_text`, text length checks) | `42` |
| `std_validate_broader` | `std.validate` broader executable predicates for float, bool, primitive optional/result state, list length, and supported map-size helper families | `0` |
| `std_small_helpers` | R6 small helper set for `std.validate`, `std.text`, `std.list` and `std.map` | `0` |
| `list_first_basic` | `std.list.first` returns `optional<int>` / `optional<text>` and `none` for empty lists | `0` |
| `list_first_type_error` | `std.list.first` rejects non-list arguments | `1` |
| `list_float_primitive_storage` | `list<float>` literal, index, update, slice and `len(list)` through primitive contiguous storage | `0` |
| `list_last_basic` | `std.list.last` returns `optional<int>` / `optional<text>` and `none` for empty lists | `0` |
| `list_last_type_error` | `std.list.last` rejects non-list arguments | `1` |
| `list_primitive_numeric_matrix` | `list<bool/int8/int16/int32/int64/u8/u16/u32/u64>` literal, index, update, slice and `len(list)` | `0` |
| `list_hof_int_basic` | `std.list` HOFs for `list<int>`: map, filter, reduce, find, any, all, count | `0` |
| `list_hof_text_basic` | `std.list` HOFs for `list<text>`: map, filter, find, any, all, count, sort_by | `0` |
| `list_hof_bool_basic` | `std.list` HOFs for `list<bool>`: map, filter, find, any, all, count, sort_by | `0` |
| `list_map_cross_type_deferred_error` | `std.list.map<T,U>` is rejected with a post-RC diagnostic in the current backend subset | `1` |
| `list_reduce_value_hof_basic` | `std.list.reduce<T,T>` for text, bool and float lists | `0` |
| `list_reduce_cross_type_deferred_error` | `std.list.reduce<T,U>` is rejected with a post-RC diagnostic in the current backend subset | `1` |
| `list_rest_basic` | `std.list.rest` returns a list without the first item and handles empty/single-item lists | `0` |
| `list_rest_type_error` | `std.list.rest` rejects non-list arguments | `1` |
| `list_skip_basic` | `std.list.skip` skips zero, negative, partial and over-length counts for int/text lists | `0` |
| `list_skip_type_error` | `std.list.skip` rejects non-integral counts | `1` |
| `list_value_api_basic` | `std.list` value-style API for `list<int>` and `list<text>`: append, prepend, contains, reverse, set, remove_*, slice, concat, index_of | `0` |
| `list_value_api_primitives` | `std.list` value-style API for specialized primitive lists: float, bool, int8 and u8 helpers including get, first, last, rest, skip, set, remove_*, slice, contains and index_of | `0` |
| `std_mem_generic_facade_basic` | `std.mem.own/view/edit` generic facade over `text`, `list<int>`, `list<float>`, `list<bool>`, `list<int8>`, `list<u8>`, and `list<text>` | `0` |
| `std_mem_generic_facade_unsupported_type_error` | `std.mem.own/view/edit` rejects types not yet in the stabilized memory-intent set | `1` |
| `std_mem_appendix_b_values` | Appendix B `std.mem.own/view/edit` support for primitive scalars, safe tuples/structs, list<tuple>, list<struct>, set<int>, set<text>, and primitive/text maps | `0` |
| `std_mem_appendix_b_deferred_type_error` | Appendix B rejects tuple/list shapes that need nested managed edit isolation before they can be stable | `1` |
| `std_mem_appendix_b_nested_list_deferred_error` | Appendix B rejects nested list values until recursive list edit isolation is stable | `1` |
| `std_mem_appendix_b_set_list_key_deferred_error` | Appendix B rejects set<list<int>> keys until collection keys have stable hash/equality | `1` |
| `std_mem_appendix_b_set_tuple_key_deferred_error` | Appendix B rejects set<tuple<...>> keys until tuple structural hash/equality is available | `1` |
| `std_mem_appendix_b_map_key_deferred_error` | Appendix B rejects tuple map keys until tuple structural hash/equality is available | `1` |
| `std_mem_appendix_b_map_nested_value_deferred_error` | Appendix B rejects recursively nested map values until deep edit isolation is proven | `1` |
| `std_mem_appendix_b_managed_struct_deferred_error` | Appendix B rejects managed structs until generated deep edit isolation is stable | `1` |
| `std_mem_appendix_b_enum_payload_deferred_error` | Appendix B rejects enum payload values until generated enum clone/edit operations are stable | `1` |
| `std_mem_appendix_b_optional_payload_deferred_error` | Appendix B rejects optional payload values until optional/result payload clone/edit operations are stable | `1` |
| `map_value_api_basic` | `std.map` value-style API for `map<text,text>`: get, contains, set, remove, keys, values, merge | `0` |
| `map_value_api_generic` | `std.map` value-style API for generated `map<int,text>` and `map<text,int>`: set, remove, keys, values, merge, get, has_key | `0` |
| `map_value_api_unsupported_key_error` | `std.map` value-style helpers reject keys outside the C backend `int/text` key subset | `1` |
| `map_struct_key_basic` | `std.map` supports generated `map<Struct,int>` set/has/remove for safe bool/int structural keys | `0` |
| `map_struct_unsupported_key_error` | `std.map` rejects `map<Struct,V>` when the key fields cannot receive generated stable hash/equality | `1` |
| `set_core_api_basic` | `std.set.empty`, `std.set.of`, `std.set.add`, `std.set.remove`, `std.set.is_empty`, `std.set.has`, `std.set.len` | `0` |
| `set_struct_key_basic` | `std.set` supports generated `set<Struct>` add/has/remove for safe bool/int structural keys | `0` |
| `set_struct_unsupported_key_error` | `std.set` rejects `set<Struct>` when a field cannot receive generated stable hash/equality | `1` |
| `set_operations_basic` | `set<int>` and `set<text>` literals plus `std.set.union`, `std.set.intersect`, `std.set.difference`, `std.set.has`, `std.set.len` | `0` |
| `set_iteration_basic` | `for item in set<T>` and `for item, index in set<T>` for `set<int>` and `set<text>` | `0` |
| `set_empty_inference_error` | `std.set.empty()` requires an expected `set<T>` type | `1` |
| `set_mutation_const_error` | `std.set.add` rejects mutation through a `const set<T>` binding | `1` |
| `set_operation_type_error` | `std.set.union` rejects sets with different element types | `1` |
| `std_math_basic` | `std.math` baseline (`abs`, `pow`, `sqrt`, `min`, `max`, `clamp`, rounding, trig, logs, constants, special float checks) | `42` |
| `std_math_nonfinite_policy` | `std.math.nan()` and `std.math.infinity()` as functions: construction, IEEE equality, infinity ordering, formatting | `0` |
| `std_regex_basic` | `std.regex` baseline (`compile`, `is_match`, `find_all`) for simple patterns and invalid pattern handling | `0` |
| `std_random_basic` | `std.random` baseline (`seed`, `next`, `between`) plus `public var` state tracking | `0` |
| `std_random_state_observability` | `std.random` public state observability (`seeded`, `last_seed`, `draw_count`, `stats`) | `0` |
| `std_random_between_branches` | `std.random.between` branch behavior (`min == max`, `max < min`) with draw count invariants | `0` |
| `std_random_format_tier5` | Tier 5 stdlib slice: `random.float_between`, `random.choice/shuffle` for `list<int>` and `list<text>`, `TextRepresentable`, and `std.format` expansion | `0` |
| `std_format_basic` | `std.format` com `BytesStyle` tipado (`hex`, `bin`, `bytes(style: ...)`, `bytes_binary`, `bytes_decimal`) | `0` |
| `fmt_interpolation_basic` | `fmt "..."` end-to-end com expressao, chamada, bool e escape de chaves | `0` |
| `float_arithmetic_nested` | Emissao C de aritmetica `float` aninhada, incluindo acumulacao decimal | `0` |
| `to_text_builtin_basic` | Builtin `to_text(value)` via `TextRepresentable` para `int` e `bool` | `0` |
| `type_conversions_basic` | `std.int`, `std.float` e `std.bool` com conversoes explicitas, texto e parse opcional | `0` |
| `todo_builtin_fail` | Builtin `todo(message)` fatal path | `runtime.todo` |
| `unreachable_builtin_fail` | Builtin `unreachable(message)` fatal path | `runtime.unreachable` |
| `check_intrinsic_message_fail` | Builtin `check(condition, message)` fatal path with custom message | `runtime.check` |
| `size_of_builtin_removed_error` | `size_of(...)` removed from global builtins; use `std.debug.size_of(...)` | `check-fail` |
| `std_fs_basic` | `std.fs` baseline (`write_text`, `exists`, `read_text`) via host runtime wrappers | `check-pass` |
| `std_fs_aliases_basic` | `std.fs` checklist aliases (`copy`, `rename`, `file_size`) plus `exists`, `is_file`, `is_dir` | `0` |
| `std_fs_ops_basic` | `std.fs` create/list/metadata/copy/move/remove com caminhos reais | `0` |
| `std_fs_path_basic` | `std.fs.path` baseline (`join`, `base`, `dir`, `ext`, 
ame_without_extension`, `has_ext`, `change_ext`, 
ormalize`, `absolute`, `relative`, `is_absolute`, `is_relative`) via compile-probe | `0` |
| `std_json_basic` | `std.json` baseline (`parse`, `stringify`, `pretty`) para objeto plano `map<text,text>` | `0` |
| `std_test_basic` | `std.test` helper direto em `main` (`skip` => skipped outcome, `fail` => failed outcome) | `test-skip` |
| `std_test_attr_pass_skip` | `zt test` com `attr test` exercitando 1 pass e 1 skip | `test ok (pass=1 skip=1)` |
| `std_test_attr_fail` | `zt test` com `attr test` exercitando 1 pass, 1 skip e 1 fail | `test failed (pass=1 skip=1 fail=1)` |
| `std_test_helpers_pass` | `std.test` helper assertions no caminho feliz (`is_true`, `is_false`, `equal_*`, `not_equal_*`) | `0` |
| `std_test_throws_pass` | `std.test.throws` aceita funcoes que chamam `panic(...)` | `0` |
| `std_time_basic` | `std.time` tipado (`Instant`, `Duration`, `now`, `now_ms`, `sleep`, `sleep_ms`, `elapsed`, `since`, `until`, conversoes unix) | `0` |
| `syntax_coherence_core` | Sintaxe 1G: `case some(name):`, `case else:`, `type` alias, `any<Trait>`, `func main()` sem retorno e closure de expressao unica | `0` |
| `syntax_coherence_inline_constraints` | Sintaxe 1G: constraints inline `<T: Trait>` e trailing `given T is Trait` | `0` |
| `std_os_basic` | `std.os` tipado (`Platform`, `Arch`, `pid`, `platform`, `arch`, `env`, `current_dir`, `change_dir`) | `0` |
| `std_os_args_basic` | `std.os.args()` inclui `argv[0]` e preserva argumentos encaminhados por `zt run ... -- <args>` | `0` |
| `std_os_process_basic` | `std.os.process` com `ExitStatus` tipado (`run`, `exit_code`) e comando explicito (`program` + `args`) | `0` |
| `std_net_basic` | `std.net` TCP client baseline (`connect`, `read_some`, `write_all`, `close`, `is_closed`) em loopback local via `run-loopback.ps1` | `0` |
| `std_http_basic` | `std.http` blocking HTTP client baseline (`get`, `post`, `Response.status`, `Response.body`) em loopback local via `run-loopback.ps1` | `0` |
| `std_shared_text_type_error` | `std.shared.create(...)` rejeita `Shared<text>` enquanto o runtime publico executavel suporta somente `Shared<int>` | `check-fail` |
| `std_atomic_bool_type_error` | `std.atomic.create(...)` rejeita `Atomic<bool>` enquanto o runtime publico executavel suporta somente `Atomic<int>` | `check-fail` |
| `tooling_gate_smoke` | projeto canario para gate de `zt fmt --check` e `zt doc check` no runner oficial | `0` |
| `multifile_import_alias` | Multi-file source root and import alias | `42` |
| `public_const_module` | Top-level `public const` imported via alias (`module.CONST`) | `42` |
| `public_var_module` | Top-level `public var` imported via alias (`module.VAR`) | `42` |
| `public_var_module_state` | `public var` shares state across functions in the owning module | `4` |
| `readability_block_depth_pass` | Readability warning (`style.block_too_deep`) is reported but does not block normal run | `0` |
| `readability_enum_default_pass` | Readability warning (`control_flow.enum_default_case`) is reported but does not block normal run | `0` |
| `readability_function_length_pass` | Readability warning (`style.function_too_long`) is reported but does not block normal run | `0` |
| `readability_warnings_pass` | Readability warnings (`name.similar`, `name.confusing`) are reported but do not block normal run | `0` |
| `attributes_v1` | Attribute warnings (`declaration.deprecated`, `declaration.todo`) and `attr skip` test-runner behavior | `0` |
| `closure_capture_basic` | Anonymous closure with immutable by-value capture | `0` |
| `lambda_hof_basic` | Lambda sugar `func(...) => expr` with `map_int`, `filter_int`, `reduce_int` and immutable capture | `0` |
| `nested_function_basic` | Local `func` declaration inside a function, called as an immutable closure with parent-scope capture | `4` |
| `lazy_explicit_order_basic` | Explicit `lazy<int>` runs the thunk only on `force_int` | `0` |
| `borealis_backend_fallback_stub` | Borealis desktop-profile request (`backend_id=1`) with safe fallback to stub when adapter is unavailable; covers window + draw + input queries | `0` |
| `borealis_raylib_binding_stub` | `borealis.raylib` binding smoke in stub-safe mode: shapes, text, input, `measure_text`, `raymath` helpers, easing functions, `require_available()`, clear empty-path errors for texture/sound and stub-safe texture draw fallback | `0` |
| `borealis_raylib_assets_real` | `borealis.raylib` real-assets probe: conditional native `.png`/`.wav` loading, texture metadata, texture draw, audio init and `load/play/stop/unload` when Raylib is available | `0` |
| `borealis_foundations_stub` | Borealis foundations smoke: typed asset loaders, asset metadata/stable ids, typed events, save, storage, services, database, UI/HUD widgets, editor metadata and persistent settings coverage, including empty-string persistence semantics, settings profiles and clear key-kind conflicts | `0` |
| `borealis_ecs_hybrid_stub` | Borealis ECS hybrid (stub run-pass): component store API em `borealis.engine.ecs` | `0` |
| `borealis_runtime_gameplay_stub` | Borealis runtime/gameplay smoke: contracts, entities, movement, controllers, vehicles, animation, audio, ai, camera, input, world and procedural working together | `0` |
| `where_contracts_ok` | Runtime `where` contracts on parameter, struct construction and field assignment | `40` |

## Invalid Projects

| Project | Expected diagnostic |
| --- | --- |
| `error_syntax` | Parser span and expectation text |
| `error_type_mismatch` | Semantic type mismatch span |
| `check_intrinsic_message_fail` | Runtime check failure preserves caller message |
| `functions_main_signature_error` | C entrypoint signature restriction |
| `functions_invalid_call_error` | Missing argument diagnostic with source span |
| `multifile_missing_import` | Missing import rejection |
| `multifile_namespace_mismatch` | Namespace/path mismatch rejection |
| `multifile_duplicate_symbol` | Duplicate effective symbol rejection |
| `multifile_import_cycle` | Import cycle rejection |
| `multifile_private_access` | Access to non-public symbol via import alias is rejected |
| `public_var_cross_namespace_write_error` | Cross-namespace mutation of `public var` via import alias is rejected |
| `readability_block_depth_strict_error` | Strict diagnostics profile promotes block-depth warning to error |
| `readability_enum_default_strict_error` | Strict diagnostics profile promotes enum-default warning to error |
| `readability_function_length_strict_error` | Strict diagnostics profile promotes function-length warning to error |
| `readability_warnings_strict_error` | Strict diagnostics profile promotes name readability warnings to error |
| `closure_mut_capture_error` | Mutation of a captured closure variable is rejected |
| `dyn_generic_trait_error` | Generic trait rejected as `any<Trait>` with generic/where guidance |
| `lambda_return_mismatch_error` | Lambda return must match the expected `func(...) -> ...` type |
| `nested_function_mut_capture_error` | Local nested function rejects mutation of captured parent variable |
| `self_field_shorthand_outside_apply_error` | `@field` outside `apply` is rejected through unresolved implicit `self` |
| `lazy_reuse_error` | Runtime contract rejects forcing the same one-shot `lazy<int>` twice |
| `lazy_generic_deferred_error` | `lazy<T>` outside the executable `int`/`float`/`bool`/`text` subset is rejected during check with post-RC guidance |
| `std_math_nan_order_error` | Runtime contract rejects ordered comparison when either float operand is `NaN` |
| `std_concurrent_boundary_copy_unsupported_error` | `std.concurrent.copy_text(...)` rejects values outside the accepted payload type |
| `extern_c_struct_unannotated_error` | `extern c` rejects a user struct by value unless it has `attr repr("c")` |
| `extern_c_struct_managed_field_error` | C-repr struct fields reject managed values such as `text` |
| `extern_c_const_managed_error` | `extern const` rejects managed values such as `text` |
| `extern_c_target_unsupported_error` | unsupported `attr target(...)` selectors are rejected |
| `std_collections_unsupported_generic_shape_error` | Advanced `std.collections` shapes outside the v1 runtime subset fail during check with post-RC guidance |
| `std_collections_nested_managed_payload_error` | Nested managed payloads such as `grid2d<list<text>>` and `circbuf<map<text,text>>` fail during check with post-RC guidance |
| `std_random_cross_namespace_write_error` | Cross-namespace mutation of `std.random` `public var` via import alias is rejected |
| `project_unknown_key_manifest` | Manifest unknown key diagnostic (`project.*`) |
| `fmt_interpolation_type_error` | `fmt` rejeita tipo sem `TextRepresentable<T>` |
| `monomorphization_limit_error` | Monomorphization gate diagnostic when generic instantiations exceed `build.monomorphization_limit` |
| `monomorphization_function_limit_error` | Monomorphization gate also counts concrete generic function specializations |
| `generic_arg_inference_basic` | Generic function call infers `T` from argument position during semantic check |
| `generic_arg_inference_missing_error` | Generic inference rejects calls where `T` appears only in return position |
| `generic_arg_inference_conflict_error` | Generic inference rejects conflicting argument evidence for the same `T` |
| `generic_monomorphization_nested_call` | Nested generic call preserves inferred concrete type across `T -> U` parameter-name boundary |
| `generic_monomorphization_text_basic` | Generic `T` specialized as `text` retains returned managed values before cleanup |
| `monomorphization_many_instances_basic` | Several concrete generic function/list specializations stay below an explicit project limit |
| `match_guard_basic` | Match `case ... given` guard filters a matching pattern before the next case runs |
| `match_guard_non_bool_error` | Match guard must type-check as `bool` |
| `const_destructuring_basic` | `const (a, b) = tuple_expr` binds immutable tuple elements |
| `const_destructuring_non_tuple_error` | Const destructuring rejects non-tuple initializers |
| `const_destructuring_arity_error` | Const destructuring rejects tuple arity mismatches |
| `multivalue_match_basic` | `match (a, b)` supports tuple case patterns through per-element comparison |
| `multivalue_match_type_error` | Multi-value match rejects tuple case patterns with incompatible element types |
| `operator_overloading_level2_basic` | Level 2 operator overloading maps `+`, `-`, `<`, `<=`, `>`, `>=` through core traits |
| `operator_overloading_missing_trait_error` | User type operators require the matching `Addable`/`Subtractable`/`Comparable` trait |
| `pipe_operator_basic` | `value |> f |> g(extra)` lowers as left-to-right function calls |
| `pipe_operator_non_callable_error` | Pipe operator rejects a non-callable right side |
| `mutability_const_reassign_error` | Const reassignment mutability diagnostic |
| `noncanonical_string_error` | Suggests `text` instead of `string` |
| `noncanonical_let_error` | Suggests `const` or `var` instead of `let` |
| `noncanonical_and_error` | Suggests `and` instead of `&&` |
| `noncanonical_or_error` | Suggests `or` instead of `||` |
| `noncanonical_not_error` | Suggests `not value` instead of `!value` |
| `noncanonical_null_error` | Suggests `optional<T>` and `none` instead of `null` |
| `noncanonical_throw_error` | Suggests `result<T,E>`, `error(...)`, or `panic(...)` instead of `throw` |
| `noncanonical_abstract_error` | Suggests `trait` instead of `abstract` |
| `noncanonical_virtual_error` | Suggests `any<Trait>` instead of `virtual` |
| `noncanonical_union_error` | Suggests `enum` with payload instead of `union` |
| `noncanonical_partial_error` | Suggests `apply` and namespace/file organization instead of `partial` |
| `optional_question_outside_optional_error` | `optional<T>?` rejected outside `optional<U>` return context |
| `result_optional_propagation_error` | `?` propagation rejected outside `result<T,E>` return context |
| `runtime_index_error` | Runtime index diagnostic from C runtime guard |
| `panic_stack_overflow` | Runtime stack guard turns runaway recursion into `runtime.panic` before native stack overflow |
| `std_test_helpers_bool_fail` | Runtime test diagnostic from `test.is_true(false)` |
| `std_test_helpers_equal_fail` | Runtime test diagnostic with expected/received values from `test.equal_int(...)` |
| `std_test_helpers_not_equal_fail` | Runtime test diagnostic from `test.not_equal_text(...)` |
| `std_test_throws_fail` | Runtime test diagnostic from `test.throws(...)` when the body does not fail |
| `todo_builtin_fail` | Runtime todo diagnostic from `todo(message)` |
| `unreachable_builtin_fail` | Runtime unreachable diagnostic from `unreachable(message)` |
| `where_contract_param_error` | Runtime contract violation on parameter `where` |
| `where_contract_construct_error` | Runtime contract violation on struct construction `where` |
| `where_contract_field_assign_error` | Runtime contract violation on field assignment `where` |

## Deferred Surface Forms

These forms remain accepted language direction but are not in the M16 executable behavior matrix:

- Generic collection iteration beyond the C backend combinations already covered by behavior tests.
- Full generic monomorphization beyond the current checked semantic model.
- Enum value construction and exhaustive enum matching in generated C (semantic coverage exists in `tests/semantic`; check path is validated with fixtures `tests/behavior/enum_match` / `tests/behavior/enum_match_non_exhaustive_error`; full build E2E remains blocked while `compiler/zir/lowering/from_hir.c` is a stub in source).
- Broader stdlib-facing collection APIs beyond the current compiler intrinsic `len(...)`.







