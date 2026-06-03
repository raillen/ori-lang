# Ori Language Documentation

This directory contains all documentation for the Ori programming language.

## Structure

```
docs/
├── CHANGELOG.md                              # Histórico de mudanças
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
│   ├── IMPLEMENTADOS.md                      # Recursos implementados e resolvidos
│   └── PENDENTES.md                          # Recursos pendentes e plano de correções com checkboxes
└── _reversa_sdd/                             # Arquivo histórico de auditorias
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
Status: 16 chapters, covering the complete language contract.

## Planning Status

The `planning/` directory tracks implementation progress and technical decisions:
- `IMPLEMENTADOS.md` — tracks what has been built, tested, and resolved (Cranelift compiler, stdlib, collections v1, concurrency and basic async).
- `PENDENTES.md` — tracks remaining features, bugs, and advanced compiler/runtime enhancements structured as a sequential phase plan.

The `_reversa_sdd/` directory contains historical audit reports. The most recent
is `auditoria-profunda-implementacao-2026-05-17.md`.

## Active Plans

| Document | Purpose | Status |
|---|---|---|
| `docs/planning/PENDENTES.md` | Sequential phase-based backlog & bug fixes | In progress |
| `docs/plano-implementacao-lsp-avancado.md` | LSP advanced features | Planned |

## Key Documents for Contributors

1. **Spec:** `docs/spec/01-overview.md` — start here for language design
2. **Architecture:** `docs/spec/14-backend-support.md` — backend architecture
3. **Implementation:** `docs/planning/IMPLEMENTADOS.md` — what's built
4. **Bugs & Backlog:** `docs/planning/PENDENTES.md` — current pending checklist
5. **Changelog:** `CHANGELOG.md` — project history
