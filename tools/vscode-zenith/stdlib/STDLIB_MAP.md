# Standard Library - Code Map

## Descricao

Mapa da stdlib do Zenith.

Objetivos:
- Deixar ownership claro entre `core`, `std.*` e `platform`.
- Evitar sobreposicao de API.
- Facilitar manutencao e onboarding.

## Modulos da Stdlib

| Modulo | Arquivos | Papel | Status |
|--------|----------|-------|--------|
| `core/` | 2 files | Documentacao e contratos internos do core implicito | Placeholder interno |
| `platform/` | 2 files | Adaptacao interna host/target para suportar `std.*` | Placeholder interno |
| `std/` | 17 files | API publica da stdlib | Ativo |
| `zdoc/std/` | - | Documentacao gerada dos modulos publicos | Ativo |

## Contrato de Ownership

- `core.*` e implicito e pertence ao nucleo da linguagem.
- API publica fica em `std.*`.
- `platform/` e interno e nao e API publica para app.

Dono de API publica:
- `std.os`: estado do processo atual, env, pid, plataforma e arquitetura.
- `std.os.process`: execucao e controle de processos filhos.

Nao permitido:
- duplicar em `platform/` APIs ja definidas em `std.os` ou `std.os.process`.

## Mapeamento por Modulo

### core/
| Arquivo | Linha | Funcao | Responsabilidade | Pode quebrar se |
|---------|-------|--------|------------------|-----------------|
| `stdlib/core/README.md` | - | contrato de camada | delimitar uso interno | virar API publica por engano |

### platform/
| Arquivo | Linha | Funcao | Responsabilidade | Pode quebrar se |
|---------|-------|--------|------------------|-----------------|
| `stdlib/platform/README.md` | - | contrato de camada | evitar conflito com `std.os*` | surgir duplicacao de ownership |

### std/
| Arquivo | Linha | Funcao | Responsabilidade | Pode quebrar se |
|---------|-------|--------|------------------|-----------------|
| `stdlib/std/os.zt` | - | `std.os` | estado do processo atual | mover API para `platform/` |
| `stdlib/std/os/process.zt` | - | `std.os.process` | processos filhos | duplicar run/spawn em outra camada |

## Estado Atual

- `core/` e `platform/` estao reservados por design.
- A superficie publica consolidada esta em `std.*`.
- O contrato formal foi fechado em `Decision 085`.

## Dependencias Externas

- `runtime/c/` -> wrappers de host e runtime
- `compiler/semantic/` -> regras de ownership e imports
- system libraries (libc e equivalentes)

## Erros Comuns

1. Tratar `platform/` como API publica.
2. Duplicar funcoes de `std.os` em camadas internas.
3. Mudar ownership sem atualizar as decisions.

## Notas de Manutencao

- Antes de abrir `platform/` de verdade, aplicar os gates da Decision 085.
- Mudancas de ownership devem atualizar docs, tests e code map.
- Manter linguagem objetiva e exemplos curtos.
<!-- CODEMAP:GENERATED:BEGIN -->
## Generated Index

- Priority: Medium
- Source files: 54
- Extracted symbols: 1038

Do not edit this block by hand. Re-run `python tools/generate_code_maps.py`.

### File Summary

| File | Lines | Symbols | Local deps |
| --- | ---: | ---: | ---: |
| `stdlib/std/bool.zt` | 10 | 3 | 0 |
| `stdlib/std/bytes.zt` | 41 | 15 | 0 |
| `stdlib/std/collections.zt` | 584 | 181 | 0 |
| `stdlib/std/concurrent.zt` | 44 | 14 | 0 |
| `stdlib/std/console.zt` | 119 | 35 | 0 |
| `stdlib/std/debug.zt` | 10 | 3 | 0 |
| `stdlib/std/float.zt` | 25 | 9 | 0 |
| `stdlib/std/format.zt` | 68 | 21 | 0 |
| `stdlib/std/fs.zt` | 279 | 43 | 0 |
| `stdlib/std/fs/path.zt` | 189 | 42 | 0 |
| `stdlib/std/http.zt` | 211 | 39 | 0 |
| `stdlib/std/int.zt` | 20 | 7 | 0 |
| `stdlib/std/io.zt` | 103 | 17 | 0 |
| `stdlib/std/json.zt` | 34 | 13 | 0 |
| `stdlib/std/lazy.zt` | 22 | 7 | 0 |
| `stdlib/std/list.zt` | 5 | 1 | 0 |
| `stdlib/std/map.zt` | 5 | 1 | 0 |
| `stdlib/std/math.zt` | 180 | 57 | 0 |
| `stdlib/std/net.zt` | 157 | 19 | 0 |
| `stdlib/std/os.zt` | 114 | 21 | 0 |
| `stdlib/std/os/process.zt` | 68 | 9 | 0 |
| `stdlib/std/random.zt` | 58 | 13 | 0 |
| `stdlib/std/regex.zt` | 100 | 25 | 0 |
| `stdlib/std/set.zt` | 5 | 1 | 0 |
| `stdlib/std/test.zt` | 105 | 29 | 0 |
| `stdlib/std/text.zt` | 266 | 31 | 0 |
| `stdlib/std/time.zt` | 96 | 24 | 0 |
| `stdlib/std/validate.zt` | 71 | 16 | 0 |
| `stdlib/zdoc/std/bool.zdoc` | 8 | 0 | 0 |
| `stdlib/zdoc/std/bytes.zdoc` | 89 | 8 | 0 |
| `stdlib/zdoc/std/collections.zdoc` | 872 | 92 | 0 |
| `stdlib/zdoc/std/concurrent.zdoc` | 74 | 9 | 0 |
| `stdlib/zdoc/std/console.zdoc` | 145 | 16 | 0 |
| `stdlib/zdoc/std/debug.zdoc` | 8 | 0 | 0 |
| `stdlib/zdoc/std/float.zdoc` | 11 | 0 | 0 |
| `stdlib/zdoc/std/format.zdoc` | 130 | 11 | 0 |
| `stdlib/zdoc/std/fs.zdoc` | 208 | 24 | 0 |
| `stdlib/zdoc/std/fs/path.zdoc` | 131 | 13 | 0 |
| `stdlib/zdoc/std/http.zdoc` | 73 | 6 | 0 |
| `stdlib/zdoc/std/int.zdoc` | 10 | 0 | 0 |
| `stdlib/zdoc/std/io.zdoc` | 129 | 12 | 0 |
| `stdlib/zdoc/std/json.zdoc` | 61 | 6 | 0 |
| `stdlib/zdoc/std/list.zdoc` | 10 | 1 | 0 |
| `stdlib/zdoc/std/map.zdoc` | 11 | 1 | 0 |
| `stdlib/zdoc/std/math.zdoc` | 291 | 33 | 0 |
| `stdlib/zdoc/std/net.zdoc` | 130 | 10 | 0 |
| `stdlib/zdoc/std/os.zdoc` | 111 | 11 | 0 |
| `stdlib/zdoc/std/os/process.zdoc` | 73 | 6 | 0 |
| `stdlib/zdoc/std/random.zdoc` | 41 | 4 | 0 |
| `stdlib/zdoc/std/regex.zdoc` | 146 | 15 | 0 |
| `stdlib/zdoc/std/test.zdoc` | 96 | 10 | 0 |
| `stdlib/zdoc/std/text.zdoc` | 145 | 18 | 0 |
| `stdlib/zdoc/std/time.zdoc` | 175 | 21 | 0 |
| `stdlib/zdoc/std/validate.zdoc` | 119 | 15 | 0 |

### Local Dependencies

- None detected

### Related Tests

