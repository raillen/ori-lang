# Changelog — Ori Language

Todas as mudanças notáveis na implementação da linguagem Ori serão documentadas
neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
e o projeto adere a [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Adicionado
- **Parser:** Token `...` (Ellipsis) para parâmetros variádicos
- **Parser:** Validação de `parse.variadic_not_last` e `parse.default_before_required`
- **Checker:** `check_loop_control()` — diagnostica `break`/`continue` fora de loop (`control.loop_required`)
- **Checker:** `expect_bool()` para operadores `and`/`or`/`not` (`type.expected_bool`)
- **Checker:** `warn_unused_result()` — warning para `result` descartado (`type.unused_result`)
- **Checker:** `check_closure_var_capture()` — rejeita captura de `var` em closure (`mut.closure_captures_var`)
- **Checker:** `infer_never_form_call()` — suporte a `panic`, `todo`, `unreachable` com tipo `never`
- **Checker:** `infer_wrapper_form_call()` — suporte parcial a `.or()` / `.or_return()`
- **Checker:** `emit_undefined_name()` — nomes desconhecidos geram `name.undefined` + `Ty::Error`
- **Checker:** Validação de runtime para map/set com `type.collection_hash_unsupported`
- **Checker:** `stdlib_native_runtime_available()` — warning para funções stdlib sem runtime nativo (`bind.stdlib_module_unavailable`)
- **Resolver:** Validação de campos duplicados em struct (`bind.duplicate_field`)
- **Resolver:** Validação de variantes duplicadas em enum (`bind.duplicate_variant`)
- **Resolver:** Validação de campos duplicados em variantes de enum (`bind.duplicate_field`)
- **Lexer:** Aceita BOM UTF-8 no início do arquivo e rejeita no meio
- **Lexer:** `find_unclosed_block_comment()` respeita strings, bytes, f-strings e triple-quoted
- **Lexer:** Diagnóstico dedicado `lex.unclosed_block_comment` com span e ação
- **Literal parser:** `parse_int_literal()` e `parse_float_literal()` com validação de sufixos, overflow e range
- **Parser:** `expr_to_lvalue_or_error()` emite `parse.invalid_lvalue` em vez de descartar silenciosamente
- **C Backend:** Propagação correta de `?` com cleanup de escopo para `result` e `optional`
- **C Backend:** `ori_abort_bounds` para acesso fora de limites em listas
- **Stdlib:** `ori.panic` como built-in com tipo `never`
- **Stdlib:** Novos módulos: `ori.deque`, `ori.queue`, `ori.stack`, `ori.linked_list`, `ori.doubly_linked_list`, `ori.tree`, `ori.hash_table`, `ori.graph`, `ori.heap`
- **Stdlib:** Novas funções em `ori.list`: `try_get`, `is_empty`, `clear`, `clone`, `to_list`, `from_list`, `try_pop`, `try_remove`
- **Stdlib:** Novas funções em `ori.map`: `try_get`, `is_empty`, `capacity`, `reserve`, `clear`, `clone`, `from_entries`, `try_remove`
- **Stdlib:** Novas funções em `ori.set`: `is_empty`, `capacity`, `reserve`, `clear`, `clone`, `to_list`, `from_list`, `try_remove`
- **Stdlib:** `ori.string.parse_int`, `ori.string.parse_float` com tipo `result<T, string>`
- **Stdlib:** `ori.string.index_of`, `ori.string.join`, `ori.string.repeat`, `ori.string.pad_left`, `ori.string.pad_right`
- **Stdlib:** `ori.string.to_bytes`, `ori.string.from_bytes`
- **Stdlib:** `ori.bytes` com `len`, `concat`, `slice`, `to_hex`, `from_hex`, `decode_utf8`, `get`
- **Stdlib:** `ori.convert` com `float_to_string`, `bool_to_string`, `string_to_int`, `string_to_float`
- **Stdlib:** `ori.iter` com `any`, `all`, `count_where`, `take`, `skip`, `reverse`, `reduce`, `find`, `sort`, `sort_by`, `unique`, `flat_map`, `zip`, `partition`, `group_by`, `flatten`
- **Stdlib:** `ori.random.choice`, `ori.random.shuffle`
- **Stdlib:** `ori.json.stringify_pretty`
- **Stdlib:** `ori.lazy.once`, `ori.lazy.force` (declarados, sem runtime nativo)
- **LSP:** Servidor LSP funcional com diagnostics, hover, go-to-definition, completions de stdlib
- **LSP:** Índice semântico para hover de structs, enums, traits, funções e bindings locais
- **LSP:** Suporte a texto em buffer (didOpen/didChange) + fallback a arquivo em disco
- **LSP:** Refatoração modular (Sprint 1): main.rs focado em orquestração, handlers/ (diagnostics, hover, completion), index/ (semantic, project), utils/ (position, uri)
- **LSP:** Sprint 2 — context-aware completions (AfterDot, Import, Default), find references (word-boundary scan), cross-file goto-definition (resolve imports via AST)
- **LSP:** Sprint 3 — diagnósticos com debounce (300ms), Document Symbols hierárquico, Code Actions (quick fixes para `type.unused_result`), Lint warnings (`lint.unused_variable`, `lint.prefer_const`)
- **Spec:** Capítulo 14 — Backend Support
- **Spec:** Capítulo 15 — Stdlib Maintenance
- **Spec:** Capítulo 16 — Runtime FFI Safety
- **CI:** `native-route.yml` validando Windows MSVC, Windows GNU, Linux GNU, macOS x86_64, macOS aarch64
- **Tooling:** `smoke_native_release.ps1` / `.sh` para validação de release package
- **Tooling:** `ORI_REQUIRE_PACKAGED_RUNTIME=1` para validar package de release

### Corrigido
- **Lexer:** BOM UTF-8 rejeitado → aceito no início do arquivo
- **Lexer:** `--|` dentro de strings tratado como comentário → tratado como texto
- **Lexer:** Comentário não fechado virava erro genérico → diagnóstico dedicado
- **Parser:** `b.value = 2` descartado silenciosamente → emite `parse.invalid_lvalue`
- **Parser:** Variadic `...` não parseava → parseia `...` e `..` (compat)
- **Parser:** Default antes de required não validado → emite `parse.default_before_required`
- **Checker:** Nomes desconhecidos passavam como `Ty::Infer(0)` → emitem `name.undefined` + `Ty::Error`
- **Checker:** `and`/`or`/`not` não validavam booleanos → validam com `expect_bool()`
- **Checker:** `break`/`continue` fora de loop passavam → emitem `control.loop_required`
- **Checker:** Result descartado sem warning → emite `type.unused_result`
- **Checker:** Closure capturando `var` → emite `mut.closure_captures_var`
- **Checker:** Literais numéricos corrompidos para zero → validados com diagnóstico
- **Codegen:** `?` no backend C sem propagação → propaga com cleanup de escopo
- **Codegen:** Runtime bounds não seguiam spec → `ori_abort_bounds` para out-of-bounds
- **Stdlib:** `panic`/`todo`/`unreachable` não implementados → implementados
- **Stdlib:** `.or`/`.or_return`/`.or_wrap` inexistentes → suporte parcial implementado
- **CLI:** `ori compile` help dizia "no C compiler needed" → atualizado para refletir dependência de linker
- **Resolver:** Campos/variantes duplicados em struct/enum não diagnosticados → emite `name.duplicate_field` / `name.duplicate_variant`
- **Lexer:** `check_unclosed_block_comments()` era no-op → removida (lógica já está em `find_unclosed_block_comment`)
- **Cargo:** Lock file v4 ilegível por Rust 1.75 → downgradado para v3
- **Spec:** `math.floor/ceil/round` tipo de retorno divergente → alinhado (`-> int`)
- **Stdlib:** `stdlib_native_runtime_available()` adicionada como infraestrutura para detectar funções sem runtime nativo

### Alterado
- **CLI:** `ori compile` é a rota nativa principal; `ori build` é o C debug backend
- **CLI:** `ori test` usa a rota nativa, não depende do C backend
- **Runtime:** `ori-runtime` (Rust) é a fonte canônica de semântica de runtime
- **Stdlib:** Manifesto centralizado em `compiler/crates/ori-types/src/stdlib.rs`
- **Documentação:** Reorganização de `docs/planning/` e `docs/spec/`

### Segurança
- **Runtime FFI:** Documentadas regras de ownership, ARC e transferência para strings, bytes, collections (spec capítulo 16)

---

## [0.1.0] — 2026-05-17 (Estado Atual)

### Adicionado
- Compilador completo escrito em Rust (~25K linhas)
- 10 crates: lexer, parser, AST, types, HIR, codegen (C + Cranelift nativo), runtime, diagnostics, LSP, driver
- Lexer com suporte a 65+ palavras-chave, BOM, todos os literais, comentários, strings
- Parser recursivo descendente com recuperação de erros
- Type checker com inferência, genéricos, traits, implementações, contratos, where constraints
- HIR com monomorphization, lowering de closures, async state machine
- Backend nativo via Cranelift com ARC, async, closures, managed types
- Backend C (debug) com runtime inline, suporte parcial
- Runtime Rust como static library com ARC, executor async, channels, atomics
- Standard library: io, string, list, map, set, math, time, format, os, random, json, fs, bytes, convert, test, task, channel, atomic, deque, queue, stack, linked_list, doubly_linked_list, tree, hash_table, graph, heap, iter, lazy
- LSP server com diagnostics, hover, go-to-definition, completions
- CLI: `check`, `compile`, `build`, `test`, `run`, `fmt`
- Multi-file imports com resolução de namespaces
- Async/await com state machine nativa e executor não-bloqueante
- Especificação formal da linguagem (16 capítulos)
- CI/CD multi-plataforma para rota nativa

### Não implementado (planejado)
- `ori.Error` como tipo rico de erro (atual: `string`)
- `.or()` / `.or_return()` / `.or_wrap()` completos
- Cycle collector para ARC
- `ori.fs.File` como tipo
- `using` dentro de `async func`
- Cancelamento público de futures/tasks
- Type alias no lado esquerdo de `where` constraints
- `lazy` runtime nativo
- `ori.iter` runtime nativo (apenas C backend)
