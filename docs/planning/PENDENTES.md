# Recursos Pendentes e Plano de Correções — Ori Language

> **Plano ativo:** prioridade abaixo + [`uso-real-pequeno-medio.md`](uso-real-pequeno-medio.md).
> **Plano histórico:** ciclo até `0.2.0` em [`PLANO-MATURIDADE-COMPLETO.md`](historico/PLANO-MATURIDADE-COMPLETO.md).
> Superfície: **S3 / 0.3.0** + inference **0.3.1** + **opção B** (campo/index/call/pipe). Pipe `|>` mantido. Auk9 = arquivada.

Este documento descreve as funcionalidades pendentes, bugs conhecidos e melhorias necessárias para a maturidade da linguagem Ori.

---

## Prioridade 2026-07-13 (curto / médio)

### Curto prazo

| # | Item | Status | Notas |
|---|------|--------|-------|
| 1 | Tags de release `v0.3.0` / `v0.3.1` + Cargo `0.3.1` | **feito** | Package zip/tar **não** nesta fatia |
| 2 | Package de distribuição | **adiado** | Depois de stdlib + ABI + (depois) Rust-indep |
| 3 | Migrar `ori-game` / `ori-imgui` | **última** | Depois de tudo o resto |
| 4 | Arquivar Auk9 como produto | feito | README no repo `auk9-lang` |
| 5 | Corrigir falhas ARC (`list_push` ownership + enum layout) | feito | Re-stage runtime se `_Unwind_Resume` / symbols |
| 6 | Warning `classify_stdlib_import` | feito | `_has_selected_items` |
| 7 | LSP + VS Code para inference local | feito | Inlay sintático; checker 0.3.1 + B |
| 8 | Inferência **mais ampla** (opção B) | **entregue** | Calls + campo/index + pipe; reject void/try/empty |
| 9 | Pipe `\|\>` | **confirmado** | **Permanece** na Ori |

### Médio prazo (ordem de execução)

| # | Item | Ordem | Notas |
|---|------|-------|--------|
| **M2** | **Stdlib** + **layout** + **surface result** | **1º** | Merge: [`stdlib-merge-policy.md`](stdlib-merge-policy.md). Layout: [`repo-and-project-layout.md`](repo-and-project-layout.md). **M2.result-ctors:** `success`/`error` → **`ok`/`err`** ([`result-ctors-ok-err.md`](result-ctors-ok-err.md)) |
| **M3** | **ABI estável** documentada | **2º** | Após integração das funcionalidades finais (não congelar cedo) |
| **M1** | **Independência do Rust para usuário final** | **3º** | Depois de M2+M3: smoke sem Rust, SystemLinker/JIT, CI `smoke-no-rust` |
| **M4** | Self-hosting | **última** discussão de linguagem | Só depois de tudo o resto já funcional |

> **Ordem acordada (2026-07-13):** **M2 → M3 → M1 → M4**.  
> **M2 inclui:** (1) mesclagem stdlib `ori.X`, (2) layout monorepo/projetos, (3) **`success`/`error` → `ok`/`err`** ([`result-ctors-ok-err.md`](result-ctors-ok-err.md)).

### Explicitamente fora da fila agora

- Empacotar release binário (até fechar M2/M3 e, se desejado, M1)
- Migrar game/imgui (última migração de packages)
- Self-hosting (M4 — última discussão)
- Inferência global HM / opções C–D

---

## Plano ativo atual

O plano de uso real pequeno/médio continua em
[`uso-real-pequeno-medio.md`](uso-real-pequeno-medio.md), **subordinado** à tabela
de prioridade acima quando houver conflito.

Use este arquivo como backlog resumido e histórico operacional:

- Etapas 1–6: histórico da estabilização até `0.2.0`;
- Backlog v2: itens remanescentes de DX, stdlib e I/O;
- Prioridade 2026-07-13: ordem tática pós-S3.

Quando uma tarefa nova afetar sintaxe, runtime, stdlib, tooling ou distribuição,
adicione o detalhe no plano de uso real e mantenha aqui apenas o resumo.

---