- `tests/behavior/std_bytes_ops/src/app/main.zt`
- `tests/behavior/std_bytes_utf8/src/app/main.zt`
- `tests/behavior/std_collections_basic/src/app/main.zt`
- `tests/behavior/std_collections_managed_arc/src/app/main.zt`
- `tests/behavior/std_collections_queue_stack_cow/src/app/main.zt`
- `tests/behavior/std_concurrent_boundary_copy_basic/src/app/main.zt`
- `tests/behavior/std_concurrent_boundary_copy_determinism/src/app/main.zt`
- `tests/behavior/std_concurrent_boundary_copy_unsupported_error/src/app/main.zt`
- `tests/behavior/std_console_basic/src/app/main.zt`
- `tests/behavior/std_debug_basic/src/app/main.zt`
- `tests/behavior/std_format_basic/src/app/main.zt`
- `tests/behavior/std_fs_aliases_basic/src/app/main.zt`
- `tests/behavior/std_fs_basic/src/app/main.zt`
- `tests/behavior/std_fs_ops_basic/src/app/main.zt`
- `tests/behavior/std_fs_path_basic/src/app/main.zt`
- `tests/behavior/std_http_basic/http-server.ps1`
- `tests/behavior/std_http_basic/run-loopback.ps1`
- `tests/behavior/std_http_basic/src/app/main.zt`
- `tests/behavior/std_io_basic/main.zt`
- `tests/behavior/std_io_basic/run.txt`
- `tests/behavior/std_io_basic/src/app/main.zt`
- `tests/behavior/std_json_basic/src/app/main.zt`
- `tests/behavior/std_math_basic/src/app/main.zt`
- `tests/behavior/std_net_basic/loopback-server.ps1`
- `tests/behavior/std_net_basic/run-loopback.ps1`
- `tests/behavior/std_net_basic/src/app/main.zt`
- `tests/behavior/std_os_args_basic/src/app/main.zt`
- `tests/behavior/std_os_basic/src/app/main.zt`
- `tests/behavior/std_os_process_basic/src/app/main.zt`
- `tests/behavior/std_os_process_capture_basic/src/app/main.zt`
- `tests/behavior/std_random_basic/src/app/main.zt`
- `tests/behavior/std_random_between_branches/src/app/main.zt`
- `tests/behavior/std_random_cross_namespace_write_error/src/app/main.zt`
- `tests/behavior/std_random_state_observability/src/app/main.zt`
- `tests/behavior/std_regex_basic/src/app/main.zt`
- `tests/behavior/std_small_helpers/src/app/main.zt`
- `tests/behavior/std_test_attr_fail/src/app/main.zt`
- `tests/behavior/std_test_attr_fail/src/app/tests.zt`
- `tests/behavior/std_test_attr_pass_skip/src/app/main.zt`
- `tests/behavior/std_test_attr_pass_skip/src/app/tests.zt`
- `tests/behavior/std_test_basic/src/app/main.zt`
- `tests/behavior/std_test_helpers_bool_fail/src/app/main.zt`
- `tests/behavior/std_test_helpers_equal_fail/src/app/main.zt`
- `tests/behavior/std_test_helpers_not_equal_fail/src/app/main.zt`
- `tests/behavior/std_test_helpers_pass/src/app/main.zt`
- `tests/behavior/std_test_throws_fail/src/app/main.zt`
- `tests/behavior/std_test_throws_pass/src/app/main.zt`
- `tests/behavior/std_text_basic/src/app/main.zt`
- `tests/behavior/std_time_basic/src/app/main.zt`
- `tests/behavior/std_validate_basic/src/app/main.zt`

### Symbol Index

#### `stdlib/std/bool.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.bool` |
| 4 | `func` | `zt_bool_to_text` |
| 7 | `func` | `to_text` |

#### `stdlib/std/bytes.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `namespace` | `std.bytes` |
| 5 | `func` | `zt_bytes_empty` |
| 6 | `func` | `zt_bytes_from_list_i64` |
| 7 | `func` | `zt_bytes_to_list_i64` |
| 8 | `func` | `zt_bytes_join` |
| 9 | `func` | `zt_bytes_starts_with` |
| 10 | `func` | `zt_bytes_ends_with` |
| 11 | `func` | `zt_bytes_contains` |
| 14 | `func` | `empty` |
| 18 | `func` | `from_list` |
| 22 | `func` | `to_list` |
| 26 | `func` | `join` |
| 30 | `func` | `starts_with` |
| 34 | `func` | `ends_with` |
| 38 | `func` | `contains` |

