# Evidencia - Etapa 5 - Performance completa

Data: 2026-05-08

## Resultado

Status da etapa: aprovado para prosseguir.

O gate completo de performance foi executado com `--release-gate` e saiu com codigo `0`.
O resumo final ficou como `warn`, nao `fail`, por uma variacao isolada de p95 em `macro_medium_check`.

## Comandos executados

```powershell
python run_suite.py release
```

Resultado:

- Status: `pass`
- Total: `365`
- Pass: `365`
- Fail: `0`
- Relatorio: `reports/suites/release__20260508T031949Z.json`
- Link atual: `reports/suites/release__latest.json`

```powershell
python tests/perf/run_perf.py --suite nightly --release-gate
```

Resultado:

- Codigo de saida: `0`
- Suite: `nightly`
- Benchmarks: `26`
- Status do resumo: `warn`
- Relatorio JSON: `reports/perf/summary-nightly.json`
- Relatorio Markdown: `reports/perf/summary-nightly.md`

## Correcao feita durante a etapa

O primeiro `release` falhou antes do fechamento da etapa por dois problemas reais:

- `borealis_foundations_stub` expunha use-after-free em atribuicao de campo gerenciado no emissor C.
- `std_collections_unsupported_generic_shape_error` nao mostrava todas as formas genericas rejeitadas porque o diagnostico era truncado antes de `btreemap`.

Correcao aplicada:

- `compiler/targets/c/emitter.c` agora trata atribuicao de campo gerenciado como transferencia segura: materializa RHS, libera valor antigo e instala o novo valor com ownership correto.
- `tests/behavior/std_collections_unsupported_generic_shape_error/src/app/main.zt` agora exercita `grid2d<T>`, `pqueue<T>`, `circbuf<T>` e `btreemap<K, V>` como assinaturas de funcao, evitando truncamento por erro duplicado.
- `tests/perf/run_perf.py` drena `stdout` e `stderr` enquanto mede memoria no Windows, evitando deadlock/timeouts quando `emit-c` gera saida grande.

## Baselines atualizadas

As baselines abaixo foram atualizadas a partir dos relatorios `reports/perf/nightly-*.json` porque estavam defasadas ou passaram a medir valores que antes ficavam zerados por limitacao do runner:

- `tests/perf/baselines/windows-AMD64/macro_medium_build_cold.json`
- `tests/perf/baselines/windows-AMD64/macro_medium_build_warm.json`
- `tests/perf/baselines/windows-AMD64/macro_medium_run.json`
- `tests/perf/baselines/windows-AMD64/macro_large_build_cold.json`
- `tests/perf/baselines/windows-AMD64/macro_large_build_warm.json`
- `tests/perf/baselines/windows-AMD64/macro_large_run.json`
- `tests/perf/baselines/windows-AMD64/m37_result_generic.json`
- `tests/perf/baselines/windows-AMD64/micro_frontend_large_check.json`
- `tests/perf/baselines/windows-AMD64/micro_lowering_large_emit_c.json`
- `tests/perf/baselines/windows-AMD64/macro_large_check.json`

Justificativa:

- Nenhuma atualizacao escondeu falha de budget absoluto.
- O runner corrigido passou a medir `peak_ws` e `alloc_proxy` para comandos grandes.
- Baselines antigas tinham tamanho de binario e memoria incompatíveis com a superficie atual da RC.

## Aviso restante

`macro_medium_check` ficou em `warn` somente por p95:

- `lat_p95_ms`: `28.749ms`
- Baseline anterior de p95: `24.761ms`
- Variacao: `16.106%`
- Budget absoluto: `pass`
- Mediana atual: `23.344ms`
- Mediana da baseline: `23.608ms`

Decisao: nao atualizar baseline para esse aviso. A mediana ficou estavel e o aviso veio de uma amostra lenta isolada (`29.838ms`) em um conjunto de 7 amostras.

## Validacao adicional apos a correcao

Como a correcao tocou ownership no emissor C, os checks de memoria tambem foram rerodados:

```powershell
wsl bash -lc 'cd /mnt/c/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/zenith-lang-v2 && python3 tests/hardening/test_runtime_sanitizers.py'
```

Resultado: `runtime sanitizer checks ok`

```powershell
wsl bash -lc 'cd /mnt/c/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/zenith-lang-v2 && python3 tests/hardening/test_runtime_memory_tool.py'
```

Resultado: `memory tool corpus ok`

## Decisao

Etapa 5 concluida. Nao ha falha de performance bloqueando a proxima etapa.
