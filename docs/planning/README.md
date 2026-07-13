# Planejamento e Backlog — Ori Language

Este diretório contém o planejamento de maturidade, histórico de design e o backlog ativo para o desenvolvimento da linguagem Ori.

## Estrutura do Diretório

```
docs/planning/
├── README.md                           # Este índice explicativo
├── uso-real-pequeno-medio.md           # [Ativo] Plano de maturidade para uso real em projetos pequenos e médios
├── PENDENTES.md                        # [Ativo] Backlog M2→M3→M1→M4
├── stdlib-merge-policy.md              # [Ativo] M2 — API canônica ori.X + mesclagem
├── repo-and-project-layout.md          # [Ativo] M2.layout — monorepo + projetos raiz-first
├── result-ctors-ok-err.md              # [Ativo] M2.result-ctors — success/error → ok/err
├── language-direction-decisions-2026-06-30.md # [Decisão] Decisões arquiteturais de direção da linguagem (ADR)
├── ori-surface-s3-auk9.md              # [Ativo] Decisões de superfície S3 (implementada no compiler; docs 0.3.0)
├── adr-ori-surface-s3-auk9.md          # [ADR] Aceito — superfície S3 / aposentar Auk9 como produto
├── pr-plan-ori-surface-s3.md           # [Concluído] PR Plan DAG 0.3.0 + 0.3.1 + opção B
├── IMPLEMENTADOS.md                    # [Histórico] Registro cronológico de recursos já implementados
└── historico/                          # [Histórico] Planos de design e propostas concluídas/arquivadas
    ├── PLANO-MATURIDADE-COMPLETO.md    # Plano mestre de maturidade do ciclo v0.2.0 (100% concluído)
    ├── c-backend-redefinition.md       # Proposta de redefinição do backend C e `ori build`
    ├── io-streams-design.md            # Design original do sistema de I/O streams
    ├── net-v2-design.md                # Design e especificações do TCP/UDP síncrono v2
    ├── registry-v2.md                  # Proposta e modelo mental para o gerenciador de pacotes local
    ├── rust-independence.md            # Estratégia de independência de Rust (Phases 1-3)
    ├── security-performance-testing.md  # Estratégia de testes de segurança, performance e orçamentos
    └── stdlib-gap-parity.md            # Auditoria de paridade da biblioteca padrão
```

## Guia de Uso dos Documentos

### 1. Documentos Ativos (Backlog Ativo)
*   **[PENDENTES.md](PENDENTES.md):** Prioridade tática **M2 → M3 → M1 → M4**.
*   **[stdlib-merge-policy.md](stdlib-merge-policy.md):** Decisão M2 de mesclagem da stdlib (`ori.X` canônico).
*   **[uso-real-pequeno-medio.md](uso-real-pequeno-medio.md):** Plano de usabilidade pequeno/médio (subordinado a PENDENTES).
*   **[PENDENTES.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/docs/planning/PENDENTES.md):** É a lista consolidada de tarefas a fazer. Serve como checklist técnico diário.

### 2. Decisões Arquiteturais (ADRs)
*   **[language-direction-decisions-2026-06-30.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/docs/planning/language-direction-decisions-2026-06-30.md):** Documenta as decisões e princípios fundamentais do design da linguagem (tratamento de erros com `try`, ARC vs cycle collector, monomorfização de genéricos, concorrência cooperativa).
*   **[ori-surface-s3-auk9.md](ori-surface-s3-auk9.md):** Registro vivo das decisões de **superfície S3** (sintaxe no estilo Auk9, features Ori). Diálogo de decisões **completo** (blocos 0–9).
*   **[adr-ori-surface-s3-auk9.md](adr-ori-surface-s3-auk9.md):** ADR aceito do S3.
*   **[pr-plan-ori-surface-s3.md](pr-plan-ori-surface-s3.md):** Plano de PRs (DAG) — marco `0.3.0` (PRs 1–10), `0.3.1` inferência (PR 11), **opção B** (PR 11b) — **concluído**.

### 3. Arquivo Histórico
*   **[IMPLEMENTADOS.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/docs/planning/IMPLEMENTADOS.md):** Contém os marcos de engenharia já completados para referência e auditorias de código.
*   **[historico/](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/docs/planning/historico):** Subdiretório contendo propostas antigas e planos de recursos cujas implementações foram concluídas com sucesso. Não devem ser alterados ou usados como backlog ativo.

---
Normativo: [Manifesto](../spec/00-manifesto.md) · [Especificação](../spec/README.md) · CHANGELOG `[0.3.0]`.

**Auk9** lab aposentada como produto; superfície vivente = Ori S3.
