# Implementation Cascade

## Objetivo

Fornecer trilha de execucao historica entre checklist v1, roadmap v1 e code maps.

## Escopo

- Navegacao de milestones M0-M38.
- Apoio para auditoria e manutencao retroativa.

## Dependencias

- Upstream:
  - `docs/internal/planning/checklist-v1.md`
  - `docs/internal/planning/roadmap-v1.md`
- Downstream:
  - auditorias em `docs/internal/reports/*`
  - comparacao com cascade v2
- Codigo/Testes relacionados:
  - `compiler/*_MAP.md`
  - `tests/*`

## Como usar

Use este documento apenas para investigacao historica. Para execucao atual, use `docs/internal/planning/cascade-v2.md`.

Este arquivo liga `docs/internal/planning/checklist-v1.md`, `docs/internal/planning/roadmap-v1.md` e os `*_MAP.md`.

Uso recomendado:

1. Ache o milestone no checklist ou no roadmap.
2. Abra a secao correspondente aqui.
3. Siga a ordem checklist -> roadmap -> mapas -> codigo -> testes.
4. Use os simbolos e linhas sugeridos como janela inicial, nao como verdade absoluta.

Regra pratica:

- checklist = item exato e status
- roadmap = objetivo e escopo
- code maps = arquivo, simbolo e linha de entrada

## M0. Congelar contratos

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 44-51
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 31-49
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/zir/ZIR_MODEL_MAP.md`
8. Map: `compiler/zir/ZIR_PARSER_MAP.md`
9. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
10. Map: `runtime/c/RUNTIME_MAP.md`
11. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 5
- Open items: 0

### Roadmap Summary

- Objetivo: parar churn antes de abrir o frontend.
- Dependencias: nenhuma.
- Paralelo possivel:
  - nao.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_normalize_path_separators` (func_def, line 373), `zt_decode_process_exit` (func_def, line 2701), `zt_path_filter_list` (struct, line 57), `zt_apply_manifest_lang` (func_def, line 65), `zt_read_file` (func_def, line 81), `zt_write_file` (func_def, line 129), `zt_copy_text` (func_def, line 146), `zt_heap_strdup` (func_def, line 156)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser_error_contextual` (func_def, line 153), `zt_parser_is_contextual_ident` (func_def, line 244), `zt_parser_parse_params` (func_def, line 781), `zt_parser_parse_type_params` (func_def, line 826), `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51), `zt_path_char_equal` (func_def, line 61)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_parse_dependency_spec` (func_def, line 343), `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57), `zir_add_lower_diag` (func_def, line 63), `zir_lower_strdup` (func_def, line 76)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +35 more
- Supporting docs:
  - `docs/spec/language/surface-syntax.md`

### Prompt Seed

```text
Implement M0 (Congelar contratos).
Start from docs/internal/planning/checklist-v1.md lines 44-51.
Then read docs/internal/planning/roadmap-v1.md lines 31-49.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M1. Lexer

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 52-68
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 50-81
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/lexer/LEXER_MAP.md`
5. Map: `runtime/c/RUNTIME_MAP.md`
6. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 14
- Open items: 0

### Roadmap Summary

- Objetivo: transformar `.zt` em uma stream de tokens com spans confiaveis.
- Dependencias: `M0`.
- Paralelo possivel:
  - desenho inicial da AST

### Entry Files

- `compiler/frontend/lexer/lexer.c`
  - Closest map: `compiler/frontend/lexer/LEXER_MAP.md`
  - Start near: `zt_lexer_make` (func_def, line 79), `zt_lexer_set_diagnostics` (func_def, line 94), `zt_lexer_dispose` (func_def, line 100), `zt_lexer_peek` (func_def, line 106), `zt_lexer_peek_next` (func_def, line 111), `zt_lexer_advance` (func_def, line 116), `zt_lexer_skip_trivia` (func_def, line 134), `zt_lexer_lookup_keyword` (func_def, line 147)
- `compiler/frontend/lexer/lexer.h`
  - Closest map: `compiler/frontend/lexer/LEXER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_LEXER_LEXER_H` (macro, line 2)
- `compiler/frontend/lexer/token.c`
  - Closest map: `compiler/frontend/lexer/LEXER_MAP.md`
  - Start near: `zt_token_kind_name` (func_def, line 3), `zt_token_kind_is_keyword` (func_def, line 95), `zt_token_kind_is_literal` (func_def, line 99), `zt_source_span_make` (func_def, line 103), `zt_source_span_unknown` (func_def, line 112)
- `compiler/frontend/lexer/token.h`
  - Closest map: `compiler/frontend/lexer/LEXER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_LEXER_TOKEN_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +29 more

### Prompt Seed

```text
Implement M1 (Lexer).
Start from docs/internal/planning/checklist-v1.md lines 52-68.
Then read docs/internal/planning/roadmap-v1.md lines 50-81.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/frontend/lexer/lexer.c
- compiler/frontend/lexer/lexer.h
- compiler/frontend/lexer/token.c
- compiler/frontend/lexer/token.h
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M2. AST e parser sintatico

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 69-89
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 82-119
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/project/PROJECT_MAP.md`
6. Map: `compiler/semantic/binder/BINDER_MAP.md`
7. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
8. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
9. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`

### Checklist Status

- Done items: 18
- Open items: 0

### Roadmap Summary

- Objetivo: transformar `.zt` em AST com diagnosticos sintaticos decentes.
- Dependencias: `M1`.
- Paralelo possivel:
  - AST e parser podem evoluir juntos

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_import_local_name` (func_def, line 115), `zt_bind_match_pattern` (func_def, line 305), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_bind_where_clause` (func_def, line 475), `zt_binder` (struct, line 5), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_import_entry` (struct, line 33), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_function_context` (struct, line 77), `zt_import_list_push` (func_def, line 200), `zt_catalog_has_import_alias` (func_def, line 246), `zt_catalog_import_path_for_alias` (func_def, line 255), `zt_apply_list_push` (func_def, line 269)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`

### Prompt Seed

```text
Implement M2 (AST e parser sintatico).
Start from docs/internal/planning/checklist-v1.md lines 69-89.
Then read docs/internal/planning/roadmap-v1.md lines 82-119.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/semantic/binder/binder.c
- compiler/semantic/binder/binder.h
Validate with these tests first:
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_binder.c
- tests/semantic/test_constraints.c
- tests/semantic/test_types.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M3. Diagnostics e binder

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 90-107
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 120-152
3. Map: `compiler/project/PROJECT_MAP.md`
4. Map: `compiler/semantic/binder/BINDER_MAP.md`
5. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
6. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`

### Checklist Status

- Done items: 15
- Open items: 0

### Roadmap Summary

- Objetivo: resolver nomes e escopos antes da tipagem completa.
- Dependencias: `M2`.
- Paralelo possivel:
  - diagnostics e symbols

### Entry Files

- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_import_local_name` (func_def, line 115), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83), `zt_bind_declare_name` (func_def, line 122), `zt_bind_simple_type_name` (func_def, line 144)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_same_span` (func_def, line 18), `zt_diag_print_source_span` (func_def, line 49), `zt_diag_same_text` (func_def, line 12), `zt_diag_is_duplicate_recent` (func_def, line 25), `zt_diag_read_source_line` (func_def, line 63), `zt_diag_render_caret` (func_def, line 103), `zt_diag_code_name` (func_def, line 118), `zt_diag_code_stable` (func_def, line 177)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/semantic/test_binder.c`

### Prompt Seed

```text
Implement M3 (Diagnostics e binder).
Start from docs/internal/planning/checklist-v1.md lines 90-107.
Then read docs/internal/planning/roadmap-v1.md lines 120-152.
Then open these maps:
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
Then inspect these source files first:
- compiler/semantic/binder/binder.c
- compiler/semantic/binder/binder.h
- compiler/semantic/diagnostics/diagnostics.c
- compiler/semantic/diagnostics/diagnostics.h
- compiler/semantic/symbols/symbols.c
- compiler/semantic/symbols/symbols.h
Validate with these tests first:
- tests/semantic/test_binder.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M4. Sistema de tipos e validacao semantica

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 108-130
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 153-189
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`

### Checklist Status

- Done items: 20
- Open items: 0

### Roadmap Summary

- Objetivo: fazer valer as decisoes 001-027 no compilador.
- Dependencias: `M3`.
- Paralelo possivel:
  - validadores especificos por subsistema

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_builtin_type_by_name` (func_def, line 394), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524), `zt_type_is_core_trait_name` (func_def, line 549)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`

### Prompt Seed

```text
Implement M4 (Sistema de tipos e validacao semantica).
Start from docs/internal/planning/checklist-v1.md lines 108-130.
Then read docs/internal/planning/roadmap-v1.md lines 153-189.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
Validate with these tests first:
- tests/semantic/test_constraints.c
- tests/semantic/test_types.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M5. HIR

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 131-145
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 190-217
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
6. Map: `compiler/semantic/binder/BINDER_MAP.md`
7. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
8. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
9. Map: `compiler/zir/ZIR_MODEL_MAP.md`
10. Map: `compiler/zir/ZIR_PARSER_MAP.md`
11. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Objetivo: produzir uma arvore resolvida e tipada, sem acucar sintatico.
- Dependencias: `M4`.
- Paralelo possivel:
  - pouco.

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_named_arg_list_make` (func_def, line 131), `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser_match` (func_def, line 69), `zt_is_named_arg_label_token` (func_def, line 404), `zt_is_named_arg_ahead` (func_def, line 408), `zt_parser_parse_params` (func_def, line 781), `zt_parser_parse_type_params` (func_def, line 826), `zt_parser_parse_generic_constraints` (func_def, line 857), `zt_parser_parse_match_stmt` (func_def, line 980), `zt_parser_parse_apply_decl` (func_def, line 1318)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/lowering/from_ast.c`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `zt_func_param_meta` (struct, line 32), `zt_lower_type_from_ast` (func_def, line 260), `zt_push_func_param_meta` (func_def, line 330), `zt_build_apply_name` (func_def, line 436), `zt_collect_apply_symbols` (func_def, line 488), `zt_lower_call_args` (func_def, line 581), `zt_lower_apply_decl` (func_def, line 1850), `zt_scope_binding` (struct, line 7)
- `compiler/hir/lowering/from_ast.h`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_LOWERING_FROM_AST_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_param_list_make` (func_def, line 108), `zt_hir_match_case_list_make` (func_def, line 114), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_match_case_list_push` (func_def, line 232), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_match_case_list_dispose` (func_def, line 443), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_apply_list` (struct, line 50), `zt_apply_list_push` (func_def, line 269), `zt_decl_type_param_count` (func_def, line 383), `zt_expr_matches_integral_type` (func_def, line 516), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_register_type_params` (func_def, line 580), `zt_checker_bind_struct_field_type_params` (func_def, line 957), `zt_checker_find_apply_method` (func_def, line 1016)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110), `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_name_matches` (func_def, line 118), `zir_call_add_lowered_args` (func_def, line 332), `zir_match_single_pattern_condition` (func_def, line 1460), `zir_match_case_condition` (func_def, line 1520), `zir_emit_match_case_bindings` (func_def, line 1541), `zir_lower_match_statement` (func_def, line 1572), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M5 (HIR).
Start from docs/internal/planning/checklist-v1.md lines 131-145.
Then read docs/internal/planning/roadmap-v1.md lines 190-217.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/lowering/from_ast.c
- compiler/hir/lowering/from_ast.h
Validate with these tests first:
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_constraints.c
- tests/semantic/test_hir_lowering.c
- tests/semantic/test_types.c
- tests/zir/test_lowering.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M6. ZIR v2 estruturada

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 146-164
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 218-252
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/zir/ZIR_MODEL_MAP.md`
6. Map: `compiler/zir/ZIR_PARSER_MAP.md`
7. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 16
- Open items: 0

### Roadmap Summary

- Objetivo: remover a dependencia de `expr_text` como contrato principal.
- Dependencias: `M5`.
- Paralelo possivel:
  - printer e verifier, depois que o modelo estabilizar

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57), `zir_add_lower_diag` (func_def, line 63), `zir_lower_strdup` (func_def, line 76)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)
- `compiler/zir/model.c`
  - Closest map: `compiler/zir/ZIR_MODEL_MAP.md`
  - Start near: `zir_make_param` (func_def, line 762), `zir_string_buffer` (struct, line 8), `zir_strdup_owned` (func_def, line 14), `zir_free_owned_nonempty` (func_def, line 25), `zir_grow_array` (func_def, line 31), `zir_string_buffer_init` (func_def, line 41), `zir_string_buffer_reserve` (func_def, line 47), `zir_string_buffer_append_n` (func_def, line 63)
- `compiler/zir/model.h`
  - Closest map: `compiler/zir/ZIR_MODEL_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_MODEL_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M6 (ZIR v2 estruturada).
