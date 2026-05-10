# Runtime C - Code Map

## 📋 Descrição

Runtime do Zenith em C. Responsável por:
- Funções de runtime para código compilado
- Memory management (GC se aplicável)
- Built-in functions (strings, lists, I/O)
- Error handling e exceptions
- Platform abstraction

## 📁 Arquivos Principais

| Arquivo | Tamanho | Responsabilidade |
|---------|---------|------------------|
| `zenith_rt.c` | 231.5 KB | Implementação completa do runtime |
| `zenith_rt.h` | 39.4 KB | Runtime interfaces, types |
| `RUNTIME_DIAGNOSTICO_COMPLETO.md` | 4.9 KB | Documentação de diagnóstico |

## 🔍 Funções Críticas

| Linha | Função | Responsabilidade | Dependencies | Pode Quebrar Se | Prioridade |
|-------|--------|------------------|--------------|-----------------|------------|
| - | - | - | - | - | 🔴 CRÍTICA |

## ⚠️ Estado Crítico

- **Runtime state**: estado global do runtime
- **Memory pools**: gerenciamento de memória
- **Built-in cache**: functions pré-alocadas

## 🔗 Dependencies Externas

- `compiler/targets/c/` → Emitter chama runtime
- Platform libs (POSIX, Windows API)

## 🐛 Erros Comuns

1. [A preencher]
2. [A preencher]
3. [A preencher]

## 📝 Notas de Manutenção

- ARQUIVO ENORME (231KB) → dividir urgentemente
- Runtime é linkado com TODO código compilado
- Bugs aqui afetam todos os programas Zenith

<!-- CODEMAP:GENERATED:BEGIN -->
## Generated Index

- Priority: Critical
- Source files: 11
- Extracted symbols: 623

Do not edit this block by hand. Re-run `python tools/generate_code_maps.py`.

### File Summary

| File | Lines | Symbols | Local deps |
| --- | ---: | ---: | ---: |
| `runtime/c/zenith_collections_generic.c` | 743 | 56 | 1 |
| `runtime/c/zenith_collections_generic.h` | 158 | 1 | 1 |
| `runtime/c/zenith_collections_rt.c` | 36 | 0 | 0 |
| `runtime/c/zenith_rt.c` | 3670 | 48 | 10 |
| `runtime/c/zenith_rt.h` | 1533 | 1 | 1 |
| `runtime/c/zenith_rt_borealis.c` | 2366 | 156 | 0 |
| `runtime/c/zenith_rt_http.c` | 231 | 9 | 0 |
| `runtime/c/zenith_rt_json.c` | 436 | 11 | 0 |
| `runtime/c/zenith_rt_net.c` | 454 | 19 | 0 |
| `runtime/c/zenith_rt_outcome.c` | 4192 | 291 | 0 |
| `runtime/c/zenith_rt_templates.h` | 2638 | 31 | 0 |

### Local Dependencies

- `runtime/c/zenith_collections_generic.h`
- `runtime/c/zenith_rt.h`
- `runtime/c/zenith_rt_templates.h`
- `zenith_collections_generic.c`
- `zenith_collections_generic.h`
- `zenith_collections_rt.c`
- `zenith_rt.h`
- `zenith_rt_borealis.c`
- `zenith_rt_http.c`
- `zenith_rt_json.c`
- `zenith_rt_net.c`
- `zenith_rt_outcome.c`

### Related Tests

- `tests/behavior/where_contract_construct_error/src/app/main.zt`
- `tests/behavior/where_contract_field_assign_error/src/app/main.zt`
- `tests/behavior/where_contract_param_error/src/app/main.zt`
- `tests/behavior/where_contract_param_where_invalid_error/src/app/main.zt`
- `tests/behavior/where_contract_param_where_non_bool_error/src/app/main.zt`
- `tests/behavior/where_contracts_ok/src/app/main.zt`
- `tests/runtime/c/README.md`
- `tests/runtime/c/test_arithmetic_overflow.c`
- `tests/runtime/c/test_collections_generic.c`
- `tests/runtime/c/test_host_fs_guardrails.c`
- `tests/runtime/c/test_map_hash_table.c`
- `tests/runtime/c/test_net_error_kind.c`
- `tests/runtime/c/test_outcome_propagate.c`
- `tests/runtime/c/test_process_run.c`
- `tests/runtime/c/test_runtime.c`
- `tests/runtime/c/test_runtime_error_tls.c`
- `tests/runtime/c/test_shared_text.c`
- `tests/runtime/c/test_text_utf8_guardrails.c`
- `tests/runtime/c/test_text_utf8_slice.c`
- `tests/runtime/c/test_thread_boundary_copy.c`
- `tests/runtime/stress_tests.c`
- `tests/runtime/test_fase11_safety.zt`

### Symbol Index

