# Ori Language Documentation

This directory contains all documentation for the Ori programming language.

## Structure

```
docs/
```

Changelog na **raiz** do repositório: `CHANGELOG.md`

```
docs/
├── plano-correcao-bugs-2026-05-17.md         # Plano de correção de bugs atuais
├── plano-implementacao-lsp-avancado.md       # Plano de implementação do LSP
├── spec/                                     # Especificação formal (normativa)
│   ├── README.md
│   ├── 01-overview.md                        # Visão geral e filosofia
│   ├── 02-lexical.md                         # Estrutura léxica
│   ├── 03-grammar.ebnf                       # Gramática EBNF
│   ├── 04-types.md                           # Sistema de tipos
│   ├── 05-expressions.md                     # Expressões
│   ├── 06-statements.md                      # Statements e control flow
│   ├── 07-functions.md                       # Funções e closures
│   ├── 08-traits.md                          # Traits e implement
│   ├── 09-errors.md                          # Erros e propagação
│   ├── 10-memory.md                          # Memória e cleanup
│   ├── 11-generics.md                        # Genéricos e constraints
│   ├── 12-stdlib.md                          # Standard library contracts
│   ├── 13-error-catalog.md                   # Catálogo de diagnósticos
│   ├── 14-backend-support.md                 # Suporte de backends
│   ├── 15-stdlib-maintenance.md              # Manutenção da stdlib
│   └── 16-runtime-ffi-safety.md              # Segurança FFI do runtime
├── planning/                                 # Planos de implementação
│   ├── README.md                             # Este índice
│   ├── uso-real-pequeno-medio.md             # Plano ativo para uso real pequeno/médio
│   ├── PLANO-MATURIDADE-COMPLETO.md          # Plano histórico do ciclo 0.2.0
│   ├── IMPLEMENTADOS.md                      # Recursos implementados e resolvidos
│   └── PENDENTES.md                          # Backlog resumido e histórico operacional
```

Histórico de auditorias (raiz do repositório, **não** sob `docs/`):

```
_reversa_sdd/                                 # Arquivo histórico de auditorias
    ├── auditoria-profunda-implementacao-2026-05-17.md  # Auditoria mais recente
    ├── auditoria-profunda-implementacao-linguagem-2026-05-13.md
    ├── analise-profunda-implementacao-linguagem.md
    ├── plano-correcao-implementacao-linguagem.md
    ├── relatorio-fechamento-correcao-implementacao-linguagem.md
    └── relatorio-fechamento-nova-rodada.md
```

## Spec Status

The `spec/` directory is the **source of truth** for the Ori language.
All compiler implementation decisions must be consistent with these documents.
Status: 18 chapters, covering the current language, stdlib, backend,
project/docs, and stability contracts.

## Planning Status

The `planning/` directory tracks implementation progress and technical decisions:
- `uso-real-pequeno-medio.md` — active plan for making Ori usable in small and medium real projects.
- `PLANO-MATURIDADE-COMPLETO.md` — historical master plan for the `0.2.0` maturity cycle, with mandatory checkboxes, tests, and gate criteria per stage.
- `IMPLEMENTADOS.md` — tracks what has been built, tested, and resolved (Cranelift compiler, stdlib, collections v1, concurrency and basic async).
- `PENDENTES.md` — condensed backlog and operational history; keep it in sync with the active plan.
- `language-direction-decisions-2026-06-30.md` — accepted direction for `try`, ARC, concurrency, FFI, packages, references, and monomorphization.
- `c-backend-redefinition.md` — plan to redefine `ori build` and reduce or isolate the C debug backend.

The `_reversa_sdd/` directory contains historical audit reports. The most recent
is `auditoria-profunda-implementacao-2026-05-17.md`.

## Quality Gates

- `../guides/testing-manual.md` - complete manual for running all project test suites.
- `../guides/first-project-and-packages.md` - first project, local path dependency, package cache, and `0.2.x` upgrade guide.
- `../guides/cookbook-pequeno-medio.md` - short recipes for CLI, config, files, time, local packages, and docs.
- `../guides/reportar-bugs.md` - bug report policy for language, stdlib, runtime, tooling, and VS Code.
- `../guides/language-comparison.md` - methodology and current results for comparing Ori with Rust, C, Python, and Node.js on equivalent workloads.
- `../../tools/quality_metrics.orl` - Ori script that runs the security/performance metric suite and writes CSV/TXT reports.
- `../../tools/compare_language_workloads.ps1` - PowerShell runner that writes CSV/TXT comparison reports for equivalent language workloads.

- `security-performance-testing.md` — current security and performance test strategy, commands, and strict performance budgets.

## Active Plans

| Document | Purpose | Status |
|---|---|---|
| `docs/planning/uso-real-pequeno-medio.md` | Active plan for 100% small/medium project usability | **Active** |
| `docs/planning/PENDENTES.md` | Condensed backlog and operational history | Active summary |
| `docs/planning/PLANO-MATURIDADE-COMPLETO.md` | Historical master plan for the `0.2.0` maturity cycle | Historical / reference |
| `docs/planning/language-direction-decisions-2026-06-30.md` | Language direction decisions | Active decision record |
| `docs/planning/c-backend-redefinition.md` | C backend and `ori build` redefinition | Proposed change |
| `docs/plano-implementacao-lsp-avancado.md` | LSP advanced features | **Historical** — see Etapa 6 of master plan |

## Key Documents for Contributors

1. **Spec:** `docs/spec/01-overview.md` — start here for language design
2. **Architecture:** `docs/spec/14-backend-support.md` — backend architecture
3. **Implementation:** `docs/planning/IMPLEMENTADOS.md` — what's built
4. **Active plan:** `docs/planning/uso-real-pequeno-medio.md` — next implementation sequence
5. **Bugs & Backlog:** `docs/planning/PENDENTES.md` — condensed pending checklist
6. **Changelog:** `CHANGELOG.md` (repo root) — project history
