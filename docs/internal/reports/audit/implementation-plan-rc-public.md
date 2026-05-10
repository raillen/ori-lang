# Plano de implementacao para RC publica

Data: 2026-05-07

Escopo: fechar as pendencias que ainda impedem uma RC publica/Estavel do Zenith.

Estado de partida:

- P1 funcional conhecido: fechado.
- O projeto pode ser tratado como candidato a RC interna.
- A RC publica ainda depende de evidencias e ajustes de contrato publico.
- A refatoracao documental foi concluida nas Etapas 1-8; qualquer ajuste
  restante e acompanhamento de release ou melhoria pos-RC.

Regra principal:

- Execute as etapas na ordem.
- Nao avance para a proxima etapa antes de marcar todos os checkboxes da etapa atual.
- Se algum comando falhar, pare, corrija, reexecute e registre a evidencia.
- Toda decisao precisa ter evidencia curta: comando, resultado, arquivo ou link de CI.

## Etapa 1 - Fechar o contrato publico de `std.collections`

Objetivo: impedir que a RC prometa colecoes genericas alem do que o v1 realmente entrega.

### Atividades

- [x] Revisar `stdlib/std/collections.zt` e confirmar a matriz publica real.
- [x] Confirmar que `list<T>`, `map<K,V>` e `set<T>` usam o runtime generico quando o backend suporta.
- [x] Confirmar que `grid2d`, `grid3d`, `pqueue`, `circbuf`, `btreemap` e `btreeset` seguem especializadas no v1.
- [x] Documentar suporte atual de `queue` e `stack`:
  - [x] APIs `int`.
  - [x] APIs `text`.
  - [x] `queue_values<T>`.
  - [x] `stack_values<T>`.
- [x] Documentar suporte atual de `grid2d` e `grid3d`:
  - [x] `grid2d<int>`.
  - [x] `grid2d<text>`.
  - [x] `grid3d<int>`.
  - [x] `grid3d<text>`.
- [x] Documentar suporte atual de `pqueue` e `circbuf`:
  - [x] `pqueue<int>`.
  - [x] `pqueue<text>`.
  - [x] `circbuf<int>`.
  - [x] `circbuf<text>`.
- [x] Documentar suporte atual de arvores:
  - [x] `btreemap<text, text>`.
  - [x] `btreeset<text>`.
- [x] Documentar que HOFs publicos atuais em `std.collections` sao `map_int`, `filter_int` e `reduce_int`.
- [x] Atualizar `docs/spec/language/stdlib-model.md`.
- [x] Atualizar `docs/spec/language/stdlib-reference-by-topic.md`.
- [x] Atualizar `stdlib/zdoc/std/collections.zdoc` se houver texto ambiguo.
- [x] Revisar se ha docs de release chamando `std.collections` de generica sem qualificar o escopo.
- [x] Registrar como divida tecnica pos-RC a expansao real das colecoes avancadas genericas.
- [x] Criar ou revisar fixture negativa para instanciacoes nao suportadas, se o erro atual for ruim.

### Divida tecnica aceita para pos-RC

Esta divida nao bloqueia a RC publica se a limitacao estiver documentada de forma clara.

- [ ] Implementar `grid2d<T>` e `grid3d<T>` como colecoes realmente genericas.
- [ ] Implementar `circbuf<T>` como colecao realmente generica.
- [ ] Implementar `pqueue<T>` generica com contrato de ordenacao claro:
  - [ ] via constraint de tipo ordenavel; ou
  - [ ] via comparador explicito.
- [ ] Implementar `btreemap<K,V>` generico com contrato de ordenacao para `K`:
  - [ ] via constraint de chave ordenavel; ou
  - [ ] via comparador explicito.