#### `runtime/c/zenith_collections_generic.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 17 | `macro` | `ZT_GENERIC_INITIAL_CAPACITY` |
| 18 | `macro` | `ZT_MAP_LOAD_FACTOR_NUM` |
| 19 | `macro` | `ZT_MAP_LOAD_FACTOR_DEN` |
| 21 | `func_def` | `zt_generic_elem_at` |
| 25 | `func_def` | `zt_generic_elem_at_const` |
| 29 | `func_def` | `zt_generic_elem_copy` |
| 37 | `func_def` | `zt_generic_elem_destroy` |
| 47 | `func_def` | `zt_list_generic_create` |
| 62 | `func_def` | `zt_list_generic_clone` |
| 89 | `func_def` | `zt_list_generic_free` |
| 99 | `func_def` | `zt_list_generic_grow` |
| 109 | `func_def` | `zt_list_generic_from_array` |
| 135 | `func_def` | `zt_list_generic_append` |
| 146 | `func_def` | `zt_list_generic_get` |
| 153 | `func_def` | `zt_list_generic_set` |
| 163 | `func_def` | `zt_list_generic_set_owned` |
| 178 | `func_def` | `zt_list_generic_remove` |
| 197 | `func_def` | `zt_list_generic_insert` |
| 219 | `func_def` | `zt_list_generic_slice` |
| 256 | `func_def` | `zt_list_generic_len` |
| 260 | `func_def` | `zt_list_generic_clear` |
| 269 | `func_def` | `zt_list_generic_raw_get` |
| 277 | `func_def` | `zt_map_generic_probe` |
| 301 | `func_def` | `zt_map_generic_rehash` |
| 334 | `func_def` | `zt_map_generic_create` |
| 357 | `func_def` | `zt_map_generic_clone` |
| 376 | `func_def` | `zt_map_generic_free` |
| 391 | `func_def` | `zt_map_generic_put` |
| 427 | `func_def` | `zt_map_generic_get` |
| 441 | `func_def` | `zt_map_generic_has` |
| 452 | `func_def` | `zt_map_generic_remove` |
| 470 | `func_def` | `zt_map_generic_len` |
| 474 | `func_def` | `zt_map_generic_clear` |
| 493 | `func_def` | `zt_set_generic_probe` |
| 517 | `func_def` | `zt_set_generic_rehash` |
| 544 | `func_def` | `zt_set_generic_create` |
| 563 | `func_def` | `zt_set_generic_clone` |
| 580 | `func_def` | `zt_set_generic_free` |
| 593 | `func_def` | `zt_set_generic_add` |
| 621 | `func_def` | `zt_set_generic_has` |
| 632 | `func_def` | `zt_set_generic_remove` |
| 649 | `func_def` | `zt_set_generic_len` |
| 653 | `func_def` | `zt_set_generic_clear` |
| 671 | `func_def` | `zt_ops_i64_copy` |
| 672 | `func_def` | `zt_ops_i64_hash` |
| 683 | `func_def` | `zt_ops_i64_equals` |
| 689 | `func_def` | `zt_ops_f64_copy` |
| 690 | `func_def` | `zt_ops_f64_hash` |
| 705 | `func_def` | `zt_ops_f64_equals` |
| 713 | `func_def` | `zt_ops_bool_copy` |
| 714 | `func_def` | `zt_ops_bool_hash` |
| 715 | `func_def` | `zt_ops_bool_equals` |
| 721 | `func_def` | `zt_ops_text_copy` |
| 726 | `func_def` | `zt_ops_text_destroy` |
| 730 | `func_def` | `zt_ops_text_hash` |
| 734 | `func_def` | `zt_ops_text_equals` |

#### `runtime/c/zenith_collections_generic.h`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 15 | `macro` | `ZENITH_COLLECTIONS_GENERIC_H` |

#### `runtime/c/zenith_collections_rt.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| - | - | No symbols extracted |

#### `runtime/c/zenith_rt.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 18 | `macro` | `WIN32_LEAN_AND_MEAN` |
| 46 | `macro` | `ZT_NET_INVALID_SOCKET` |
| 49 | `macro` | `ZT_NET_INVALID_SOCKET` |
| 53 | `macro` | `ZT_THREAD_LOCAL` |
| 55 | `macro` | `ZT_THREAD_LOCAL` |
| 57 | `macro` | `ZT_THREAD_LOCAL` |
| 62 | `func_decl` | `zt_pqueue_i64_ensure_capacity` |
| 63 | `func_decl` | `zt_pqueue_text_ensure_capacity` |
| 64 | `func_decl` | `zt_btreemap_text_text_ensure_capacity` |
| 65 | `func_decl` | `zt_btreeset_text_ensure_capacity` |
| 67 | `macro` | `ZT_DYNAMIC_HEAP_BASE` |
| 68 | `macro` | `ZT_DYNAMIC_HEAP_CAPACITY` |
| 70 | `struct` | `zt_dynamic_heap_entry` |
| 79 | `func_def` | `zt_find_dynamic_heap_entry` |
| 98 | `func_def` | `zt_safe_message` |
| 102 | `func_def` | `zt_text_equals_literal` |
| 121 | `func_def` | `zt_runtime_append_text` |
| 137 | `func_def` | `zt_try_add_size` |
| 149 | `macro` | `ZT_USE_COMPILER_OVERFLOW_BUILTINS` |
| 151 | `macro` | `ZT_USE_COMPILER_OVERFLOW_BUILTINS` |
| 154 | `func_def` | `zt_try_add_i64` |
| 168 | `func_def` | `zt_try_sub_i64` |
| 182 | `func_def` | `zt_try_mul_i64` |
| 238 | `func_def` | `zt_require_added_size` |
| 252 | `func_def` | `zt_runtime_store_error` |
| 269 | `func_def` | `zt_runtime_stable_code` |
| 287 | `func_def` | `zt_runtime_default_help` |
| 318 | `func_def` | `zt_runtime_print_error` |
| 360 | `macro` | `POOL_SIZE` |
| 362 | `struct` | `zt_pool` |
| 369 | `func_def` | `zt_text_pool_alloc` |
| 376 | `func_def` | `zt_text_pool_free` |
| 384 | `func_def` | `zt_validate_pointer` |
| 388 | `func_def` | `zt_runtime_safe_function_example` |
| 398 | `func_def` | `zt_validate_and_free_text` |
| 406 | `func_def` | `zt_validate_and_free_list_i64` |
| 414 | `func_def` | `zt_validate_and_free_map_text_text` |
| 422 | `func_def` | `zt_host_default_read_file` |
| 511 | `func_def` | `zt_host_default_write_file` |
| 546 | `func_def` | `zt_host_default_path_exists` |
| 563 | `func_def` | `zt_host_default_fs_append_text` |
| 599 | `func_def` | `zt_host_default_fs_is_file` |
| 618 | `func_def` | `zt_host_default_fs_is_dir` |
| 637 | `func_def` | `zt_host_default_fs_create_dir` |
| 651 | `func_def` | `zt_host_default_fs_create_dir_all` |
| 665 | `func_def` | `zt_host_default_fs_list` |
| 767 | `func_def` | `zt_host_default_fs_remove_file` |
| 784 | `func_def` | `zt_host_default_fs_remove_dir` |

#### `runtime/c/zenith_rt.h`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `macro` | `ZENITH_NEXT_RUNTIME_C_ZENITH_RT_H` |

