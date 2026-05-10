# Reports

Este diretorio guarda relatorios curados.

Use aqui apenas documentos que precisam ser lidos depois.
Saidas geradas por teste, benchmark ou build ficam em `reports/`.

## Fontes atuais

| Necessidade | Fonte |
| --- | --- |
| Release readiness atual | `docs/internal/reports/release/1.0-readiness-report.md` |
| Estado P0/P1 atual | `docs/internal/reports/release/1.0-no-p0-p1-record.md` |
| Divida tecnica e lacunas pos-RC | `docs/spec/language/post-v1-remaining-language-work.md` |
| Contrato de linguagem ativo | `docs/spec/language/final-language-contract.md` |
| Planejamento ativo | `docs/internal/planning/README.md` |
| Plano concluido de limpeza documental | `docs/internal/planning/documentation-cleanup-plan-2026-05-09.md` |
| Migracao de specs para `docs/` | `docs/internal/reports/documentation-spec-migration-2026-05-09.md` |

`reports/pending-language-issues-current.md` e historico do ciclo corretivo de
2026-04-23. Ele nao e a fonte atual de bloqueios de release.

Relatorios antigos em `docs/internal/reports/` sao evidencia historica,
fechamento de milestone ou suporte para auditoria. Eles nao substituem as
fontes atuais acima.

## Estrutura

- `audit/`
  Auditorias tecnicas.
- `compatibility/`
  Relatorios de compatibilidade por versao.
- `fuzz/`
  Evidencias de fuzzing.
- `overrides/`
  Justificativas de aceite excepcional.
- `perf/`
  Metodologia, comparativos e resumos curados.
- `raw/`
  Historico bruto preservado sem normalizacao editorial.
- `release/`
  Notas e relatorios de release.
- `semantic/`
  Relatorios de semantica e matriz negativa.
- `triage/`
  Triage automatizada. Use `latest.md` e `latest.json` como entrada principal.

## Regra para novos relatorios

Crie novo arquivo aqui quando:

- a informacao precisa sobreviver ao build local;
- a evidencia fecha uma milestone;
- o conteudo tem decisao, risco ou contexto humano.

Nao crie novo arquivo aqui quando:

- o arquivo e log bruto;
- o arquivo e resultado local reproduzivel;
- o arquivo e screenshot temporario;
- o arquivo apenas duplica o contrato final, o planejamento ativo ou uma evidencia ja arquivada.

## Relatorios principais

- `release/1.0-readiness-report.md`
- `release/1.0-no-p0-p1-record.md`
- `release/1.0-stable-feature-coverage.md`
- `audit/rc-public-release-gap-closure-2026-05-08.md`
- `audit/language-complete-analysis-2026-05-08.md`
- `documentation-inventory-2026-05-09.md`
- `documentation-spec-migration-2026-05-09.md`

## Relatorios historicos arquivados

- `docs/internal/archive/reports/legacy-main/audit-report.md`
- `docs/internal/archive/reports/legacy-main/implementation-deep-analysis.md`
- `docs/internal/archive/reports/legacy-main/checklist-deep-analysis-report.md`
- `docs/internal/archive/reports/legacy-main/checklist-final-analysis-report.md`
- `docs/internal/archive/reports/legacy-main/gate-red-fixed-report.md`
- `docs/internal/archive/reports/legacy-main/R3-risk-matrix.md`
- `docs/internal/archive/reports/legacy-main/R3.M5-phase1-phase2-checkpoint.md`
- `docs/internal/archive/reports/legacy-main/stdlib-public-var-analysis-2026-04-22.md`

Esses arquivos permanecem como evidencia ou historico. Nao use como status
atual sem revalidar contra as fontes atuais.