#### `stdlib/std/collections.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 10 | `namespace` | `std.collections` |
| 12 | `struct` | `QueueNumberDequeueResult` |
| 17 | `struct` | `QueueTextDequeueResult` |
| 22 | `struct` | `StackNumberPopResult` |
| 27 | `struct` | `StackTextPopResult` |
| 32 | `func` | `optional_number` |
| 36 | `func` | `optional_text_value` |
| 41 | `func` | `zt_queue_i64_new` |
| 42 | `func` | `zt_queue_i64_enqueue` |
| 43 | `func` | `zt_queue_i64_dequeue` |
| 44 | `func` | `zt_queue_i64_peek` |
| 46 | `func` | `zt_queue_text_new` |
| 47 | `func` | `zt_queue_text_enqueue` |
| 48 | `func` | `zt_queue_text_dequeue` |
| 49 | `func` | `zt_queue_text_peek` |
| 51 | `func` | `zt_stack_i64_new` |
| 52 | `func` | `zt_stack_i64_push` |
| 53 | `func` | `zt_stack_i64_pop` |
| 54 | `func` | `zt_stack_i64_peek` |
| 56 | `func` | `zt_stack_text_new` |
| 57 | `func` | `zt_stack_text_push` |
| 58 | `func` | `zt_stack_text_pop` |
| 59 | `func` | `zt_stack_text_peek` |
| 61 | `func` | `zt_grid2d_i64_new` |
| 62 | `func` | `zt_grid2d_i64_get` |
| 63 | `func` | `zt_grid2d_i64_set_owned` |
| 64 | `func` | `zt_grid2d_i64_fill_owned` |
| 65 | `func` | `zt_grid2d_i64_rows` |
| 66 | `func` | `zt_grid2d_i64_cols` |
| 68 | `func` | `zt_grid2d_text_new` |
| 69 | `func` | `zt_grid2d_text_get` |
| 70 | `func` | `zt_grid2d_text_set_owned` |
| 71 | `func` | `zt_grid2d_text_fill_owned` |
| 72 | `func` | `zt_grid2d_text_rows` |
| 73 | `func` | `zt_grid2d_text_cols` |
| 75 | `func` | `zt_pqueue_i64_new` |
| 76 | `func` | `zt_pqueue_i64_push_owned` |
| 77 | `func` | `zt_pqueue_i64_pop` |
| 78 | `func` | `zt_pqueue_i64_peek` |
| 79 | `func` | `zt_pqueue_i64_len` |
| 81 | `func` | `zt_pqueue_text_new` |
| 82 | `func` | `zt_pqueue_text_push_owned` |
| 83 | `func` | `zt_pqueue_text_pop` |
| 84 | `func` | `zt_pqueue_text_peek` |
| 85 | `func` | `zt_pqueue_text_len` |
| 87 | `func` | `zt_circbuf_i64_new` |
| 88 | `func` | `zt_circbuf_i64_push_owned` |
| 89 | `func` | `zt_circbuf_i64_pop` |
| 90 | `func` | `zt_circbuf_i64_peek` |
| 91 | `func` | `zt_circbuf_i64_len` |
| 92 | `func` | `zt_circbuf_i64_capacity` |
| 93 | `func` | `zt_circbuf_i64_is_full` |
| 95 | `func` | `zt_circbuf_text_new` |
| 96 | `func` | `zt_circbuf_text_push_owned` |
| 97 | `func` | `zt_circbuf_text_pop` |
| 98 | `func` | `zt_circbuf_text_peek` |
| 99 | `func` | `zt_circbuf_text_len` |
| 100 | `func` | `zt_circbuf_text_capacity` |
| 101 | `func` | `zt_circbuf_text_is_full` |
| 103 | `func` | `zt_btreemap_text_text_new` |
| 104 | `func` | `zt_btreemap_text_text_set_owned` |
| 105 | `func` | `zt_btreemap_text_text_get` |
| 106 | `func` | `zt_btreemap_text_text_get_optional` |
| 107 | `func` | `zt_btreemap_text_text_contains` |
| 108 | `func` | `zt_btreemap_text_text_remove_owned` |
| 109 | `func` | `zt_btreemap_text_text_len` |
| 111 | `func` | `zt_btreeset_text_new` |
| 112 | `func` | `zt_btreeset_text_insert_owned` |
| 113 | `func` | `zt_btreeset_text_contains` |
| 114 | `func` | `zt_btreeset_text_remove_owned` |
| 115 | `func` | `zt_btreeset_text_len` |
| 117 | `func` | `zt_grid3d_i64_new` |
| 118 | `func` | `zt_grid3d_i64_get` |
| 119 | `func` | `zt_grid3d_i64_set_owned` |
| 120 | `func` | `zt_grid3d_i64_fill_owned` |
| 121 | `func` | `zt_grid3d_i64_depth` |
| 122 | `func` | `zt_grid3d_i64_rows` |
| 123 | `func` | `zt_grid3d_i64_cols` |
| 125 | `func` | `zt_grid3d_text_new` |
| 126 | `func` | `zt_grid3d_text_get` |
| 127 | `func` | `zt_grid3d_text_set_owned` |
| 128 | `func` | `zt_grid3d_text_fill_owned` |
| 129 | `func` | `zt_grid3d_text_depth` |
| 130 | `func` | `zt_grid3d_text_rows` |
| 131 | `func` | `zt_grid3d_text_cols` |
| 134 | `func` | `queue_int_new` |
| 138 | `func` | `queue_int_enqueue` |
| 142 | `func` | `queue_int_dequeue` |
| 156 | `func` | `queue_int_peek` |
| 160 | `func` | `map_int` |
| 165 | `const` | `value` |
| 173 | `func` | `filter_int` |
| 178 | `const` | `value` |
| 188 | `func` | `reduce_int` |
| 193 | `const` | `value` |
| 201 | `func` | `queue_text_new` |
| 205 | `func` | `queue_text_enqueue` |
| 209 | `func` | `queue_text_dequeue` |
| 223 | `func` | `queue_text_peek` |
| 227 | `func` | `stack_int_new` |
| 231 | `func` | `stack_int_push` |
| 235 | `func` | `stack_int_pop` |
| 236 | `const` | `size` |
| 258 | `func` | `stack_int_peek` |
| 262 | `func` | `stack_text_new` |
| 266 | `func` | `stack_text_push` |
| 270 | `func` | `stack_text_pop` |
| 271 | `const` | `size` |
| 293 | `func` | `stack_text_peek` |
| 297 | `func` | `grid2d_int_new` |
| 301 | `func` | `grid2d_int_get` |
| 305 | `func` | `grid2d_int_set` |
| 309 | `func` | `grid2d_int_fill` |
| 313 | `func` | `grid2d_int_rows` |
| 317 | `func` | `grid2d_int_cols` |
| 321 | `func` | `grid2d_text_new` |
| 325 | `func` | `grid2d_text_get` |
| 329 | `func` | `grid2d_text_set` |
| 333 | `func` | `grid2d_text_fill` |
| 337 | `func` | `grid2d_text_rows` |
| 341 | `func` | `grid2d_text_cols` |
| 345 | `func` | `pqueue_int_new` |
| 349 | `func` | `pqueue_int_push` |
| 353 | `func` | `pqueue_int_pop` |
| 357 | `func` | `pqueue_int_peek` |
| 361 | `func` | `pqueue_int_len` |
| 365 | `func` | `pqueue_text_new` |
| 369 | `func` | `pqueue_text_push` |
| 373 | `func` | `pqueue_text_pop` |
| 377 | `func` | `pqueue_text_peek` |
| 381 | `func` | `pqueue_text_len` |
| 385 | `func` | `circbuf_int_new` |
| 389 | `func` | `circbuf_int_push` |
| 393 | `func` | `circbuf_int_pop` |
| 397 | `func` | `circbuf_int_peek` |
| 401 | `func` | `circbuf_int_len` |
| 405 | `func` | `circbuf_int_capacity` |
| 409 | `func` | `circbuf_int_is_full` |
| 413 | `func` | `circbuf_text_new` |
| 417 | `func` | `circbuf_text_push` |
| 421 | `func` | `circbuf_text_pop` |
| 425 | `func` | `circbuf_text_peek` |
| 429 | `func` | `circbuf_text_len` |
| 433 | `func` | `circbuf_text_capacity` |
| 437 | `func` | `circbuf_text_is_full` |
| 441 | `func` | `btreemap_text_new` |
| 445 | `func` | `btreemap_text_set` |
| 449 | `func` | `btreemap_text_get` |
| 453 | `func` | `btreemap_text_get_optional` |
| 457 | `func` | `btreemap_text_contains` |
| 461 | `func` | `btreemap_text_remove` |
| 465 | `func` | `btreemap_text_len` |
| 469 | `func` | `btreeset_text_new` |
| 473 | `func` | `btreeset_text_insert` |
| 477 | `func` | `btreeset_text_contains` |
| 481 | `func` | `btreeset_text_remove` |
| 485 | `func` | `btreeset_text_len` |
| 489 | `func` | `grid3d_int_new` |
| 493 | `func` | `grid3d_int_get` |
| 497 | `func` | `grid3d_int_set` |
| 501 | `func` | `grid3d_int_fill` |
| 505 | `func` | `grid3d_int_depth` |
| 509 | `func` | `grid3d_int_rows` |
| 513 | `func` | `grid3d_int_cols` |
| 517 | `func` | `grid3d_text_new` |
| 521 | `func` | `grid3d_text_get` |
| 525 | `func` | `grid3d_text_set` |
| 529 | `func` | `grid3d_text_fill` |
| 533 | `func` | `grid3d_text_depth` |
| 537 | `func` | `grid3d_text_rows` |
| 541 | `func` | `grid3d_text_cols` |
| 545 | `func` | `pqueue_int_is_empty` |
| 549 | `func` | `pqueue_text_is_empty` |
| 553 | `func` | `circbuf_int_is_empty` |
| 557 | `func` | `circbuf_text_is_empty` |
| 561 | `func` | `btreemap_text_is_empty` |
| 565 | `func` | `btreeset_text_is_empty` |
| 569 | `func` | `grid2d_int_size` |
| 573 | `func` | `grid2d_text_size` |
| 577 | `func` | `grid3d_int_size` |
| 581 | `func` | `grid3d_text_size` |

#### `stdlib/std/concurrent.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.concurrent` |
| 6 | `func` | `zt_thread_boundary_copy_text` |
| 7 | `func` | `zt_thread_boundary_copy_bytes` |
| 8 | `func` | `zt_thread_boundary_copy_list_i64` |
| 9 | `func` | `zt_thread_boundary_copy_list_text` |
| 10 | `func` | `zt_thread_boundary_copy_map_text_text` |
| 13 | `func` | `copy_int` |
| 17 | `func` | `copy_bool` |
| 21 | `func` | `copy_float` |
| 25 | `func` | `copy_text` |
| 29 | `func` | `copy_bytes` |
| 33 | `func` | `copy_list_int` |
| 37 | `func` | `copy_list_text` |
| 41 | `func` | `copy_map_text_text` |

