# Plano de correcao e implementacao da linguagem Ori

Status: atualizado em 2026-05-17.

Legenda:

- `[x]` concluido e validado.
- `pendente` exige nova etapa ou decisao explicita.

Objetivo: corrigir bugs reais da linguagem, fechar vazamentos de memoria, alinhar runtime/codegen/testes e manter as dividas estruturais visiveis sem misturar mudancas de risco.

## 0. Resultado desta rodada

- [x] Corrigido contrato binario de `bytes` com NUL.
- [x] Corrigido `fs.read_bytes` e `fs.write_bytes` para preservar `\0`.
- [x] Corrigido `bytes.decode_utf8` e `string.from_bytes` para rejeitar NUL enquanto `string` continuar nul-terminated.
- [x] Corrigido `string.index_of` para retornar indice por caractere Unicode, nao byte offset.
- [x] Centralizado ownership ARC de `tree` e `graph` no runtime.
- [x] Removidos registros ARC duplicados do backend nativo para `tree` e `graph`.
- [x] Corrigidas posicoes Unicode/CRLF no LSP: diagnosticos, hover e go-to-definition.
- [x] Mantida correcao de ARC em `list`, `deque`, `set` e `map`.
- [x] Mantida correcao do vazamento do comparador customizado de `heap`.
- [x] Mantida correcao do `clippy` bloqueante em `ori_heap_into_sorted_list`.
- [x] Adicionados testes de regressao para os bugs corrigidos.
- [x] Rodados gates de qualidade principais.

## 1. Validacao executada

- [x] `git status --short`.
- [x] `cargo fmt --all`.
- [x] `cargo fmt --all -- --check`.
- [x] `cargo check --workspace`.
- [x] `cargo test -p ori-runtime -- --test-threads=1`.
- [x] `cargo test -p ori-lsp`.
- [x] `cargo build -p ori-driver`.
- [x] `powershell -ExecutionPolicy Bypass -File tools\stage_native_runtime.ps1 -Profile debug`.
- [x] `cargo test -p ori-driver preserve_nul -- --nocapture --test-threads=1`.
- [x] `cargo test -p ori-driver compile_runs_unicode_string_len_and_slice_native -- --nocapture --test-threads=1`.
- [x] `cargo test --workspace`.
- [x] `cargo clippy --workspace --all-targets`.
- [x] `cargo run -p ori-driver --bin ori -- run examples\bytes_usage.orl`.
- [x] `cargo run -p ori-driver --bin ori -- run examples\native_showcase.orl`.
- [x] `cargo run -p ori-driver --bin ori -- run examples\collections_demo.orl`.
- [x] `cargo test -p ori-codegen simple_async_state_machine_plan_accepts_nested_await_return_expression -- --nocapture`.
- [x] `cargo test -p ori-driver compile_runs_async_await_ -- --nocapture --test-threads=1`.
- [x] `cargo test -p ori-diagnostics`.

Observacao: `cargo clippy --workspace --all-targets` terminou com exit code 0. Ainda existem avisos antigos nao bloqueantes, principalmente `missing_safety_doc` no runtime FFI e avisos de estilo em arquivos grandes.

## 2. `bytes` com NUL

Problema: `bytes` usava caminhos baseados em C string em algumas bordas, o que quebrava dados binarios com `\0`.

- [x] Mapear funcoes de runtime que tratam `bytes`.
- [x] Preservar `\x00` em literais `b"..."`.
- [x] Preservar `\x00` em `bytes.len()`.
- [x] Preservar `\x00` em `bytes.to_hex()`.
- [x] Preservar `\x00` em `string.from_hex()`.
- [x] Corrigir `fs.read_bytes` para aceitar arquivo binario com NUL.
- [x] Corrigir `fs.write_bytes` para gravar `bytes_payload`, nao `CStr`.
- [x] Definir contrato atual: `string` continua nul-terminated.
- [x] Definir contrato atual: `bytes.decode_utf8(b"A\0B")` retorna erro.
- [x] Definir contrato atual: `string.from_bytes(b"A\0B")` retorna erro.
- [x] Adicionar teste runtime para leitura/escrita de `A\0B`.
- [x] Adicionar teste nativo para `b"\x41\x00\x42"`.
- [x] Adicionar teste nativo para `fs.read_bytes` e `fs.write_bytes` com `A\0B`.

