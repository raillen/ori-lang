# Status das Fases — Ori Language

> Última atualização: 2026-05-18
> Baseline: 100% testes passando (257/257 multifile, 102/102 ori_spec, 38/38 concurrency)

---

## Fase 0 — Correção de Bugs ✅

| Bug | Descrição | Commit |
|-----|-----------|--------|
| 0.1 | Heap custom (output mismatch) | `dd50347` — `ori_arc_register_edge` antes de `heap_push_raw` |
| 0.2 | Iterable custom (output mismatch) | `dc335b5` — retain por iteração no header do for-loop |

---

## Fase 1 — Features Bloqueadoras (7 itens)

| # | Item | Status | Notas |
|---|------|--------|-------|
| 1.1 | Igualdade estrutural (`==`/`!=`) | ✅ Parcial | `optional<T>`, `result<T,E>`, `tuple<...>`, `bytes`, `list<T>`, structs sem genéricos, `set<int|string>` e `map<int|string, V>` nos backends nativo e C. Pendente: structs genéricas e chaves customizadas |
| 1.2 | `.or()` / `.or_return()` / `.or_wrap()` | ✅ | `.or()`: parser, checker, lowering, backend nativo e C backend completos. `.or_return()`: completo (desugar → `?`). `.or_wrap(context)`: completo para `result<T, string>`, com contexto avaliado apenas em `error(_)` |
| 1.3 | `ori.Error` como tipo rico | ✅ | Campo `cause: string` adicionado; `optional<any<Error>>` bloqueado por tipos recursivos no C backend |
| 1.4 | `await` dentro de if/else/match/loop | ❌ | Requer redesign da state machine para branching states |
| 1.5 | `using` dentro de `async func` | ✅ | State machine armazena recurso no frame; dispose pendente nos terminais |
| 1.6 | `ori.fs.File` como tipo | ❌ | Requer ~15 arquivos de boilerplate (novo Ty variant, runtime, stdlib, codegen) |
| 1.7 | Cancelamento público de futures/tasks | ❌ | Requer novos tipos (`CancelToken`), runtime e API pública |

---

## Fase 2 — Compilador (11 itens)

| # | Item | Status | Notas |
|---|------|--------|-------|
| 2.1 | Igualdade para `bytes` | ✅ | Feito em 1.1 (chama `ori_bytes_eq`) |
| 2.2 | Igualdade para `any<Trait>` | ❌ | Requer method lookup via vtable |
| 2.3 | Type alias no lado esquerdo de `where` | ✅ | `resolve_trait_through_aliases()` segue cadeia de aliases |
| 2.4 | `Displayable` trait-driven conversion | ✅ | `string(value)` e f-strings chamam `display(self)` para tipos concretos definidos pelo usuário nos backends nativo e C |
| 2.5 | Associated types em traits | ❌ | Feature grande de type system |
| 2.6 | Const generics | ❌ | Feature grande de type system |
| 2.7 | Higher-kinded types (HKT) | ❌ | Feature grande de type system |
| 2.8 | Igualdade para tipos Opaque | ❌ | Deque, Queue, Stack etc. — requer runtime ou desugar |
| 2.9 | `Equatable`/`Hashable` para coleções aninhadas | ❌ | Requer trait propagation |
| 2.10 | Lazy/general iterators para handles opacos | ❌ | Documentado como v2 |
| 2.11 | Structured JSON object/array API | ❌ | `ori.json.Value` atualmente é `string`; requer tipo recursivo |

---

## Fase 3 — Runtime e ARC (2 itens)

| # | Item | Status | Notas |
|---|------|--------|-------|
| 3.1 | Destrutores tipo-específicos completos | ❌ | Todos os shapes de alocação com destruição adequada |
| 3.2 | Cycle collector para ARC | ❌ | Detecção de ciclos em grafos de referência |

---

## Fase 4 — LSP e Tooling (4 itens)