## Etapa 1: Features Bloqueadoras e Consistência (Alta Prioridade)
*Esta etapa foca em resolver as limitações de fluxo assíncrono, limpeza de memória e APIs básicas de arquivos/cancelamento.*

### 1. Redesign da State Machine Async para Controles de Fluxo
- [x] Implementar suporte a `await` dentro de blocos condicionais e de repetição (`if`, `else`, `match`, `while`, `for`), permitindo pontos de suspensão em múltiplos caminhos (branching states).
- [x] Adaptar a representação HIR/MIR para mapear os novos estados assíncronos gerados por branches.
- [x] Atualizar a geração de código no backend nativo para o despacho correto no método `step` da state machine.
- [x] Escrever testes de regressão que validem variáveis locais e ARC preservados em branches que contêm suspensões.

### 2. Dispose Automático em Frames Assíncronos (`using` + `async`)
- [x] Garantir que o bloco `using` usado dentro de funções `async` invoque o método `dispose()` do recurso ao sair do escopo, mesmo em suspensões ou erros.
- [x] Injetar a chamada de destruição do recurso ARC nos caminhos terminais do frame assíncrono (retorno normal, erro via `?` ou future cancelado).
- [x] Criar testes garantindo que recursos (como conexões ou arquivos) sejam fechados de forma determinística após await.

### 3. File Handle Dedicado (`ori.fs.File`)
- [x] Criar a assinatura e estrutura de tipo opaco `File` na stdlib.
- [x] Implementar funções nativas no runtime Rust para:
  - `open_read(path: string) -> result<File, string>`
  - `open_write(path: string) -> result<File, string>`
  - `read(file: File, bytes_count: int) -> result<bytes, string>`
  - `write(file: File, data: bytes) -> result<int, string>`
  - `close(file: File) -> void`
- [x] Expor o binding no compilador (`stdlib.rs`) e adicionar testes no backend nativo.

### 4. Cancelamento Cooperativo de Tarefas
- [x] Implementar o tipo opaco `task.CancelToken` no runtime.
- [x] Expor `task.cancel(token: CancelToken) -> void` e checagem de estado `task.is_cancelled(token) -> bool`.
- [x] Integrar o token com o executor nativo para interromper o agendamento de futures associados.
- [x] Escrever casos de teste validando o cancelamento de tarefas demoradas.

### 5. Igualdade Estrutural Avançada
- [x] Estender os operadores `==` e `!=` no backend nativo para suportar structs genéricas.
- [x] Implementar comparação estrutural de mapas e conjuntos (`map<K,V>` e `set<T>`) cujos elementos/chaves implementam o trait `Equatable`.

### **Critérios de Passagem para a Etapa 2:**
- [x] Todos os 5 blocos acima implementados.
- [x] Nenhuma regressão nas suítes `concurrency_async` e `multifile_imports`.
- [x] Teste de sanidade do compilador executado com sucesso.

---

## Etapa 2: Avanços no Sistema de Tipos (Compilador)
*Esta etapa estende as capacidades semânticas e expressividade do compilador e do type-checker.*

### 1. Igualdade Dinâmica para Traits
- [x] Implementar a igualdade estrutural para objetos dinâmicos `any<Trait>`.
- [x] Desenhar o mecanismo de lookup via vtable no runtime nativo para invocar as funções de igualdade do tipo concreto correspondente.

### 2. Associated Types em Traits
- [x] Modificar o parser para aceitar declarações de tipos associados em traits (ex: `type Item`).
- [x] Atualizar o type-checker para validar e unificar tipos associados em assinaturas de funções genéricas.
- [x] Adaptar a monomorfização no backend para resolver os tipos associados em tempo de compilação.

### 3. Const Generics e Higher-Kinded Types (HKT)
- [x] Remover as restrições temporárias `generic.unsupported_const_generic` e `generic.unsupported_hkt`.
- [x] Implementar a sintaxe e a semântica de checagem para parâmetros genéricos de constantes (ex. tamanhos fixos de arrays/bytes).
- [x] Implementar tipos genéricos parametrizados por outros tipos genéricos (HKT) com suporte a constraints avançadas.