#### `stdlib/std/console.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.console` |
| 6 | `func` | `zt_host_read_line_stdin` |
| 7 | `func` | `zt_host_write_stdout` |
| 8 | `func` | `zt_host_write_stderr` |
| 9 | `func` | `zt_host_console_is_terminal` |
| 10 | `func` | `zt_host_console_columns` |
| 11 | `func` | `zt_host_console_rows` |
| 12 | `func` | `zt_host_console_clear` |
| 13 | `func` | `zt_host_console_set_color` |
| 14 | `func` | `zt_host_console_set_style` |
| 15 | `func` | `zt_host_console_reset_style` |
| 16 | `func` | `zt_host_console_read_key` |
| 19 | `struct` | `Size` |
| 24 | `func` | `write_line` |
| 30 | `func` | `error_line` |
| 36 | `func` | `pause` |
| 38 | `const` | `ignored` |
| 42 | `func` | `prompt` |
| 44 | `const` | `line` |
| 45 | `const` | `empty` |
| 55 | `func` | `confirm` |
| 56 | `const` | `suffix` |
| 57 | `const` | `full_message` |
| 58 | `const` | `prompted` |
| 59 | `const` | `answer` |
| 80 | `func` | `is_terminal` |
| 84 | `func` | `size` |
| 85 | `const` | `current` |
| 92 | `func` | `columns` |
| 96 | `func` | `rows` |
| 100 | `func` | `clear` |
| 104 | `func` | `color` |
| 108 | `func` | `style` |
| 112 | `func` | `reset_style` |
| 116 | `func` | `read_key` |

#### `stdlib/std/debug.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.debug` |
| 4 | `func` | `zt_debug_size_of` |
| 7 | `func` | `size_of` |

#### `stdlib/std/float.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.float` |
| 4 | `func` | `zt_float_to_int` |
| 5 | `func` | `zt_float_round_to_int` |
| 6 | `func` | `zt_float_to_text` |
| 7 | `func` | `zt_float_parse` |
| 10 | `func` | `to_int` |
| 14 | `func` | `round` |
| 18 | `func` | `to_text` |
| 22 | `func` | `parse` |

#### `stdlib/std/format.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 8 | `namespace` | `std.format` |
| 10 | `enum` | `BytesStyle` |
| 16 | `func` | `zt_format_number` |
| 17 | `func` | `zt_format_percent` |
| 18 | `func` | `zt_format_date` |
| 19 | `func` | `zt_format_datetime` |
| 20 | `func` | `zt_format_date_pattern` |
| 21 | `func` | `zt_format_datetime_pattern` |
| 22 | `func` | `zt_format_hex_i64` |
| 23 | `func` | `zt_format_bin_i64` |
| 24 | `func` | `zt_format_bytes_binary` |
| 25 | `func` | `zt_format_bytes_decimal` |
| 28 | `func` | `number` |
| 32 | `func` | `percent` |
| 36 | `func` | `date` |
| 40 | `func` | `datetime` |
| 44 | `func` | `date_pattern` |
| 48 | `func` | `datetime_pattern` |
| 52 | `func` | `bytes` |
| 61 | `func` | `hex` |
| 65 | `func` | `bin` |

#### `stdlib/std/fs.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.fs` |
| 5 | `enum` | `Error` |
| 16 | `struct` | `Metadata` |
| 25 | `func` | `zt_host_read_file` |
| 26 | `func` | `zt_host_write_file` |
| 27 | `func` | `zt_host_path_exists` |
| 28 | `func` | `zt_host_fs_append_text_core` |
| 29 | `func` | `zt_host_fs_is_file_core` |
| 30 | `func` | `zt_host_fs_is_dir_core` |
| 31 | `func` | `zt_host_fs_create_dir_core` |
| 32 | `func` | `zt_host_fs_create_dir_all_core` |
| 33 | `func` | `zt_host_fs_list_core` |
| 34 | `func` | `zt_host_fs_remove_file_core` |
| 35 | `func` | `zt_host_fs_remove_dir_core` |
| 36 | `func` | `zt_host_fs_remove_dir_all_core` |
| 37 | `func` | `zt_host_fs_copy_file_core` |
| 38 | `func` | `zt_host_fs_move_core` |
| 39 | `func` | `zt_host_fs_size_core` |
| 40 | `func` | `zt_host_fs_modified_at_core` |
| 41 | `func` | `zt_host_fs_created_at_core` |
| 44 | `func` | `map_host_error` |
| 84 | `func` | `read_text` |
| 93 | `func` | `write_text` |
| 102 | `func` | `append_text` |
| 111 | `func` | `exists` |
| 112 | `const` | `val` |
| 116 | `func` | `is_file` |
| 125 | `func` | `is_dir` |
| 134 | `func` | `create_dir` |
| 143 | `func` | `create_dir_all` |
| 152 | `func` | `list_dir` |
| 161 | `func` | `remove_file` |
| 170 | `func` | `remove_dir` |
| 179 | `func` | `remove_dir_all` |
| 188 | `func` | `copy_file` |
| 197 | `func` | `copy` |
| 201 | `func` | `move` |
| 210 | `func` | `rename` |
| 214 | `func` | `metadata` |
| 249 | `func` | `size` |
| 258 | `func` | `file_size` |
| 262 | `func` | `modified_at` |
| 271 | `func` | `created_at` |

#### `stdlib/std/fs/path.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 6 | `namespace` | `std.fs.path` |
| 9 | `func` | `zt_text_concat` |
| 10 | `func` | `zt_text_eq` |
| 11 | `func` | `zt_path_normalize` |
| 12 | `func` | `zt_path_is_absolute` |
| 13 | `func` | `zt_path_absolute` |
| 14 | `func` | `zt_path_relative` |
| 17 | `func` | `_text_eq` |
| 21 | `func` | `_last_index_of` |
| 34 | `func` | `join` |
| 43 | `const` | `part` |
| 46 | `const` | `slash` |
| 47 | `const` | `last_char` |
| 51 | `const` | `with_sep` |
| 64 | `func` | `normalize` |
| 68 | `func` | `is_absolute` |
| 72 | `func` | `is_relative` |
| 79 | `func` | `absolute` |
| 83 | `func` | `relative` |
| 87 | `func` | `base_name` |
| 92 | `const` | `slash` |
| 93 | `const` | `slash_index` |
| 105 | `func` | `name_without_extension` |
| 106 | `const` | `file_name` |
| 111 | `const` | `dot` |
| 112 | `const` | `dot_index` |
| 120 | `func` | `extension` |
| 121 | `const` | `file_name` |
| 126 | `const` | `dot` |
| 127 | `const` | `dot_index` |
| 135 | `func` | `parent` |
| 140 | `const` | `slash` |
| 141 | `const` | `slash_index` |
| 153 | `func` | `has_extension` |
| 154 | `const` | `ext_val` |
| 159 | `const` | `with_dot` |
| 163 | `func` | `change_extension` |
| 164 | `const` | `file_name` |
| 169 | `const` | `directory` |
| 170 | `const` | `stem` |
| 171 | `const` | `root` |
| 175 | `const` | `stem_with_dot` |

#### `stdlib/std/http.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.http` |
| 3 | `enum` | `ErrorKind` |
| 10 | `struct` | `Error` |
| 15 | `struct` | `Response` |
| 22 | `func` | `zt_http_get_core` |
| 23 | `func` | `zt_http_post_core` |
| 26 | `func` | `make_error` |
| 30 | `func` | `starts_at` |
| 53 | `func` | `starts_with` |
| 57 | `func` | `slice_from` |
| 70 | `func` | `index_of` |
| 72 | `const` | `max_index` |
| 91 | `func` | `map_core_error` |
| 104 | `func` | `digit_value` |
| 138 | `func` | `status_from_response` |
| 140 | `const` | `err` |
| 144 | `const` | `err` |
| 148 | `const` | `hundreds` |
| 149 | `const` | `tens` |
| 150 | `const` | `ones` |
| 153 | `const` | `err` |
| 157 | `const` | `status_hundreds` |
| 158 | `const` | `status_tens` |
| 159 | `const` | `status_prefix` |
| 160 | `const` | `status` |
| 164 | `func` | `body_from_response` |
| 165 | `const` | `crlf_split` |
| 167 | `const` | `body` |
| 171 | `const` | `lf_split` |
| 173 | `const` | `body` |
| 177 | `const` | `err` |
| 181 | `func` | `response_from_raw` |
| 182 | `const` | `status` |
| 183 | `const` | `body` |
| 184 | `const` | `response` |
| 192 | `func` | `get` |
| 197 | `const` | `http_err` |
| 202 | `func` | `post` |
| 207 | `const` | `http_err` |