- [ ] Implementar `btreeset<T>` generico com contrato de ordenacao claro.
- [ ] Estender runtime, emitter C, ARC/COW e testes para tipos alem de `int` e `text`.
- [ ] Cobrir tipos gerenciados e compostos, como `text`, structs com campos gerenciados e listas aninhadas.
- [ ] Adicionar testes positivos e negativos para cada nova shape generica.
- [ ] Atualizar ZDoc e referencia publica somente depois da implementacao estar validada.

### Validacao obrigatoria

- [x] Rodar `.\zt.exe doc check zenith.ztproj`.
- [x] Rodar `python tools/check_docs_current_syntax.py`.
- [x] Rodar `.\zt.exe check tests\behavior\std_collections_basic\zenith.ztproj --ci`.
- [x] Rodar `.\zt.exe run tests\behavior\std_collections_values_iteration\zenith.ztproj --ci`.
- [x] Rodar `.\zt.exe run tests\behavior\std_collections_queue_stack_cow\zenith.ztproj --ci`.
- [x] Rodar `.\zt.exe check tests\behavior\std_collections_managed_arc\zenith.ztproj --ci`.

Evidencia 2026-05-07:

- `python build.py`: passou.
- `.\zt.exe doc check zenith.ztproj`: passou.
- `python tools/check_docs_current_syntax.py`: passou.
- `.\zt.exe check tests\behavior\std_collections_basic\zenith.ztproj --ci`: passou.
- `.\zt.exe run tests\behavior\std_collections_values_iteration\zenith.ztproj --ci`: passou.
- `.\zt.exe run tests\behavior\std_collections_queue_stack_cow\zenith.ztproj --ci`: passou.
- `.\zt.exe check tests\behavior\std_collections_managed_arc\zenith.ztproj --ci`: passou.
- `.\zt.exe check tests\behavior\std_collections_unsupported_generic_shape_error\zenith.ztproj --ci`: falhou como esperado, com diagnosticos para `grid2d<bool>`, `pqueue<float>`, `circbuf<bool>` e `btreemap<text,int>`.
- Verificacao de fragmentos em `tests\fixtures\diagnostics\std_collections_unsupported_generic_shape_error.contains.txt`: passou.

### Gate para prosseguir

- [x] Nenhuma documentacao publica promete `std.collections` generica para qualquer `T/K/V`.
- [x] A matriz suportada esta escrita em linguagem direta.
- [x] Os fixtures de `std.collections` passam.
- [x] Unsupported shapes estao documentadas ou falham com diagnostico claro.
- [x] A expansao futura de `grid2d<T>`, `circbuf<T>`, `pqueue<T>` e `btreemap<K,V>` esta registrada como divida tecnica pos-RC.

## Etapa 2 - Refatorar e consolidar a documentacao publica

Objetivo: reduzir drift documental antes da RC publica e garantir que usuarios leiam uma fonte coerente, atual e acessivel.

Contexto atual:

- `docs/DOCS-STRUCTURE.md` define camadas por publico-alvo.
- `docs/internal/release/docs-canonical-policy.md` define que `docs/spec/language/final-language-contract.md` vence quando docs discordam.
- `docs/internal/planning/tier-7-documentation-reset-plan.md` registra que a documentacao precisa de reset, nao apenas polimento.
- `python tools/check_docs_paths.py` reportou 300 caminhos ausentes no snapshot inicial de 2026-05-07.

### Atividades

- [x] Revisar `docs/DOCS-STRUCTURE.md` e confirmar as camadas oficiais:
  - [x] `docs/public/` para guias de usuario.
  - [x] `docs/reference/` para referencia consultavel.
  - [x] `docs/internal/` para planejamento, evidencias e manutencao.
  - [x] `docs/spec/language/` para verdade normativa da linguagem.
  - [x] `docs/internal/decisions/language/` para historico e racional.