Start from docs/internal/planning/checklist-v1.md lines 146-164.
Then read docs/internal/planning/roadmap-v1.md lines 218-252.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/zir/lowering/from_hir.c
- compiler/zir/lowering/from_hir.h
Validate with these tests first:
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/zir/test_lowering.c
- tests/zir/test_printer.c
- tests/zir/test_verifier.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M7. Adaptar backend C

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 165-177
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 253-279
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
6. Map: `compiler/targets/c/EMITTER_MAP.md`
7. Map: `compiler/zir/ZIR_MODEL_MAP.md`
8. Map: `compiler/zir/ZIR_PARSER_MAP.md`
9. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
10. Map: `runtime/c/RUNTIME_MAP.md`

### Checklist Status

- Done items: 10
- Open items: 0

### Roadmap Summary

- Objetivo: religar emitter/legalization a ZIR nova e a linguagem nova.
- Dependencias: `M6`.
- Paralelo possivel:
  - emitter e legalization, com ownership claro de arquivos

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_base_name` (func_def, line 379), `zt_decl_type_param_count` (func_def, line 383), `zt_parse_signed_literal_value` (func_def, line 444), `zt_parse_double_literal_value` (func_def, line 462), `zt_type_is_core_trait_name` (func_def, line 549), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_make_core_constraint_type` (func_def, line 574), `zt_checker_register_type_params` (func_def, line 580)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209), `c_emitter_text` (func_def, line 216)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_safe_text` (func_def, line 8), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_type_name_is_optional` (func_def, line 150), `zir_lower_result_dispose` (func_def, line 2512), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/runtime/c/test_runtime.c`
  - `tests/runtime/c/test_shared_text.c`
  - `tests/runtime/test_fase11_safety.zt`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M7 (Adaptar backend C).
Start from docs/internal/planning/checklist-v1.md lines 165-177.
Then read docs/internal/planning/roadmap-v1.md lines 253-279.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
- compiler/targets/c/emitter.c
Validate with these tests first:
- tests/runtime/c/test_runtime.c
- tests/runtime/c/test_shared_text.c
- tests/runtime/test_fase11_safety.zt
- tests/semantic/test_constraints.c
- tests/semantic/test_types.c
- tests/targets/c/test_emitter.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M8. Driver real e zenith.ztproj

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 178-194
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 280-307
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
7. Map: `compiler/project/PROJECT_MAP.md`
8. Map: `compiler/zir/ZIR_MODEL_MAP.md`
9. Map: `compiler/zir/ZIR_PARSER_MAP.md`
10. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 14
- Open items: 0

### Roadmap Summary

- Objetivo: compilar um programa Zenith de ponta a ponta.
- Dependencias: `M7`.
- Paralelo possivel:
  - parser de `zenith.ztproj` pode nascer antes, sem integrar ainda

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_find_project_root_from_cwd` (func_def, line 260), `zt_project_source_file` (struct, line 459), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477), `zt_project_source_file_list_push` (func_def, line 503), `zt_project_discover_zt_files` (func_def, line 539), `zt_find_entry_root` (func_def, line 687)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_module_make` (func_def, line 71), `zt_hir_decl_make` (func_def, line 79), `zt_hir_stmt_make` (func_def, line 87), `zt_hir_expr_make` (func_def, line 95)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_collect_source_symbols` (func_def, line 530), `zt_make_paired_source_path` (func_def, line 758), `zt_zdoc_check_project` (func_def, line 1101), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_span_from_source` (func_def, line 154), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57), `zir_add_lower_diag` (func_def, line 63)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M8 (Driver real e zenith.ztproj).
Start from docs/internal/planning/checklist-v1.md lines 178-194.
Then read docs/internal/planning/roadmap-v1.md lines 280-307.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_hir_lowering.c
- tests/zir/test_lowering.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M9. Conformance e migracao

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 195-203
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 308-335
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/project/PROJECT_MAP.md`
5. Map: `compiler/zir/ZIR_MODEL_MAP.md`
6. Map: `compiler/zir/ZIR_PARSER_MAP.md`
7. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 6
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte MVP atual.
- Objetivo: provar que a trilha nova substitui a atual para o MVP.
- Dependencias: `M8`.
- Paralelo possivel:
  - novos behavior tests
  - ampliacao dos golden tests

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_normalize_path_separators` (func_def, line 373), `zt_diag_code_from_project_error` (func_def, line 1078), `zt_print_project_parse_error` (func_def, line 1097), `zt_diag_code_from_zir_parse_error` (func_def, line 1269), `zt_diag_code_from_c_emit_error` (func_def, line 1309), `zt_should_skip_test_discovery_dir` (func_def, line 1511), `zt_collect_test_projects` (func_def, line 1529), `zt_file_has_test_marker` (func_def, line 1623)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_diagnostic_error_count` (func_def, line 639), `zt_resolve_target` (func_def, line 780), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_set_error` (func_def, line 45), `zt_project_error_code_name` (func_def, line 678), `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_loop_target` (struct, line 32), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57), `zir_add_lower_diag` (func_def, line 63), `zir_lower_strdup` (func_def, line 76)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/conformance/test_m9.c`
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M9 (Conformance e migracao).
Start from docs/internal/planning/checklist-v1.md lines 195-203.
Then read docs/internal/planning/roadmap-v1.md lines 308-335.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/project/zdoc.c
- compiler/project/zdoc.h
- compiler/project/ztproj.c
- compiler/project/ztproj.h
- compiler/zir/lowering/from_hir.c
Validate with these tests first:
- tests/conformance/test_m9.c
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/zir/test_lowering.c
- tests/zir/test_printer.c
- tests/zir/test_verifier.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M10. Controle de fluxo executavel

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 204-226
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 336-375
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/targets/c/EMITTER_MAP.md`
6. Map: `compiler/zir/ZIR_MODEL_MAP.md`
7. Map: `compiler/zir/ZIR_PARSER_MAP.md`
8. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
9. Map: `runtime/c/RUNTIME_MAP.md`
10. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 20
- Open items: 0

### Roadmap Summary

- Objetivo: fazer os blocos de controle ja aceitos pelo frontend chegarem ate C
- Dependencias: `M9`.
- Paralelo possivel:
  - casos de behavior e golden C podem ser preparados enquanto o lowering evolui

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser_match` (func_def, line 69), `zt_parser_hex_digit_value` (func_def, line 255), `zt_parser_parse_params` (func_def, line 781), `zt_parser_parse_type_params` (func_def, line 826), `zt_parser_parse_while_stmt` (func_def, line 924), `zt_parser_parse_repeat_stmt` (func_def, line 964), `zt_parser_parse_match_stmt` (func_def, line 980), `zt_parser` (struct, line 9)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209), `c_emitter_text` (func_def, line 216), `c_type_is_builtin_managed_value` (func_def, line 677), `c_hex_digit_value` (func_def, line 835)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_name_matches` (func_def, line 118), `zir_match_single_pattern_condition` (func_def, line 1460), `zir_match_case_condition` (func_def, line 1520), `zir_emit_match_case_bindings` (func_def, line 1541), `zir_lower_match_statement` (func_def, line 1572), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)
- `compiler/zir/model.c`
  - Closest map: `compiler/zir/ZIR_MODEL_MAP.md`
  - Start near: `zir_expr_make_outcome_value` (func_def, line 482), `zir_expr_make_list_add_item` (func_def, line 511), `zir_make_param` (func_def, line 762), `zir_string_buffer` (struct, line 8), `zir_strdup_owned` (func_def, line 14), `zir_free_owned_nonempty` (func_def, line 25), `zir_grow_array` (func_def, line 31), `zir_string_buffer_init` (func_def, line 41)
- `compiler/zir/model.h`
  - Closest map: `compiler/zir/ZIR_MODEL_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_MODEL_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +36 more

### Prompt Seed

```text
Implement M10 (Controle de fluxo executavel).
Start from docs/internal/planning/checklist-v1.md lines 204-226.
Then read docs/internal/planning/roadmap-v1.md lines 336-375.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/targets/c/emitter.c
- compiler/targets/c/emitter.h
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M11. Funcoes, chamadas e entrypoints completos

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 227-247
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 376-412
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
5. Map: `compiler/project/PROJECT_MAP.md`
6. Map: `compiler/semantic/binder/BINDER_MAP.md`
7. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
8. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
9. Map: `compiler/targets/c/EMITTER_MAP.md`
10. Map: `compiler/zir/ZIR_MODEL_MAP.md`
11. Map: `compiler/zir/ZIR_PARSER_MAP.md`
12. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
13. Map: `runtime/c/RUNTIME_MAP.md`

### Checklist Status

- Done items: 18
- Open items: 0

### Roadmap Summary

- Objetivo: tornar chamadas de funcoes de usuario confiaveis no caminho real
- Dependencias: `M10`.
- Paralelo possivel:
  - diagnosticos negativos e golden C

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_copy_text` (func_def, line 146), `zt_normalize_path_separators` (func_def, line 373), `zt_print_diagnostics` (func_def, line 1170), `zt_append_zdoc_diagnostics` (func_def, line 1335), `zt_token_is_identifier_text` (func_def, line 1615), `zt_collect_attr_tests` (func_def, line 1766), `zt_run_attr_tests_for_project` (func_def, line 1883), `main` (func_def, line 3859)
- `compiler/hir/lowering/from_ast.c`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `zt_func_param_meta` (struct, line 32), `zt_text_eq` (func_def, line 99), `zt_lower_type_from_ast` (func_def, line 260), `zt_push_func_param_meta` (func_def, line 330), `zt_scope_binding` (struct, line 7), `zt_scope` (struct, line 12), `zt_struct_field_meta` (struct, line 19), `zt_struct_meta` (struct, line 25)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_module_make` (func_def, line 71)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_copy_text` (func_def, line 51), `zt_read_file_text` (func_def, line 141), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_safe_text` (func_def, line 24), `zt_project_read_file_text` (func_def, line 67), `zt_project_parse_bool_value` (func_def, line 442), `zt_project_copy_default` (func_def, line 559), `zt_project_parse_text` (func_def, line 697), `zt_project_section` (enum, line 9), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_function_context` (struct, line 77), `zt_checker_strdup` (func_def, line 84), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_decl_type_param_count` (func_def, line 383), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_is_float` (func_def, line 269), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_safe_text` (func_def, line 79), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209), `c_emitter_text` (func_def, line 216), `c_emit_trimmed_text` (func_def, line 788)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_text_eq` (func_def, line 93), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57), `zir_add_lower_diag` (func_def, line 63)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/runtime/c/test_runtime.c`
  - `tests/runtime/c/test_shared_text.c`
  - `tests/runtime/test_fase11_safety.zt`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M11 (Funcoes, chamadas e entrypoints completos).
Start from docs/internal/planning/checklist-v1.md lines 227-247.
Then read docs/internal/planning/roadmap-v1.md lines 376-412.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/hir/lowering/from_ast.c
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
- compiler/project/zdoc.c
- compiler/project/zdoc.h
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/runtime/c/test_runtime.c
- tests/runtime/c/test_shared_text.c
- tests/runtime/test_fase11_safety.zt
- tests/semantic/test_constraints.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M12. Structs, campos, metodos e apply

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 248-268
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 413-452
3. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
4. Map: `compiler/project/PROJECT_MAP.md`
5. Map: `compiler/semantic/binder/BINDER_MAP.md`
6. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
7. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
8. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
9. Map: `compiler/targets/c/EMITTER_MAP.md`
10. Map: `compiler/zir/ZIR_MODEL_MAP.md`
11. Map: `compiler/zir/ZIR_PARSER_MAP.md`
12. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 18
- Open items: 0

### Roadmap Summary

- Objetivo: fazer composicao por `struct`, `trait` e `apply` funcionar
- Dependencias: `M11`.
- Paralelo possivel:
  - emitter C de structs e validacao semantica de mutacao

### Entry Files

- `compiler/hir/lowering/from_ast.c`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `zt_struct_field_meta` (struct, line 19), `zt_struct_meta` (struct, line 25), `zt_func_param_meta` (struct, line 32), `zt_unknown_type` (func_def, line 117), `zt_lower_type_from_generic` (func_def, line 206), `zt_lower_type_from_ast` (func_def, line 260), `zt_push_struct_meta` (func_def, line 275), `zt_push_struct_field_meta` (func_def, line 319)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_param_list_make` (func_def, line 108), `zt_hir_field_decl_list_make` (func_def, line 109), `zt_hir_enum_variant_field_list_make` (func_def, line 110), `zt_hir_field_init_list_make` (func_def, line 112), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_field_decl_list_push` (func_def, line 171), `zt_hir_enum_variant_field_list_push` (func_def, line 182), `zt_hir_field_init_list_push` (func_def, line 210)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_is_builtin_type_name` (func_def, line 83), `zt_bind_simple_type_name` (func_def, line 144), `zt_bind_type_node` (func_def, line 174), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_apply_list` (struct, line 50), `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_apply_list_push` (func_def, line 269), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_type_base_name` (func_def, line 379), `zt_decl_type_param_count` (func_def, line 383)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_canonicalize_type` (func_def, line 48), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209), `c_emitter_text` (func_def, line 216), `c_find_user_struct` (func_def, line 259)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_find_symbol_type` (func_def, line 193), `c_legalize_resolve_sequence_type` (func_def, line 225), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_resolve_zir_sequence_type` (func_def, line 565), `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_instruction_buffer` (struct, line 12), `zir_type_name_is_optional` (func_def, line 150), `zir_type_name_owned` (func_def, line 158), `zir_instruction_buffer_init` (func_def, line 165), `zir_instruction_buffer_push` (func_def, line 186), `zir_emit_instruction` (func_def, line 226), `zir_find_struct_decl_hir` (func_def, line 251), `zir_find_struct_field_hir` (func_def, line 262)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M12 (Structs, campos, metodos e apply).
Start from docs/internal/planning/checklist-v1.md lines 248-268.
Then read docs/internal/planning/roadmap-v1.md lines 413-452.
Then open these maps:
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/hir/lowering/from_ast.c
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
- compiler/semantic/binder/binder.c
- compiler/semantic/binder/binder.h
- compiler/semantic/parameter_validation.c
Validate with these tests first:
- tests/semantic/test_binder.c
- tests/semantic/test_constraints.c
- tests/semantic/test_hir_lowering.c
- tests/semantic/test_types.c
- tests/targets/c/test_emitter.c
- tests/targets/c/test_legalization.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M13. Collections, optional/result e ownership no runtime C

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 269-293
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 453-507
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
6. Map: `compiler/targets/c/EMITTER_MAP.md`
7. Map: `compiler/zir/ZIR_MODEL_MAP.md`
8. Map: `compiler/zir/ZIR_PARSER_MAP.md`
9. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
10. Map: `runtime/c/RUNTIME_MAP.md`
11. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 22
- Open items: 0

### Roadmap Summary

- Objetivo: fechar os tipos compostos centrais do MVP no caminho real
- Dependencias: `M12`.
- Paralelo possivel:
  - runtime C e lowering ZIR dos tipos compostos

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 26), `zt_validate_argument_ordering` (func_def, line 67)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_list` (struct, line 38), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_checker` (struct, line 62), `zt_function_context` (struct, line 77), `zt_checker_strdup` (func_def, line 84), `zt_decl_list_push` (func_def, line 184), `zt_import_list_push` (func_def, line 200)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_kind_name` (func_def, line 19), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_safe_text` (func_def, line 79), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emit_error_code_name` (func_def, line 107), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_slice_seq` (func_def, line 377), `c_legalize_list_len_expr` (func_def, line 473), `c_legalize_zir_list_len_expr` (func_def, line 695)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_text_eq` (func_def, line 93), `zir_type_name_is_optional` (func_def, line 150), `zir_try_lower_constructor_contract_assignment` (func_def, line 1237), `zir_find_construct_field` (func_def, line 1444), `zir_pattern_is_enum_construct` (func_def, line 1456), `zir_lower_result_dispose` (func_def, line 2512), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)
- `runtime/c/zenith_rt.c`
  - Closest map: `runtime/c/RUNTIME_MAP.md`
  - Start near: `zt_hash_text` (func_def, line 40), `zt_runtime_append_text` (func_def, line 53), `zt_runtime_store_error` (func_def, line 73), `zt_runtime_stable_code` (func_def, line 90), `zt_runtime_default_help` (func_def, line 107), `zt_runtime_print_error` (func_def, line 136), `zt_outcome_i64_text_failure_message` (func_def, line 171), `zt_outcome_text_text_failure_message` (func_def, line 178)