#### `stdlib/std/int.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.int` |
| 4 | `func` | `zt_int_to_float` |
| 5 | `func` | `zt_int_to_text` |
| 6 | `func` | `zt_int_parse` |
| 9 | `func` | `to_float` |
| 13 | `func` | `to_text` |
| 17 | `func` | `parse` |

#### `stdlib/std/io.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 7 | `namespace` | `std.io` |
| 9 | `struct` | `Input` |
| 13 | `struct` | `Output` |
| 17 | `enum` | `Error` |
| 23 | `const` | `input` |
| 24 | `const` | `output` |
| 25 | `const` | `stderr` |
| 28 | `func` | `zt_host_read_line_stdin` |
| 29 | `func` | `zt_host_read_all_stdin` |
| 30 | `func` | `zt_host_write_stdout` |
| 31 | `func` | `zt_host_write_stderr` |
| 34 | `func` | `map_host_error` |
| 53 | `func` | `to_core_error` |
| 64 | `func` | `read_line` |
| 73 | `func` | `read_all` |
| 82 | `func` | `write` |
| 100 | `func` | `print` |

#### `stdlib/std/json.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.json` |
| 6 | `func` | `zt_json_parse_map_text_text` |
| 7 | `func` | `zt_json_stringify_map_text_text` |
| 8 | `func` | `zt_json_pretty_map_text_text` |
| 9 | `func` | `zt_host_read_file` |
| 10 | `func` | `zt_host_write_file` |
| 13 | `func` | `parse` |
| 17 | `func` | `stringify` |
| 21 | `func` | `pretty` |
| 25 | `func` | `read` |
| 26 | `const` | `raw` |
| 30 | `func` | `write` |
| 31 | `const` | `raw` |

#### `stdlib/std/lazy.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.lazy` |
| 6 | `func` | `zt_lazy_i64_once` |
| 7 | `func` | `zt_lazy_i64_force` |
| 8 | `func` | `zt_lazy_i64_is_consumed` |
| 11 | `func` | `once_int` |
| 15 | `func` | `force_int` |
| 19 | `func` | `is_consumed_int` |

#### `stdlib/std/list.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `namespace` | `std.list` |

#### `stdlib/std/map.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `namespace` | `std.map` |

#### `stdlib/std/math.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 10 | `namespace` | `std.math` |
| 13 | `func` | `zt_math_pow` |
| 14 | `func` | `zt_math_sqrt` |
| 15 | `func` | `zt_math_floor` |
| 16 | `func` | `zt_math_ceil` |
| 17 | `func` | `zt_math_round_half_away_from_zero` |
| 18 | `func` | `zt_math_trunc` |
| 19 | `func` | `zt_math_sin` |
| 20 | `func` | `zt_math_cos` |
| 21 | `func` | `zt_math_tan` |
| 22 | `func` | `zt_math_asin` |
| 23 | `func` | `zt_math_acos` |
| 24 | `func` | `zt_math_atan` |
| 25 | `func` | `zt_math_atan2` |
| 26 | `func` | `zt_math_ln` |
| 27 | `func` | `zt_math_log_ten` |
| 28 | `func` | `zt_math_log2` |
| 29 | `func` | `zt_math_log` |
| 30 | `func` | `zt_math_exp` |
| 31 | `func` | `zt_math_is_nan` |
| 32 | `func` | `zt_math_is_infinite` |
| 33 | `func` | `zt_math_is_finite` |
| 36 | `const` | `pi` |
| 37 | `const` | `e` |
| 38 | `const` | `tau` |
| 40 | `func` | `infinity` |
| 41 | `const` | `zero` |
| 45 | `func` | `nan` |
| 46 | `const` | `zero` |
| 50 | `func` | `abs` |
| 57 | `func` | `min` |
| 64 | `func` | `max` |
| 71 | `func` | `clamp` |
| 81 | `func` | `pow` |
| 85 | `func` | `sqrt` |
| 89 | `func` | `floor` |
| 93 | `func` | `ceil` |
| 97 | `func` | `round` |
| 101 | `func` | `trunc` |
| 105 | `func` | `deg_to_rad` |
| 109 | `func` | `rad_to_deg` |
| 113 | `func` | `approx_equal` |
| 121 | `func` | `sin` |
| 125 | `func` | `cos` |
| 129 | `func` | `tan` |
| 133 | `func` | `asin` |
| 137 | `func` | `acos` |
| 141 | `func` | `atan` |
| 145 | `func` | `atan2` |
| 149 | `func` | `ln` |
| 153 | `func` | `log_ten` |
| 157 | `func` | `log2` |
| 161 | `func` | `log` |
| 165 | `func` | `exp` |
| 169 | `func` | `is_nan` |
| 173 | `func` | `is_infinite` |
| 177 | `func` | `is_finite` |

#### `stdlib/std/net.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `namespace` | `std.net` |
| 3 | `enum` | `Error` |
| 17 | `struct` | `Connection` |
| 21 | `func` | `zt_net_connect` |
| 22 | `func` | `zt_net_read_some` |
| 23 | `func` | `zt_net_write_all` |
| 24 | `func` | `zt_net_close` |
| 25 | `func` | `zt_net_is_closed` |
| 26 | `func` | `zt_net_error_kind_index` |
| 29 | `func` | `timeout_to_ms` |
| 38 | `func` | `map_core_error` |
| 39 | `const` | `kind_index` |
| 75 | `func` | `to_core_error` |
| 102 | `func` | `connect` |
| 115 | `func` | `read_some` |
| 128 | `func` | `write_all` |
| 141 | `func` | `close` |
| 150 | `func` | `is_closed` |
| 154 | `func` | `kind` |

#### `stdlib/std/os.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 8 | `namespace` | `std.os` |
| 10 | `enum` | `Platform` |
| 17 | `enum` | `Arch` |
| 24 | `enum` | `Error` |
| 32 | `func` | `zt_host_os_args` |
| 33 | `func` | `zt_host_os_env` |
| 34 | `func` | `zt_host_os_pid` |
| 35 | `func` | `zt_host_os_platform` |
| 36 | `func` | `zt_host_os_arch` |
| 37 | `func` | `zt_host_os_current_dir_core` |
| 38 | `func` | `zt_host_os_change_dir_core` |
| 41 | `func` | `map_host_error` |
| 57 | `func` | `args` |
| 61 | `func` | `env` |
| 65 | `func` | `pid` |
| 69 | `func` | `platform` |
| 70 | `const` | `raw` |
| 83 | `func` | `arch` |
| 84 | `const` | `raw` |
| 97 | `func` | `current_dir` |
| 106 | `func` | `change_dir` |

#### `stdlib/std/os/process.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 6 | `namespace` | `std.os.process` |
| 8 | `struct` | `ExitStatus` |
| 12 | `struct` | `CapturedRun` |
| 18 | `enum` | `Error` |
| 27 | `func` | `zt_host_process_run_core` |
| 28 | `func` | `zt_host_process_run_capture_core` |
| 31 | `func` | `map_host_error` |
| 51 | `func` | `run` |
| 60 | `func` | `run_capture` |

#### `stdlib/std/random.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 4 | `namespace` | `std.random` |
| 6 | `struct` | `Stats` |
| 17 | `func` | `zt_host_random_seed` |
| 18 | `func` | `zt_host_random_next_i64` |
| 21 | `func` | `seed` |
| 28 | `func` | `next` |
| 29 | `const` | `value` |
| 34 | `func` | `between` |
| 36 | `const` | `err` |
| 44 | `const` | `span` |
| 45 | `const` | `raw` |
| 46 | `const` | `offset` |
| 51 | `func` | `stats` |

