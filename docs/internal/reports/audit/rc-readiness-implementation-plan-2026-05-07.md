# Plano de implementacao das pendencias para RC

Data: 2026-05-07

Objetivo: fechar as pendencias restantes antes de declarar RC publica ou Estavel.

Regra de execucao:

- Concluir cada etapa antes de iniciar a proxima.
- Marcar todos os checkboxes da etapa.
- Registrar evidencia objetiva: comando, resultado, arquivo de log ou relatorio gerado.
- Se uma etapa falhar, corrigir a falha antes de prosseguir.

## Estado inicial

P1 funcional conhecido: fechado.

Pendencias atuais:

- Evidencia de sanitizer em ambiente compativel.
- Evidencia de Valgrind ou ferramenta equivalente.
- Evidencia de performance completa, alem de `perf/quick`.
- Clareza do contrato publico de `std.collections`.
- Consolidacao final do relatorio de RC.

## Etapa 1 - Congelar o contrato de `std.collections`

Meta: remover ambiguidade entre colecoes genericas reais e colecoes avancadas especializadas.

- [ ] Confirmar, no codigo atual, a matriz suportada de `std.collections`.
- [ ] Registrar que `list<T>`, `map<K,V>` e `set<T>` usam runtime generico quando suportado pelo backend.
- [ ] Registrar que `grid2d`, `grid3d`, `pqueue`, `circbuf`, `btreemap` e `btreeset` sao especializadas no v1.
- [ ] Documentar a matriz publica suportada:
  - [ ] `queue` e `stack`: APIs `int`/`text`, mais snapshots `queue_values<T>` e `stack_values<T>`.
  - [ ] `grid2d` e `grid3d`: `int` e `text`.
  - [ ] `pqueue` e `circbuf`: `int` e `text`.
  - [ ] `btreemap`: `text, text`.
  - [ ] `btreeset`: `text`.
  - [ ] HOFs em `std.collections`: `map_int`, `filter_int`, `reduce_int`.
- [ ] Atualizar `docs/spec/language/stdlib-model.md` com a diferenca entre generico real e especializacao atual.
- [ ] Atualizar `docs/spec/language/stdlib-reference-by-topic.md` com a matriz suportada.
- [ ] Atualizar `stdlib/zdoc/std/collections.zdoc` se a ZDoc publica ainda permitir leitura ambigua.
- [ ] Adicionar ou revisar fixtures negativas para instanciacoes nao suportadas, se o diagnostico atual for confuso.
- [ ] Rodar `.\zt.exe doc check zenith.ztproj`.
- [ ] Rodar `python tools/check_docs_current_syntax.py`.
- [ ] Rodar os fixtures direcionados de `std.collections`.

Gate de avanco:

- [ ] A documentacao nao promete `std.collections` generica para qualquer `T/K/V`.
- [ ] Os testes direcionados passam no escopo documentado.
- [ ] Unsupported shapes falham de forma clara ou estao explicitamente documentadas como fora do v1.

## Etapa 2 - Confirmar sanitizer em ambiente de release

Meta: obter evidencia real de ASAN/UBSAN ou equivalente.

- [ ] Confirmar que o job Linux de sanitizer esta presente no CI.
- [ ] Executar o CI Linux com sanitizer.
- [ ] Se o CI nao estiver disponivel, executar em ambiente local compativel com clang/gcc e flags ASAN/UBSAN.
- [ ] Arquivar o resultado em `docs/internal/reports/audit/evidence/`.
- [ ] Se houver falha, abrir item corretivo com arquivo, comando e stack trace.
- [ ] Corrigir qualquer falha de sanitizer antes de avancar.
- [ ] Reexecutar o sanitizer depois da correcao.

Gate de avanco:

- [ ] Sanitizer passou em ambiente compativel.
- [ ] A evidencia esta arquivada.
- [ ] Nenhuma falha de memoria/UB ficou sem decisao.

## Etapa 3 - Rodar Valgrind ou ferramenta equivalente