- [x] Revisar `docs/internal/release/docs-canonical-policy.md`.
- [x] Revisar `docs/internal/planning/tier-7-documentation-inventory.md`.
- [x] Revisar `docs/internal/planning/tier-7-documentation-reset-plan.md`.
- [x] Declarar no relatorio de RC qual arquivo e a fonte normativa final.
- [x] Confirmar se `docs/spec/language/zenith-language-spec.md` ja e a especificacao consolidada atual ou se ainda precisa de complemento.
- [x] Confirmar quais arquivos antigos continuam ativos e quais viraram historicos.
- [x] Criar uma lista curta de documentos canonicos para RC publica:
  - [x] Especificacao final da linguagem.
  - [x] Modelo da stdlib.
  - [x] Referencia publica da stdlib.
  - [x] Guia de aprendizado inicial.
  - [x] Cookbook.
  - [x] Guia de tooling/CLI.
  - [x] Politica de release.
- [x] Revisar a documentacao publica para remover promessa maior que a implementacao.
- [x] Revisar docs publicas para manter linguagem acessivel para TDAH e dislexia:
  - [x] secoes curtas;
  - [x] frases diretas;
  - [x] exemplos pequenos;
  - [x] progressao em passos;
  - [x] sem jargoes desnecessarios.
- [x] Corrigir docs que ainda tratam material historico como comportamento atual.
- [x] Resolver ou arquivar referencias antigas para:
  - [x] documentos antigos de surface syntax;
  - [x] documentos antigos de dyn-dispatch;
  - [x] documentos antigos de callables/delegates;
  - [x] roadmaps/checklists antigos ausentes;
  - [x] paths antigos de Borealis Studio;
  - [x] paths antigos de testes legados.
- [x] Decidir se arvores publicas traduzidas entram na RC publica agora.
- [x] Se essas traducoes nao entrarem agora, marcar como pos-RC e remover links publicos quebrados.
- [x] Arquivar ou rebaixar docs historicas que hoje geram links quebrados.
- [x] Evitar apagar material historico antes de mover o conteudo util para a fonte canonica.
- [x] Atualizar `docs/README.md`, `docs/public/README.md` e `docs/reference/README.md` para apontarem apenas para rotas existentes.
- [x] Atualizar `docs/reference/language/feature-matrix.md` para nao apontar para caminhos futuros inexistentes como se fossem atuais.
- [x] Atualizar relatorios de release que referenciam docs publicas ausentes.
- [x] Criar uma evidencia curta com o resultado de `python tools/check_docs_paths.py` antes/depois.

### Validacao obrigatoria

- [x] Rodar `python tools/check_docs_paths.py`.
- [x] Rodar `python tools/check_docs_current_syntax.py`.
- [x] Rodar `.\zt.exe doc check zenith.ztproj`.
- [x] Rodar busca por termos historicos em docs publicas:
  - [x] `dyn` fora de contexto historico;
  - [x] `fmt "` como sintaxe atual;
  - [x] `assert` como feature atual;
  - [x] `case default` como fallback atual;
  - [x] `uint8`, `uint16`, `uint32`, `uint64` como nomes preferenciais;
  - [x] `size_of` global se nao for API atual.
- [x] Rodar busca por links para arquivos ausentes apos a refatoracao.
- [x] Conferir manualmente a trilha inicial de usuario.

Evidencia 2026-05-07:

- Relatorio criado: `docs/internal/reports/audit/rc-public-docs-consolidation-2026-05-07.md`.
- `python tools/check_docs_paths.py` antes: falhou com 300 caminhos ausentes, concentrados em planos, decisoes, relatorios e referencias historicas.
- `python tools/check_docs_paths.py` depois: passou.
- `python tools/check_docs_current_syntax.py`: passou.
- `.\zt.exe doc check zenith.ztproj`: passou.
- Busca de termos historicos em docs publicas/reference: sem `dyn`, `fmt "`, `assert`, `case default`, `uint8/uint16/uint32/uint64` como ensino atual; unico match foi `std.debug.size_of(value)`, que e API escopada, nao `size_of` global.
- Trilha inicial manual: `docs/public/README.md` aponta para `learn-zenith-in-30-minutes.md`, `language-reference.md`, `cookbook.md`, `stdlib-reference.md`, `tooling-guide.md` e `language-comparison.md`, todos existentes.