Arquivos principais:

- `compiler/crates/ori-runtime/src/lib.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`

## 3. `string` Unicode

Problema: `string.index_of` retornava byte offset, enquanto `len` e `slice` seguem indice por caractere Unicode.

- [x] Confirmar contrato: APIs de `string` usam indice por caractere Unicode.
- [x] Manter `string.len()` por caractere Unicode.
- [x] Manter `string.slice(start, end)` por caractere Unicode.
- [x] Corrigir `string.index_of` para contar `chars()` antes do match.
- [x] Garantir `-1` quando substring nao existe.
- [x] Adicionar teste para `"a\u{00e9}".index_of("\u{00e9}") == 1`.
- [x] Adicionar teste para `"\u{1f642}x".index_of("x") == 1`.
- [x] Adicionar teste nativo com `len`, `slice` e `index_of`.

Arquivos principais:

- `compiler/crates/ori-runtime/src/lib.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`

## 4. Ownership ARC em colecoes

Problema: caminhos de remocao/substituicao precisavam remover arestas ARC de forma consistente.

- [x] `list.remove` desregistra aresta.
- [x] `list.clear` desregistra arestas.
- [x] `deque.pop_front` desregistra aresta.
- [x] `deque.pop_back` desregistra aresta.
- [x] `deque.clear` desregistra arestas.
- [x] `set.remove` desregistra aresta.
- [x] `set.clear` desregistra arestas.
- [x] `map.remove` desregistra chave e valor.
- [x] `map.clear` desregistra chaves e valores.
- [x] `map.set` com substituicao libera valor antigo.
- [x] Teste cobre inserir/remover, limpar e substituir valor gerenciado.
- [x] Teste cobre duplicatas e contagem de referencias.

Teste principal:

- `collection_removal_paths_unregister_arc_edges`

## 5. `tree` e `graph`

Problema: runtime removia arestas ARC, mas insercao dependia do codegen. Isso deixava a FFI insegura para outros backends.

- [x] Decisao: runtime registra arestas de `tree` e `graph`.
- [x] `tree_push_node` registra valor gerenciado.
- [x] `tree.remove_subtree` remove aresta.
- [x] `tree.clone` e `tree.clone_subtree` usam a regra do runtime sem registro duplicado.
- [x] `graph_add_node_raw` registra no gerenciado.
- [x] `graph.remove_node` remove aresta.
- [x] `graph.clone` e `graph.transitive_closure` usam a regra do runtime sem registro duplicado.
- [x] Backend nativo nao registra arestas duplicadas para `tree.new`.
- [x] Backend nativo nao registra arestas duplicadas para `tree.add_child`.
- [x] Backend nativo nao registra arestas duplicadas para `graph.add_node`.
- [x] Backend nativo nao registra arestas duplicadas para `graph.add_edge`.
- [x] Backend nativo nao registra arestas duplicadas para `graph.add_weighted_edge`.
- [x] Teste cobre `tree` com `string`.
- [x] Teste cobre `graph` com `string`.
- [x] Teste cobre remocao de subarvore/no e fechamento por `transitive_closure`.

Arquivos principais:

- `compiler/crates/ori-runtime/src/lib.rs`
- `compiler/crates/ori-codegen/src/native_backend.rs`

## 6. `heap`

Problema original: comparador customizado fazia retain temporario sem release.

- [x] Confirmar necessidade do retain temporario durante comparacao.
- [x] Garantir release depois da comparacao.
- [x] Adicionar teste com itens gerenciados.
- [x] Adicionar teste com muitas comparacoes.
- [x] Confirmar que refcount nao cresce indefinidamente.

Teste principal:

- `heap_custom_compare_releases_temporary_retains`

## 7. `clippy`

Problema original: `clippy` falhava em `ori_heap_into_sorted_list` com `while_immutable_condition`.

- [x] Reescrever loop para evitar falso positivo.
- [x] Rodar `cargo clippy --workspace --all-targets`.
- [x] Confirmar exit code 0.
- [x] Reclassificar avisos antigos nao bloqueantes como manutencao futura, sem bloquear gate.

Avisos ainda visiveis:

- `missing_safety_doc` em muitas funcoes `unsafe extern "C"` do runtime.
- `too_many_arguments` em `ori-types` e `ori-hir`.
- Avisos de estilo menores em parser, codegen e LSP.

## 8. Async nativo

Problema: o backend nativo ainda nao cobre todos os formatos validos de `await`.

- [x] Existe lowering/state machine para formatos simples de `await`.
- [x] Existe teste positivo para `await` simples.
- [x] Existe teste positivo para dois awaits no mesmo bloco.
- [x] Existe teste positivo para `return await`.
- [x] Existe teste positivo para parametro gerenciado e binding gerenciado.
- [x] Existe teste positivo para `const x = (await value)?`.
- [x] Existe teste negativo com diagnostico claro para shape ainda nao suportado.
- [x] Implementar `await` em argumento de chamada.
- [x] Implementar `await` dentro de operador.
- [x] Implementar `await` em condicao.
- [x] Adicionar testes nativos para `await` em chamada, operador e condicao.
- [x] Reclassificar `await` dentro de `if`/blocos aninhados como trabalho futuro de CFG/continuation.
- [x] Manter teste negativo de bloco aninhado ate o lowering completo existir.

Ponto atual: a linguagem ainda nao deve ser anunciada como async nativo completo. O contrato correto e "async nativo parcial com erro claro para awaits dentro de corpos aninhados".

Decisao: nao fazer atalho com `task_block_on` nem hoist inseguro de `await` aninhado. Isso preserva semantica e evita regressao silenciosa. O fechamento correto exige state machine por CFG.

## 9. LSP: Unicode e CRLF

Problema: LSP espera colunas UTF-16, mas partes do servidor usavam bytes.

- [x] Criar conversao byte offset -> linha/coluna UTF-16.
- [x] Criar conversao linha/coluna UTF-16 -> byte offset.
- [x] Corrigir `range_for_label`.
- [x] Corrigir `offset_at_position`.
- [x] Corrigir `range_for_symbol_in_line`.
- [x] Adicionar teste com acentos antes do simbolo.
- [x] Adicionar teste com emoji antes do simbolo.
- [x] Adicionar teste com CRLF.
- [x] Avaliar CLI: coluna humana deve ser por caractere, nao por byte.
- [x] Corrigir `SourceFile::line_col` para coluna por caractere.
- [x] Ajustar underline da CLI para tamanho por caractere.

Arquivo principal:

- `compiler/crates/ori-lsp/src/main.rs`
- `compiler/crates/ori-diagnostics/src/source.rs`
- `compiler/crates/ori-driver/src/emit.rs`

## 10. Metadados da stdlib

Problema: a stdlib historicamente podia divergir entre manifesto, typecheck, HIR e backend nativo.

- [x] `STDLIB_RUNTIME_FUNCTIONS` existe como manifesto central de caminhos, aliases e simbolos.
- [x] `stdlib_func_sig` esta em `ori-types::stdlib`.
- [x] `stdlib_native_abi` esta em `ori-types::stdlib`.
- [x] HIR consulta `stdlib_func_sig`.
- [x] Codegen nativo consulta `stdlib_native_abi`.
- [x] Teste garante que entradas do manifesto resolvem tipo semantico.
- [x] Teste garante que entradas native runtime resolvem ABI nativa.
- [x] Documentar fluxo oficial para adicionar nova funcao de stdlib.
- [x] Reduzir fallback antigo em `ori-types/src/check.rs` para delegar em `ori-types::stdlib::stdlib_func_sig`.

## 11. `backend.native_unsupported`

Inventario atual dos pontos explicitos:

- [x] Async fora do subset da state machine: intencional, com erro claro.
- [x] `await` direto fora do lowering async: intencional, com erro claro.
- [x] Indexed assignment fora do subset nativo: erro defensivo; fixture positiva cobre lista suportada.
- [x] `for` iterable/element fora do ABI nativo: erro defensivo; fixtures positivas cobrem iterables suportados.
- [x] Chamadas runtime desconhecidas de map/hash_table/graph/set/tree/heap: erro correto de defesa interna.
- [x] Criar matriz publica `feature x backend`.
- [x] Separar docs entre "linguagem prometida", "implementado no nativo" e "implementado no C/debug".
- [x] Confirmar fixture positiva para indexed assignment: `compile_runs_list_index_set_and_len`.
- [x] Confirmar fixtures positivas para iterables nativos: map, iterable customizado e iter stdlib.