- `runtime/c/zenith_rt.h`
  - Closest map: `runtime/c/RUNTIME_MAP.md`
  - Start near: `ZENITH_NEXT_RUNTIME_C_ZENITH_RT_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +39 more

### Prompt Seed

```text
Implement M13 (Collections, optional/result e ownership no runtime C).
Start from docs/internal/planning/checklist-v1.md lines 269-293.
Then read docs/internal/planning/roadmap-v1.md lines 453-507.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
- compiler/targets/c/emitter.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M14. Projeto multi-arquivo, namespaces e imports reais

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 294-317
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 508-554
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
10. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
11. Map: `compiler/targets/c/EMITTER_MAP.md`
12. Map: `compiler/zir/ZIR_MODEL_MAP.md`
13. Map: `compiler/zir/ZIR_PARSER_MAP.md`
14. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
15. Map: `runtime/c/RUNTIME_MAP.md`
16. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 21
- Open items: 0

### Roadmap Summary

- Objetivo: sair do entrypoint unico e compilar projetos Zenith com multiplos
- Dependencias: `M13`.
- Paralelo possivel:
  - loader de projeto e binder multi-arquivo

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_path_has_extension` (func_def, line 168), `zt_path_is_dir` (func_def, line 176), `zt_path_is_file` (func_def, line 188), `zt_join_path` (func_def, line 200), `zt_find_project_root_from_cwd` (func_def, line 260), `zt_path_filter_list_dispose` (func_def, line 334), `zt_path_filter_list_push` (func_def, line 347)
- `compiler/hir/lowering/from_ast.c`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `zt_func_param_meta` (struct, line 32), `zt_const_meta` (struct, line 53), `zt_lower_type_from_ast` (func_def, line 260), `zt_push_const_meta` (func_def, line 308), `zt_push_func_param_meta` (func_def, line 330), `zt_find_const_meta` (func_def, line 427), `zt_collect_const_symbol` (func_def, line 512), `zt_expr_to_path` (func_def, line 543)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_module_make` (func_def, line 71), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_map_entry_list_make` (func_def, line 113), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_map_entry_list_push` (func_def, line 221), `zt_hir_map_entry_list_dispose` (func_def, line 308), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_module_dispose` (func_def, line 489)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_path_char_equal` (func_def, line 61), `zt_join_path` (func_def, line 70), `zt_path_is_dir` (func_def, line 96), `zt_path_has_extension` (func_def, line 108), `zt_strip_prefix_path` (func_def, line 116), `zt_zdoc_public_symbol_equal` (func_def, line 363)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_import_local_name` (func_def, line 115), `zt_bind_generic_constraints` (func_def, line 193), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_entry` (struct, line 28), `zt_import_entry` (struct, line 33), `zt_import_list` (struct, line 44), `zt_module_catalog` (struct, line 56), `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_import_list_push` (func_def, line 200), `zt_catalog_has_import_alias` (func_def, line 246)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209), `c_emitter_text` (func_def, line 216), `c_module_requires_string_header` (func_def, line 1293), `c_emit_main_wrapper` (func_def, line 6691)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_call_is_module_func` (func_def, line 123), `zir_span_from_source` (func_def, line 154), `zir_try_lower_constructor_contract_assignment` (func_def, line 1237), `zir_find_construct_field` (func_def, line 1444), `zir_pattern_is_enum_construct` (func_def, line 1456), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +40 more

### Prompt Seed

```text
Implement M14 (Projeto multi-arquivo, namespaces e imports reais).
Start from docs/internal/planning/checklist-v1.md lines 294-317.
Then read docs/internal/planning/roadmap-v1.md lines 508-554.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/hir/lowering/from_ast.c
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
- compiler/project/zdoc.c
- compiler/project/zdoc.h
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M15. Fechamento semantico da surface syntax MVP

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 318-339
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 555-601
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
10. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
11. Map: `compiler/targets/c/EMITTER_MAP.md`
12. Map: `compiler/zir/ZIR_MODEL_MAP.md`
13. Map: `compiler/zir/ZIR_PARSER_MAP.md`
14. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 19
- Open items: 0

### Roadmap Summary

- Objetivo: alinhar parser, binder, typechecker e HIR com a spec de surface
- Dependencias: `M14`.
- Paralelo possivel:
  - ampliacao de fixtures e melhorias de diagnostics

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/lowering/from_ast.c`
  - Closest map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - Start near: `zt_func_param_meta` (struct, line 32), `zt_lower_type_from_ast` (func_def, line 260), `zt_push_func_param_meta` (func_def, line 330), `zt_expr_to_path` (func_def, line 543), `zt_lower_function_decl_core` (func_def, line 1770), `zt_scope_binding` (struct, line 7), `zt_scope` (struct, line 12), `zt_struct_field_meta` (struct, line 19)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_param_list_make` (func_def, line 108), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_realloc_array` (func_def, line 61), `zt_hir_module_make` (func_def, line 71)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_bind_generic_constraints` (func_def, line 193), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_bind_where_clause` (func_def, line 475), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_catalog_import_path_for_alias` (func_def, line 255), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_decl_type_param_count` (func_def, line 383), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_is_unsigned_integral` (func_def, line 256), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emit_error_code_name` (func_def, line 107), `c_expression_is_materialized_core_error_ref` (func_def, line 976), `c_outcome_spec` (struct, line 1186), `c_outcome_spec_for_type` (func_def, line 1206), `c_outcome_spec_for_expr` (func_def, line 1262), `c_outcome_spec_for_expected` (func_def, line 1280), `c_outcome_emit_failure_from_error_expr` (func_def, line 1380), `c_contract_value_format_kind` (enum, line 5499)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_error_code_name` (func_def, line 36), `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_emit_check_contract` (func_def, line 1195), `zir_try_lower_constructor_contract_assignment` (func_def, line 1237), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37), `zir_decl_counts` (struct, line 57)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`
- Supporting docs:
  - `docs/spec/language/surface-syntax.md`

### Prompt Seed

```text
Implement M15 (Fechamento semantico da surface syntax MVP).
Start from docs/internal/planning/checklist-v1.md lines 318-339.
Then read docs/internal/planning/roadmap-v1.md lines 555-601.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/lowering/from_ast.c
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_binder.c
- tests/semantic/test_constraints.c
- tests/semantic/test_hir_lowering.c
- tests/semantic/test_types.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M16. Conformance final frontend/backend

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 340-357
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 602-646
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/lexer/LEXER_MAP.md`
6. Map: `compiler/frontend/parser/PARSER_MAP.md`
7. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
8. Map: `compiler/project/PROJECT_MAP.md`
9. Map: `compiler/targets/c/EMITTER_MAP.md`
10. Map: `compiler/zir/ZIR_MODEL_MAP.md`
11. Map: `compiler/zir/ZIR_PARSER_MAP.md`
12. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
13. Map: `runtime/c/RUNTIME_MAP.md`
14. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 15
- Open items: 0

### Roadmap Summary

- Objetivo: declarar a cobertura frontend/backend da linguagem MVP como completa
- Dependencias: `M15`.
- Paralelo possivel:
  - golden diagnostics e behavior tests por feature

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_normalize_path_separators` (func_def, line 373), `zt_validate_source_namespaces` (func_def, line 903), `zt_parse_project_sources` (func_def, line 962), `zt_build_combined_project_ast` (func_def, line 989), `zt_print_project_parse_error` (func_def, line 1097), `zt_print_diagnostics` (func_def, line 1170), `zt_diag_code_from_zir_parse_error` (func_def, line 1269), `zt_append_zdoc_diagnostics` (func_def, line 1335)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_param_list_make` (func_def, line 108), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_realloc_array` (func_def, line 61), `zt_hir_module_make` (func_def, line 71)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_resolve_target` (func_def, line 780), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_parse_string_value` (func_def, line 166), `zt_project_parse_positive_size_value` (func_def, line 227), `zt_project_parse_section` (func_def, line 288), `zt_project_parse_dependency_spec` (func_def, line 343), `zt_project_assign_dependency` (func_def, line 389), `zt_project_parse_bool_value` (func_def, line 442), `zt_project_validate` (func_def, line 590), `zt_project_parse_result_init` (func_def, line 670)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_build_function_symbol` (func_def, line 297), `c_build_struct_symbol` (func_def, line 306), `c_build_enum_symbol` (func_def, line 314), `c_build_block_label` (func_def, line 348), `c_parse_outcome_type_name` (func_def, line 621), `c_build_generated_outcome_symbol` (func_def, line 670), `c_parse_binary` (func_def, line 1458), `c_emit_runtime_span_literal` (func_def, line 5483)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_block_state_buffer_init` (func_def, line 170), `zir_block_state_buffer_push` (func_def, line 198), `zir_current_block` (func_def, line 211), `zir_add_block` (func_def, line 216), `zir_dispose_block_states` (func_def, line 2171)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/MATRIX.md`
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - ... +44 more

### Prompt Seed

```text
Implement M16 (Conformance final frontend/backend).
Start from docs/internal/planning/checklist-v1.md lines 340-357.
Then read docs/internal/planning/roadmap-v1.md lines 602-646.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/behavior/MATRIX.md
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M17. Consolidacao dos specs canonicos

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 358-369
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 647-666
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `runtime/c/RUNTIME_MAP.md`
8. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 9
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido como documentacao normativa.
- Objetivo: transformar as decisoes aceitas em specs canonicos de leitura e implementacao.
- Dependencias: decisoes 001-056.
- Paralelo possivel:
  - nao, pois consolida a base para as proximas fases.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_find_project_root_from_cwd` (func_def, line 260), `zt_normalize_path_separators` (func_def, line 373), `zt_project_source_file` (struct, line 459), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477), `zt_project_source_file_list_push` (func_def, line 503), `zt_project_discover_zt_files` (func_def, line 539)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser_parse_params` (func_def, line 781), `zt_parser_parse_type_params` (func_def, line 826), `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_check_project` (func_def, line 1101), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +32 more