### Gate para prosseguir

- [x] Existe uma lista curta de fontes canonicas para RC publica.
- [x] Docs publicas nao contradizem `docs/spec/language/final-language-contract.md`.
- [x] Docs publicas nao ensinam sintaxe historica como atual.
- [x] Links quebrados publicos foram corrigidos, arquivados ou marcados como pos-RC.
- [x] `python tools/check_docs_paths.py` passa ou tem uma excecao documentada e aprovada para material historico.
- [x] A refatoracao respeita a regra de nao apagar historia antes de preservar o contexto util.

## Etapa 3 - Confirmar sanitizer em ambiente compativel

Objetivo: obter evidencia real de ASAN/UBSAN ou equivalente antes da RC publica.

### Atividades

- [x] Revisar `.github/workflows/ci.yml` e confirmar que o job de sanitizer existe.
- [x] Confirmar qual compilador Linux sera usado no CI. Resultado: `gcc` em `ubuntu-latest`.
- [x] Executar o job de sanitizer no CI Linux ou registrar fallback local equivalente. Resultado: CI remoto nao foi acionado neste turno; WSL2 Ubuntu/GCC foi usado como fallback compativel.
- [x] Se o CI nao puder ser usado, preparar ambiente local compativel com clang/gcc. Resultado: WSL2 Ubuntu, Python 3.12.3, GCC 13.3.0.
- [x] Rodar `python tests\hardening\test_runtime_sanitizers.py` no ambiente compativel.
- [x] Arquivar stdout/stderr do sanitizer.
- [x] Criar pasta `docs/internal/reports/audit/evidence/` se ela ainda nao existir.
- [x] Salvar a evidencia em `docs/internal/reports/audit/evidence/runtime-sanitizers-2026-05-07.md`.
- [x] Se houver falha, classificar a falha:
  - [x] Use-after-free. ASAN encontrou UAF real em `zt_outcome_void_text_propagate`.
  - [x] Leak. Sem falha de leak classificada nesta etapa; leak dedicado continua na Etapa 4.
  - [x] Undefined behavior. UBSAN passou no rerun final.
  - [x] Falso positivo. Nenhum falso positivo usado para liberar a etapa.
  - [x] Limitacao de toolchain. C11/POSIX e `libm` exigiam flags de build no script.
- [x] Corrigir toda falha real.
- [x] Reexecutar sanitizer depois da correcao.

### Gate para prosseguir

- [x] Sanitizer passou em ambiente compativel.
- [x] A evidencia foi arquivada.
- [x] Toda falha real foi corrigida.
- [x] Todo falso positivo tem justificativa curta. N/A: nenhum falso positivo foi usado.

## Etapa 4 - Rodar Valgrind ou ferramenta equivalente

Objetivo: ter uma segunda evidencia de memoria para release publica.

### Atividades

- [x] Escolher a ferramenta oficial desta rodada:
  - [x] Valgrind. Disponivel no WSL e executado em 2026-05-08.
  - [x] Dr. Memory. Instalado e executado em 2026-05-08; nao aceito como gate porque falha com `Floating point exception` ate em `/bin/true`. Evidencia: `docs/internal/reports/audit/evidence/drmemory-2026-05-08.md`.
  - [x] LLVM LeakSanitizer/UBSan via GCC/ASAN em WSL, como validacao complementar anterior.
  - [x] Outra ferramenta equivalente, com justificativa. Justificativa anterior arquivada em `docs/internal/reports/audit/evidence/memory-tool-2026-05-07.md`.
- [x] Definir corpus minimo de execucao:
  - [x] Runtime core.
  - [x] Stdlib core.
  - [x] `std.collections`.
  - [x] ARC/ORC.
  - [x] Fuzz replay.