#### `stdlib/std/regex.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.regex` |
| 5 | `enum` | `Error` |
| 9 | `struct` | `Regex` |
| 14 | `func` | `zt_regex_validate_core` |
| 15 | `func` | `zt_regex_is_match_core` |
| 16 | `func` | `zt_regex_full_match_core` |
| 17 | `func` | `zt_regex_first_core` |
| 18 | `func` | `zt_regex_count_core` |
| 19 | `func` | `zt_regex_find_all_core` |
| 20 | `func` | `zt_regex_split_core` |
| 21 | `func` | `zt_regex_replace_all_core` |
| 22 | `func` | `zt_regex_escape_core` |
| 25 | `func` | `map_core_error` |
| 33 | `func` | `compile` |
| 42 | `func` | `is_valid` |
| 51 | `func` | `is_match` |
| 55 | `func` | `contains` |
| 59 | `func` | `matches` |
| 68 | `func` | `full_match` |
| 77 | `func` | `first` |
| 81 | `func` | `count` |
| 85 | `func` | `find_all` |
| 89 | `func` | `split` |
| 93 | `func` | `replace_all` |
| 97 | `func` | `escape` |

#### `stdlib/std/set.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `namespace` | `std.set` |

#### `stdlib/std/test.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.test` |
| 6 | `func` | `zt_text_concat` |
| 9 | `func` | `_join3` |
| 10 | `const` | `left` |
| 14 | `func` | `_join4` |
| 15 | `const` | `left` |
| 19 | `func` | `zt_test_fail` |
| 20 | `const` | `_message` |
| 24 | `func` | `zt_test_skip` |
| 25 | `const` | `_reason` |
| 29 | `func` | `zt_test_throws_closure` |
| 30 | `const` | `_body` |
| 34 | `func` | `fail` |
| 38 | `func` | `skip` |
| 42 | `func` | `throws` |
| 50 | `func` | `is_true` |
| 58 | `func` | `is_false` |
| 66 | `func` | `equal_int` |
| 68 | `const` | `expected_text` |
| 69 | `const` | `actual_text` |
| 70 | `const` | `message` |
| 77 | `func` | `equal_text` |
| 79 | `const` | `message` |
| 86 | `func` | `not_equal_int` |
| 88 | `const` | `expected_text` |
| 89 | `const` | `actual_text` |
| 90 | `const` | `message` |
| 97 | `func` | `not_equal_text` |
| 99 | `const` | `message` |

#### `stdlib/std/text.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 2 | `namespace` | `std.text` |
| 5 | `func` | `zt_text_to_utf8_bytes` |
| 6 | `func` | `zt_text_from_utf8_bytes` |
| 7 | `func` | `zt_text_eq` |
| 8 | `func` | `zt_text_concat` |
| 11 | `func` | `_eq` |
| 15 | `func` | `_slice_from` |
| 28 | `func` | `_slice_to` |
| 41 | `func` | `_is_whitespace_char` |
| 48 | `func` | `_is_ascii_digit_char` |
| 58 | `func` | `_starts_at` |
| 81 | `func` | `to_utf8` |
| 85 | `func` | `from_utf8` |
| 89 | `func` | `trim` |
| 93 | `func` | `trim_start` |
| 106 | `func` | `trim_end` |
| 119 | `func` | `contains` |
| 123 | `func` | `join` |
| 135 | `func` | `replace_all` |
| 156 | `func` | `starts_with` |
| 160 | `func` | `ends_with` |
| 170 | `func` | `has_prefix` |
| 174 | `func` | `has_suffix` |
| 178 | `func` | `has_whitespace` |
| 191 | `func` | `index_of` |
| 193 | `const` | `max_index` |
| 212 | `func` | `last_index_of` |
| 232 | `func` | `is_empty` |
| 236 | `func` | `is_blank` |
| 240 | `func` | `is_digits` |
| 257 | `func` | `limit` |

#### `stdlib/std/time.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 9 | `namespace` | `std.time` |
| 11 | `struct` | `Instant` |
| 15 | `struct` | `Duration` |
| 20 | `func` | `zt_host_time_now_unix_ms` |
| 21 | `func` | `zt_host_time_sleep_ms` |
| 24 | `func` | `now` |
| 28 | `func` | `now_ms` |
| 32 | `func` | `sleep` |
| 36 | `func` | `sleep_ms` |
| 40 | `func` | `since` |
| 44 | `func` | `until` |
| 48 | `func` | `elapsed` |
| 49 | `const` | `span` |
| 53 | `func` | `diff` |
| 57 | `func` | `add` |
| 61 | `func` | `sub` |
| 65 | `func` | `from_unix` |
| 69 | `func` | `from_unix_ms` |
| 73 | `func` | `to_unix` |
| 77 | `func` | `to_unix_ms` |
| 81 | `func` | `milliseconds` |
| 85 | `func` | `seconds` |
| 89 | `func` | `minutes` |
| 93 | `func` | `hours` |

#### `stdlib/std/validate.zt`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 3 | `namespace` | `std.validate` |
| 5 | `func` | `between` |
| 9 | `func` | `positive` |
| 13 | `func` | `non_negative` |
| 17 | `func` | `negative` |
| 21 | `func` | `non_zero` |
| 25 | `func` | `one_of` |
| 34 | `func` | `one_of_text` |
| 43 | `func` | `not_empty` |
| 47 | `func` | `not_empty_text` |
| 51 | `func` | `min_length` |
| 55 | `func` | `min_len` |
| 59 | `func` | `max_length` |
| 63 | `func` | `max_len` |
| 67 | `func` | `length_between` |
| 68 | `const` | `count` |

#### `stdlib/zdoc/std/bool.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| - | - | No symbols extracted |

#### `stdlib/zdoc/std/bytes.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 8 | `doc_target` | `empty` |
| 18 | `doc_target` | `from_list` |
| 30 | `doc_target` | `to_list` |
| 42 | `doc_target` | `join` |
| 54 | `doc_target` | `starts_with` |
| 66 | `doc_target` | `ends_with` |
| 78 | `doc_target` | `contains` |