| # | Item | Status | Notas |
|---|------|--------|-------|
| 4.1 | Índice semântico cross-module completo | ❌ | Workspace index em vez de por-arquivo |
| 4.2 | Testes end-to-end do LSP | ❌ | Testes de integração com tower-lsp |
| 4.3 | `ori fmt` cobertura de state machine async | ❌ | Formatter para sintaxe async completa |
| 4.4 | Diagnósticos de projeto | ❌ | `project.circular_import`, `project.entry_not_found`, etc. |

---

## Fase 5 — Diagnósticos Planejados (29 códigos; 5 emitidos)

Subconjunto de códigos planejados rastreados nesta fase. O catálogo completo,
incluindo aliases reservados, fica em `docs/spec/13-error-catalog.md`.

| Código | Severidade | Descrição |
|--------|-----------|-----------|
| `bind.undefined` | error | Alias reservado; emitido como `name.undefined` |
| `contract.check_failure` | runtime | Falha de contrato em runtime |
| `contract.field_violation` | runtime | Violação de contrato de campo |
| `contract.param_violation` | runtime | Violação de contrato de parâmetro |
| `doc.missing_return` | warning | Documentação sem `@return` |
| `doc.unclosed_block` | error | Bloco de documentação não fechado |
| `extern.managed_type_in_ffi` | error | Tipo managed em FFI |
| `extern.unknown_abi` | error | ABI desconhecida |
| `generic.ambiguous_type_arg` | error | Argumento de tipo ambíguo |
| `match.duplicate_case` | warning | Case duplicado em match |
| `match.guard_not_exhaustive` | warning | Guarda não exaustiva |
| `match.unreachable_case` | warning | Case inalcançável |
| `mut.field_mutation_in_func` | error | Mutação de campo em função |
| `parse.invalid_range` | error | Range inválido |
| `parse.missing_else_in_if_expr` | error | `else` ausente em if-expr |
| `parse.unterminated_block` | error | Bloco não terminado |
| `parse.unterminated_string` | error | String não terminada |
| `project.circular_import` | error | Import circular |
| `project.entry_not_found` | error | Entry point não encontrado |
| `project.namespace_file_mismatch` | warning | Namespace não bate com arquivo |
| `project.no_proj_file` | error | Arquivo de projeto ausente |
| `type.ambiguous_generic` | error | Genérico ambíguo |
| `type.annotation_required` | error | Anotação de tipo necessária |
| `type.equality_unsupported_field` | error | Campo sem suporte a igualdade |
| `using.non_result_init` | error | `using` sem result |

---

## Fase 6 — Finalização

| # | Item | Status |
|---|------|--------|
| 6.1 | Atualizar CHANGELOG.md | ✅ Parcial |
| 6.2 | Atualizar `docs/spec/` com status "Implemented" | ✅ Parcial |
| 6.3 | Atualizar AGENTS.md com status final | ❌ |
| 6.4 | `cargo test --workspace` 100% | ✅ |
| 6.5 | `git gc` e push final | ❌ |

---

## Resumo Geral

| Fase | Total | Concluído | Parcial | Pendente |
|------|-------|-----------|---------|----------|
| 0 — Bugs | 2 | 2 | 0 | 0 |
| 1 — Bloqueadoras | 7 | 3 | 1 | 3 |
| 2 — Compilador | 11 | 3 | 0 | 8 |
| 3 — Runtime/ARC | 2 | 0 | 0 | 2 |
| 4 — LSP/Tooling | 4 | 0 | 0 | 4 |
| 5 — Diagnósticos | 29 | 5 | 0 | 24 |
| 6 — Finalização | 5 | 1 | 2 | 2 |
| **TOTAL** | **60** | **14** | **3** | **43** |

### Itens GRANDES (redesign de subsistemas)
1.4 await aninhado, 1.6 ori.fs.File, 1.7 Cancelamento, 2.5 Associated types, 2.6 Const generics, 2.7 HKT, 3.1 Destrutores, 3.2 Cycle collector

### Itens MÉDIOS (implementáveis com esforço moderado)
2.2 any\<Trait\> equality, 2.8 Opaque equality, 2.9 Nested Equatable/Hashable, 2.11 JSON estruturado, 4.1-4.4 LSP, 5.x Diagnósticos

### Itens PEQUENOS (documentação, testes, finalização)
2.1, 2.4 (completo), 6.1-6.5
