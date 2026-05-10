# Documentacao Zenith

> Mapa principal da documentacao do projeto.
> Status: atual.
> Superficie: indice.

## Objetivo

Separar documentacao por publico-alvo.

Isso evita que usuarios leiam checklists internos como se fossem guia da linguagem,
e evita que contribuidores usem tutoriais como fonte normativa de implementacao.

## Pastas principais

| Pasta | Publico | Uso |
| --- | --- | --- |
| `docs/public/` | usuarios | guias publicos (onboarding, tutorial, cookbook e comparativos) |
| `docs/reference/` | usuarios avancados e autores de packages | regras consultaveis, KB e referencias estaveis |
| `docs/internal/` | mantenedores e contribuidores | roadmaps, checklists, reports, governance e arquitetura interna |
| `docs/wiki/` | publico | fonte sincronizada para GitHub Wiki |
| `docs/spec/language/` | mantenedores e implementacao | especificacao normativa bruta da linguagem |
| `docs/internal/decisions/language/` | mantenedores e implementacao | decisoes de linguagem com contexto historico |

## Fontes atuais

| Necessidade | Fonte atual |
| --- | --- |
| Aprender a usar | `docs/public/README.md` |
| Consultar regra curta | `docs/reference/` |
| Contrato de linguagem ativo | `docs/spec/language/final-language-contract.md` |
| Planejamento ativo | `docs/internal/planning/README.md` |
| Especificacao normativa | `docs/spec/language/README.md` |
| Decisoes de linguagem | `docs/internal/decisions/language/README.md` |
| Relatorios e evidencia | `docs/internal/reports/` |
| Saidas locais de teste/build | `reports/` |
| Mapas tecnicos colocalizados | `docs/internal/architecture/codebase-map.md` |
| Plano concluido de limpeza documental | `docs/internal/planning/documentation-cleanup-plan-2026-05-09.md` |
| Relatorio da migracao de specs | `docs/internal/reports/documentation-spec-migration-2026-05-09.md` |
| Release readiness atual | `docs/internal/reports/release/1.0-readiness-report.md` |

## Regra simples

- Se ensina usuario a usar Zenith, coloque em `docs/public/` somente apos conferir `docs/spec/language/final-language-contract.md`.
- Se define uma regra consultavel, coloque em `docs/reference/`.
- Se planeja, registra risco, evidencia ou implementacao, coloque em `docs/internal/`.
- Se e saida gerada, coloque em `reports/` ou gere sob demanda pelo tooling.
- Se define comportamento normativo do compilador, mantenha em `docs/spec/language/`.
- Se registra motivo e contexto de decisao, mantenha em `docs/internal/decisions/language/`.
- Se documenta manutencao direta de um subsistema, pode ficar perto do codigo e ser indexado em `docs/internal/architecture/codebase-map.md`.

## Ordem de leitura para mantenedores

1. `README.md` na raiz.
2. `docs/DOCS-STRUCTURE.md`.
3. `docs/spec/language/final-language-contract.md`.
4. `docs/spec/language/README.md`.
5. `docs/internal/planning/README.md`.
6. `docs/reference/README.md`.
7. `docs/internal/decisions/language/README.md`.
8. `docs/internal/architecture/codebase-map.md`.
9. `compiler/CODE_MAP.md`.
10. `docs/internal/reports/README.md`.

## Padrao editorial

Use `docs/internal/standards/documentation-style-guide.md`.
Para paginas de usuario, use tambem `docs/internal/standards/user-doc-template.md`.

Resumo:

- frases curtas;
- titulos previsiveis;
- exemplos pequenos;
- status explicito;
- atual, historico e futuro sempre separados.

## Validacao

Antes de fechar uma reorganizacao documental, rode:

```powershell
python tools/check_docs_paths.py
git diff --check
```