#### `stdlib/zdoc/std/collections.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 8 | `doc_target` | `QueueNumberDequeueResult` |
| 19 | `doc_target` | `QueueTextDequeueResult` |
| 30 | `doc_target` | `StackNumberPopResult` |
| 41 | `doc_target` | `StackTextPopResult` |
| 52 | `doc_target` | `queue_int_new` |
| 61 | `doc_target` | `queue_text_new` |
| 70 | `doc_target` | `queue_int_enqueue` |
| 79 | `doc_target` | `queue_text_enqueue` |
| 88 | `doc_target` | `queue_int_dequeue` |
| 97 | `doc_target` | `queue_text_dequeue` |
| 106 | `doc_target` | `queue_int_peek` |
| 117 | `doc_target` | `queue_text_peek` |
| 128 | `doc_target` | `stack_int_new` |
| 137 | `doc_target` | `stack_text_new` |
| 146 | `doc_target` | `stack_int_push` |
| 155 | `doc_target` | `stack_text_push` |
| 164 | `doc_target` | `stack_int_pop` |
| 173 | `doc_target` | `stack_text_pop` |
| 182 | `doc_target` | `stack_int_peek` |
| 193 | `doc_target` | `stack_text_peek` |
| 204 | `doc_target` | `grid2d_int_new` |
| 213 | `doc_target` | `grid2d_text_new` |
| 222 | `doc_target` | `grid2d_int_get` |
| 231 | `doc_target` | `grid2d_text_get` |
| 240 | `doc_target` | `grid2d_int_set` |
| 249 | `doc_target` | `grid2d_text_set` |
| 258 | `doc_target` | `grid2d_int_fill` |
| 267 | `doc_target` | `grid2d_text_fill` |
| 276 | `doc_target` | `grid2d_int_rows` |
| 285 | `doc_target` | `grid2d_text_rows` |
| 294 | `doc_target` | `grid2d_int_cols` |
| 303 | `doc_target` | `grid2d_text_cols` |
| 312 | `doc_target` | `grid2d_int_size` |
| 323 | `doc_target` | `grid2d_text_size` |
| 334 | `doc_target` | `grid3d_int_new` |
| 343 | `doc_target` | `grid3d_text_new` |
| 352 | `doc_target` | `grid3d_int_get` |
| 361 | `doc_target` | `grid3d_text_get` |
| 370 | `doc_target` | `grid3d_int_set` |
| 379 | `doc_target` | `grid3d_text_set` |
| 388 | `doc_target` | `grid3d_int_fill` |
| 397 | `doc_target` | `grid3d_text_fill` |
| 406 | `doc_target` | `grid3d_int_depth` |
| 415 | `doc_target` | `grid3d_text_depth` |
| 424 | `doc_target` | `grid3d_int_rows` |
| 439 | `doc_target` | `cols` |
| 454 | `doc_target` | `size e grid3d_text_rows` |
| 469 | `doc_target` | `cols` |
| 484 | `doc_target` | `size` |
| 499 | `doc_target` | `pqueue_int_new` |
| 508 | `doc_target` | `pqueue_text_new` |
| 517 | `doc_target` | `pqueue_int_push` |
| 526 | `doc_target` | `pqueue_text_push` |
| 535 | `doc_target` | `pqueue_int_pop` |
| 544 | `doc_target` | `pqueue_text_pop` |
| 553 | `doc_target` | `pqueue_int_peek` |
| 562 | `doc_target` | `pqueue_text_peek` |
| 571 | `doc_target` | `pqueue_int_len` |
| 580 | `doc_target` | `pqueue_text_len` |
| 589 | `doc_target` | `pqueue_int_is_empty` |
| 600 | `doc_target` | `pqueue_text_is_empty` |
| 611 | `doc_target` | `circbuf_int_new` |
| 620 | `doc_target` | `circbuf_text_new` |
| 629 | `doc_target` | `circbuf_int_push` |
| 638 | `doc_target` | `circbuf_text_push` |
| 647 | `doc_target` | `circbuf_int_pop` |
| 656 | `doc_target` | `circbuf_text_pop` |
| 665 | `doc_target` | `circbuf_int_peek` |
| 674 | `doc_target` | `circbuf_text_peek` |
| 683 | `doc_target` | `circbuf_int_len` |
| 692 | `doc_target` | `circbuf_text_len` |
| 701 | `doc_target` | `circbuf_int_capacity` |
| 710 | `doc_target` | `circbuf_text_capacity` |
| 719 | `doc_target` | `circbuf_int_is_full` |
| 728 | `doc_target` | `circbuf_text_is_full` |
| 737 | `doc_target` | `circbuf_int_is_empty` |
| 748 | `doc_target` | `circbuf_text_is_empty` |
| 759 | `doc_target` | `btreemap_text_new` |
| 767 | `doc_target` | `btreemap_text_set` |
| 775 | `doc_target` | `btreemap_text_get` |
| 783 | `doc_target` | `btreemap_text_get_optional` |
| 791 | `doc_target` | `btreemap_text_contains` |
| 799 | `doc_target` | `btreemap_text_remove` |
| 807 | `doc_target` | `btreemap_text_len` |
| 815 | `doc_target` | `btreemap_text_is_empty` |
| 825 | `doc_target` | `btreeset_text_new` |
| 833 | `doc_target` | `btreeset_text_insert` |
| 841 | `doc_target` | `btreeset_text_contains` |
| 849 | `doc_target` | `btreeset_text_remove` |
| 857 | `doc_target` | `btreeset_text_len` |
| 865 | `doc_target` | `btreeset_text_is_empty` |

#### `stdlib/zdoc/std/concurrent.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `copy_int` |
| 18 | `doc_target` | `copy_bool` |
| 26 | `doc_target` | `copy_float` |
| 34 | `doc_target` | `copy_text` |
| 42 | `doc_target` | `copy_bytes` |
| 50 | `doc_target` | `copy_list_int` |
| 58 | `doc_target` | `copy_list_text` |
| 66 | `doc_target` | `copy_map_text_text` |

#### `stdlib/zdoc/std/console.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 11 | `doc_target` | `write_line` |
| 19 | `doc_target` | `error_line` |
| 27 | `doc_target` | `pause` |
| 37 | `doc_target` | `prompt` |
| 46 | `doc_target` | `confirm` |
| 56 | `doc_target` | `Size` |
| 68 | `doc_target` | `is_terminal` |
| 76 | `doc_target` | `size` |
| 84 | `doc_target` | `columns` |
| 93 | `doc_target` | `rows` |
| 102 | `doc_target` | `clear` |
| 111 | `doc_target` | `color` |
| 120 | `doc_target` | `style` |
| 129 | `doc_target` | `reset_style` |
| 137 | `doc_target` | `read_key` |

#### `stdlib/zdoc/std/debug.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| - | - | No symbols extracted |

#### `stdlib/zdoc/std/float.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| - | - | No symbols extracted |

#### `stdlib/zdoc/std/format.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `BytesStyle` |
| 21 | `doc_target` | `number` |
| 33 | `doc_target` | `percent` |
| 46 | `doc_target` | `date` |
| 58 | `doc_target` | `datetime` |
| 72 | `doc_target` | `date_pattern` |
| 84 | `doc_target` | `datetime_pattern` |
| 96 | `doc_target` | `bytes` |
| 109 | `doc_target` | `hex` |
| 120 | `doc_target` | `bin` |

#### `stdlib/zdoc/std/fs.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `Error` |
| 26 | `doc_target` | `Metadata` |
| 41 | `doc_target` | `read_text` |
| 49 | `doc_target` | `write_text` |
| 57 | `doc_target` | `append_text` |
| 65 | `doc_target` | `exists` |
| 73 | `doc_target` | `is_file` |
| 81 | `doc_target` | `is_dir` |
| 89 | `doc_target` | `create_dir` |
| 97 | `doc_target` | `create_dir_all` |
| 105 | `doc_target` | `list_dir` |
| 113 | `doc_target` | `remove_file` |
| 121 | `doc_target` | `remove_dir` |
| 129 | `doc_target` | `remove_dir_all` |
| 137 | `doc_target` | `copy_file` |
| 145 | `doc_target` | `copy` |
| 153 | `doc_target` | `move` |
| 161 | `doc_target` | `rename` |
| 169 | `doc_target` | `metadata` |
| 177 | `doc_target` | `size` |
| 185 | `doc_target` | `file_size` |
| 193 | `doc_target` | `modified_at` |
| 201 | `doc_target` | `created_at` |

#### `stdlib/zdoc/std/fs/path.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `join` |
| 20 | `doc_target` | `base_name` |
| 31 | `doc_target` | `name_without_extension` |
| 39 | `doc_target` | `extension` |
| 50 | `doc_target` | `parent` |
| 61 | `doc_target` | `normalize` |
| 69 | `doc_target` | `absolute` |
| 82 | `doc_target` | `relative` |
| 90 | `doc_target` | `is_absolute` |
| 98 | `doc_target` | `is_relative` |
| 106 | `doc_target` | `has_extension` |
| 119 | `doc_target` | `change_extension` |

#### `stdlib/zdoc/std/http.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 13 | `doc_target` | `ErrorKind` |
| 26 | `doc_target` | `Error` |
| 37 | `doc_target` | `Response` |
| 50 | `doc_target` | `get` |
| 61 | `doc_target` | `post` |

#### `stdlib/zdoc/std/int.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| - | - | No symbols extracted |

