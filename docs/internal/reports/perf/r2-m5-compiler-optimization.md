# R2.M5 - Otimizacao do Compilador (Evidencia)

Data: 2026-04-21

## Escopo executado

Arquivos alterados:
- `compiler/driver/pipeline.c`
- `compiler/frontend/parser/parser.c`
- `compiler/hir/lowering/from_ast.c`
- `compiler/targets/c/emitter.c`

Objetivo:
- reduzir trabalho redundante no build/run;
- reduzir alocacoes em caminhos quentes de lowering e emitter;
- reduzir custo de comparacao de identificadores contextuais no parser.

## Mudancas tecnicas

1. Driver (`pipeline.c`)
- Cache de objeto do runtime (`.ztc-tmp/runtime/zenith_rt.o`).
- Recompila o runtime apenas quando `zenith_rt.c/.h/_templates.h` mudam.
- Link final agora reutiliza o objeto em cache.

2. Lowering (`from_ast.c`)
- `zt_lower_call_args` com pre-alocacao da lista de argumentos.
- Fast path com stack buffer para ate 16 parametros.
- Fallback em heap para assinaturas maiores.
- Menos `calloc/free` por chamada pequena.

3. Emitter (`emitter.c`)
- `c_emit_call_expr` remove `malloc/free` por chamada.
- Usa buffers locais fixos para `callee` e simbolo mangled.

4. Parser (`parser.c`)
- Helper de comparacao literal com tamanho conhecido.
- Call sites de `fmt`, `hex`, `bytes`, `test` usando tamanho literal.
- Evita `strlen` repetido nesses pontos.

## Medicao (antes/depois)

Fonte before:
- `docs/internal/reports/perf/r2m5-before-summary-quick.json`

Fonte after:
- `docs/internal/reports/perf/r2m5-after-summary-quick.json`

Comparacao principal (lat_median_ms):

- `macro_small_build_cold`: 1726.019 -> 251.240 (-85.44%)
- `macro_small_build_warm`: 1744.094 -> 251.017 (-85.61%)
- `macro_small_run`: 1760.214 -> 277.281 (-84.25%)
- `macro_small_test`: 1997.230 -> 285.168 (-85.72%)

Medicao direta adicional (binario novo, mesmo projeto):
- `zt_m5.exe build tests/behavior/std_io_basic`
  - cold: `2226.35 ms`
  - warm: `336.19 ms`

## Validacao funcional

Comandos com `zt_m5.exe`:
- `check tests/behavior/std_io_basic` -> `verification ok`
- `check tests/behavior/simple_app` -> `verification ok`
- `check tests/behavior/optional_result_basic` -> `verification ok`
- `check tests/behavior/std_format_basic` -> `verification ok`
- `run tests/behavior/std_io_basic` -> exit `0`

## Observacoes importantes

- `zt.exe` local estava bloqueado por processo externo (access denied), entao o build de validacao foi feito em `zt_m5.exe`.
- Por esse lock, alguns gates padrao que dependem estritamente de `zt.exe` nao puderam ser executados no fluxo normal nesta rodada.
- Mesmo assim, o benchmark before/after foi preservado e o ganho principal de pipeline foi confirmado.