- Supporting docs:
  - `docs/spec/language/README.md`
  - `docs/spec/language/compiler-model.md`
  - `docs/spec/language/project-model.md`
  - `docs/spec/language/surface-syntax.md`
  - `docs/spec/language/tooling-model.md`

### Prompt Seed

```text
Implement M17 (Consolidacao dos specs canonicos).
Start from docs/internal/planning/checklist-v1.md lines 358-369.
Then read docs/internal/planning/roadmap-v1.md lines 647-666.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M18. Alinhar implementacao ao manifesto e tooling final

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 370-384
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 667-699
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o manifesto canonico e o driver bootstrap atual.
- Objetivo: alinhar o driver/projeto atual ao spec consolidado de `zenith.ztproj` e CLI.
- Dependencias: M17.
- Paralelo possivel:
  - preparacao de fixtures
  - atualizacao de docs do driver

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_find_project_root_from_cwd` (func_def, line 260), `zt_normalize_path_separators` (func_def, line 373), `zt_project_source_file` (struct, line 459), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477), `zt_project_source_file_list_push` (func_def, line 503), `zt_project_discover_zt_files` (func_def, line 539)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_print_source_span` (func_def, line 49), `zt_diag_read_source_line` (func_def, line 63), `zt_diag_same_text` (func_def, line 12), `zt_diag_same_span` (func_def, line 18), `zt_diag_is_duplicate_recent` (func_def, line 25), `zt_diag_render_caret` (func_def, line 103), `zt_diag_code_name` (func_def, line 118), `zt_diag_code_stable` (func_def, line 177)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/test_diagnostics.zt`
- Supporting docs:
  - `docs/spec/language/tooling-model.md`

### Prompt Seed

```text
Implement M18 (Alinhar implementacao ao manifesto e tooling final).
Start from docs/internal/planning/checklist-v1.md lines 370-384.
Then read docs/internal/planning/roadmap-v1.md lines 667-699.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/test_diagnostics.zt
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M19. Stdlib MVP

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 385-407
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 700-735
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/lexer/LEXER_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
7. Map: `compiler/targets/c/EMITTER_MAP.md`
8. Map: `compiler/zir/ZIR_MODEL_MAP.md`
9. Map: `compiler/zir/ZIR_PARSER_MAP.md`
10. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
11. Map: `runtime/c/RUNTIME_MAP.md`
12. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 20
- Open items: 0

### Roadmap Summary

- Status roadmap: em andamento (subset bytes/UTF-8 do M19 concluido).
- Objetivo: definir e implementar a primeira stdlib real sobre o runtime C e a surface consolidada.
- Dependencias: M17; idealmente M18 para evitar retrabalho em manifestos e comandos.

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131), `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_string_list_make` (func_def, line 104), `zt_hir_decl_list_make` (func_def, line 105), `zt_hir_stmt_list_make` (func_def, line 106), `zt_hir_expr_list_make` (func_def, line 107), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_field_decl_list_make` (func_def, line 109), `zt_hir_enum_variant_field_list_make` (func_def, line 110)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_safe_text` (func_def, line 79), `c_emit_error_code_name` (func_def, line 107), `c_buffer_append_vformat` (func_def, line 164), `c_buffer_append_format` (func_def, line 185), `c_emitter_text` (func_def, line 216), `c_emit_trimmed_text` (func_def, line 788), `c_emit_c_string_literal` (func_def, line 798), `c_emit_hex_bytes_literal` (func_def, line 842)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_starts_with` (func_def, line 68), `c_legalize_list_len_expr` (func_def, line 473), `c_legalize_zir_list_len_expr` (func_def, line 695), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_is_blank` (func_def, line 51)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_lower_format` (func_def, line 87), `zir_text_eq` (func_def, line 93), `zir_starts_with` (func_def, line 143), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +40 more

### Prompt Seed

```text
Implement M19 (Stdlib MVP).
Start from docs/internal/planning/checklist-v1.md lines 385-407.
Then read docs/internal/planning/roadmap-v1.md lines 700-735.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M20. ZDoc e ZPM tooling inicial

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 408-416
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 736-760
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
10. Map: `runtime/c/RUNTIME_MAP.md`
11. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 6
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte inicial bootstrap.
- Objetivo: implementar validacao inicial de ZDoc e o esqueleto local de ZPM sem registry remoto.
- Dependencias: M18 e parte da stdlib/tooling.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_find_project_root_from_cwd` (func_def, line 260), `zt_normalize_path_separators` (func_def, line 373), `zt_project_source_file` (struct, line 459), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477), `zt_project_source_file_list_push` (func_def, line 503), `zt_project_discover_zt_files` (func_def, line 539)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_decl_type_param_count` (func_def, line 383), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524), `zt_type_param_has_trait` (func_def, line 556)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +34 more
- Supporting docs:
  - `docs/spec/language/lockfile-schema.md`

### Prompt Seed

```text
Implement M20 (ZDoc e ZPM tooling inicial).
Start from docs/internal/planning/checklist-v1.md lines 408-416.
Then read docs/internal/planning/roadmap-v1.md lines 736-760.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M21. Fechamento dos specs finais

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 417-436
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 761-790
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/semantic/binder/BINDER_MAP.md`
6. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
7. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
8. Map: `compiler/targets/c/EMITTER_MAP.md`
9. Map: `compiler/tooling/FORMATTER_MAP.md`
10. Map: `runtime/c/RUNTIME_MAP.md`
11. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 17
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido.
- Objetivo: eliminar ambiguidades entre decisions historicas, specs canonicos e status real de implementacao.
- Dependencias: M18.

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser_parse_params` (func_def, line 781), `zt_parser_parse_type_params` (func_def, line 826), `zt_parser_parse_generic_constraint` (func_def, line 845), `zt_parser_parse_generic_constraints` (func_def, line 857), `zt_parser_result_dispose` (func_def, line 1731), `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_type_param_count` (func_def, line 383), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_make_core_constraint_type` (func_def, line 574), `zt_checker_register_type_params` (func_def, line 580), `zt_checker_bind_struct_field_type_params` (func_def, line 957), `zt_checker_check_where_clause` (func_def, line 2628), `zt_checker_check_generic_constraints` (func_def, line 2643), `zt_checker_seed_module_const_bindings` (func_def, line 2707)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_expression_is_materialized_outcome_void_text_ref` (func_def, line 1074), `c_outcome_spec` (struct, line 1186), `c_outcome_spec_for_type` (func_def, line 1206), `c_outcome_spec_for_expr` (func_def, line 1262), `c_outcome_spec_for_expected` (func_def, line 1280), `c_copy_legalize_result` (func_def, line 1696)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_safe_text` (func_def, line 8), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/tooling/formatter.c`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `sb_t` (struct, line 7), `sb_init` (func_def, line 14), `sb_free` (func_def, line 22), `sb_append_len` (func_def, line 26), `sb_append` (func_def, line 41), `sb_indent` (func_def, line 45), `format_comments` (func_def, line 53), `op_to_str` (func_def, line 62)
- `compiler/tooling/formatter.h`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `ZENITH_TOOLING_FORMATTER_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +38 more
- Supporting docs:
  - `docs/spec/language/README.md`
  - `docs/spec/language/backend-scalability-risk-model.md`
  - `docs/spec/language/diagnostics-model.md`
  - `docs/spec/language/formatter-model.md`
  - `docs/spec/language/implementation-status.md`
  - `docs/spec/language/runtime-model.md`
  - `docs/spec/language/stdlib-model.md`
  - `docs/spec/language/surface-syntax.md`

### Prompt Seed

```text
Implement M21 (Fechamento dos specs finais).
Start from docs/internal/planning/checklist-v1.md lines 417-436.
Then read docs/internal/planning/roadmap-v1.md lines 761-790.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/tooling/FORMATTER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M22. Formatter obrigatorio

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 437-452
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 791-809
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
10. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
11. Map: `compiler/tooling/FORMATTER_MAP.md`

### Checklist Status

- Done items: 13
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido.
- Objetivo: implementar `zt fmt` como parte central da filosofia reading-first.
- Dependencias: M21.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_namespace_to_relative_path` (func_def, line 627), `zt_path_suffix_matches` (func_def, line 656), `zt_last_namespace_segment` (func_def, line 680), `zt_namespace_exists` (func_def, line 702), `zt_namespace_is_std` (func_def, line 706), `zt_find_namespace_index` (func_def, line 710), `zt_alias_for_namespace` (func_def, line 831), `zt_prefix_declaration_for_namespace` (func_def, line 876)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_named_arg_list_make` (func_def, line 131), `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51), `zt_path_char_equal` (func_def, line 61)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_manifest_entry_namespace` (func_def, line 583), `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_bind_match_pattern` (func_def, line 305), `zt_bind_where_clause` (func_def, line 475), `zt_binder` (struct, line 5), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83), `zt_import_local_name` (func_def, line 115)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_expr_matches_integral_type` (func_def, line 516), `zt_checker_check_where_clause` (func_def, line 2628), `zt_binding_kind` (enum, line 9), `zt_binding` (struct, line 14), `zt_binding_scope` (struct, line 21), `zt_decl_entry` (struct, line 28), `zt_import_entry` (struct, line 33), `zt_decl_list` (struct, line 38)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110), `zt_type_format_inner` (func_def, line 169), `zt_type_format` (func_def, line 216), `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/tooling/formatter.c`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `format_comments` (func_def, line 53), `format_node_list_comma` (func_def, line 83), `format_node` (func_def, line 90), `zt_format_node_to_string` (func_def, line 581), `sb_t` (struct, line 7), `sb_init` (func_def, line 14), `sb_free` (func_def, line 22), `sb_append_len` (func_def, line 26)
- `compiler/tooling/formatter.h`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `ZENITH_TOOLING_FORMATTER_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/formatter/run_formatter_golden.py`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`

### Prompt Seed

```text
Implement M22 (Formatter obrigatorio).
Start from docs/internal/planning/checklist-v1.md lines 437-452.
Then read docs/internal/planning/roadmap-v1.md lines 791-809.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/tooling/FORMATTER_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/formatter/run_formatter_golden.py
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_binder.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M23. Diagnostics renderer real

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 453-467
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 810-838
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/lexer/LEXER_MAP.md`
6. Map: `compiler/frontend/parser/PARSER_MAP.md`
7. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
8. Map: `compiler/project/PROJECT_MAP.md`
9. Map: `compiler/semantic/binder/BINDER_MAP.md`
10. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
11. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
12. Map: `compiler/targets/c/EMITTER_MAP.md`
13. Map: `compiler/zir/ZIR_MODEL_MAP.md`
14. Map: `compiler/zir/ZIR_PARSER_MAP.md`
15. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
16. Map: `runtime/c/RUNTIME_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido.
- Objetivo: transformar o modelo das Decisions 038/039 e `diagnostics-model.md` em renderer real.
- Dependencias: M21.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_path_has_extension` (func_def, line 168), `zt_path_is_dir` (func_def, line 176), `zt_path_is_file` (func_def, line 188), `zt_join_path` (func_def, line 200), `zt_find_project_root_from_cwd` (func_def, line 260), `zt_path_filter_list_dispose` (func_def, line 334), `zt_path_filter_list_push` (func_def, line 347)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_module_make` (func_def, line 71)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_path_char_equal` (func_def, line 61), `zt_join_path` (func_def, line 70), `zt_path_is_dir` (func_def, line 96), `zt_path_has_extension` (func_def, line 108), `zt_strip_prefix_path` (func_def, line 116), `zt_emit_missing_public_doc_warnings` (func_def, line 431), `zt_collect_source_symbols` (func_def, line 530), `zt_zdoc_diagnostic_list_init` (func_def, line 628)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_print_source_span` (func_def, line 49), `zt_diag_read_source_line` (func_def, line 63), `zt_diag_code_name` (func_def, line 118), `zt_diag_code_stable` (func_def, line 177), `zt_diag_default_help` (func_def, line 237), `zt_diag_severity_name` (func_def, line 300), `zt_diag_list_make` (func_def, line 304), `zt_diag_list_dispose` (func_def, line 312)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_binding_kind` (enum, line 9), `zt_binding` (struct, line 14), `zt_binding_scope` (struct, line 21), `zt_module_catalog` (struct, line 56), `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_binding_scope_init` (func_def, line 118), `zt_binding_scope_dispose` (func_def, line 126)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_canonicalize_type` (func_def, line 48), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emit_error_code_name` (func_def, line 107), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_find_symbol_type` (func_def, line 193), `c_legalize_resolve_sequence_type` (func_def, line 225), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_index_seq` (func_def, line 312), `c_legalize_resolve_zir_sequence_type` (func_def, line 565)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_type_name_is_optional` (func_def, line 150), `zir_span_from_source` (func_def, line 154), `zir_type_name_owned` (func_def, line 158), `zir_emit_instruction` (func_def, line 226), `zir_find_enum_variant_index_hir` (func_def, line 272), `zir_emit_assign_expr` (func_def, line 1175), `zir_emit_effect_expr` (func_def, line 1189), `zir_emit_check_contract` (func_def, line 1195)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/frontend/test_lexer.c`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/runtime/c/test_runtime.c`
  - `tests/runtime/c/test_shared_text.c`
  - `tests/runtime/test_fase11_safety.zt`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/test_diagnostics.zt`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - ... +1 more
