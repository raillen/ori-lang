# Evidencia - Etapa 7 - Limpeza e revisao de diff

Data: 2026-05-08

## Resultado

Status da etapa: aprovado para prosseguir.

O diff foi revisado por categoria. Nao ha artefato temporario versionavel no
indice da RC.

## Comandos executados

```powershell
git status --short --untracked-files=all
git diff --stat
git diff --name-only
git ls-files --others --exclude-standard
git diff --check
python tools\check_docs_paths.py
```

Resultado:

- `git diff --check`: codigo `0`; apenas avisos LF/CRLF normais do Git no Windows.
- `python tools\check_docs_paths.py`: `docs path check ok`.
- `git ls-files --others --exclude-standard` nao listou `.ztc-tmp`, `reports/suites`, `reports/perf`, binarios, logs soltos ou arquivos `tmp/*_raw.txt`.

## Separacao do diff

### Implementacao

Arquivos principais:

- `.github/workflows/ci.yml`
- `.gitignore`
- `compiler/driver/main.c`
- `compiler/driver/zpm.c`
- `compiler/project/zdoc.c`
- `compiler/semantic/types/checker.c`
- `compiler/targets/c/emitter.c`
- `runtime/c/*`
- `run_suite.py`
- `zenith.ztproj`

Resumo:

- Correcoes de CLI/ZPM para comandos publicos da RC.
- Correcoes de ZDoc e diagnosticos.
- Correcoes de ownership/temporarios no emissor C.
- Ajustes de runtime/hardening e suite de release.

### Documentacao publica e referencia

Arquivos principais:

- `docs/public/*`
- `docs/reference/*`
- `docs/spec/language/*`
- `docs/internal/decisions/language/INDEX.md`
- `packages/borealis/README.md`
- `stdlib/zdoc/std/*`

Resumo:

- Contrato publico de `std.collections` explicitado.
- Documentacao publica consolidada para o escopo atual da RC.
- `zpm` remoto/registry sync registrado como fora do contrato publico da RC.
- ZDoc da stdlib ampliada para `zt doc check zenith.ztproj` passar.

### Documentacao interna e evidencias

Arquivos principais:

- `docs/internal/archive/tier7-doc-reset/README.md`
- `docs/internal/reports/audit/implementation-plan-rc-public.md`
- `docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md`
- `docs/internal/reports/audit/evidence/*`
- `docs/internal/reports/release/1.0-readiness-report.md`

Resumo:

- Evidencias de sanitizer, memoria equivalente, performance completa, comandos publicos e revisao de diff arquivadas em `docs/internal/reports/audit/evidence/`.
- Material historico movido para area interna de archive, sem apagar contexto util.

### Testes e fixtures

Arquivos principais:

- `tests/behavior/*`
- `tests/driver/test_cli_output_clean.py`
- `tests/driver/test_zpm_lockfile.py`
- `tests/fixtures/diagnostics/*`
- `tests/hardening/*`
- `tests/root/*`
- `tests/runtime/c/test_runtime.c`
- `tests/suites/suite_definitions.py`

Resumo:

- Fixtures permanentes para literal inteiro fora de range, stack overflow e shapes genericas nao suportadas em `std.collections`.
- Cobertura para `zpm install` sem registry local e para ajuda publica de ZPM sem comandos inexistentes.
- Corpus de memoria equivalente e sanitizer documentado.

### Performance

Arquivos principais:

- `tests/perf/run_perf.py`
- `tests/perf/baselines/windows-AMD64/*.json`
- `tests/perf/m36_*`
- `tests/perf/m37_result_generic`
- `tests/perf/primitive_numeric_lists`

Resumo:

- `run_perf.py` drena `stdout/stderr` para evitar deadlock em comandos com saida grande.
- Baselines de performance foram atualizadas a partir dos relatorios nightly locais.

## Baselines revisadas

Total: 22 baselines modificadas.

Arquivos:

- `m37_result_generic.json`
- `macro_large_build_cold.json`
- `macro_large_build_warm.json`
- `macro_large_check.json`
- `macro_large_run.json`
- `macro_medium_build_cold.json`
- `macro_medium_build_warm.json`
- `macro_medium_check.json`
- `macro_medium_run.json`
- `macro_small_build_cold.json`
- `macro_small_build_warm.json`
- `macro_small_check.json`
- `macro_small_run.json`
- `macro_small_test.json`
- `micro_frontend_large_check.json`
- `micro_frontend_small_check.json`
- `micro_lambda_hof_run.json`
- `micro_lowering_large_emit_c.json`
- `micro_lowering_small_emit_c.json`
- `micro_primitive_numeric_lists.json`
- `micro_runtime_core.json`
- `micro_stdlib_core.json`

Justificativa:

- As baselines antigas eram de `2026-04-26`.
- As novas baselines sao de `2026-05-07` e `2026-05-08`.
- Nenhuma atualizacao escondeu falha de budget absoluto.
- A performance completa foi rodada com `python tests/perf/run_perf.py --suite nightly --release-gate`.
- O comando saiu com codigo `0`.
- O unico `warn` restante foi `macro_medium_check` por p95 isolado; a mediana ficou estavel e o aviso foi documentado em `performance-2026-05-08.md`.

## Artefatos temporarios

Nao entram na RC:

- `.ztc-tmp/`
- `reports/suites/`
- `reports/perf/`
- `zt.exe`
- `zpm.exe`
- logs soltos como `emit_debug.txt`, `emit_stdout.txt`, `out.txt`, `test_output.txt`
- `tmp/*_raw.txt`

`.gitignore` cobre os logs soltos conhecidos.

## Decisao

Etapa 7 concluida. O diff esta revisado, as baselines tem justificativa, e nao ha artefato temporario versionavel bloqueando a RC.