#### `runtime/c/zenith_rt_borealis.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `macro` | `ZT_BOREALIS_BACKEND_STUB` |
| 2 | `macro` | `ZT_BOREALIS_BACKEND_RAYLIB` |
| 3 | `macro` | `ZT_BOREALIS_STUB_WINDOW_ID` |
| 4 | `macro` | `ZT_BOREALIS_RAYLIB_WINDOW_ID` |
| 5 | `macro` | `ZT_BOREALIS_MAX_WINDOWS` |
| 6 | `macro` | `ZT_BOREALIS_MAX_KEYS_PER_WINDOW` |
| 7 | `macro` | `ZT_BOREALIS_MAX_RAYLIB_TEXTURES` |
| 8 | `macro` | `ZT_BOREALIS_MAX_RAYLIB_SOUNDS` |
| 9 | `macro` | `ZT_BOREALIS_MAX_RAYLIB_MODELS` |
| 10 | `macro` | `ZT_BOREALIS_PATH_CAPACITY` |
| 13 | `macro` | `ZT_BOREALIS_RAYLIB_PLATFORM_DIR` |
| 14 | `macro` | `ZT_BOREALIS_RAYLIB_OS_DIR` |
| 18 | `macro` | `ZT_BOREALIS_RAYLIB_PLATFORM_DIR` |
| 20 | `macro` | `ZT_BOREALIS_RAYLIB_PLATFORM_DIR` |
| 22 | `macro` | `ZT_BOREALIS_RAYLIB_OS_DIR` |
| 25 | `macro` | `ZT_BOREALIS_RAYLIB_PLATFORM_DIR` |
| 27 | `macro` | `ZT_BOREALIS_RAYLIB_PLATFORM_DIR` |
| 29 | `macro` | `ZT_BOREALIS_RAYLIB_OS_DIR` |
| 33 | `struct` | `zt_borealis_key_state` |
| 41 | `struct` | `zt_borealis_window_state` |
| 52 | `func_def` | `zt_borealis_backend_missing_error` |
| 58 | `func_def` | `zt_borealis_backend_missing_i64` |
| 65 | `func_def` | `zt_borealis_backend_missing_void` |
| 72 | `func_def` | `zt_borealis_find_window_state` |
| 83 | `func_def` | `zt_borealis_alloc_window_state` |
| 100 | `func_def` | `zt_borealis_free_window_state` |
| 107 | `func_def` | `zt_borealis_is_stub_window` |
| 112 | `func_def` | `zt_borealis_open_stub_window` |
| 121 | `func_def` | `zt_borealis_set_desktop_api` |
| 125 | `func_def` | `zt_borealis_get_desktop_api` |
| 129 | `struct` | `zt_borealis_raylib_color` |
| 136 | `struct` | `zt_borealis_raylib_vector2` |
| 141 | `struct` | `zt_borealis_raylib_rectangle` |
| 148 | `struct` | `zt_borealis_raylib_vector3` |
| 154 | `struct` | `zt_borealis_raylib_vector4` |
| 163 | `struct` | `zt_borealis_raylib_matrix` |
| 182 | `struct` | `zt_borealis_raylib_texture` |
| 190 | `struct` | `zt_borealis_raylib_audio_stream` |
| 198 | `struct` | `zt_borealis_raylib_sound` |
| 203 | `struct` | `zt_borealis_raylib_camera3d` |
| 211 | `struct` | `zt_borealis_raylib_mesh` |
| 230 | `struct` | `zt_borealis_raylib_shader` |
| 235 | `struct` | `zt_borealis_raylib_material_map` |
| 241 | `struct` | `zt_borealis_raylib_material` |
| 247 | `struct` | `zt_borealis_raylib_transform` |
| 255 | `struct` | `zt_borealis_raylib_bone_info` |
| 260 | `struct` | `zt_borealis_raylib_model_skeleton` |
| 266 | `struct` | `zt_borealis_raylib_model` |
| 331 | `struct` | `zt_borealis_raylib_runtime` |
| 384 | `struct` | `zt_borealis_raylib_texture_slot` |
| 390 | `struct` | `zt_borealis_raylib_sound_slot` |
| 396 | `struct` | `zt_borealis_raylib_model_slot` |
| 410 | `func_def` | `zt_borealis_dynlib_open` |
| 418 | `func_def` | `zt_borealis_dynlib_symbol` |
| 429 | `func_def` | `zt_borealis_dynlib_close` |
| 440 | `func_def` | `zt_borealis_copy_cstr` |
| 453 | `func_def` | `zt_borealis_path_is_sep` |
| 457 | `func_def` | `zt_borealis_path_join` |
| 482 | `func_def` | `zt_borealis_path_dirname_in_place` |
| 512 | `func_def` | `zt_borealis_get_cwd` |
| 523 | `func_def` | `zt_borealis_get_executable_dir` |
| 560 | `func_def` | `zt_borealis_color_u8` |
| 566 | `func_def` | `zt_borealis_make_raylib_color` |
| 575 | `func_def` | `zt_borealis_make_raylib_vector3` |
| 583 | `func_def` | `zt_borealis_make_raylib_rectangle` |
| 596 | `func_def` | `zt_borealis_make_raylib_camera3d` |
| 622 | `func_def` | `zt_borealis_raylib_mode3d_ready` |
| 629 | `func_def` | `zt_borealis_raylib_model_loaded` |
| 636 | `func_def` | `zt_borealis_raylib_assign_required_symbols` |
| 702 | `func_def` | `zt_borealis_raylib_reset_failed_candidate` |
| 711 | `func_def` | `zt_borealis_raylib_open_candidate` |
| 731 | `func_def` | `zt_borealis_raylib_try_names_in_dir` |
| 751 | `func_def` | `zt_borealis_raylib_try_relative_dir` |
| 762 | `func_def` | `zt_borealis_raylib_try_module_layout` |
| 793 | `func_def` | `zt_borealis_raylib_try_module_layout_upwards` |
| 813 | `func_def` | `zt_borealis_raylib_try_env_path` |
| 828 | `func_def` | `zt_borealis_raylib_try_load` |
| 877 | `func_def` | `zt_borealis_raylib_available` |
| 881 | `func_def` | `zt_borealis_raylib_loaded_path` |
| 888 | `func_def` | `zt_borealis_raylib_find_texture` |
| 899 | `func_def` | `zt_borealis_raylib_alloc_texture` |
| 916 | `func_def` | `zt_borealis_raylib_find_sound` |
| 927 | `func_def` | `zt_borealis_raylib_alloc_sound` |
| 944 | `func_def` | `zt_borealis_raylib_find_model` |
| 955 | `func_def` | `zt_borealis_raylib_alloc_model` |
| 972 | `func_def` | `zt_borealis_raylib_release_all_textures` |
| 985 | `func_def` | `zt_borealis_raylib_release_all_sounds` |
| 998 | `func_def` | `zt_borealis_raylib_release_all_models` |
| 1011 | `func_def` | `zt_borealis_raylib_open_window` |
| 1046 | `func_def` | `zt_borealis_raylib_close_window` |
| 1079 | `func_def` | `zt_borealis_raylib_window_should_close` |
| 1086 | `func_def` | `zt_borealis_raylib_begin_frame` |
| 1107 | `func_def` | `zt_borealis_raylib_end_frame` |
| 1125 | `func_def` | `zt_borealis_raylib_draw_rect` |
| 1148 | `func_def` | `zt_borealis_raylib_draw_line` |
| 1171 | `func_def` | `zt_borealis_raylib_draw_rect_outline` |
| 1206 | `func_def` | `zt_borealis_raylib_draw_circle` |
| 1227 | `func_def` | `zt_borealis_raylib_draw_circle_outline` |
| 1258 | `func_def` | `zt_borealis_raylib_draw_text` |
| 1284 | `func_def` | `zt_borealis_raylib_is_key_down` |
| 1291 | `func_def` | `zt_borealis_raylib_is_key_pressed` |
| 1298 | `func_def` | `zt_borealis_raylib_is_key_released` |
| 1305 | `func_def` | `zt_borealis_raylib_draw_triangle` |
| 1341 | `func_def` | `zt_borealis_raylib_draw_ellipse` |
| 1370 | `func_def` | `zt_borealis_raylib_measure_text` |
| 1382 | `func_def` | `zt_borealis_raylib_load_texture` |
| 1409 | `func_def` | `zt_borealis_raylib_unload_texture` |
| 1421 | `func_def` | `zt_borealis_raylib_texture_width` |
| 1426 | `func_def` | `zt_borealis_raylib_texture_height` |
| 1431 | `func_def` | `zt_borealis_raylib_draw_texture` |
| 1464 | `func_def` | `zt_borealis_raylib_draw_texture_ex` |
| 1503 | `func_def` | `zt_borealis_raylib_init_audio_device` |
| 1511 | `func_def` | `zt_borealis_raylib_close_audio_device` |
| 1518 | `func_def` | `zt_borealis_raylib_is_audio_device_ready` |
| 1525 | `func_def` | `zt_borealis_raylib_set_master_volume` |
| 1533 | `func_def` | `zt_borealis_raylib_load_sound` |
| 1560 | `func_def` | `zt_borealis_raylib_unload_sound` |
| 1572 | `func_def` | `zt_borealis_raylib_play_sound` |
| 1584 | `func_def` | `zt_borealis_raylib_stop_sound` |
| 1595 | `func_def` | `zt_borealis_raylib_set_sound_volume` |
| 1607 | `func_def` | `zt_borealis_raylib_begin_mode3d` |
| 1655 | `func_def` | `zt_borealis_raylib_end_mode3d` |
| 1674 | `func_def` | `zt_borealis_raylib_draw_cube` |
| 1706 | `func_def` | `zt_borealis_raylib_draw_grid` |
| 1727 | `func_def` | `zt_borealis_raylib_load_model` |
| 1754 | `func_def` | `zt_borealis_raylib_unload_model` |
| 1766 | `func_def` | `zt_borealis_raylib_draw_model` |
| 1837 | `func_def` | `zt_borealis_raylib_draw_billboard` |
| 1914 | `func_def` | `zt_borealis_raylib_vector2_length` |
| 1918 | `func_def` | `zt_borealis_raylib_vector2_distance` |
| 1924 | `func_def` | `zt_borealis_raylib_lerp` |
| 1928 | `func_def` | `zt_borealis_raylib_ease_linear` |
| 1933 | `func_def` | `zt_borealis_raylib_ease_sine_in` |
| 1939 | `func_def` | `zt_borealis_raylib_ease_sine_out` |
| 1945 | `func_def` | `zt_borealis_raylib_ease_sine_in_out` |
| 1951 | `func_def` | `zt_borealis_raylib_ease_quad_in` |
| 1957 | `func_def` | `zt_borealis_raylib_ease_quad_out` |
| 1963 | `func_def` | `zt_borealis_raylib_ease_quad_in_out` |
| 1990 | `func_def` | `zt_borealis_try_register_builtin_desktop_api` |
| 2001 | `func_def` | `zt_borealis_find_key_state` |
| 2032 | `func_def` | `zt_borealis_open_window` |
| 2049 | `func_def` | `zt_borealis_close_window` |
| 2064 | `func_def` | `zt_borealis_window_should_close` |
| 2078 | `func_def` | `zt_borealis_begin_frame` |
| 2105 | `func_def` | `zt_borealis_end_frame` |
| 2119 | `func_def` | `zt_borealis_draw_rect` |
| 2142 | `func_def` | `zt_borealis_draw_line` |
| 2165 | `func_def` | `zt_borealis_draw_rect_outline` |
| 2189 | `func_def` | `zt_borealis_draw_circle` |
| 2211 | `func_def` | `zt_borealis_draw_circle_outline` |
| 2234 | `func_def` | `zt_borealis_draw_text` |
| 2257 | `func_def` | `zt_borealis_is_key_down` |
| 2281 | `func_def` | `zt_borealis_is_key_pressed` |
| 2305 | `func_def` | `zt_borealis_is_key_released` |
| 2329 | `func_def` | `zt_borealis_stub_set_key_down` |
| 2351 | `func_def` | `zt_borealis_stub_reset_input` |