- [x] Criar comando documentado para rodar a ferramenta: `python3 tests/hardening/test_runtime_memory_tool.py --tool valgrind`.
- [x] Executar a ferramenta no corpus minimo.
- [x] Arquivar logs em `docs/internal/reports/audit/evidence/`. Resultado: `docs/internal/reports/audit/evidence/valgrind-2026-05-08.md`.
- [x] Classificar todos os achados.
- [x] Corrigir falhas reais. Resultado: vazamento de `list_set` copy-on-write corrigido no emissor C.
- [x] Reexecutar a ferramenta depois das correcoes.

### Gate para prosseguir

- [x] Ferramenta passou sem vazamentos relevantes.
- [x] Ferramenta passou sem UB relevante.
- [x] Logs foram arquivados.
- [x] Achados restantes, se houver, tem decisao explicita. N/A: nenhum achado restante bloqueia a etapa.

## Etapa 5 - Rodar performance completa

Objetivo: trocar evidencia parcial por evidencia forte de release.

### Atividades

- [x] Confirmar qual comando representa o gate completo de performance. Resultado: `python tests/perf/run_perf.py --suite nightly --release-gate`.
- [x] Rodar `python run_suite.py release`. Resultado: `reports/suites/release__20260508T031949Z.json`, 365/365.
- [x] Rodar o gate completo de performance definido para RC publica. Resultado: `reports/perf/summary-nightly.json`, codigo de saida `0`.
- [x] Comparar resultados com baselines atuais.
- [x] Identificar regressao acima do limite aceito. Resultado: nenhuma falha de budget absoluto; um `warn` nao bloqueante em `macro_medium_check`.
- [x] Investigar cada regressao relevante. Resultado: `macro_medium_check` ficou com mediana estavel e p95 alto por uma amostra lenta isolada.
- [x] Corrigir regressao causada por bug. Resultado: ownership de campo gerenciado no emissor C e deadlock de pipe no runner de performance corrigidos.
- [x] Atualizar baseline somente quando a mudanca for justificada.
- [x] Arquivar relatorio em `reports/suites/` ou `docs/internal/reports/audit/evidence/`. Resultado: `docs/internal/reports/audit/evidence/performance-2026-05-08.md`.

### Gate para prosseguir

- [x] `python run_suite.py release` passa.
- [x] Gate completo de performance passa.
- [x] Toda regressao relevante foi corrigida ou justificada.
- [x] Evidencia foi arquivada.

## Etapa 6 - Validar comandos publicos de release

Objetivo: garantir que comandos de usuario e release nao estejam quebrados.

### Atividades

- [x] Rodar `python build.py`. Resultado: `SUCCESS`, `zt.exe` e `zpm.exe` gerados.
- [x] Rodar `.\zt.exe check zenith.ztproj --all --ci`. Resultado: `check ok`.
- [x] Rodar `.\zt.exe test zenith.ztproj --ci`. Resultado: `test ok (pass=1 skip=0)`.
- [x] Rodar `.\zt.exe fmt zenith.ztproj --check`. Resultado: `fmt check ok`.
- [x] Rodar `.\zt.exe doc check zenith.ztproj`. Resultado: `doc check ok`.
- [x] Rodar `.\zt.exe help`. Resultado: passou.
- [x] Rodar `.\zt.exe help zpm`. Resultado: passou depois de alinhar a ajuda com os comandos existentes.
- [x] Rodar smoke/release suite oficial. Resultado: `reports/suites/release__20260508T044114Z.json`, 365/365.
- [x] Validar comandos de pacote, install ou distribuicao, se existirem para a RC.
- [x] Registrar lacunas de comandos que ainda nao existem. Resultado: registry remoto, instalador nativo e `login/search/info` ficam fora do contrato publico da RC.
- [x] Corrigir comando oficial quebrado. Resultado: `zpm install` sem registry local, `zt help zpm` e cache invalido de `update-registry` corrigidos.

### Gate para prosseguir