## 12. Arquivos gigantes e duplicacao de teste

Problema: alguns arquivos ainda sao grandes demais para revisao rapida.

- [x] Testes atuais usam helpers existentes de `TestDir`, `run_compile`, `exe_path` e normalizacao de stdout.
- [x] Extrair helper `compile_and_run`.
- [x] Padronizar helper inicial para CRLF/LF.
- [x] Dividir `compiler/crates/ori-driver/tests/multifile_imports.rs` por dominio inicial: `multifile_imports/collections.rs`.
- [x] Dividir `compiler/crates/ori-codegen/src/native_backend.rs` por responsabilidade inicial: testes em `native_backend/tests.rs`.
- [x] Dividir `compiler/crates/ori-runtime/src/lib.rs` por dominio inicial: testes em `runtime/tests.rs`.
- [x] Reduzir `compiler/crates/ori-types/src/check.rs` removendo fallback duplicado da stdlib.

Regra de seguranca: fazer cada extracao em PR/commit separado, com `cargo test --workspace` depois de cada fatia.

## 13. Runtime FFI

Problema: o runtime ainda tem muitas funcoes `unsafe extern "C"` sem documentacao `# Safety`.

- [x] Identificado pelo `clippy`.
- [x] Mantido como aviso nao bloqueante, pois o gate retorna exit code 0.
- [x] Documentar funcoes criticas de ARC e memoria.
- [x] Documentar contrato de `string` e `bytes` em `docs/spec/16-runtime-ffi-safety.md`.
- [x] Documentar contrato de colecoes em `docs/spec/16-runtime-ffi-safety.md`.
- [x] Avaliar modularizacao por dominio: manter como trabalho futuro apos split gradual do runtime.

Prioridade recomendada:

1. ARC/memoria.
2. `string`/`bytes`.
3. colecoes.
4. async/runtime.

## 14. Higiene do repositorio

Problema: existem artefatos grandes e uma decisao pendente sobre lockfile.

- [x] Confirmado que `full_diff.patch` existe no root.
- [x] Confirmado que `local_changes.patch` existe no root.
- [x] Confirmado que `Cargo.lock` existe no root.
- [x] Confirmado que `.gitignore` ignorava `Cargo.lock` por `*.lock`.
- [x] Decidir se `full_diff.patch` deve continuar versionado: manter por agora, sem apagar arquivo legado.
- [x] Decidir se `local_changes.patch` deve continuar versionado: manter por agora, sem apagar arquivo legado.
- [x] Decidir se `Cargo.lock` deve ser versionado: sim, para build reprodutivel do workspace Rust.
- [x] Ajustar `.gitignore` para permitir `Cargo.lock`.

Regra: nao remover esses arquivos sem confirmacao explicita.

## 15. Gate final

- [x] Bugs P1/P2 originais fechados ou reclassificados corretamente.
- [x] Repros principais adicionados como testes.
- [x] Runtime validado.
- [x] Driver nativo validado.
- [x] LSP validado.
- [x] Workspace testado.
- [x] Clippy executado com exit code 0.
- [x] Exemplos principais executados.
- [x] Commit final autorizado pelo pedido "prossiga ate finalizar todos esses topicos".

## 16. Trabalho futuro fora deste plano

Estes pontos nao sao bugfix pequeno. Eles ficam registrados como proximas frentes,
mas nao bloqueiam o fechamento deste plano:

1. Async nativo completo para `await` dentro de corpos aninhados: exige state machine por CFG/continuation.
2. Rustdoc `# Safety` por funcao FFI: fazer junto com split gradual de `ori-runtime/src/lib.rs`.
3. Reduzir avisos de estilo antigos de `clippy`: executar em rodada propria para evitar refactor amplo demais.
4. Continuar dividindo arquivos gigantes por dominio, em commits pequenos e testados.
5. Avaliar remocao futura de `full_diff.patch` e `local_changes.patch` apenas com confirmacao explicita.

Essa ordem reduz risco: primeiro fecha comportamento da linguagem, depois contrato publico, depois manutencao.