#### `runtime/c/zenith_rt_http.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `struct` | `zt_http_url_parts` |
| 7 | `func_def` | `zt_http_failure` |
| 14 | `func_def` | `zt_http_url_parts_dispose` |
| 23 | `func_def` | `zt_http_copy_range` |
| 35 | `func_def` | `zt_http_parse_port` |
| 51 | `func_def` | `zt_http_parse_url` |
| 100 | `func_def` | `zt_http_request_core` |
| 216 | `func_def` | `zt_http_get_core` |
| 225 | `func_def` | `zt_http_post_core` |

#### `runtime/c/zenith_rt_json.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `func_def` | `zt_json_skip_whitespace` |
| 8 | `func_def` | `zt_json_buffer_reserve` |
| 36 | `func_def` | `zt_json_buffer_append_char` |
| 43 | `func_def` | `zt_json_buffer_append_bytes` |
| 54 | `func_def` | `zt_json_buffer_append_escaped_text` |
| 99 | `func_def` | `zt_json_parse_string` |
| 183 | `func_def` | `zt_json_parse_unquoted_value` |
| 225 | `func_def` | `zt_json_parse_map_text_text` |
| 325 | `func_decl` | `zt_outcome_map_text_text_core_error_failure_message` |
| 328 | `func_def` | `zt_json_stringify_map_text_text` |
| 374 | `func_def` | `zt_json_pretty_map_text_text` |