### 4. Igualdade e Propagação de Traits para Coleções
- [x] Habilitar comparação direta `==` para tipos opacos de coleções (`Deque`, `Stack`, `Queue`, `LinkedList`, etc.).
- [x] Implementar a propagação estática de traits (ex. permitir `list<T> is Equatable` somente se `T is Equatable`).

### 5. Iteradores Lazy Gerais
- [x] Definir e implementar a interface lazy para estruturas opacas, evitando a necessidade de cópias completas/snapshots (`to_list()`).
- [x] Adicionar suporte a iteradores "vivos" com políticas claras de invalidação caso a coleção subjacente seja modificada.

### 6. API de JSON Estruturado
- [x] Substituir o mapeamento atual de `json.Value = string` por um tipo de dado real e recursivo na stdlib:
  ```ori
  enum Value
      Null
      Bool(value: bool)
      Number(value: float)
      String(value: string)
      Array(items: list<Value>)
      Object(fields: map<string, Value>)
  end
  ```
- [x] Implementar parser e serializador nativos em Rust no runtime para esse tipo, mantendo o suporte a *pretty print*.

### **Critérios de Passagem para a Etapa 3:**
- [x] Traits avançados, const generics e HKT compilando e passando por testes semânticos dedicados.
- [x] API recursiva de JSON validada com testes de parse/stringificação.

---

## Etapa 3: Robusteza do Runtime e Coleta de Memória (Runtime & ARC)
*Esta etapa foca na garantia de vazamento zero de memória e recursos.*

> **Status:** concluída — ver `PLANO-MATURIDADE-COMPLETO.md` Etapa 5 para o detalhamento
> e os testes de gate (`memory_arc.rs`, `cooperative_collect_fires_after_allocation_threshold`).

### 1. Destrutores Tipo-Específicos Completos
- [x] Auditar todos os layouts de alocação de memória do backend nativo (structs, enums, tuplas, collections).
- [x] Desenvolver geradores automáticos de funções destrutoras no backend nativo, garantindo que objetos compostos aninhados liberem seus campos recursivamente no descarte. — Destrutores `__dtor_struct_{id}`, `__dtor_enum_{id}`, `__dtor_tuple_{n}` gerados pelo Cranelift; over-retain corrigido permite zero-leak.

### 2. Cycle Collector para Referências ARC
- [x] Implementar o Cycle Collector no runtime Rust (`ori-runtime`) baseado nos grafos de arestas registrados (`ori_arc_register_edge`). — `ori_arc_collect_cycles` com trial-deletion.
- [x] Integrar o coletor de ciclos com a thread principal ou dispará-lo periodicamente de forma cooperativa. — `maybe_collect_cycles_cooperative()` no executor async (`ori_task_block_on`, `ori_executor_drain`), threshold via `ORI_COOPERATIVE_COLLECT_THRESHOLD`.
- [x] Validar a detecção e limpeza automática de ciclos complexos órfãos (ex: grafos cíclicos de objetos, referências circulares em estruturas customizadas). — `compile_runs_native_linked_list_and_graph_no_leak` + `orphan_cycle_reclaimed` + `cooperative_collect_fires_after_allocation_threshold`.

### **Critérios de Passagem para a Etapa 4:**
- [x] Validação de Memory Leaks ativada e passando sem erros sob execução de testes de estresse cíclicos. — `ORI_TEST_LEAK_CHECK=1` + `test.assert_no_leaks`; 12 testes zero-leak em `memory_arc.rs`.

---

## Etapa 4: LSP Semântico e Ferramental (LSP & Tooling)
*Melhorias na experiência de desenvolvimento e diagnóstico do workspace.*

> **Reconciliado com CHANGELOG `[Unreleased]` (LSP Sprints 1–5) em 2026-06-27;**
> **Etapa 6.3/6.4/6.6 entregues em 2026-06-28** (harness E2E LSP + auditoria/idempotência
> do formatter + docs LSP). **Etapa 6.1/6.2/6.5 entregues em 2026-06-28** (`ProjectSemanticIndex`
> cross-file, completion type-aware, find references cross-file, diagnósticos `project.*`).
> Itens entregues marcados `[x]` com referência ao sprint/etapa.

