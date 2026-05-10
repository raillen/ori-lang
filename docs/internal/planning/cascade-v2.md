# Implementation Cascade 2.0

## Objetivo

Traduzir roadmap e checklist em caminho pratico ate codigo e testes.

## Escopo

- Navegacao por milestone do ciclo 2.0.
- Ponte entre documentacao, maps, codigo e validacao.

## Dependencias

- Upstream:
  - `docs/internal/planning/roadmap-v2.md`
  - `docs/internal/planning/checklist-v2.md`
- Downstream:
  - implementacao no codigo
  - evidencias em `docs/internal/reports/*`
- Codigo/Testes relacionados:
  - `compiler/*_MAP.md`
  - `tests/behavior/*`
  - `tests/perf/*`

## Como usar

1. Comece no checklist v2.
2. Passe pelo roadmap v2 para contexto.
3. Use este cascade para localizar arquivos e testes.

Este arquivo conecta:

- `docs/internal/planning/roadmap-v2.md`
- `docs/internal/planning/checklist-v2.md`
- documentacao canonica em `docs/spec/language/`
- `*_MAP.md`
- codigo e suites de teste/performance

Objetivo:

- reduzir tempo de descoberta ("onde mexer")
- evitar drift entre spec, implementacao e testes
- orientar LLMs e humanos por milestone do ciclo 2.0

Uso recomendado:

1. Escolha a milestone no checklist 2.0.
2. Abra a secao equivalente neste arquivo.
3. Siga a ordem: checklist -> roadmap -> spec -> maps -> codigo -> testes.
4. Execute validacao minima e registre evidencia.

Regra pratica:

- checklist = estado e entrega concreta
- roadmap = objetivo e criterio de aceite
- cascade = caminho de execucao
- maps = simbolos/arquivos de entrada

## Referencias fixas (sempre abertas)

- `docs/internal/planning/roadmap-v2.md`
- `docs/internal/planning/checklist-v2.md`
- `docs/internal/planning/cascade-v1.md`
- `compiler/CODE_MAP.md`
- `docs/spec/language/compiler-model.md`
- `docs/spec/language/surface-syntax.md`
- `docs/spec/language/conformance-matrix.md`
- `docs/spec/language/formatter-model.md`
- `docs/spec/language/implementation-status.md`
- `tests/behavior/README.md`
- `tests/behavior/MATRIX.md`
- `tests/perf/README.md`

## Maps por camada

- Frontend:
  - `compiler/frontend/lexer/LEXER_MAP.md`
  - `compiler/frontend/parser/PARSER_MAP.md`
  - `compiler/frontend/ast/AST_MAP.md`
- Semantic:
  - `compiler/semantic/binder/BINDER_MAP.md`
  - `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
  - `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
- IR:
  - `compiler/hir/lowering/HIR_LOWERING_MAP.md`
  - `compiler/zir/ZIR_MODEL_MAP.md`
  - `compiler/zir/ZIR_PARSER_MAP.md`
  - `compiler/zir/ZIR_VERIFIER_MAP.md`
- Backend/Runtime:
  - `compiler/targets/c/EMITTER_MAP.md`
  - `runtime/c/RUNTIME_MAP.md`
- Tooling/Project:
  - `compiler/tooling/FORMATTER_MAP.md`
  - `compiler/driver/DRIVER_MAP.md`
  - `compiler/project/PROJECT_MAP.md`
  - `stdlib/STDLIB_MAP.md`

## Validacao minima padrao

Sempre rodar:

1. `python build.py`
2. `python run_all_tests.py`

Para milestones com perf:

1. `tests/perf/gate_pr.ps1`
2. `tests/perf/gate_nightly.ps1` (quando indicado pela milestone)

Para milestones com parser/robustez:

1. `tests/frontend/test_lexer.c`
2. `tests/frontend/test_parser.c`
3. `tests/frontend/test_parser_error_recovery.c`

## R2.M0 - Baseline e governanca

Cascade order:

1. Checklist: secao `R2.M0` em `docs/internal/planning/checklist-v2.md`
2. Roadmap: secao `R2.M0` em `docs/internal/planning/roadmap-v2.md`
3. Docs:
   - `docs/spec/language/conformance-matrix.md`
   - `tests/perf/README.md`
