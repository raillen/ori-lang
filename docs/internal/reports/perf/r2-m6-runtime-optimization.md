# R2.M6 - Otimizacao do Runtime (Evidencia)

Data: 2026-04-21

## Escopo executado

Arquivos alterados:
- `runtime/c/zenith_rt.c`
- `runtime/c/zenith_rt_templates.h`

Objetivo:
- reduzir custo de `text` em caminhos quentes;
- reduzir custo de ownership em `collections` (`optional` em pop/dequeue);
- reduzir custo de construtores de `optional/result`;
- revisar custo de retain/release em hot paths sem quebrar ownership.

## Mudancas tecnicas

1. `text` (`zenith_rt.c`)
- `zt_text_concat`: fast path quando um dos lados e vazio (retorna alias com `retain` em vez de alocar/copiar).
- `zt_text_slice`: fast path para slice completo (`0..len-1`) com retorno por alias retido.
- `zt_text_eq`: atalho de mismatch pelo primeiro byte antes do `memcmp`.

2. `collections` (`zenith_rt.c` + templates)
- `zt_queue_text_dequeue` e `zt_stack_text_pop`: transferencia direta de ownership para `optional<text>` (remove ciclo retain+release redundante).
- `ZT_DEFINE_PQUEUE_IMPL(..._pop)` e `ZT_DEFINE_CIRCBUF_IMPL(..._pop)` (casos ponteiro): mesma transferencia direta de ownership para `optional`.

3. `optional/result` (`zenith_rt_templates.h` + `zenith_rt.c`)
- `optional_*_present/coalesce`: evita chamada de `retain` para ponteiro nulo.
- `optional_*_empty`: remove `memset` e inicializa campos diretamente.
- `outcome<..., text>` (template): remove `memset` em construtores e inicializa campos diretamente.
- `outcome<..., core.Error>` (implementacoes concretas): remove `memset` dos construtores de sucesso/falha e inicializa campos diretamente.

## Medicao (antes/depois)

Fonte before:
- `docs/internal/reports/perf/r2m6-before-summary-quick.json`

Fonte after:
- `docs/internal/reports/perf/r2m6-after-summary-quick.json`

Comparacao (lat_median_ms):
- `micro_runtime_core`: 82.107 -> 75.190 (-8.42%)
- `micro_stdlib_core`: 94.756 -> 78.606 (-17.04%)
- `m37_result_generic`: 80.531 -> 99.702 (+23.81%)

Leitura tecnica:
- ganho claro nos microbenchmarks de runtime/stdlib;
- `m37_result_generic` mostrou regressao na amostra quick e foi levado para validacao no nightly.

## Validacao funcional e estabilidade

Comandos executados:
- `python run_suite.py smoke` -> PASS
- `powershell -NoProfile -ExecutionPolicy Bypass -File tests/perf/gate_smoke.ps1` -> PASS
- `powershell -NoProfile -ExecutionPolicy Bypass -File tests/perf/gate_nightly.ps1` -> WARN
- `gcc ... test_shared_text.c` + execucao `test_shared_text_r2m6.exe` -> PASS

Resumo nightly:
- status geral: `warn`
- `micro_runtime_core`: `pass`
- `micro_stdlib_core`: `pass`
- avisos: `macro_large_check`, `m37_result_generic`
- relatorios: `reports/perf/summary-nightly.json` e `reports/perf/summary-nightly.md`

Observacao importante:
- `tests/runtime/c/test_runtime.c` esta desalinhado com a API atual de `optional/outcome` (usa ponteiros para tipos por valor) e falha na compilacao sem relacao direta com este milestone.

## Riscos residuais

- Variabilidade alta em benchmark quick para `m37_result_generic`.
- `m37_result_generic` e `macro_large_check` seguem com `warn` no nightly; manter monitoramento no proximo ciclo de tuning.