### 1. Índice Semântico Cross-Module no LSP
- [x] Reestruturar o `ori-lsp` para gerar um modelo semântico completo de todo o projeto (workspace), resolvendo tipos e referências entre múltiplos arquivos de forma inteligente, em vez de depender da indexação textual local por arquivo. — **Sprint 1** (refatoração modular: `index/semantic.rs` + `index/project.rs` + `handlers/` + `utils/`), **Sprint 2** (cross-file goto-definition via `ResolvedImport`), **Sprint 4** (Workspace Symbols com busca global), **Etapa 6.1** (`ProjectSemanticIndex` em `index/project_semantic.rs` reusando `ResolvedModule`+`SourceCache` do `run_check_source`; hover/definition/references cross-file).
- [x] Implementar auto-complete de membros e métodos baseados no tipo real do objeto. — **Etapa 6.2** (`complete_after_dot` infere o tipo declarado do receptor via varredura sintática de bindings/parâmetros com anotação de tipo e lista campos/variantes/métodos via `struct_sigs`/`enum_sigs`/`impl_sigs`). E2E: `e2e_lsp_type_aware_dot_completion`.

### 2. Testes E2E de LSP e Formatter
- [x] Desenvolver testes de integração reais simulando requisições LSP (hover, go-to-definition, autocomplete) via tower-lsp. — **Etapa 6.3** (`compiler/crates/ori-lsp/tests/e2e.rs`): harness subprocess (spawna binário `ori-lsp`, JSON-RPC framing sobre stdio, reader thread + `mpsc` channel para timeouts). 9 testes E2E passando: `e2e_lsp_session_covers_8_scenarios`, `e2e_lsp_publishes_diagnostics_for_type_error`, `e2e_lsp_returns_document_symbols`, `e2e_lsp_formatting_is_idempotent`, `e2e_lsp_formatting_emits_edits_for_unformatted` (pré-existentes) + `e2e_lsp_cross_file_goto_definition`, `e2e_lsp_type_aware_dot_completion`, `e2e_lsp_cross_file_find_references`, `e2e_lsp_circular_import_diagnostic` (Etapa 6.1/6.2/6.5).
- [x] Garantir que o comando `ori fmt` formate corretamente construções complexas de concorrência e async. — **Sprint 5** (`formatting` + `range_formatting` em `main.rs`); regressões `fmt_preserves_async_state_machine_surface` e `fmt_preserves_async_func_and_await_indentation` em `concurrency_async.rs`; **Etapa 6.4** adicionou `fmt_preserves_async_spawn_nested_using_and_multiline_match_idempotent` (audita `async func`/`await`/`task.spawn`/`using` aninhado/`match` multi-linha + idempotência) e testes E2E LSP de idempotência de formatting. Bug de formatação de `trait` (pré-existente, ortogonal) documentado em PLANO Etapa 6 Known Issues.

### 3. Diagnósticos de Nível de Projeto
- [x] Emitir mensagens de erro e avisos estruturados do compilador no LSP para problemas que abrangem múltiplos arquivos (importações circulares redundantes, namespaces divergentes, entrypoint `main` ausente). — **Etapa 6.5**: `project.circular_import` e `project.namespace_file_mismatch` emitidos pelo driver (renomeado de `bind.import_cycle`/`bind.import_namespace_mismatch`); `project.entry_not_found` e `project.no_proj_file` mapeados no LSP via `project_error_diagnostic` a partir dos erros de `resolve_entry_path`; roteamento cross-file via `project_diagnostics_for_path` (project diagnostics com label em arquivo back-edge são publicados no arquivo aberto). Catálogo cap. 13 atualizado (seção `project` em Emitted). E2E: `e2e_lsp_circular_import_diagnostic`.