#### `runtime/c/zenith_rt_net.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `func_def` | `zt_net_startup` |
| 25 | `func_def` | `zt_net_last_error_code` |
| 33 | `func_def` | `zt_net_would_block_code` |
| 41 | `func_def` | `zt_net_format_error` |
| 53 | `func_def` | `zt_net_set_nonblocking` |
| 70 | `func_def` | `zt_net_wait_socket` |
| 114 | `func_def` | `zt_net_socket_error` |
| 135 | `func_def` | `zt_net_connection_new` |
| 152 | `func_def` | `zt_net_core_error_from_prefixed_message` |
| 176 | `func_def` | `zt_net_connection_core_error_failure_prefixed` |
| 183 | `func_def` | `zt_net_optional_bytes_core_error_failure_prefixed` |
| 190 | `func_def` | `zt_net_void_core_error_failure_prefixed` |
| 197 | `func_def` | `zt_net_connect` |
| 301 | `func_def` | `zt_net_effective_timeout_ms` |
| 305 | `func_def` | `zt_net_read_some` |
| 370 | `func_def` | `zt_net_write_all` |
| 420 | `func_def` | `zt_net_close` |
| 433 | `func_def` | `zt_net_is_closed` |
| 438 | `func_def` | `zt_net_error_kind_index` |