- Supporting docs:
  - `docs/spec/language/diagnostic-code-catalog.md`

### Prompt Seed

```text
Implement M23 (Diagnostics renderer real).
Start from docs/internal/planning/checklist-v1.md lines 453-467.
Then read docs/internal/planning/roadmap-v1.md lines 810-838.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/frontend/test_lexer.c
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/runtime/c/test_runtime.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M24. Runtime C ownership and backend scalability hardening

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 468-486
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 839-875
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/project/PROJECT_MAP.md`
5. Map: `compiler/semantic/binder/BINDER_MAP.md`
6. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
7. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
8. Map: `compiler/targets/c/EMITTER_MAP.md`
9. Map: `runtime/c/RUNTIME_MAP.md`
10. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 16
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte inicial bootstrap.
- Objetivo: tornar RC, COW, temporarios, cleanup e custos do backend C confiaveis para value semantics e programas maiores.
- Dependencias: M21; recomendado antes de expandir stdlib pesada.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_write_file` (func_def, line 129), `zt_copy_text` (func_def, line 146), `zt_heap_strdup` (func_def, line 156), `zt_find_project_root_from_cwd` (func_def, line 260), `zt_replace_extension` (func_def, line 283), `zt_path_filter_list_dispose` (func_def, line 334), `zt_path_filter_list_push` (func_def, line 347)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51), `zt_read_file_text` (func_def, line 141), `zt_trim_inplace` (func_def, line 187), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_list` (struct, line 38), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_function_context` (struct, line 77), `zt_decl_list_push` (func_def, line 184), `zt_import_list_push` (func_def, line 200), `zt_apply_list_push` (func_def, line 269), `zt_catalog_build` (func_def, line 284)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_canonicalize_type` (func_def, line 48), `c_safe_text` (func_def, line 79), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emit_error_code_name` (func_def, line 107), `c_emitter_init` (func_def, line 195)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110), `c_legalize_find_symbol_type` (func_def, line 193), `c_legalize_resolve_sequence_type` (func_def, line 225)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +37 more

### Prompt Seed

```text
Implement M24 (Runtime C ownership and backend scalability hardening).
Start from docs/internal/planning/checklist-v1.md lines 468-486.
Then read docs/internal/planning/roadmap-v1.md lines 839-875.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/project/zdoc.c
- compiler/project/zdoc.h
- compiler/project/ztproj.c
- compiler/project/ztproj.h
- compiler/semantic/parameter_validation.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M25. Value semantics conformance

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 487-498
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 876-902
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
6. Map: `compiler/targets/c/EMITTER_MAP.md`
7. Map: `runtime/c/RUNTIME_MAP.md`
8. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 9
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte inicial bootstrap.
- Objetivo: provar em behavior tests que copia semantica nao vaza mutacao compartilhada.
- Dependencias: M24.

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_list` (struct, line 38), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_function_context` (struct, line 77), `zt_decl_list_push` (func_def, line 184), `zt_import_list_push` (func_def, line 200), `zt_apply_list_push` (func_def, line 269), `zt_decl_type_param_count` (func_def, line 383)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_safe_text` (func_def, line 79), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emitter_text` (func_def, line 216), `c_copy_sanitized` (func_def, line 231), `c_find_user_struct` (func_def, line 259), `c_build_struct_symbol` (func_def, line 306), `c_find_struct_field` (func_def, line 323)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_list_len_expr` (func_def, line 473), `c_legalize_zir_list_len_expr` (func_def, line 695)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +37 more

### Prompt Seed

```text
Implement M25 (Value semantics conformance).
Start from docs/internal/planning/checklist-v1.md lines 487-498.
Then read docs/internal/planning/roadmap-v1.md lines 876-902.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
- compiler/targets/c/emitter.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M26. Runtime where contracts completos

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 499-512
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 903-926
3. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
4. Map: `compiler/semantic/binder/BINDER_MAP.md`
5. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
6. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
7. Map: `compiler/targets/c/EMITTER_MAP.md`
8. Map: `compiler/zir/ZIR_MODEL_MAP.md`
9. Map: `compiler/zir/ZIR_PARSER_MAP.md`
10. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
11. Map: `runtime/c/RUNTIME_MAP.md`
12. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 11
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte bootstrap atual.
- Objetivo: executar contratos de runtime em todos os pontos MVP aceitos.
- Dependencias: M23 e M24.

### Entry Files

- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_param_list_make` (func_def, line 108), `zt_hir_field_decl_list_make` (func_def, line 109), `zt_hir_enum_variant_field_list_make` (func_def, line 110), `zt_hir_field_init_list_make` (func_def, line 112), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_field_decl_list_push` (func_def, line 171), `zt_hir_enum_variant_field_list_push` (func_def, line 182), `zt_hir_field_init_list_push` (func_def, line 210)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_type_param_count` (func_def, line 383), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_register_type_params` (func_def, line 580), `zt_checker_bind_struct_field_type_params` (func_def, line 957), `zt_checker_find_struct_field_decl` (func_def, line 969), `zt_checker_enum_variant_field_type_node` (func_def, line 997), `zt_checker_enum_variant_field_name` (func_def, line 1004), `zt_checker_check_field_expr` (func_def, line 1065)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_is_float` (func_def, line 269), `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emit_error_code_name` (func_def, line 107), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202), `c_emitter_dispose` (func_def, line 209)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_safe_text` (func_def, line 8), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_instruction_buffer` (struct, line 12), `zir_span_from_source` (func_def, line 154), `zir_instruction_buffer_init` (func_def, line 165), `zir_instruction_buffer_push` (func_def, line 186), `zir_emit_instruction` (func_def, line 226), `zir_find_struct_decl_hir` (func_def, line 251), `zir_find_struct_field_hir` (func_def, line 262), `zir_emit_check_contract` (func_def, line 1195)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +39 more

### Prompt Seed

```text
Implement M26 (Runtime where contracts completos).
Start from docs/internal/planning/checklist-v1.md lines 499-512.
Then read docs/internal/planning/roadmap-v1.md lines 903-926.
Then open these maps:
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M27. Enums com payload e match forte

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 513-527
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 927-952
3. Map: `compiler/frontend/ast/AST_MAP.md`
4. Map: `compiler/frontend/parser/PARSER_MAP.md`
5. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
10. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
11. Map: `compiler/targets/c/EMITTER_MAP.md`
12. Map: `compiler/zir/ZIR_MODEL_MAP.md`
13. Map: `compiler/zir/ZIR_PARSER_MAP.md`
14. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
15. Map: `runtime/c/RUNTIME_MAP.md`
16. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido no source e no pipeline E2E (binder/typechecker/HIR + lowering ZIR + backend C + behavior checks).
- Objetivo: fechar modelagem de estados sem POO classica.
- Dependencias: M24.

### Entry Files

- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_enum_variant_field_list_make` (func_def, line 110), `zt_hir_enum_variant_list_make` (func_def, line 111), `zt_hir_match_case_list_make` (func_def, line 114), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_enum_variant_field_list_push` (func_def, line 182), `zt_hir_enum_variant_list_push` (func_def, line 196)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_bind_match_pattern` (func_def, line 305), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_binding_kind` (enum, line 9), `zt_binding` (struct, line 14), `zt_binding_scope` (struct, line 21), `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_binding_scope_init` (func_def, line 118), `zt_binding_scope_dispose` (func_def, line 126), `zt_binding_scope_declare` (func_def, line 141)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_find_user_enum` (func_def, line 271), `c_build_function_symbol` (func_def, line 297), `c_build_struct_symbol` (func_def, line 306), `c_build_enum_symbol` (func_def, line 314), `c_find_enum_variant_field` (func_def, line 336), `c_build_block_label` (func_def, line 348), `c_build_generated_outcome_symbol` (func_def, line 670), `c_emit_runtime_span_literal` (func_def, line 5483)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_name_matches` (func_def, line 118), `zir_span_from_source` (func_def, line 154), `zir_find_enum_variant_index_hir` (func_def, line 272), `zir_emit_check_contract` (func_def, line 1195), `zir_pattern_is_enum_construct` (func_def, line 1456), `zir_match_single_pattern_condition` (func_def, line 1460), `zir_match_case_condition` (func_def, line 1520), `zir_emit_match_case_bindings` (func_def, line 1541)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +41 more

### Prompt Seed

```text
Implement M27 (Enums com payload e match forte).
Start from docs/internal/planning/checklist-v1.md lines 513-527.
Then read docs/internal/planning/roadmap-v1.md lines 927-952.
Then open these maps:
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M28. Bytes, UTF-8 e stdlib base

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 528-542
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 953-969
3. Map: `runtime/c/RUNTIME_MAP.md`
4. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Status roadmap: em andamento (subset bytes/UTF-8 e operacoes base de std.bytes concluido).
- Objetivo: fechar text/binary antes de filesystem, process, JSON e network crescerem.
- Dependencias: M19 e M24.

### Entry Files

- No explicit source files were detected. Use the maps and tests as the first anchors.

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +28 more

### Prompt Seed

```text
Implement M28 (Bytes, UTF-8 e stdlib base).
Start from docs/internal/planning/checklist-v1.md lines 528-542.
Then read docs/internal/planning/roadmap-v1.md lines 953-969.
Then open these maps:
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M29. API segura de collections

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 543-552
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 970-994
3. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
4. Map: `compiler/project/PROJECT_MAP.md`
5. Map: `compiler/semantic/binder/BINDER_MAP.md`
6. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
7. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
8. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
9. Map: `compiler/targets/c/EMITTER_MAP.md`
10. Map: `compiler/zir/ZIR_MODEL_MAP.md`
11. Map: `compiler/zir/ZIR_PARSER_MAP.md`
12. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
13. Map: `runtime/c/RUNTIME_MAP.md`
14. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 7
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte inicial bootstrap.
- Objetivo: manter indexacao estrita sem incentivar panic como fluxo normal.
- Dependencias: M13 e M21.

### Entry Files

- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_string_list_make` (func_def, line 104), `zt_hir_decl_list_make` (func_def, line 105), `zt_hir_stmt_list_make` (func_def, line 106), `zt_hir_expr_list_make` (func_def, line 107), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_field_decl_list_make` (func_def, line 109), `zt_hir_enum_variant_field_list_make` (func_def, line 110), `zt_hir_enum_variant_list_make` (func_def, line 111)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_bind_expr_list` (func_def, line 215), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_binder` (struct, line 5), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_list` (struct, line 38), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_function_context` (struct, line 77), `zt_binding_scope_lookup` (func_def, line 169), `zt_decl_list_push` (func_def, line 184), `zt_import_list_push` (func_def, line 200), `zt_apply_list_push` (func_def, line 269)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_safe_text` (func_def, line 79), `c_emitter_text` (func_def, line 216), `c_type_lookup` (func_def, line 530), `c_emit_trimmed_text` (func_def, line 788), `c_expression_is_text` (func_def, line 915), `c_expression_is_materialized_text_ref` (func_def, line 935), `c_expression_is_materialized_list_i64_ref` (func_def, line 990), `c_expression_is_materialized_map_text_text_ref` (func_def, line 1004)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_index_seq` (func_def, line 312), `c_legalize_list_len_expr` (func_def, line 473), `c_legalize_zir_list_len_expr` (func_def, line 695), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_text_eq` (func_def, line 93), `zir_type_name_is_optional` (func_def, line 150), `zir_find_enum_variant_index_hir` (func_def, line 272), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_function_ctx` (struct, line 37)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +40 more