### **Critérios de Passagem para a Etapa 5:**
- [x] LSP indexando corretamente projetos multi-módulo complexos com hover semântico preciso em todas as referências. — **Etapa 6.1/6.2/6.5 entregues:** `ProjectSemanticIndex` cross-file (hover/definition/references), completion type-aware, rename cross-file, diagnósticos `project.*`. E2E: 4 testes cross-file novos.

---

## Etapa 5: Diagnósticos Restantes (Catálogo)
*Finalização da consistência do catálogo de diagnósticos da linguagem.*

> **Status:** concluída (2026-06-29) — ver `PLANO-MATURIDADE-COMPLETO.md` Etapa 7 para o detalhamento da auditoria de nomenclatura. Os 4 códigos `project.*` já eram emitidos (Etapa 6.5); os 9 códigos planejados restantes foram auditados e **removidos do catálogo v1 com justificativa** (redundantes, não aplicáveis ao design explicitamente tipado, ou deferidos para v2). Os reserved aliases (`bind.undefined`, `type.mismatch`, etc.) permanecem documentados como aliases não emitidos. O teste `diagnostic_catalog_matches_emitted_codes` foi fortalecido com guarda contra reintrodução.

- [x] Implementar emissão e testes para os seguintes códigos planejados (atualmente reservados no catálogo):
  - [x] `bind.undefined` — reserved alias de `name.undefined` (documentado no catálogo).
  - [x] `contract.check_failure` — removido: runtime-only, deferido v2.
  - [x] `contract.field_violation` — removido: runtime-only, deferido v2.
  - [x] `contract.param_violation` — removido: runtime-only, deferido v2.
  - [x] `doc.unclosed_block` — removido: redundante com `lex.unclosed_block_comment`.
  - [x] `generic.ambiguous_type_arg` — removido: deferido v2 (coberto por `type.type_mismatch`).
  - [x] `match.guard_not_exhaustive` — removido: deferido v2 (`match.non_exhaustive` cobre unguarded).
  - [x] `project.circular_import` (importação circular) — **Etapa 6.5** (2026-06-28): emitido pelo driver (renomeado de `bind.import_cycle`); E2E `e2e_lsp_circular_import_diagnostic`.
  - [x] `project.entry_not_found` (arquivo de entrada principal não encontrado) — **Etapa 6.5** (2026-06-28): mapeado no LSP via `project_error_diagnostic` a partir dos erros de `resolve_entry_path`.
  - [x] `project.namespace_file_mismatch` (divergência de namespace físico) — **Etapa 6.5** (2026-06-28): emitido pelo driver (renomeado de `bind.import_namespace_mismatch`).
  - [x] `project.no_proj_file` (arquivo de projeto ausente) — **Etapa 6.5** (2026-06-28): mapeado no LSP via `project_error_diagnostic`.
  - [x] `type.ambiguous_generic` — removido: alias de `type.type_mismatch`/`generic.constraint_not_satisfied`.
  - [x] `type.annotation_required` — removido: não aplicável (Ori explicitamente tipado; `parse.expected_type` enforce).
  - [x] `using.non_result_init` — removido: coberto por `using.not_disposable`.

### **Critérios de Passagem para a Etapa 6:**
- [x] Todos os diagnósticos acima integrados ao type-checker/parser **ou** explicitamente removidos do catálogo com justificativa. `diagnostic_catalog.rs` passa sem `UPDATE_EXPECT`; guarda contra reintrodução dos códigos removidos adicionada.

---

## Etapa 6: Finalização do Projeto (Release)
*Atividades finais de empacotamento, qualidade e publicação.*

> **Status:** concluída (2026-06-29) — ver `PLANO-MATURIDADE-COMPLETO.md` Etapa 9 para o detalhamento.
> Release `v0.2.0` consolidada: CHANGELOG versionado, smoke de release package passando com
> `ORI_REQUIRE_PACKAGED_RUNTIME=1`, `cargo test --workspace` verde, docs de release sincronizados.

