# Plano de Fechamento de Lacunas — Ori Language

> Data: 2026-05-17 | Status: Em execução

Plano mestre para implementar todas as funcionalidades faltantes, corrigir bugs
e eliminar dívidas técnicas da linguagem Ori.

---

## Fase 0 — Correção de Bugs [2 itens]

### Bug 0.1: `heap_stdlib_native` — output mismatch (heap)
- **Teste:** `collections::compile_runs_heap_stdlib_native`
- **Sintoma:** Esperado "2", obtido "7"
- **Causa provável:** Lógica de heap (min-heap vs max-heap, ou ordem de extração)
- **Arquivo:** `ori-runtime/src/collections.rs` ou `ori-codegen/src/native_backend.rs`

### Bug 0.2: `custom_iterable_native` — output mismatch (iterable)
- **Teste:** `collections::compile_runs_custom_iterable_native`
- **Sintoma:** Esperado "9", obtido "3"
- **Causa provável:** Iterador customizado não avança corretamente no runtime
- **Arquivo:** `ori-runtime/src/collections.rs`

### Bug 0.3: Memory cleanup — `git gc` warnings
- `git prune` para limpar objetos unreachable no repositório

---

## Fase 1 — Features Bloqueadoras (Alta Prioridade) [7 itens]

### 1.1 Igualdade estrutural (`==` / `!=`) para todos os tipos ✅
- **Escopo:** optional<T>, result<T,E>, tuple<...>, bytes, list<T>
- **Status:** Implementado. Codegen nativo gera comparação inline para estes tipos.
  - `optional<T>`: compara tags + valores internos
  - `result<T,E>`: compara tags + ok/err valores
  - `tuple<...>`: compara elemento por elemento
  - `bytes`: chama `ori_bytes_eq` no runtime
  - `list<T>`: compara tamanho + elementos em ordem
- **Pendente:** map<K,V>, set<T>, struct — requerem runtime helpers ou trait Equatable
- **Arquivos:** `check.rs`, `native_backend.rs`

### 1.2 `.or()` / `.or_return()` / `.or_wrap()` para optional e result ✅
- **Status:** `.or()`, `.or_return()` e `.or_wrap(context)` implementados.
  - `.or(fallback)`: parser aceita `.or` como nome de membro; checker valida tipos; lowering emite `__ori_builtin_or`; backend nativo e C backend fazem unwrap com fallback preguiçoso.
  - `.or_return()`: checker valida; lowering reescreve para operador `?` (Propagate)
  - `.or_wrap(context)`: checker valida `result<T, string>`; lowering emite `__ori_builtin_or_wrap`; backend nativo e C backend mantêm `success(v)` e transformam `error(e)` em `error(context + ": " + e)`, com contexto avaliado apenas no caminho de erro.
- **Arquivos:** `parser.rs`, `parse_expr.rs`, `check.rs`, `lower.rs`, `native_backend.rs`, `c_backend.rs`, `multifile_imports.rs`

### 1.3 `ori.Error` como tipo rico de erro ✅
- **Status:** Struct `ori.Error` agora possui campo `cause: string` para encadeamento básico.
  - String vazia indica ausência de causa.
  - Futuro: migrar para `optional<any<Error>>` quando C backend suportar tipos recursivos.
- **Arquivos:** `resolve.rs`, `lower.rs`, `multifile_imports.rs`

### 1.4 `await` dentro de corpos aninhados (if/else/match/loop) ⏳
- **Status:** NÃO implementado. State machine atual só suporta awaits sequenciais.
- **Complexidade:** Alta — requer redesign da state machine para branching states.
- **Planejado para v2.**

### 1.5 `using` dentro de `async func` ✅
- **Status:** Permitido. State machine armazena recurso no frame como local.
  - Recurso é ARC-gerenciado e liberado com cleanup do frame.
  - Dispose (trait Disposable) NÃO é chamado automaticamente nos terminais — TODO.
- **Arquivos:** `check.rs`, `native_backend.rs`, `ori_spec.rs`, `concurrency_async.rs`
- **Blocante:** Sem cleanup em código async

### 1.6 `ori.fs.File` como tipo
- **Escopo:** File handle com operações (open_read, open_write, read, write, close)
- **Arquivos:** `ori-types/src/stdlib.rs`, runtime
- **Spec:** `docs/spec/12-stdlib.md:178`
- **Blocante:** API de arquivos truncada

### 1.7 Cancelamento público de futures/tasks
- **Escopo:** `task.CancelToken` público, `task.cancel(token)`
- **Arquivos:** `ori-runtime/src/task.rs`
- **Especificação:** `docs/planning/IMPLEMENTATION_CHECKLIST_2.md:221-223`

---