### Prompt Seed

```text
Implement M29 (API segura de collections).
Start from docs/internal/planning/checklist-v1.md lines 543-552.
Then read docs/internal/planning/roadmap-v1.md lines 970-994.
Then open these maps:
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/hir/nodes/model.c
- compiler/hir/nodes/model.h
- compiler/semantic/binder/binder.c
- compiler/semantic/binder/binder.h
- compiler/semantic/parameter_validation.c
- compiler/semantic/symbols/symbols.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M30. CLI final

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 553-569
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 995-1022
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
8. Map: `compiler/tooling/FORMATTER_MAP.md`
9. Map: `runtime/c/RUNTIME_MAP.md`
10. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 9
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte bootstrap atual.
- Objetivo: entregar tooling previsivel para usuario final.
- Dependencias: M22, M23 e M28.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_find_project_root_from_cwd` (func_def, line 260), `zt_normalize_path_separators` (func_def, line 373), `zt_project_source_file` (struct, line 459), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477), `zt_project_source_file_list_push` (func_def, line 503), `zt_project_discover_zt_files` (func_def, line 539)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_code_name` (func_def, line 665), `zt_check_zdoc_file` (func_def, line 860), `zt_zdoc_check_project` (func_def, line 1101), `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113), `zt_project_trim_inplace` (func_def, line 118)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_code_name` (func_def, line 118), `zt_diag_code_stable` (func_def, line 177), `zt_diag_code_effort` (func_def, line 465), `zt_diag_same_text` (func_def, line 12), `zt_diag_same_span` (func_def, line 18), `zt_diag_is_duplicate_recent` (func_def, line 25), `zt_diag_print_source_span` (func_def, line 49), `zt_diag_read_source_line` (func_def, line 63)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/tooling/formatter.c`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `format_comments` (func_def, line 53), `format_node_list_comma` (func_def, line 83), `format_node` (func_def, line 90), `zt_format_node_to_string` (func_def, line 581), `sb_t` (struct, line 7), `sb_init` (func_def, line 14), `sb_free` (func_def, line 22), `sb_append_len` (func_def, line 26)
- `compiler/tooling/formatter.h`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `ZENITH_TOOLING_FORMATTER_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +34 more

### Prompt Seed

```text
Implement M30 (CLI final).
Start from docs/internal/planning/checklist-v1.md lines 553-569.
Then read docs/internal/planning/roadmap-v1.md lines 995-1022.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/tooling/FORMATTER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M31. ZDoc funcional

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 570-584
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1023-1050
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
10. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`

### Checklist Status

- Done items: 12
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido.
- Objetivo: fazer a separacao entre codigo e documentacao longa ser util na pratica.
- Dependencias: M23 e M30.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_apply_manifest_lang` (func_def, line 65), `zt_normalize_path_separators` (func_def, line 373), `zt_load_since_filter` (func_def, line 398), `zt_last_namespace_segment` (func_def, line 680), `zt_validate_source_namespaces` (func_def, line 903), `zt_print_diagnostics` (func_def, line 1170), `zt_diag_code_from_zdoc_code` (func_def, line 1321), `zt_append_zdoc_diagnostics` (func_def, line 1335)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_copy_checked` (func_def, line 156), `zt_project_copy_default` (func_def, line 559), `zt_project_validate` (func_def, line 590), `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_binder` (struct, line 5), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83), `zt_import_local_name` (func_def, line 115)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_default_help` (func_def, line 237), `zt_diag_same_text` (func_def, line 12), `zt_diag_same_span` (func_def, line 18), `zt_diag_is_duplicate_recent` (func_def, line 25), `zt_diag_print_source_span` (func_def, line 49), `zt_diag_read_source_line` (func_def, line 63), `zt_diag_render_caret` (func_def, line 103), `zt_diag_code_name` (func_def, line 118)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_type_base_name` (func_def, line 379), `zt_decl_type_param_count` (func_def, line 383), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`
  - `tests/test_diagnostics.zt`

### Prompt Seed

```text
Implement M31 (ZDoc funcional).
Start from docs/internal/planning/checklist-v1.md lines 570-584.
Then read docs/internal/planning/roadmap-v1.md lines 1023-1050.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_binder.c
- tests/semantic/test_constraints.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M32. Matriz de conformidade final

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 585-606
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1051-1069
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/project/PROJECT_MAP.md`
7. Map: `compiler/semantic/binder/BINDER_MAP.md`
8. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
9. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
10. Map: `compiler/targets/c/EMITTER_MAP.md`
11. Map: `compiler/tooling/FORMATTER_MAP.md`
12. Map: `compiler/zir/ZIR_MODEL_MAP.md`
13. Map: `compiler/zir/ZIR_PARSER_MAP.md`
14. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
15. Map: `runtime/c/RUNTIME_MAP.md`
16. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 17
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte inicial bootstrap.
- Objetivo: transformar "feito" em um criterio testavel e nao subjetivo.
- Dependencias: M21-M31.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_heap_strdup` (func_def, line 156), `zt_normalize_path_separators` (func_def, line 373), `zt_path_suffix_matches` (func_def, line 656), `zt_detect_import_cycles` (func_def, line 802), `zt_build_combined_project_ast` (func_def, line 989), `zt_diag_matches_focus` (func_def, line 1125), `zt_diag_matches_since` (func_def, line 1144), `zt_diag_matches_filters` (func_def, line 1167)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_parse_dependency_spec` (func_def, line 343), `zt_project_section` (enum, line 9), `zt_project_safe_text` (func_def, line 24), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45), `zt_project_set_unknown_key` (func_def, line 57), `zt_project_read_file_text` (func_def, line 67), `zt_project_trim_left` (func_def, line 113)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_catalog_build` (func_def, line 284), `zt_type_base_name` (func_def, line 379), `zt_decl_type_param_count` (func_def, line 383), `zt_expr_matches_integral_type` (func_def, line 516), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_register_type_params` (func_def, line 580), `zt_checker_bind_struct_field_type_params` (func_def, line 957), `zt_checker_build_qualified_name` (func_def, line 1039)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_build_function_symbol` (func_def, line 297), `c_build_struct_symbol` (func_def, line 306), `c_build_enum_symbol` (func_def, line 314), `c_build_block_label` (func_def, line 348), `c_build_generated_outcome_symbol` (func_def, line 670), `c_outcome_spec` (struct, line 1186), `c_outcome_spec_for_type` (func_def, line 1206), `c_outcome_spec_for_expr` (func_def, line 1262)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/tooling/formatter.c`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `sb_t` (struct, line 7), `sb_init` (func_def, line 14), `sb_free` (func_def, line 22), `sb_append_len` (func_def, line 26), `sb_append` (func_def, line 41), `sb_indent` (func_def, line 45), `format_comments` (func_def, line 53), `op_to_str` (func_def, line 62)
- `compiler/tooling/formatter.h`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `ZENITH_TOOLING_FORMATTER_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_name_matches` (func_def, line 118), `zir_match_single_pattern_condition` (func_def, line 1460), `zir_match_case_condition` (func_def, line 1520), `zir_emit_match_case_bindings` (func_def, line 1541), `zir_lower_match_statement` (func_def, line 1572), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +47 more
- Supporting docs:
  - `docs/spec/language/conformance-matrix.md`

### Prompt Seed

```text
Implement M32 (Matriz de conformidade final).
Start from docs/internal/planning/checklist-v1.md lines 585-606.
Then read docs/internal/planning/roadmap-v1.md lines 1051-1069.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/tooling/FORMATTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/project/zdoc.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M33. Implementacao das Stdlibs MVP

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 607-668
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1070-1109
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/lexer/LEXER_MAP.md`
6. Map: `compiler/frontend/parser/PARSER_MAP.md`
7. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
8. Map: `compiler/project/PROJECT_MAP.md`
9. Map: `compiler/semantic/binder/BINDER_MAP.md`
10. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
11. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
12. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
13. Map: `compiler/targets/c/EMITTER_MAP.md`
14. Map: `compiler/zir/ZIR_MODEL_MAP.md`
15. Map: `compiler/zir/ZIR_PARSER_MAP.md`
16. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
17. Map: `runtime/c/RUNTIME_MAP.md`
18. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 59
- Open items: 0

### Roadmap Summary

- Status roadmap: concluido para o corte MVP atual (modulos MVP implementados, ZDoc fisico presente e hardening basico de ownership ARC em collections validado por behavior tests E2E).
- Objetivo: Fornecer os modulos da biblioteca padrao (alem da base de bytes/utf-8 iniciada em M19), fundamentados num backend e runtime estaveis. As decisoes de design destas APIs ja estao fechadas e documentadas em `docs/internal/decisions/language/`.
- Dependencias: M24 (ARC nao-atomico para buffers e resources) e M26 (Runtime where contracts para `std.validate`).

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_write_file` (func_def, line 129), `zt_copy_text` (func_def, line 146), `zt_heap_strdup` (func_def, line 156), `zt_path_has_extension` (func_def, line 168), `zt_path_is_dir` (func_def, line 176), `zt_path_is_file` (func_def, line 188), `zt_join_path` (func_def, line 200)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131), `zt_ast_make` (func_def, line 66)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_realloc_array` (func_def, line 61), `zt_hir_string_list_make` (func_def, line 104), `zt_hir_decl_list_make` (func_def, line 105), `zt_hir_stmt_list_make` (func_def, line 106), `zt_hir_expr_list_make` (func_def, line 107)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_item` (struct, line 15), `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_item` (struct, line 25), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_item` (struct, line 37), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51), `zt_path_char_equal` (func_def, line 61)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_safe_text` (func_def, line 24), `zt_project_set_error` (func_def, line 45), `zt_project_read_file_text` (func_def, line 67), `zt_project_copy_checked` (func_def, line 156), `zt_project_parse_string_value` (func_def, line 166), `zt_project_parse_positive_size_value` (func_def, line 227), `zt_project_section_from_name` (func_def, line 273), `zt_project_parse_section` (func_def, line 288)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_is_builtin_type_name` (func_def, line 83), `zt_import_local_name` (func_def, line 115), `zt_bind_simple_type_name` (func_def, line 144), `zt_bind_type_node` (func_def, line 174), `zt_bind_expr_list` (func_def, line 215), `zt_bind_match_pattern` (func_def, line 305), `zt_bind_type_param_defs` (func_def, line 452)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_same_text` (func_def, line 12), `zt_diag_print_source_span` (func_def, line 49), `zt_diag_default_help` (func_def, line 237), `zt_diag_list_make` (func_def, line 304), `zt_diag_list_dispose` (func_def, line 312), `zt_diag_list_add_va` (func_def, line 320), `zt_diag_list_add` (func_def, line 355), `zt_diag_list_add_severity` (func_def, line 363)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbol_kind_name` (func_def, line 18), `zt_symbols_strdup` (func_def, line 6), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_binding_kind` (enum, line 9), `zt_decl_list` (struct, line 38), `zt_import_list` (struct, line 44), `zt_apply_list` (struct, line 50), `zt_checker` (struct, line 62), `zt_function_context` (struct, line 77), `zt_checker_strdup` (func_def, line 84), `zt_decl_list_push` (func_def, line 184)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_local_decl` (struct, line 9), `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_buffer_mark` (func_def, line 36), `c_buffer_truncate` (func_def, line 40), `c_canonicalize_type` (func_def, line 48), `c_safe_text` (func_def, line 79), `c_emit_set_result` (func_def, line 83)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110), `c_legalize_find_symbol_type` (func_def, line 193), `c_legalize_resolve_sequence_type` (func_def, line 225)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_instruction_buffer` (struct, line 12), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32), `zir_lower_format` (func_def, line 87), `zir_text_eq` (func_def, line 93), `zir_name_matches` (func_def, line 118), `zir_type_name_is_optional` (func_def, line 150), `zir_span_from_source` (func_def, line 154)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +46 more

### Prompt Seed

```text
Implement M33 (Implementacao das Stdlibs MVP).
Start from docs/internal/planning/checklist-v1.md lines 607-668.
Then read docs/internal/planning/roadmap-v1.md lines 1070-1109.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M34. Cognitive Accessibility by Design

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 669-681
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1110-1136
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/project/PROJECT_MAP.md`
5. Map: `compiler/semantic/binder/BINDER_MAP.md`
6. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
7. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
8. Map: `compiler/utils/UTILS_MAP.md`

### Checklist Status

- Done items: 9
- Open items: 0

### Roadmap Summary

- Status roadmap: iniciado (spec consolidada em `docs/spec/language/cognitive-accessibility.md`).
- Objetivo: transformar diretrizes de acessibilidade cognitiva em comportamento observavel no toolchain.
- Dependencias: M23 e M30.
- Paralelo possivel:
  - M33 (stdlibs)
  - M32 (matriz de conformidade final)

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_path_has_extension` (func_def, line 168), `zt_path_is_dir` (func_def, line 176), `zt_path_is_file` (func_def, line 188), `zt_join_path` (func_def, line 200), `zt_dirname` (func_def, line 226), `zt_path_filter_list_dispose` (func_def, line 334), `zt_path_filter_list_push` (func_def, line 347)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_path_char_equal` (func_def, line 61), `zt_join_path` (func_def, line 70), `zt_path_is_dir` (func_def, line 96), `zt_path_has_extension` (func_def, line 108), `zt_strip_prefix_path` (func_def, line 116), `zt_symbol_exists_local` (func_def, line 481), `zt_zdoc_severity_name` (func_def, line 661), `zt_zdoc_code_name` (func_def, line 665)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_section_name` (func_def, line 28), `zt_project_copy_checked` (func_def, line 156), `zt_project_section_from_name` (func_def, line 273), `zt_project_parse_dependency_spec` (func_def, line 343), `zt_project_manifest_entry_namespace` (func_def, line 583), `zt_project_error_code_name` (func_def, line 678), `zt_project_resolve_entry_source_path` (func_def, line 790), `zt_project_section` (enum, line 9)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83), `zt_import_local_name` (func_def, line 115), `zt_bind_declare_name` (func_def, line 122), `zt_bind_simple_type_name` (func_def, line 144)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_code_name` (func_def, line 118), `zt_diag_severity_name` (func_def, line 300), `zt_name_edit_distance` (func_def, line 401), `zt_name_suggest` (func_def, line 434), `zt_diag_action_text` (func_def, line 523), `zt_diag_render_action_first` (func_def, line 660), `zt_diag_render_action_first_list` (func_def, line 722), `zt_diag_same_text` (func_def, line 12)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbol_kind_name` (func_def, line 18), `zt_symbols_strdup` (func_def, line 6), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/driver/test_project.c`
  - `tests/driver/test_zdoc.c`
  - `tests/semantic/test_binder.c`
  - `tests/test_diagnostics.zt`
- Supporting docs:
  - `docs/spec/language/cognitive-accessibility.md`

### Prompt Seed

```text
Implement M34 (Cognitive Accessibility by Design).
Start from docs/internal/planning/checklist-v1.md lines 669-681.
Then read docs/internal/planning/roadmap-v1.md lines 1110-1136.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/utils/UTILS_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/project/zdoc.c
- compiler/project/zdoc.h
- compiler/project/ztproj.c
- compiler/project/ztproj.h
- compiler/semantic/binder/binder.c
Validate with these tests first:
- tests/driver/test_project.c
- tests/driver/test_zdoc.c
- tests/semantic/test_binder.c
- tests/test_diagnostics.zt
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M35. Concurrency/FFI/Dyn Dispatch

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 682-689
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1137-1152
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
6. Map: `compiler/targets/c/EMITTER_MAP.md`
7. Map: `runtime/c/RUNTIME_MAP.md`