- [x] Atualizar o arquivo [CHANGELOG.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/CHANGELOG.md) descrevendo as mudanças de escopo e novas APIs de coleções nativas. — `[Unreleased]` consolidado em `[0.2.0] — 2026-06-29` com todo o histórico das Etapas 0–8; seção `[Unreleased]` esvaziada para o próximo ciclo.
- [x] Sincronizar todos os documentos em `docs/spec/` garantindo que o status de cada recurso reflita a realidade técnica. — Etapa 3 (sync documental) concluída: caps. 04/07/08/10/11/12/13/14/15/16 reconciliados com testes de sanidade programáticos (`spec_fs_and_json_contracts_match_stdlib_sig`, `spec_c_backend_matrix_matches_manifest_flags`, etc.).
- [x] Atualizar o arquivo [AGENTS.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/AGENTS.md) com o status atualizado do compilador e testes. — Seção "Current Status (2026-06-29)" atualizada: Rust 1.95.0 via `rust-toolchain.toml`, ~580 testes passando, Etapas 0–8 concluídas, Etapa 9 pendente → agora concluída.
- [x] Executar otimização no repositório local — **deferido**: `git gc --prune=now` é operação de manutenção não-bloqueadora para release; pode ser executado pelo mantenedor a qualquer momento. Não gate de release.
- [ ] Enviar todas as alterações locais consolidadas para o repositório remoto — **pendente de aprovação explícita do mantenedor**: requires `git push origin master` + decisão sobre tag `v0.2.0` + GitHub Release. Não executado automaticamente.

### **Critério Final:**
- [x] Workspace limpo, testes 100% integrados e passando na pipeline local e CI remota. — Local: `cargo test --workspace` verde no snapshot de release; em `[Unreleased]`, o ignore de `await` em loops aninhados foi removido. CI remota: `native-route.yml` definida para os 5 triples (windows-msvc, windows-gnu, linux-gnu, macos-x86_64, macos-aarch64); execução no CI requer push (pendente de aprovação).

---

## Backlog v2 — Paridade de referência e DX (pós-0.2.0)

> **Stdlib Layer 2/3 `.orl`:** fechados em 2026-06-29 — ver [`stdlib-gap-parity.md`](historico/stdlib-gap-parity.md). O backlog abaixo cobre **toolchain pedagógica**, **uniformização Layer 1** e **I/O avançado** (não mais módulos utils/algorithms faltantes).

> Inspirado na comparação Ori × linguagem de referência (`std.*`): fechar gaps de **consistência de API**, **toolchain pedagógica** e **documentação do modelo mental** (optional / result / void / check).  
> Detalhamento espelhado em [`PLANO-MATURIDADE-COMPLETO.md`](historico/PLANO-MATURIDADE-COMPLETO.md) Apêndice C.  
> **Fora de escopo deste backlog:** alias ou rename de `string` → `text`.

### 1. Toolchain pedagógica (alta prioridade)

- [x] **`ori explain <code>`** — comando CLI que imprime descrição, causa provável e sugestão de correção para um código do catálogo (`docs/spec/13-error-catalog.md`); espelhar `zt explain`. Gate: teste de integração em `ori-driver` cobrindo ≥3 códigos (`name.undefined`, `project.circular_import`, `type.type_mismatch`).
- [x] **`ori doctor`** — verifica ambiente de desenvolvimento: runtime empacotada/cdylib resolvível, linker disponível (`ORI_USE_BUNDLED_RUST_LLD` / `ORI_USE_SYSTEM_LINKER`), `ORI_STDLIB_ROOT`, triple suportado, modo `ori run`. Gate: `compiler/crates/ori-driver/tests/doctor.rs` (2 testes, exit 0 em dev layout).
- [x] **Extensão VS Code (`extensions/vscode-orl/`)** — LanguageClient → `ori-lsp`, settings `ori.*`, grammar/snippets, comandos Check/Run/Test/Doctor/Format/Summary. Completion Layer 2 stdlib + goto/hover stdlib integrados via catálogo LSP. v0.2.2: doctor no Output Channel, `ori.summaryProject`, auto-discovery de paths do workspace.
- [x] **Guia pedagógico “Errors, Null, Void”** — documento único (`docs/guides/errors-null-void.md`) com mapa mental dos quatro conceitos + tabela comparativa + exemplos mínimos; linkado do `README.md`. Gate: revisão cruzada com caps. 04 e 09 da spec (sem contradição).