4. Codigo:
   - `run_all_tests.py`
   - `tests/perf/run_perf.py`
   - `tests/perf/gate_pr.ps1`
   - `tests/perf/gate_nightly.ps1`

Risco principal:

- baseline incoerente entre ambientes.

## R2.M1 - Infra de testes 2.0

Cascade order:

1. Checklist `R2.M1`
2. Roadmap `R2.M1`
3. Docs:
   - `tests/behavior/README.md`
   - `tests/perf/README.md`
4. Codigo:
   - `run_all_tests.py`
   - `tests/perf/run_perf.py`
   - `tests/perf/gate_pr.ps1`
   - `tests/perf/gate_nightly.ps1`

Risco principal:

- suites novas sem reproducibilidade.

## R2.M2 - Robustez de frontend (fuzzing)

Cascade order:

1. Checklist `R2.M2`
2. Roadmap `R2.M2`
3. Spec:
   - `docs/spec/language/surface-syntax.md`
4. Maps:
   - `compiler/frontend/lexer/LEXER_MAP.md`
   - `compiler/frontend/parser/PARSER_MAP.md`
   - `compiler/frontend/ast/AST_MAP.md`
5. Codigo:
   - `compiler/frontend/lexer/lexer.c`
   - `compiler/frontend/parser/parser.c`
   - `compiler/frontend/ast/model.h`

Testes de entrada:

- `tests/frontend/test_lexer.c`
- `tests/frontend/test_parser.c`
- `tests/frontend/test_parser_error_recovery.c`

Risco principal:

- fix local que quebra error recovery.

## R2.M3 - Hardening semantico e propriedades

Cascade order:

1. Checklist `R2.M3`
2. Roadmap `R2.M3`
3. Spec:
   - `docs/spec/language/surface-syntax.md`
   - `docs/spec/language/implementation-status.md`
4. Maps:
   - `compiler/semantic/binder/BINDER_MAP.md`
   - `compiler/semantic/types/TYPE_SYSTEM_MAP.md`
   - `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`
5. Codigo:
   - `compiler/semantic/binder/binder.c`
   - `compiler/semantic/types/checker.c`
   - `compiler/semantic/diagnostics/diagnostics.c`

Testes de entrada:

- `tests/semantic/test_binder.c`
- `tests/semantic/test_types.c`
- `tests/semantic/test_constraints.c`

Risco principal:

- diagnostico muda sem atualizar casos negativos.

## R2.M4 - Observabilidade de performance

Cascade order:

1. Checklist `R2.M4`
2. Roadmap `R2.M4`
3. Docs:
   - `tests/perf/README.md`
   - `docs/spec/language/conformance-matrix.md`
4. Codigo:
   - `tests/perf/run_perf.py`
   - `tests/perf/gate_pr.ps1`
   - `tests/perf/gate_nightly.ps1`
   - `tests/perf/budgets/*.json`

Risco principal:

- otimizar sem baseline confiavel.

## R2.M5 - Otimizacao do compilador

Cascade order:

1. Checklist `R2.M5`
2. Roadmap `R2.M5`
3. Spec:
   - `docs/spec/language/compiler-model.md`
4. Maps:
   - `compiler/driver/DRIVER_MAP.md`
   - `compiler/frontend/parser/PARSER_MAP.md`
   - `compiler/hir/lowering/HIR_LOWERING_MAP.md`
   - `compiler/targets/c/EMITTER_MAP.md`
5. Codigo:
   - `compiler/driver/main.c`
   - `compiler/frontend/parser/parser.c`
   - `compiler/hir/lowering/from_ast.c`
   - `compiler/targets/c/emitter.c`

Suites de validacao:

- `tests/perf/gate_pr.ps1`
- `tests/perf/run_perf.py`
- `python run_all_tests.py`

Risco principal:

- ganho de tempo com regressao semantica.

## R2.M6 - Otimizacao do runtime

Cascade order:

1. Checklist `R2.M6`
2. Roadmap `R2.M6`
3. Spec:
   - `docs/spec/language/compiler-model.md`
4. Map:
   - `runtime/c/RUNTIME_MAP.md`
5. Codigo:
   - `runtime/c/zenith_rt.c`
   - `runtime/c/zenith_rt.h`
   - `runtime/c/zenith_rt_templates.h`

Suites de validacao:

- `tests/perf/gate_pr.ps1`
- `tests/perf/gate_nightly.ps1`
- `tests/runtime/c/*`

Risco principal:

- otimizar hot path e introduzir bug de ownership.

## R2.M7 - Auditoria spec x implementacao

Cascade order:

1. Checklist `R2.M7`
2. Roadmap `R2.M7`
3. Specs:
   - `docs/spec/language/surface-syntax.md`
   - `docs/spec/language/compiler-model.md`
   - `docs/spec/language/formatter-model.md`
   - `docs/spec/language/implementation-status.md`
4. Maps:
   - todos os `*_MAP.md` da area auditada
5. Codigo:
   - iniciar por `compiler/CODE_MAP.md` e descer por camada

Risco principal:

- resolver um drift e criar outro em camada diferente.

## R2.M8 - Fechar deferreds P0 (inclui fmt v2)

Cascade order:

1. Checklist `R2.M8`
2. Roadmap `R2.M8`
3. Spec:
   - `docs/spec/language/surface-syntax.md`
   - `docs/internal/decisions/language/036-text-concatenation-and-interpolation.md`
4. Maps:
   - `compiler/frontend/parser/PARSER_MAP.md`
   - `compiler/hir/lowering/HIR_LOWERING_MAP.md`
   - `compiler/targets/c/EMITTER_MAP.md`
   - `compiler/tooling/FORMATTER_MAP.md`
   - `runtime/c/RUNTIME_MAP.md`
5. Codigo inicial:
   - `compiler/frontend/parser/parser.c`
   - `compiler/hir/lowering/from_ast.c`
   - `compiler/zir/lowering/from_hir.c`
   - `compiler/targets/c/emitter.c`
   - `compiler/tooling/formatter.c`
   - `runtime/c/zenith_rt.c`

Testes alvo:

- `tests/frontend/test_parser.c`
- `tests/frontend/test_parser_error_recovery.c`
- `tests/semantic/test_hir_lowering.c`
- `tests/targets/c/test_emitter.c`
- `tests/formatter/run_formatter_golden.py`
- `tests/behavior/*` (casos com `fmt`)

Risco principal:

- parser aceitar forma que backend nao baixa.

## R2.M9 - Feature Pack A (linguagem)

Cascade order:

1. Checklist `R2.M9`
2. Roadmap `R2.M9`
3. Specs relevantes por feature
4. Maps por camada tocada
5. Codigo + testes da feature

Regra de entrada:

- nenhuma feature inicia sem mini design e plano de teste.

Risco principal:

- ampliar linguagem sem cobertura negativa.

## R2.M10 - Feature Pack B (stdlib e tooling)

Cascade order:

1. Checklist `R2.M10`
2. Roadmap `R2.M10`
3. Specs:
   - `docs/spec/language/formatter-model.md`
   - `docs/spec/language/conformance-matrix.md`
4. Maps:
   - `stdlib/STDLIB_MAP.md`
   - `compiler/tooling/FORMATTER_MAP.md`
   - `compiler/driver/DRIVER_MAP.md`
5. Codigo:
   - `stdlib/std/*.zt`
   - `compiler/tooling/formatter.c`
   - `compiler/driver/main.c`
   - `compiler/project/zdoc.c`

Risco principal:

- tool melhora, mas quebra fluxo canonico de check/fmt/doc.

## R2.M11 - Release candidate 2.0

Cascade order:

1. Checklist `R2.M11`
2. Roadmap `R2.M11`
3. Docs finais:
   - `docs/spec/language/conformance-matrix.md`
   - `CHANGELOG.md`
4. Validacao completa:
   - `python build.py`
   - `python run_all_tests.py`
   - `tests/perf/gate_pr.ps1`
   - `tests/perf/gate_nightly.ps1`

Risco principal:

- aceitar release com risco P0 nao rastreado.

## Prompt seed 2.0 (padrao)

```text
Implement milestone <R2.Mx> from docs/internal/planning/checklist-v2.md.
Read docs/internal/planning/roadmap-v2.md and docs/internal/planning/cascade-v2.md first.
Follow: checklist -> roadmap -> specs -> *_MAP.md -> code -> tests.
Return only:
1) files to edit
2) first symbols to inspect
3) validation plan
4) main regression risk
5) expected performance impact (if any)
```