#### `runtime/c/zenith_rt_outcome.c`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `func_def` | `zt_fs_is_separator` |
| 5 | `func_def` | `zt_fs_core_error_from_code_message` |
| 9 | `func_def` | `zt_fs_core_error_from_errno` |
| 63 | `func_def` | `zt_fs_windows_error_message` |
| 89 | `func_def` | `zt_fs_core_error_from_windows` |
| 128 | `macro` | `ZT_DEFINE_FS_FAILURE_HELPER` |
| 129 | `func_def` | `NAME` |
| 135 | `func_def` | `zt_fs_join_path` |
| 192 | `func_def` | `zt_fs_create_dir_path` |
| 221 | `func_def` | `zt_fs_create_dir_all_path` |
| 269 | `func_decl` | `zt_host_default_read_file` |
| 270 | `func_decl` | `zt_host_default_write_file` |
| 271 | `func_decl` | `zt_host_default_path_exists` |
| 272 | `func_decl` | `zt_host_default_read_line_stdin` |
| 273 | `func_decl` | `zt_host_default_read_all_stdin` |
| 274 | `func_decl` | `zt_host_default_write_stdout` |
| 275 | `func_decl` | `zt_host_default_write_stderr` |
| 276 | `func_decl` | `zt_host_default_time_now_unix_ms` |
| 277 | `func_decl` | `zt_host_default_time_sleep_ms` |
| 278 | `func_decl` | `zt_host_default_random_seed` |
| 279 | `func_decl` | `zt_host_default_random_next_i64` |
| 280 | `func_decl` | `zt_host_default_os_current_dir` |
| 281 | `func_decl` | `zt_host_default_os_change_dir` |
| 282 | `func_decl` | `zt_host_default_os_args` |
| 283 | `func_decl` | `zt_host_default_os_env` |
| 284 | `func_decl` | `zt_host_default_os_pid` |
| 285 | `func_decl` | `zt_host_default_os_platform` |
| 286 | `func_decl` | `zt_host_default_os_arch` |
| 287 | `func_decl` | `zt_host_default_fs_append_text` |
| 288 | `func_decl` | `zt_host_default_fs_is_file` |
| 289 | `func_decl` | `zt_host_default_fs_is_dir` |
| 290 | `func_decl` | `zt_host_default_fs_create_dir` |
| 291 | `func_decl` | `zt_host_default_fs_create_dir_all` |
| 292 | `func_decl` | `zt_host_default_fs_list` |
| 293 | `func_decl` | `zt_host_default_fs_remove_file` |
| 294 | `func_decl` | `zt_host_default_fs_remove_dir` |
| 295 | `func_decl` | `zt_host_default_fs_remove_dir_all` |
| 296 | `func_decl` | `zt_host_default_fs_copy_file` |
| 297 | `func_decl` | `zt_host_default_fs_move` |
| 298 | `func_decl` | `zt_host_default_fs_size` |
| 299 | `func_decl` | `zt_host_default_fs_modified_at` |
| 300 | `func_decl` | `zt_host_default_fs_created_at` |
| 301 | `func_decl` | `zt_host_prepare_path_copy` |
| 302 | `func_decl` | `zt_host_default_process_run` |
| 303 | `func_decl` | `zt_host_default_process_run_capture` |
| 330 | `struct` | `zt_process_capture_redirect` |
| 340 | `func_decl` | `zt_host_restore_process_stdio` |
| 341 | `func_decl` | `zt_process_captured_run_retain` |
| 342 | `func_decl` | `zt_process_captured_run_dispose` |
| 344 | `func_def` | `zt_runtime_span_unknown` |
| 352 | `func_def` | `zt_runtime_make_span` |
| 360 | `func_def` | `zt_runtime_span_is_known` |
| 367 | `func_def` | `zt_runtime_last_error` |
| 371 | `func_def` | `zt_runtime_clear_error` |
| 381 | `func_def` | `zt_error_kind_name` |
| 412 | `func_def` | `zt_header_from_ref` |
| 416 | `func_def` | `zt_free_text` |
| 427 | `func_def` | `zt_free_bytes` |
| 438 | `func_def` | `zt_free_closure` |
| 454 | `func_def` | `zt_free_lazy_i64` |
| 469 | `func_def` | `zt_free_dyn_text_repr` |
| 482 | `func_def` | `zt_free_list_dyn_text_repr` |
| 497 | `func_def` | `zt_net_close_socket_handle` |
| 511 | `func_def` | `zt_free_net_connection` |
| 524 | `func_def` | `zt_runtime_require_text` |
| 530 | `func_def` | `zt_runtime_require_bytes` |
| 536 | `func_def` | `zt_runtime_require_net_connection` |
| 542 | `func_def` | `zt_runtime_require_list_i64` |
| 548 | `func_def` | `zt_runtime_require_list_text` |
| 554 | `func_def` | `zt_runtime_require_dyn_text_repr` |
| 560 | `func_def` | `zt_runtime_require_list_dyn_text_repr` |
| 566 | `func_def` | `zt_runtime_require_map_text_text` |
| 572 | `func_def` | `zt_normalize_slice_end` |
| 593 | `func_def` | `zt_text_hash` |
| 620 | `func_def` | `zt_i64_hash` |
| 647 | `macro` | `ZT_SET_EMPTY` |
| 648 | `macro` | `ZT_SET_OCCUPIED` |
| 649 | `macro` | `ZT_SET_DELETED` |
| 650 | `macro` | `ZT_SET_INITIAL_CAP` |
| 652 | `func_def` | `zt_free_set_i64` |
| 659 | `func_decl` | `zt_set_i64_grow` |
| 661 | `func_def` | `zt_set_i64_create` |
| 679 | `func_def` | `zt_set_i64_from_array` |
| 689 | `func_def` | `zt_set_i64_add` |
| 710 | `func_def` | `zt_set_i64_has` |
| 722 | `func_def` | `zt_set_i64_remove` |
| 737 | `func_def` | `zt_set_i64_len` |
| 742 | `func_def` | `zt_set_i64_value_at` |
| 758 | `func_def` | `zt_set_i64_union` |
| 778 | `func_def` | `zt_set_i64_intersect` |
| 791 | `func_def` | `zt_set_i64_difference` |
| 804 | `func_def` | `zt_set_i64_grow` |
| 833 | `func_def` | `zt_free_set_text` |
| 846 | `func_decl` | `zt_set_text_grow` |
| 848 | `func_def` | `zt_set_text_create` |
| 866 | `func_def` | `zt_set_text_from_array` |
| 876 | `func_def` | `zt_set_text_add` |
| 898 | `func_def` | `zt_set_text_has` |
| 910 | `func_def` | `zt_set_text_remove` |
| 927 | `func_def` | `zt_set_text_len` |
| 932 | `func_def` | `zt_set_text_value_at` |
| 949 | `func_def` | `zt_set_text_union` |
| 969 | `func_def` | `zt_set_text_intersect` |
| 983 | `func_def` | `zt_set_text_difference` |
| 997 | `func_def` | `zt_set_text_grow` |
| 1032 | `func_def` | `zt_utf8_is_continuation` |
| 1036 | `func_def` | `zt_utf8_validate` |
| 1280 | `func_def` | `zt_utf8_sequence_width_or_error` |
| 1360 | `func_def` | `zt_text_codepoint_count` |
| 1376 | `func_def` | `zt_text_byte_offset_for_codepoint` |
| 1421 | `func_decl` | `zt_free_grid2d_i64` |
| 1422 | `func_decl` | `zt_free_grid2d_text` |
| 1423 | `func_decl` | `zt_free_pqueue_i64` |
| 1424 | `func_decl` | `zt_free_pqueue_text` |
| 1425 | `func_decl` | `zt_free_circbuf_i64` |
| 1426 | `func_decl` | `zt_free_circbuf_text` |
| 1427 | `func_decl` | `zt_free_btreemap_text_text` |
| 1428 | `func_decl` | `zt_free_btreeset_text` |
| 1429 | `func_decl` | `zt_free_grid3d_i64` |
| 1430 | `func_decl` | `zt_free_grid3d_text` |
| 1431 | `func_decl` | `zt_free_net_connection` |
| 1433 | `func_def` | `zt_register_dynamic_heap_kind` |
| 1460 | `func_def` | `zt_retain` |
| 1480 | `func_def` | `zt_release` |
| 1623 | `func_def` | `zt_deep_copy` |
| 1837 | `struct` | `zt_shared_ops` |
| 1841 | `struct` | `zt_shared_handle` |
| 1855 | `func_def` | `zt_shared_text_snapshot_value` |
| 1862 | `func_def` | `zt_shared_bytes_snapshot_value` |
| 1877 | `func_def` | `zt_shared_handle_init` |
| 1888 | `func_def` | `zt_shared_handle_retain` |
| 1911 | `func_def` | `zt_shared_handle_release` |
| 1934 | `func_def` | `zt_shared_handle_borrow` |
| 1942 | `func_def` | `zt_shared_handle_snapshot` |
| 1947 | `func_def` | `zt_shared_handle_ref_count` |
| 1955 | `func_def` | `zt_shared_text_new` |
| 1968 | `func_def` | `zt_shared_text_retain` |
| 1977 | `func_def` | `zt_shared_text_release` |
| 1988 | `func_def` | `zt_shared_text_borrow` |
| 1994 | `func_def` | `zt_shared_text_snapshot` |
| 2000 | `func_def` | `zt_shared_text_ref_count` |
| 2006 | `func_def` | `zt_shared_bytes_new` |
| 2019 | `func_def` | `zt_shared_bytes_retain` |
| 2028 | `func_def` | `zt_shared_bytes_release` |
| 2039 | `func_def` | `zt_shared_bytes_borrow` |
| 2045 | `func_def` | `zt_shared_bytes_snapshot` |
| 2051 | `func_def` | `zt_shared_bytes_ref_count` |
| 2057 | `func_def` | `zt_runtime_report_error` |
| 2064 | `func_def` | `zt_runtime_exit_code_for_kind` |
| 2075 | `func_def` | `zt_runtime_error_ex` |
| 2084 | `func_def` | `zt_runtime_error_with_span` |
| 2088 | `func_def` | `zt_runtime_error` |
| 2092 | `func_def` | `zt_check` |
| 2098 | `func_def` | `zt_todo` |
| 2103 | `func_def` | `zt_unreachable` |
| 2108 | `func_def` | `zt_panic` |
| 2114 | `func_def` | `zt_builtin_print` |
| 2122 | `func_def` | `zt_builtin_read` |
| 2130 | `func_def` | `zt_builtin_debug` |
| 2139 | `func_def` | `zt_builtin_type_name` |
| 2144 | `func_def` | `zt_debug_size_of` |
| 2149 | `func_def` | `zt_builtin_range3` |
| 2167 | `func_def` | `zt_builtin_range2` |
| 2173 | `func_def` | `zt_test_fail` |
| 2179 | `func_def` | `zt_test_skip` |
| 2185 | `func_def` | `zt_test_throws_closure` |
| 2205 | `func_def` | `zt_contract_failed` |
| 2216 | `func_def` | `zt_contract_failed_with_suffix` |
| 2245 | `func_def` | `zt_contract_failed_i64` |
| 2251 | `func_def` | `zt_contract_failed_float` |
| 2257 | `func_def` | `zt_contract_failed_bool` |
| 2262 | `func_def` | `zt_text_from_utf8_unchecked` |
| 2289 | `func_def` | `zt_text_from_utf8` |
| 2314 | `func_def` | `zt_text_from_utf8_literal` |
| 2322 | `func_def` | `zt_text_concat` |
| 2354 | `func_def` | `zt_closure_create` |
| 2358 | `func_def` | `zt_closure_create_with_drop` |
| 2373 | `func_def` | `zt_lazy_i64_once` |
| 2394 | `func_def` | `zt_lazy_i64_force` |
| 2413 | `func_def` | `zt_lazy_i64_is_consumed` |
| 2420 | `func_def` | `zt_text_index` |
| 2443 | `func_def` | `zt_text_slice` |
| 2483 | `func_def` | `zt_text_eq` |
| 2506 | `func_def` | `zt_text_len` |
| 2511 | `func_def` | `zt_text_data` |
| 2516 | `func_def` | `zt_text_deep_copy` |
| 2521 | `enum` | `zt_regex_atom_kind` |
| 2530 | `struct` | `zt_regex_atom` |
| 2538 | `func_def` | `zt_regex_is_quantifier` |
| 2542 | `func_def` | `zt_regex_is_word_char` |
| 2547 | `func_def` | `zt_regex_find_class_end` |
| 2584 | `func_def` | `zt_regex_class_range_is_valid` |
| 2610 | `func_def` | `zt_regex_validate_pattern_data` |
| 2695 | `func_def` | `zt_regex_parse_atom` |
| 2760 | `func_def` | `zt_regex_class_content_matches` |
| 2800 | `func_def` | `zt_regex_atom_matches` |
| 2818 | `func_def` | `zt_regex_match_here` |
| 2898 | `func_def` | `zt_regex_match_from` |
| 2911 | `func_def` | `zt_regex_search_from` |
| 2946 | `func_def` | `zt_regex_append_bytes` |
| 2986 | `func_def` | `zt_regex_append_char` |
| 2994 | `func_def` | `zt_regex_escape_requires_backslash` |
| 3011 | `func_def` | `zt_regex_validate_core` |
| 3022 | `func_def` | `zt_regex_is_match_core` |
| 3035 | `func_def` | `zt_regex_full_match_core` |
| 3048 | `func_def` | `zt_regex_first_core` |
| 3070 | `func_def` | `zt_regex_count_core` |
| 3099 | `func_def` | `zt_regex_find_all_core` |
| 3133 | `func_def` | `zt_regex_split_core` |
| 3180 | `func_def` | `zt_regex_replace_all_core` |
| 3226 | `func_def` | `zt_regex_escape_core` |
| 3248 | `func_def` | `zt_bytes_empty` |
| 3252 | `func_def` | `zt_bytes_from_array` |
| 3279 | `func_def` | `zt_bytes_from_list_i64` |
| 3302 | `func_def` | `zt_bytes_to_list_i64` |
| 3322 | `func_def` | `zt_bytes_join` |
| 3341 | `func_def` | `zt_bytes_starts_with` |
| 3356 | `func_def` | `zt_bytes_ends_with` |
| 3371 | `func_def` | `zt_bytes_contains` |
| 3393 | `func_def` | `zt_text_to_utf8_bytes` |
| 3398 | `func_def` | `zt_text_from_utf8_bytes` |
| 3424 | `func_def` | `zt_bytes_len` |
| 3429 | `func_def` | `zt_bytes_get` |
| 3439 | `func_def` | `zt_bytes_slice` |
| 3466 | `func_def` | `zt_list_i64_get_optional` |
| 3475 | `func_def` | `zt_list_i64_last_optional` |
| 3483 | `func_def` | `zt_list_i64_rest` |
| 3491 | `func_def` | `zt_list_i64_skip` |
| 3507 | `func_def` | `zt_list_text_get_optional` |
| 3521 | `func_def` | `zt_list_text_last_optional` |
| 3535 | `func_def` | `zt_list_text_rest` |
| 3543 | `func_def` | `zt_list_text_skip` |
| 3555 | `func_def` | `zt_queue_i64_new` |
| 3559 | `func_def` | `zt_queue_i64_enqueue` |
| 3563 | `func_def` | `zt_queue_i64_enqueue_owned` |
| 3567 | `func_def` | `zt_queue_i64_dequeue` |
| 3583 | `func_def` | `zt_queue_i64_peek` |
| 3591 | `func_def` | `zt_queue_text_new` |
| 3595 | `func_def` | `zt_queue_text_enqueue` |
| 3599 | `func_def` | `zt_queue_text_enqueue_owned` |
| 3603 | `func_def` | `zt_queue_text_dequeue` |
| 3625 | `func_def` | `zt_queue_text_peek` |
| 3638 | `func_def` | `zt_stack_i64_new` |
| 3642 | `func_def` | `zt_stack_i64_push` |
| 3646 | `func_def` | `zt_stack_i64_push_owned` |
| 3650 | `func_def` | `zt_stack_i64_pop` |
| 3663 | `func_def` | `zt_stack_i64_peek` |
| 3671 | `func_def` | `zt_stack_text_new` |
| 3675 | `func_def` | `zt_stack_text_push` |
| 3679 | `func_def` | `zt_stack_text_push_owned` |
| 3683 | `func_def` | `zt_stack_text_pop` |
| 3702 | `func_def` | `zt_stack_text_peek` |
| 3717 | `func_def` | `zt_dyn_text_repr_alloc` |
| 3728 | `func_def` | `zt_dyn_text_repr_from_i64` |
| 3734 | `func_def` | `zt_dyn_text_repr_from_float` |
| 3740 | `func_def` | `zt_dyn_text_repr_from_bool` |
| 3746 | `func_def` | `zt_dyn_text_repr_from_text_owned` |
| 3754 | `func_def` | `zt_dyn_text_repr_from_text` |
| 3759 | `func_def` | `zt_dyn_text_repr_clone` |
| 3778 | `func_def` | `zt_dyn_text_repr_to_text` |
| 3801 | `func_def` | `zt_dyn_text_repr_text_len` |
| 3812 | `func_def` | `zt_list_dyn_text_repr_new` |
| 3831 | `func_def` | `zt_list_dyn_text_repr_reserve` |
| 3857 | `func_def` | `zt_list_dyn_text_repr_push` |
| 3867 | `func_def` | `zt_list_dyn_text_repr_from_array` |
| 3879 | `func_def` | `zt_list_dyn_text_repr_from_array_owned` |
| 3893 | `func_def` | `zt_list_dyn_text_repr_get` |
| 3908 | `func_def` | `zt_list_dyn_text_repr_len` |
| 3913 | `func_def` | `zt_list_dyn_text_repr_slice` |
| 3942 | `func_def` | `zt_list_dyn_text_repr_deep_copy` |
| 3958 | `func_def` | `zt_thread_boundary_copy_text` |
| 3963 | `func_def` | `zt_thread_boundary_copy_bytes` |
| 3968 | `func_def` | `zt_thread_boundary_copy_list_i64` |
| 3973 | `func_def` | `zt_thread_boundary_copy_list_text` |
| 3978 | `func_def` | `zt_thread_boundary_copy_map_text_text` |
| 3983 | `func_def` | `zt_thread_boundary_copy_dyn_text_repr` |
| 3989 | `func_def` | `zt_thread_boundary_copy_list_dyn_text_repr` |
| 3994 | `func_def` | `zt_core_error_make` |
| 4029 | `func_def` | `zt_core_error_from_message` |
| 4042 | `func_def` | `zt_core_error_from_text` |
| 4053 | `func_def` | `zt_core_error_clone` |
| 4064 | `func_def` | `zt_core_error_dispose` |
| 4074 | `func_def` | `zt_core_error_message_or_default` |
| 4082 | `func_def` | `zt_outcome_process_captured_run_core_error_success` |
| 4091 | `func_def` | `zt_outcome_process_captured_run_core_error_failure` |
| 4101 | `func_def` | `zt_outcome_process_captured_run_core_error_failure_message` |
| 4108 | `func_def` | `zt_outcome_process_captured_run_core_error_is_success` |
| 4112 | `func_def` | `zt_outcome_process_captured_run_core_error_value` |
| 4122 | `func_def` | `zt_outcome_process_captured_run_core_error_propagate` |
| 4129 | `func_def` | `zt_outcome_process_captured_run_core_error_dispose` |
| 4142 | `func_def` | `zt_outcome_text_text_eq` |