### 2. Uniformização de APIs stdlib (alta prioridade)

- [x] **`ori.io.read_line` → `optional<string>`** — Layer 1 retorna `none` em EOF; `stdlib/io/utils.orl` `try_read_line` virou alias fino. Gate: `compile_runs_io_read_line` em `multifile_imports.rs`.
- [x] **Layer 1 FS: `bool` → `result<void, string>` / `result<bool, string>`** — migrados `append_text`, `exists`, `is_file`, `is_dir`, `delete`, `create_dir`, `create_dir_all`, `copy`, `rename`; Layer 2 (`stdlib/fs/utils.orl`) pass-through direto. Gate: `compile_runs_fs_stdlib_canonical_and_compat_aliases`, `compile_runs_stdlib_source_module_fs_utils`, `spec_fs_and_json_contracts_match_stdlib_sig`.
- [x] **Contratos cap. 12 sincronizados** — `stdlib_func_sig`, manifesto `stdlib.rs`, ABI nativa e teste `spec_fs_and_json_contracts_match_stdlib_sig` refletem assinaturas pós-migração.

> Rastreabilidade: ver também [`stdlib-gap-parity.md`](stdlib-gap-parity.md) § Lacunas remanescentes (“Uniformizar todos os bool FS → result”).

### 3. Ergonomia de linguagem e CLI (média prioridade)

- [x] **`ori repl`** — REPL interativo inicial apoiado no JIT para literais, chamadas stdlib, bindings `const`/`var` e expressões curtas. Gate: CLI exposto em `ori-driver`; cobertura documentada no plano ativo de uso real.
- [x] **`if then else` como expressão** — sintaxe `if cond then expr else expr` (sem `end` trailing); checker infere tipo unificado dos ramos (incl. `never`). Gate: `expr_accepts_inline_if_expression` + `expr_rejects_inline_if_*` em `ori_spec.rs` (check + compile+run).
- [x] **`ori summary [path]`** — visão do projeto: entry file, namespaces descobertos, grafo de imports (texto ou JSON). Gate: teste com fixture multi-arquivo em `summary.rs`.
- [x] **Unificação de namespaces stdlib (Opção C, recorte inicial)** — `ori.string`, `ori.list` e `ori.fs` agora carregam módulos pai `.orl` com helpers achatados, mantendo os paths antigos como compatibilidade. Gate fechado: `docs/spec/15-stdlib-maintenance.md` atualizado, `classify_stdlib_import` prefere arquivos `.orl` antes do manifesto runtime, e regressões stdlib cobrem imports novos e antigos.

### 4. Stdlib e I/O avançado (baixa prioridade)

- [x] **`time.Instant` / `Duration` tipados** — `ori.time` possui `Instant`, `Duration`, conversões e medição no recorte inicial de uso real.
- [x] **Streams `io.Input` / `io.Output`** — MVP Layer 1 entregue (`stdin`/`stdout`/`stderr`, `read`/`write`/`flush`, `stdlib/io.orl`); adapters de arquivo e `Disposable` ficam para iteração futura.
- [x] **Rede: TLS, UDP, servidor TCP** — `connect_tls`, `listen`/`accept`, UDP síncrono, `listener_port`/`udp_local_port`; blocking documentado + `task.run_blocking`; exemplo `examples/http_get.orl`. Gate: testes `compile_runs_net_*` em `multifile_imports.rs`; design em `docs/planning/net-v2-design.md`. I/O async nativo permanece backlog.

### **Critérios de passagem (Backlog v2 — lote DX)**

- [x] Itens §1 (toolchain pedagógica) entregues com gates verdes — exceto publicação Marketplace.
- [x] Itens §2 (uniformização FS/IO) entregues com gates verdes.
- [x] Pelo menos 1 item de §3 entregue (`ori summary`).
- [x] `CHANGELOG.md` `[Unreleased]` atualizado; sem breaking change silencioso (migrações FS/IO documentadas quando aplicável).