## Fase 2 — Compilador (Média Prioridade) [11 itens]

### 2.1 Igualdade para `bytes`
- **Arquivo:** `ori-types/src/check.rs:~5831`

### 2.2 Igualdade para `any<Trait>`
- **Arquivo:** `ori-types/src/check.rs:~2206`

### 2.3 Type alias no lado esquerdo de `where` constraints
- **Arquivos:** parser, checker

### 2.4 `Displayable` trait-driven conversion ✅
- **Status:** Implementado para tipos concretos definidos pelo usuário.
- **Escopo:** `string(value)` e f-strings chamam `display(self)` quando o tipo implementa `ori.core.Displayable`.
- **Backends:** Nativo e C backend.
- **Arquivos:** `check.rs`, `lower.rs`, `c_backend.rs`, `multifile_imports.rs`

### 2.5 Associated types em traits
- **Arquivo:** `ori-parser/src/parse_item.rs:~495`

### 2.6 Const generics
- **Arquivo:** `ori-parser/src/parser.rs:~210`

### 2.7 Higher-kinded types (HKT)
- **Arquivo:** `ori-parser/src/parser.rs:~221`

### 2.8 Equality para tipos opacos (Opaque)
- **Arquivo:** `ori-types/src/check.rs:~5850`

### 2.9 `Equatable`/`Hashable` para coleções aninhadas
- **Arquivos:** checker + runtime

### 2.10 Lazy/general iterators para handles opacos
- **Decisão:** v2 (documentar como v2)

### 2.11 Structured JSON object/array API
- **Arquivos:** stdlib + runtime

---

## Fase 3 — Runtime e ARC [2 itens]

### 3.1 Destrutores tipo-específicos completos
- **Escopo:** Todos os shapes de alocação com destruição adequada
- **Arquivos:** `ori-runtime/src/`, `ori-codegen/src/native_backend.rs`

### 3.2 Cycle collector para ARC
- **Escopo:** Detecção de ciclos em grafos de referência
- **Arquivos:** `ori-runtime/src/arc.rs` ou similar

---

## Fase 4 — LSP e Tooling [4 itens]

### 4.1 Índice semântico cross-module completo
- **Escopo:** Workspace index em vez de por-arquivo
- **Arquivos:** `ori-lsp/src/index/semantic.rs`, `project.rs`

### 4.2 Testes end-to-end do LSP
- **Escopo:** Testes de integração com tower-lsp test utilities
- **Arquivos:** `ori-lsp/tests/`

### 4.3 `ori fmt` cobertura de state machine async
- **Arquivo:** `ori-driver/src/pipeline.rs`

### 4.4 Diagnósticos de projeto
- **Códigos:** `project.circular_import`, `project.entry_not_found`, etc.
- **Arquivos:** `ori-driver/src/pipeline.rs`, checker

---

## Fase 5 — Diagnósticos Planejados [29 códigos; 6 emitidos]

Emitir o subconjunto de códigos planejados rastreados nesta fase. O catálogo
completo também mantém aliases reservados para compatibilidade de ferramentas.

Emitidos nesta fase:

- `bind.duplicate_param`
- `bind.self_outside_method`
- `parse.import_after_declaration`
- `parse.missing_else_in_if_expr`
- `parse.namespace_missing`
- `parse.namespace_not_first`

Ainda planejados:

- `bind.undefined`
- `contract.check_failure`, `contract.field_violation`, `contract.param_violation`
- `doc.missing_return`, `doc.unclosed_block`
- `extern.managed_type_in_ffi`, `extern.unknown_abi`
- `generic.ambiguous_type_arg`
- `match.duplicate_case`, `match.guard_not_exhaustive`, `match.unreachable_case`
- `mut.field_mutation_in_func`
- `parse.invalid_range`, `parse.unterminated_block`, `parse.unterminated_string`
- `project.circular_import`, `project.entry_not_found`, `project.namespace_file_mismatch`, `project.no_proj_file`
- `type.ambiguous_generic`, `type.annotation_required`, `type.equality_unsupported_field`
- `using.non_result_init`

---

## Fase 6 — Finalização

- Atualizar CHANGELOG.md com todas as mudanças
- Atualizar docs/spec/ com status "Implemented"
- Atualizar AGENTS.md com status final
- Rodar `cargo test --workspace` e garantir 100% de passagem
- `git gc` e push final

---

## Ordem de Execução

```
Fase 0 (bugs) → Fase 1 (bloqueadoras) → Fase 2 (médias) → Fase 3 (runtime)
→ Fase 4 (LSP/tooling) → Fase 5 (diagnósticos) → Fase 6 (finalização)
```

Cada item concluído será marcado com `[x]` e terá um commit dedicado.