- [x] Build passa.
- [x] Check passa.
- [x] Test passa.
- [x] Fmt passa.
- [x] Doc check passa.
- [x] Help passa.
- [x] Suite oficial passa.
- [x] Nenhum comando publico quebrado ficou sem decisao.

## Etapa 7 - Limpar artefatos e revisar diff

Objetivo: impedir que arquivos locais, logs soltos ou baselines acidentais entrem na RC.

### Atividades

- [x] Rodar `git status --short`.
- [x] Revisar arquivos modificados.
- [x] Separar mudancas de implementacao, docs, testes e evidencias.
- [x] Separar refatoracao documental de mudancas funcionais.
- [x] Confirmar que novos arquivos de evidencia estao no local correto. Resultado: `docs/internal/reports/audit/evidence/`.
- [x] Confirmar que artefatos temporarios nao foram adicionados.
- [x] Confirmar que docs arquivadas foram movidas para pasta interna apropriada. Resultado: `docs/internal/archive/tier7-doc-reset/`.
- [x] Revisar alteracoes de baseline de performance. Resultado: 22 baselines revisadas e justificadas em `docs/internal/reports/audit/evidence/diff-cleanup-2026-05-08.md`.
- [x] Rodar `git diff --check`.
- [x] Corrigir whitespace ou EOF se necessario. Resultado: N/A; `git diff --check` passou.

### Gate para prosseguir

- [x] Diff esta revisado.
- [x] Nao ha artefatos locais indevidos.
- [x] `git diff --check` passa.
- [x] Baselines alterados tem justificativa.
- [x] Material historico arquivado nao aparece mais como doc publica ativa.

## Etapa 8 - Atualizar o relatorio final de RC

Objetivo: transformar validacao em decisao objetiva.

### Atividades

- [x] Atualizar `docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md`.
- [x] Registrar comandos executados.
- [x] Registrar resultados finais.
- [x] Referenciar evidencias arquivadas.
- [x] Registrar resultado da refatoracao documental.
- [x] Registrar a divida tecnica pos-RC de colecoes avancadas genericas.
- [x] Separar claramente:
  - [x] Bloqueios fechados.
  - [x] Dividas tecnicas aceitas para pos-RC.
  - [x] Evolucoes futuras.
  - [x] Riscos residuais.
- [x] Declarar status final. Resultado: aprovado para RC publica local.
- [x] Registrar alternativas rejeitadas. Resultado: nao e apenas RC interna; a decisao local e aprovacao para RC publica.
- [x] Registrar bloqueios atuais quando existirem. N/A: como o status final e aprovado, nao ha bloqueios locais a listar.
- [x] Se aprovado, listar criterios de aprovacao.
- [x] Confirmar que a Etapa 8 nao deixa checkboxes abertos. Resultado: alternativas de status foram registradas como rejeitadas, nao como atividades futuras.

### Gate final

- [x] Nenhum P1 aberto.
- [x] Nenhum P2 sem decisao explicita.
- [x] Evidencias de sanitizer estao arquivadas.
- [x] Evidencias de Valgrind estao arquivadas.
- [x] Evidencias de performance completa estao arquivadas.
- [x] Contrato publico de `std.collections` esta claro.
- [x] Documentacao publica esta consolidada ou tem excecao aprovada.
- [x] Relatorio final declara a decisao de RC em linguagem direta.

## Ordem obrigatoria de execucao

1. Fechar o contrato publico de `std.collections`.
2. Refatorar e consolidar a documentacao publica.
3. Confirmar sanitizer em ambiente compativel.
4. Rodar Valgrind ou equivalente.
5. Rodar performance completa.
6. Validar comandos publicos de release.
7. Limpar artefatos e revisar diff.
8. Atualizar o relatorio final de RC.

Nao pule etapas. Se uma etapa precisar ser pulada por limite de ambiente, registre o motivo e marque a RC publica como bloqueada ou condicionada.