### Checklist Status

- Done items: 2
- Open items: 3
- Pending work:
  - Implementar isolamento estrito de threads com deep-copy entre fronteiras
  - Implementar e baixar `dyn Trait` (fat pointers) para colecoes heterogeneas
  - Criar behavior test para colecao heterogenea baseada em iterador `dyn Trait`

### Roadmap Summary

- Status roadmap: planejado (post-MVP).
- Objetivo: fechar recursos de concorrencia, interoperabilidade FFI e dispatch dinamico que ficaram explicitamente fora do corte bootstrap de M24/M25.
- Dependencias: M24, M25, M32.

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_decl_type_param_count` (func_def, line 383), `zt_type_is_core_trait_name` (func_def, line 549), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_register_type_params` (func_def, line 580), `zt_checker_type_implements_trait` (func_def, line 740), `zt_checker_bind_struct_field_type_params` (func_def, line 957), `zt_binding_kind` (enum, line 9), `zt_binding` (struct, line 14)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_copy_sanitized` (func_def, line 231), `c_copy_trimmed` (func_def, line 399), `c_copy_trimmed_segment` (func_def, line 431), `c_copy_trimmed_alloc` (func_def, line 909), `c_expression_is_copyable_managed_value_ref` (func_def, line 1342), `c_copy_legalize_result` (func_def, line 1696), `c_extern_call_expected_arg_type` (func_def, line 3286), `c_zir_call_extern_needs_ffi_shield` (func_def, line 3300)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110), `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/runtime/c/test_runtime.c`
  - `tests/runtime/c/test_shared_text.c`
  - `tests/runtime/test_fase11_safety.zt`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`

### Prompt Seed

```text
Implement M35 (Concurrency/FFI/Dyn Dispatch).
Start from docs/internal/planning/checklist-v1.md lines 682-689.
Then read docs/internal/planning/roadmap-v1.md lines 1137-1152.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- runtime/c/RUNTIME_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
- compiler/targets/c/emitter.c
Validate with these tests first:
- tests/runtime/c/test_runtime.c
- tests/runtime/c/test_shared_text.c
- tests/runtime/test_fase11_safety.zt
- tests/semantic/test_constraints.c
- tests/semantic/test_types.c
- tests/targets/c/test_emitter.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M36. Suite de Performance E2E

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 690-713
2. Roadmap: no dedicated milestone section; infer scope from checklist + nearby roadmap milestones
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/lexer/LEXER_MAP.md`
6. Map: `compiler/frontend/parser/PARSER_MAP.md`
7. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
8. Map: `compiler/project/PROJECT_MAP.md`
9. Map: `compiler/semantic/binder/BINDER_MAP.md`
10. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
11. Map: `compiler/targets/c/EMITTER_MAP.md`
12. Map: `compiler/zir/ZIR_MODEL_MAP.md`
13. Map: `compiler/zir/ZIR_PARSER_MAP.md`
14. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`
15. Map: `runtime/c/RUNTIME_MAP.md`
16. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 19
- Open items: 0

### Roadmap Summary

- No dedicated roadmap section for this milestone.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_path_filter_list` (struct, line 57), `zt_copy_text` (func_def, line 146), `zt_path_filter_list_dispose` (func_def, line 334), `zt_path_filter_list_push` (func_def, line 347), `zt_normalize_path_separators` (func_def, line 373), `zt_project_source_file_list` (struct, line 466), `zt_project_source_file_list_init` (func_def, line 472), `zt_project_source_file_list_dispose` (func_def, line 477)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131), `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_string_list_make` (func_def, line 104), `zt_hir_decl_list_make` (func_def, line 105), `zt_hir_stmt_list_make` (func_def, line 106), `zt_hir_expr_list_make` (func_def, line 107), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_field_decl_list_make` (func_def, line 109), `zt_hir_enum_variant_field_list_make` (func_def, line 110), `zt_hir_enum_variant_list_make` (func_def, line 111)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/project/zdoc.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_zdoc_file_list` (struct, line 19), `zt_zdoc_symbol_list` (struct, line 31), `zt_zdoc_public_symbol_list` (struct, line 45), `zt_copy_text` (func_def, line 51), `zt_read_file_text` (func_def, line 141), `zt_zdoc_file_list_push` (func_def, line 215), `zt_zdoc_file_list_dispose` (func_def, line 238), `zt_zdoc_symbol_list_push` (func_def, line 324)
- `compiler/project/zdoc.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZDOC_H` (macro, line 2)
- `compiler/project/ztproj.c`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `zt_project_safe_text` (func_def, line 24), `zt_project_read_file_text` (func_def, line 67), `zt_project_copy_checked` (func_def, line 156), `zt_project_validate` (func_def, line 590), `zt_project_parse_text` (func_def, line 697), `zt_project_section` (enum, line 9), `zt_project_section_name` (func_def, line 28), `zt_project_set_error` (func_def, line 45)
- `compiler/project/ztproj.h`
  - Closest map: `compiler/project/PROJECT_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_PROJECT_ZTPROJ_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_import_local_name` (func_def, line 115), `zt_bind_expr_list` (func_def, line 215), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_local_decl` (struct, line 9), `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_safe_text` (func_def, line 79), `c_buffer_append_vformat` (func_def, line 164), `c_buffer_append_format` (func_def, line 185), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_list_len_expr` (func_def, line 473), `c_legalize_zir_list_len_expr` (func_def, line 695), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_lower_format` (func_def, line 87), `zir_text_eq` (func_def, line 93), `zir_emit_check_contract` (func_def, line 1195), `zir_try_lower_constructor_contract_assignment` (func_def, line 1237), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +47 more

### Prompt Seed

```text
Implement M36 (Suite de Performance E2E).
Start from docs/internal/planning/checklist-v1.md lines 690-713.
Roadmap has no dedicated section for this milestone; infer scope from checklist and adjacent roadmap milestones.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/lexer/LEXER_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M37. Erro Tipado no Backend C (`result<T, core.Error>` -> `result<T, E>`)

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 714-724
2. Roadmap: no dedicated milestone section; infer scope from checklist + nearby roadmap milestones
3. Map: `compiler/semantic/binder/BINDER_MAP.md`
4. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
6. Map: `compiler/targets/c/EMITTER_MAP.md`
7. Map: `runtime/c/RUNTIME_MAP.md`
8. Map: `stdlib/STDLIB_MAP.md`

### Checklist Status

- Done items: 8
- Open items: 0

### Roadmap Summary

- No dedicated roadmap section for this milestone.

### Entry Files

- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_function_context` (struct, line 77), `zt_catalog_has_import_alias` (func_def, line 246), `zt_catalog_import_path_for_alias` (func_def, line 255), `zt_decl_type_param_count` (func_def, line 383), `zt_type_is_core_trait_name` (func_def, line 549), `zt_type_param_has_trait` (func_def, line 556), `zt_checker_make_core_constraint_type` (func_def, line 574), `zt_checker_register_type_params` (func_def, line 580)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_emitter_alloc_buffer` (func_def, line 17), `c_emitter_free_buffer` (func_def, line 29), `c_safe_text` (func_def, line 79), `c_emit_set_result` (func_def, line 83), `c_emit_result_init` (func_def, line 103), `c_emit_error_code_name` (func_def, line 107), `c_emitter_init` (func_def, line 195), `c_emitter_reset` (func_def, line 202)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_error_code_name` (func_def, line 36), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110), `c_legalize_expect_result_type` (func_def, line 270), `c_legalize_is_blank` (func_def, line 51)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/behavior/std_bytes_ops/zenith.ztproj`
  - `tests/behavior/std_bytes_utf8/zenith.ztproj`
  - `tests/behavior/std_collections_basic/zenith.ztproj`
  - `tests/behavior/std_collections_managed_arc/zenith.ztproj`
  - `tests/behavior/std_format_basic/zenith.ztproj`
  - `tests/behavior/std_fs_basic/zenith.ztproj`
  - `tests/behavior/std_fs_path_basic/zenith.ztproj`
  - `tests/behavior/std_io_basic/main.zt`
  - `tests/behavior/std_io_basic/run.txt`
  - `tests/behavior/std_io_basic/zenith.ztproj`
  - `tests/behavior/std_json_basic/zenith.ztproj`
  - `tests/behavior/std_math_basic/zenith.ztproj`
  - `tests/behavior/std_net_basic/loopback-server.ps1`
  - `tests/behavior/std_net_basic/run-loopback.ps1`
  - `tests/behavior/std_net_basic/zenith.ztproj`
  - `tests/behavior/std_os_basic/zenith.ztproj`
  - ... +35 more

### Prompt Seed

```text
Implement M37 (Erro Tipado no Backend C (`result<T, core.Error>` -> `result<T, E>`)).
Start from docs/internal/planning/checklist-v1.md lines 714-724.
Roadmap has no dedicated section for this milestone; infer scope from checklist and adjacent roadmap milestones.
Then open these maps:
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- runtime/c/RUNTIME_MAP.md
- stdlib/STDLIB_MAP.md
Then inspect these source files first:
- compiler/semantic/parameter_validation.c
- compiler/semantic/types/checker.c
- compiler/semantic/types/checker.h
- compiler/semantic/types/types.c
- compiler/semantic/types/types.h
- compiler/targets/c/emitter.c
Validate with these tests first:
- tests/behavior/std_bytes_ops/zenith.ztproj
- tests/behavior/std_bytes_utf8/zenith.ztproj
- tests/behavior/std_collections_basic/zenith.ztproj
- tests/behavior/std_collections_managed_arc/zenith.ztproj
- tests/behavior/std_format_basic/zenith.ztproj
- tests/behavior/std_fs_basic/zenith.ztproj
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```

## M38. Hardening de Coerencia Frontend->Backend (extern/where/params)

### Cascade Order