Meta: complementar sanitizer com uma segunda evidencia de memoria.

- [ ] Escolher a ferramenta oficial para esta rodada: Valgrind, Dr. Memory, LLVM leak sanitizer ou equivalente documentado.
- [ ] Definir o corpus minimo: runtime core, stdlib core, collections, ARC/ORC e fuzz replay.
- [ ] Rodar a ferramenta no corpus minimo.
- [ ] Arquivar logs em `docs/internal/reports/audit/evidence/`.
- [ ] Classificar cada achado como falha real, falso positivo ou limitacao da ferramenta.
- [ ] Corrigir falhas reais antes de avancar.
- [ ] Reexecutar a ferramenta depois das correcoes.

Gate de avanco:

- [ ] A ferramenta passou sem vazamentos/UB relevantes.
- [ ] Falsos positivos estao documentados com justificativa curta.
- [ ] A evidencia esta arquivada.

## Etapa 4 - Reexecutar performance completa

Meta: substituir evidencia parcial por evidencia de release.

- [ ] Confirmar qual comando representa o gate completo de performance no estado atual do repo.
- [ ] Rodar `python run_suite.py release`.
- [ ] Rodar o gate completo de performance definido para release publica.
- [ ] Comparar resultados com baselines atuais.
- [ ] Investigar regressao acima do limite aceito.
- [ ] Atualizar baselines apenas se a mudanca for justificada e documentada.
- [ ] Arquivar relatorio gerado em `reports/suites/` ou `docs/internal/reports/audit/evidence/`.

Gate de avanco:

- [ ] `release` passa.
- [ ] Performance completa passa ou tem excecao aprovada e documentada.
- [ ] Nenhuma regressao relevante ficou sem explicacao.

## Etapa 5 - Validar comandos publicos de release

Meta: garantir que o usuario consegue usar o projeto pelos comandos oficiais.

- [ ] Rodar `python build.py`.
- [ ] Rodar `.\zt.exe check zenith.ztproj --all --ci`.
- [ ] Rodar `.\zt.exe test zenith.ztproj --ci`.
- [ ] Rodar `.\zt.exe fmt zenith.ztproj --check`.
- [ ] Rodar `.\zt.exe help zpm`.
- [ ] Rodar os comandos de pacote/install/distribuicao definidos para a RC, se houver.
- [ ] Registrar qualquer comando que ainda nao exista ou nao esteja documentado.

Gate de avanco:

- [ ] Todos os comandos publicos passam.
- [ ] Nao existe comando oficial quebrado ou ambiguo para release.
- [ ] Lacunas de pacote/install foram corrigidas ou marcadas como fora do escopo da RC.

## Etapa 6 - Consolidar o relatorio final de RC

Meta: transformar as evidencias em decisao objetiva.

- [ ] Atualizar `docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md`.
- [ ] Listar evidencias finais com data, comando e resultado.
- [ ] Separar claramente:
  - [ ] Bloqueios fechados.
  - [ ] Dividas tecnicas aceitas para pos-RC.
  - [ ] Evolucoes futuras opcionais.
  - [ ] Qualquer risco residual.
- [ ] Declarar uma das decisoes:
  - [ ] Aprovado para RC publica.
  - [ ] Aprovado apenas para RC interna.
  - [ ] Bloqueado, com lista objetiva de bloqueios.
- [ ] Rodar `git diff --check`.
- [ ] Revisar arquivos alterados para evitar artefatos locais.

Gate final:

- [ ] Relatorio final atualizado.
- [ ] Evidencias arquivadas.
- [ ] Nenhum P1 aberto.
- [ ] Nenhum P2 sem decisao explicita.
- [ ] Decisao de RC escrita em linguagem direta.

## Ordem obrigatoria

1. Congelar contrato de `std.collections`.
2. Confirmar sanitizer.
3. Rodar Valgrind ou equivalente.
4. Reexecutar performance completa.
5. Validar comandos publicos de release.
6. Consolidar relatorio final de RC.

Nao pular etapas sem registrar motivo no relatorio final.