#### `runtime/c/zenith_rt_templates.h`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 17 | `macro` | `ZENITH_RT_TEMPLATES_H` |
| 22 | `macro` | `ZT_TEMPLATE_CAT_INNER` |
| 23 | `macro` | `ZT_TEMPLATE_CAT` |
| 24 | `macro` | `ZT_TEMPLATE_IF_0` |
| 25 | `macro` | `ZT_TEMPLATE_IF_1` |
| 26 | `macro` | `ZT_TEMPLATE_IF` |
| 27 | `macro` | `ZT_TEMPLATE_IF_NOT_0` |
| 28 | `macro` | `ZT_TEMPLATE_IF_NOT_1` |
| 29 | `macro` | `ZT_TEMPLATE_IF_NOT` |
| 49 | `macro` | `ZT_DEFINE_LIST_STRUCT` |
| 58 | `macro` | `ZT_DEFINE_LIST_IMPL` |
| 281 | `macro` | `ZT_DEFINE_LIST` |
| 285 | `macro` | `ZT_DEFINE_MAP_STRUCT` |
| 308 | `macro` | `ZT_DEFINE_MAP_IMPL` |
| 756 | `macro` | `ZT_DEFINE_MAP` |
| 793 | `macro` | `ZT_DEFINE_GRID2D_IMPL` |
| 998 | `macro` | `ZT_DEFINE_GRID3D_IMPL` |
| 1224 | `macro` | `ZT_DEFINE_PQUEUE_IMPL` |
| 1417 | `macro` | `ZT_DEFINE_CIRCBUF_IMPL` |
| 1588 | `macro` | `ZT_DEFINE_BTREEMAP_IMPL` |
| 1895 | `macro` | `ZT_DEFINE_BTREESET_IMPL` |
| 2118 | `macro` | `ZT_DEFINE_OPTIONAL` |
| 2166 | `macro` | `ZT_DEFINE_OPTIONAL_IMPL` |
| 2223 | `macro` | `ZT_DEFINE_OUTCOME_IMPL` |
| 2296 | `macro` | `ZT_DEFINE_OUTCOME_VOID_TEXT_ERROR_IMPL` |
| 2352 | `macro` | `ZT_DEFINE_OUTCOME_VALUE_STRUCT` |
| 2359 | `macro` | `ZT_DEFINE_OUTCOME_VOID_STRUCT` |
| 2391 | `macro` | `ZT_DEFINE_OUTCOME_CORE_ERROR_PRIMITIVE_IMPL` |
| 2448 | `macro` | `ZT_DEFINE_OUTCOME_CORE_ERROR_PTR_IMPL` |
| 2515 | `macro` | `ZT_DEFINE_OUTCOME_VOID_CORE_ERROR_IMPL` |
| 2562 | `macro` | `ZT_DEFINE_OUTCOME_CORE_ERROR_OPTIONAL_PTR_IMPL` |

### Manual Notes

- Critical flow:
- Break conditions:
- Related docs or decisions:
- Extra test cases worth adding:
<!-- CODEMAP:GENERATED:END -->