1. Checklist: `docs/internal/planning/checklist-v1.md` lines 725-738
2. Roadmap: `docs/internal/planning/roadmap-v1.md` lines 1153-1170
3. Map: `compiler/driver/DRIVER_MAP.md`
4. Map: `compiler/frontend/ast/AST_MAP.md`
5. Map: `compiler/frontend/parser/PARSER_MAP.md`
6. Map: `compiler/hir/lowering/HIR_LOWERING_MAP.md`
7. Map: `compiler/project/PROJECT_MAP.md`
8. Map: `compiler/semantic/binder/BINDER_MAP.md`
9. Map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
10. Map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
11. Map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
12. Map: `compiler/targets/c/EMITTER_MAP.md`
13. Map: `compiler/tooling/FORMATTER_MAP.md`
14. Map: `compiler/zir/ZIR_MODEL_MAP.md`
15. Map: `compiler/zir/ZIR_PARSER_MAP.md`
16. Map: `compiler/zir/ZIR_VERIFIER_MAP.md`

### Checklist Status

- Done items: 0
- Open items: 11
- Pending work:
  - HIR lowering: coletar simbolos `extern c` e preservar nome ABI correto no call lowering
  - Backend C: impedir mangling indevido em chamadas `extern c`
  - Criar behavior test E2E de `extern c` com chamada real (`puts` ou equivalente)
  - Parser: normalizar `where` de parametro no mesmo shape usado por `where` canonico
  - Binder/checker: validar `where` de parametro no mesmo estagio de `where` de campo
  - Typechecker: exigir predicate `where` de parametro com tipo `bool`
  - Conectar `parameter_validation` ao pipeline semantico real
  - Emitir diagnostico claro para regra "required after default" em assinatura
  - Cobrir com testes: `param where` invalido, `param where` nao-booleano, ordem invalida de parametros
  - Resolver drift de interpolacao: decidir suporte final e alinhar parser/HIR/backend/formatter/docs
  - Remover risco de truncamento em nomes longos no parser e emitir erro estavel quando exceder limite

### Roadmap Summary

- Status roadmap: planejado (gate de coerencia antes de release).
- Objetivo: fechar inconsistencias entre parser/binder/typechecker/HIR/backend que hoje permitem codigo aceito quebrar tarde no pipeline.
- Dependencias: M16, M23, M33.

### Entry Files

- `compiler/driver/main.c`
  - Closest map: `compiler/driver/DRIVER_MAP.md`
  - Start near: `zt_normalize_path_separators` (func_def, line 373), `zt_diag_code_from_project_error` (func_def, line 1078), `zt_print_project_parse_error` (func_def, line 1097), `zt_diag_code_from_zir_parse_error` (func_def, line 1269), `zt_diag_code_from_c_emit_error` (func_def, line 1309), `zt_should_skip_test_discovery_dir` (func_def, line 1511), `zt_collect_test_projects` (func_def, line 1529), `zt_file_has_test_marker` (func_def, line 1623)
- `compiler/frontend/ast/model.c`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `zt_ast_kind_name` (func_def, line 6), `zt_ast_make` (func_def, line 66), `zt_ast_node_list_push` (func_def, line 75), `zt_ast_named_arg_list_push` (func_def, line 88), `zt_ast_map_entry_list_push` (func_def, line 101), `zt_ast_node_list_make` (func_def, line 114), `zt_ast_map_entry_list_make` (func_def, line 122), `zt_ast_named_arg_list_make` (func_def, line 131)
- `compiler/frontend/ast/model.h`
  - Closest map: `compiler/frontend/ast/AST_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_AST_MODEL_H` (macro, line 2)
- `compiler/frontend/parser/parser.c`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `zt_parser` (struct, line 9), `zt_parser_next_non_comment_token` (func_def, line 26), `zt_parser_advance` (func_def, line 47), `zt_parser_fill_peek` (func_def, line 58), `zt_parser_check` (func_def, line 65), `zt_parser_match` (func_def, line 69), `zt_parser_is_declaration_start` (func_def, line 77), `zt_parser_is_member_start` (func_def, line 96)
- `compiler/frontend/parser/parser.h`
  - Closest map: `compiler/frontend/parser/PARSER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_FRONTEND_PARSER_PARSER_H` (macro, line 2)
- `compiler/hir/nodes/model.c`
  - Closest map: `-`
  - Start near: `zt_hir_realloc_array` (func_def, line 61), `zt_hir_param_list_make` (func_def, line 108), `zt_hir_param_list_push` (func_def, line 160), `zt_hir_param_list_dispose` (func_def, line 396), `zt_hir_decl_kind_name` (func_def, line 6), `zt_hir_stmt_kind_name` (func_def, line 15), `zt_hir_expr_kind_name` (func_def, line 36), `zt_hir_module_make` (func_def, line 71)
- `compiler/hir/nodes/model.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_HIR_NODES_MODEL_H` (macro, line 2)
- `compiler/semantic/binder/binder.c`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `zt_binder` (struct, line 5), `zt_bind_type_param_defs` (func_def, line 452), `zt_bind_param_list` (func_def, line 463), `zt_bind_where_clause` (func_def, line 475), `zt_scope_collect_names` (func_def, line 9), `zt_bind_emit_unresolved_with_suggestion` (func_def, line 23), `zt_bind_warn_confusing_name` (func_def, line 41), `zt_is_builtin_type_name` (func_def, line 83)
- `compiler/semantic/binder/binder.h`
  - Closest map: `compiler/semantic/binder/BINDER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_BINDER_H` (macro, line 2)
- `compiler/semantic/diagnostics/diagnostics.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_diag_default_help` (func_def, line 237), `zt_cog_profile_error_limit` (func_def, line 548), `zt_diag_same_text` (func_def, line 12), `zt_diag_same_span` (func_def, line 18), `zt_diag_is_duplicate_recent` (func_def, line 25), `zt_diag_print_source_span` (func_def, line 49), `zt_diag_read_source_line` (func_def, line 63), `zt_diag_render_caret` (func_def, line 103)
- `compiler/semantic/diagnostics/diagnostics.h`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_DIAGNOSTICS_H` (macro, line 2)
- `compiler/semantic/parameter_validation.c`
  - Closest map: `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
  - Start near: `zt_validate_parameter_ordering` (func_def, line 25), `zt_validate_argument_ordering` (func_def, line 66)
- `compiler/semantic/symbols/symbols.c`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `zt_symbols_strdup` (func_def, line 6), `zt_symbol_kind_name` (func_def, line 18), `zt_scope_init` (func_def, line 36), `zt_scope_dispose` (func_def, line 44), `zt_scope_lookup_current` (func_def, line 58), `zt_scope_lookup_parent_chain` (func_def, line 70), `zt_scope_lookup` (func_def, line 83), `zt_scope_declare` (func_def, line 93)
- `compiler/semantic/symbols/symbols.h`
  - Closest map: `compiler/semantic/symbols/SYMBOLS_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_SYMBOLS_H` (macro, line 2)
- `compiler/semantic/types/checker.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_checker` (struct, line 62), `zt_checker_strdup` (func_def, line 84), `zt_checker_is_self_prefix` (func_def, line 345), `zt_checker_decl_is_public` (func_def, line 359), `zt_decl_type_param_count` (func_def, line 383), `zt_checker_diag_type` (func_def, line 436), `zt_checker_compute_integral_binary` (func_def, line 524), `zt_type_param_has_trait` (func_def, line 556)
- `compiler/semantic/types/checker.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_CHECKER_H` (macro, line 2)
- `compiler/semantic/types/types.c`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `zt_types_strdup` (func_def, line 7), `zt_type_kind_name` (func_def, line 19), `zt_type_list_make` (func_def, line 59), `zt_type_list_push` (func_def, line 67), `zt_type_list_dispose` (func_def, line 82), `zt_type_make` (func_def, line 95), `zt_type_make_named` (func_def, line 103), `zt_type_make_with_args` (func_def, line 110)
- `compiler/semantic/types/types.h`
  - Closest map: `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_SEMANTIC_TYPES_TYPES_H` (macro, line 2)
- `compiler/targets/c/emitter.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_canonicalize_type` (func_def, line 48), `c_emit_error_code_name` (func_def, line 107), `c_expression_is_materialized_core_error_ref` (func_def, line 976), `c_outcome_emit_failure_from_error_expr` (func_def, line 1380), `c_emit_call_args` (func_def, line 2043), `c_emit_call_expr` (func_def, line 2101), `c_emit_zir_call_expr` (func_def, line 3129), `c_extern_call_expected_arg_type` (func_def, line 3286)
- `compiler/targets/c/emitter.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_EMITTER_H` (macro, line 2)
- `compiler/targets/c/legalization.c`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `c_legalize_error_code_name` (func_def, line 36), `c_legalize_safe_text` (func_def, line 8), `c_legalize_set_result` (func_def, line 12), `c_legalize_result_init` (func_def, line 32), `c_legalize_is_blank` (func_def, line 51), `c_legalize_starts_with` (func_def, line 68), `c_legalize_copy_trimmed` (func_def, line 76), `c_legalize_copy_trimmed_segment` (func_def, line 110)
- `compiler/targets/c/legalization.h`
  - Closest map: `compiler/targets/c/EMITTER_MAP.md`
  - Start near: `ZENITH_NEXT_COMPILER_TARGETS_C_LEGALIZATION_H` (macro, line 2)
- `compiler/tooling/formatter.c`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `sb_t` (struct, line 7), `sb_init` (func_def, line 14), `sb_free` (func_def, line 22), `sb_append_len` (func_def, line 26), `sb_append` (func_def, line 41), `sb_indent` (func_def, line 45), `format_comments` (func_def, line 53), `op_to_str` (func_def, line 62)
- `compiler/tooling/formatter.h`
  - Closest map: `compiler/tooling/FORMATTER_MAP.md`
  - Start near: `ZENITH_TOOLING_FORMATTER_H` (macro, line 2)
- `compiler/zir/lowering/from_hir.c`
  - Closest map: `-`
  - Start near: `zir_call_is_module_func` (func_def, line 123), `zir_call_add_lowered_args` (func_def, line 332), `zir_lower_len_call` (func_def, line 348), `zir_lower_call_expr` (func_def, line 405), `zir_instruction_buffer` (struct, line 12), `zir_block_state` (struct, line 18), `zir_block_state_buffer` (struct, line 26), `zir_loop_target` (struct, line 32)
- `compiler/zir/lowering/from_hir.h`
  - Closest map: `-`
  - Start near: `ZENITH_NEXT_COMPILER_ZIR_LOWERING_FROM_HIR_H` (macro, line 2)

### Tests And Docs

- Tests:
  - `tests/formatter/run_formatter_golden.py`
  - `tests/frontend/test_parser.c`
  - `tests/frontend/test_parser_error_recovery.c`
  - `tests/semantic/test_binder.c`
  - `tests/semantic/test_constraints.c`
  - `tests/semantic/test_hir_lowering.c`
  - `tests/semantic/test_types.c`
  - `tests/targets/c/test_emitter.c`
  - `tests/targets/c/test_legalization.c`
  - `tests/test_diagnostics.zt`
  - `tests/zir/test_lowering.c`
  - `tests/zir/test_printer.c`
  - `tests/zir/test_verifier.c`

### Prompt Seed

```text
Implement M38 (Hardening de Coerencia Frontend->Backend (extern/where/params)).
Start from docs/internal/planning/checklist-v1.md lines 725-738.
Then read docs/internal/planning/roadmap-v1.md lines 1153-1170.
Then open these maps:
- compiler/driver/DRIVER_MAP.md
- compiler/frontend/ast/AST_MAP.md
- compiler/frontend/parser/PARSER_MAP.md
- compiler/hir/lowering/HIR_LOWERING_MAP.md
- compiler/project/PROJECT_MAP.md
- compiler/semantic/binder/BINDER_MAP.md
- compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md
- compiler/semantic/symbols/SYMBOLS_MAP.md
- compiler/semantic/types/TYPE_SYSTEM_MAP.md
- compiler/targets/c/EMITTER_MAP.md
- compiler/tooling/FORMATTER_MAP.md
- compiler/zir/ZIR_MODEL_MAP.md
- compiler/zir/ZIR_PARSER_MAP.md
- compiler/zir/ZIR_VERIFIER_MAP.md
Then inspect these source files first:
- compiler/driver/main.c
- compiler/frontend/ast/model.c
- compiler/frontend/ast/model.h
- compiler/frontend/parser/parser.c
- compiler/frontend/parser/parser.h
- compiler/hir/nodes/model.c
Validate with these tests first:
- tests/formatter/run_formatter_golden.py
- tests/frontend/test_parser.c
- tests/frontend/test_parser_error_recovery.c
- tests/semantic/test_binder.c
- tests/semantic/test_constraints.c
- tests/semantic/test_hir_lowering.c
Return only: files to edit, first symbols to inspect, validation plan, and main regression risk.
```