#### `stdlib/zdoc/std/io.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 10 | `doc_target` | `Input` |
| 21 | `doc_target` | `Output` |
| 32 | `doc_target` | `Error` |
| 44 | `doc_target` | `input` |
| 52 | `doc_target` | `output` |
| 60 | `doc_target` | `stderr` |
| 68 | `doc_target` | `read_line` |
| 81 | `doc_target` | `read_all` |
| 92 | `doc_target` | `write` |
| 105 | `doc_target` | `print` |
| 118 | `doc_target` | `to_core_error` |

#### `stdlib/zdoc/std/json.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 17 | `doc_target` | `parse` |
| 28 | `doc_target` | `stringify` |
| 36 | `doc_target` | `pretty` |
| 46 | `doc_target` | `read` |
| 54 | `doc_target` | `write` |

#### `stdlib/zdoc/std/list.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |

#### `stdlib/zdoc/std/map.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |

#### `stdlib/zdoc/std/math.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 8 | `doc_target` | `pi` |
| 16 | `doc_target` | `e` |
| 24 | `doc_target` | `tau` |
| 32 | `doc_target` | `infinity` |
| 40 | `doc_target` | `nan` |
| 48 | `doc_target` | `abs` |
| 57 | `doc_target` | `min` |
| 65 | `doc_target` | `max` |
| 73 | `doc_target` | `clamp` |
| 86 | `doc_target` | `pow` |
| 94 | `doc_target` | `sqrt` |
| 102 | `doc_target` | `floor` |
| 110 | `doc_target` | `ceil` |
| 118 | `doc_target` | `round` |
| 126 | `doc_target` | `trunc` |
| 134 | `doc_target` | `deg_to_rad` |
| 142 | `doc_target` | `rad_to_deg` |
| 150 | `doc_target` | `approx_equal` |
| 163 | `doc_target` | `sin` |
| 171 | `doc_target` | `cos` |
| 179 | `doc_target` | `tan` |
| 187 | `doc_target` | `asin` |
| 195 | `doc_target` | `acos` |
| 203 | `doc_target` | `atan` |
| 211 | `doc_target` | `atan2` |
| 223 | `doc_target` | `ln` |
| 231 | `doc_target` | `log_ten` |
| 239 | `doc_target` | `log2` |
| 247 | `doc_target` | `log` |
| 259 | `doc_target` | `exp` |
| 267 | `doc_target` | `is_nan` |
| 275 | `doc_target` | `is_infinite` |
| 283 | `doc_target` | `is_finite` |

#### `stdlib/zdoc/std/net.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 14 | `doc_target` | `Error` |
| 34 | `doc_target` | `Connection` |
| 43 | `doc_target` | `connect` |
| 60 | `doc_target` | `read_some` |
| 78 | `doc_target` | `write_all` |
| 95 | `doc_target` | `close` |
| 106 | `doc_target` | `is_closed` |
| 114 | `doc_target` | `kind` |
| 122 | `doc_target` | `to_core_error` |

#### `stdlib/zdoc/std/os.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 8 | `doc_target` | `Platform` |
| 22 | `doc_target` | `Arch` |
| 36 | `doc_target` | `Error` |
| 49 | `doc_target` | `args` |
| 57 | `doc_target` | `env` |
| 69 | `doc_target` | `pid` |
| 77 | `doc_target` | `platform` |
| 85 | `doc_target` | `arch` |
| 93 | `doc_target` | `current_dir` |
| 101 | `doc_target` | `change_dir` |

#### `stdlib/zdoc/std/os/process.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `ExitStatus` |
| 20 | `doc_target` | `CapturedRun` |
| 32 | `doc_target` | `Error` |
| 47 | `doc_target` | `run` |
| 61 | `doc_target` | `run_capture` |

#### `stdlib/zdoc/std/random.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 8 | `doc_target` | `seed` |
| 19 | `doc_target` | `next` |
| 29 | `doc_target` | `between` |

#### `stdlib/zdoc/std/regex.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 17 | `doc_target` | `Error` |
| 27 | `doc_target` | `Regex` |
| 38 | `doc_target` | `compile` |
| 48 | `doc_target` | `is_match` |
| 57 | `doc_target` | `is_valid` |
| 66 | `doc_target` | `contains` |
| 75 | `doc_target` | `matches` |
| 84 | `doc_target` | `full_match` |
| 93 | `doc_target` | `first` |
| 102 | `doc_target` | `count` |
| 111 | `doc_target` | `find_all` |
| 120 | `doc_target` | `split` |
| 129 | `doc_target` | `replace_all` |
| 138 | `doc_target` | `escape` |

#### `stdlib/zdoc/std/test.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 10 | `doc_target` | `fail` |
| 21 | `doc_target` | `skip` |
| 32 | `doc_target` | `throws` |
| 43 | `doc_target` | `is_true` |
| 52 | `doc_target` | `is_false` |
| 61 | `doc_target` | `equal_int` |
| 70 | `doc_target` | `equal_text` |
| 79 | `doc_target` | `not_equal_int` |
| 88 | `doc_target` | `not_equal_text` |

#### `stdlib/zdoc/std/text.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `to_utf8` |
| 17 | `doc_target` | `from_utf8` |
| 26 | `doc_target` | `trim` |
| 34 | `doc_target` | `trim_start` |
| 42 | `doc_target` | `trim_end` |
| 50 | `doc_target` | `contains` |
| 58 | `doc_target` | `starts_with` |
| 66 | `doc_target` | `ends_with` |
| 74 | `doc_target` | `has_prefix` |
| 82 | `doc_target` | `has_suffix` |
| 90 | `doc_target` | `has_whitespace` |
| 98 | `doc_target` | `index_of` |
| 106 | `doc_target` | `last_index_of` |
| 114 | `doc_target` | `is_empty` |
| 122 | `doc_target` | `is_blank` |
| 130 | `doc_target` | `is_digits` |
| 138 | `doc_target` | `limit` |

#### `stdlib/zdoc/std/time.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 9 | `doc_target` | `Instant` |
| 19 | `doc_target` | `Duration` |
| 29 | `doc_target` | `now` |
| 37 | `doc_target` | `now_ms` |
| 45 | `doc_target` | `sleep` |
| 56 | `doc_target` | `sleep_ms` |
| 64 | `doc_target` | `since` |
| 72 | `doc_target` | `until` |
| 80 | `doc_target` | `diff` |
| 88 | `doc_target` | `elapsed` |
| 96 | `doc_target` | `add` |
| 104 | `doc_target` | `sub` |
| 112 | `doc_target` | `from_unix` |
| 120 | `doc_target` | `from_unix_ms` |
| 128 | `doc_target` | `to_unix` |
| 136 | `doc_target` | `to_unix_ms` |
| 144 | `doc_target` | `milliseconds` |
| 152 | `doc_target` | `seconds` |
| 160 | `doc_target` | `minutes` |
| 168 | `doc_target` | `hours` |

#### `stdlib/zdoc/std/validate.zdoc`

| Line | Kind | Symbol |
| ---: | --- | --- |
| 1 | `doc_target` | `namespace` |
| 8 | `doc_target` | `between` |
| 16 | `doc_target` | `positive` |
| 24 | `doc_target` | `non_negative` |
| 32 | `doc_target` | `negative` |
| 40 | `doc_target` | `non_zero` |
| 48 | `doc_target` | `one_of` |
| 56 | `doc_target` | `one_of_text` |
| 64 | `doc_target` | `not_empty` |
| 72 | `doc_target` | `not_empty_text` |
| 80 | `doc_target` | `min_length` |
| 88 | `doc_target` | `min_len` |
| 96 | `doc_target` | `max_length` |
| 104 | `doc_target` | `max_len` |
| 112 | `doc_target` | `length_between` |

### Manual Notes

- Critical flow:
- Break conditions:
- Related docs or decisions:
- Extra test cases worth adding:
<!-- CODEMAP:GENERATED:END -->
