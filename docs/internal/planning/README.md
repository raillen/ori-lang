# Planning Docs

Este diretorio guarda planos de execucao.

## Fonte ativa

| Tipo | Documento |
| --- | --- |
| Roadmap self-hosting | `selfhosted-roadmap-v1.md` |
| Checklist self-hosting | `selfhosted-checklist-v1.md` |
| Roadmap LSP 1.0 | `lsp-1.0-roadmap.md` |
| Roadmap editor | `editor-roadmap-v1.md` |
| Checklist editor | `editor-checklist-v1.md` |
| Plano de lacunas da linguagem 0.4.2-beta.rc1 | `0.4.2-beta.rc1-language-gap-implementation-plan.md` |
| Plano concluido de migracao documental | `documentation-cleanup-plan-2026-05-09.md` |

O planejamento de linguagem v7/language-readiness foi substituido pelo contrato
final em `docs/spec/language/final-language-contract.md`. Os specs post-v1 em
`docs/spec/language/` permanecem como evidencia detalhada e trilha de
implementacao. A trilha ativa para fechar as lacunas restantes de implementacao
da linguagem em `0.4.2-beta.rc1` esta em
`0.4.2-beta.rc1-language-gap-implementation-plan.md`.

Os documentos `selfhosted-*-v1.md` guardam a trilha especifica de self-hosting
e dogfood SH1. Eles nao substituem os specs post-v1 atuais.

Os documentos `editor-*-v1.md` guardam a trilha ativa dos editores:

- Track A: `tools/keter-micro/`
- Track B: `tools/zenith-ide/` planejado

Os arquivos de planejamento antigos foram removidos, fechados ou movidos para o
historico Git. A trilha ativa de linguagem começa em
`docs/spec/language/final-language-contract.md`.

A decisao de design que originou o v7 esta em
`docs/internal/decisions/language/093-language-design-session-v7.md`.

## Historico

Os roadmaps v1-v6 e checklists v1-v6 foram removidos em 2026-04-27.
Consulte o historico git para referencia.

Arquivos de apoio que permanecem:

- `cascade-v1.md`, `cascade-v2.md` â€” historico de sessoes.
- `r3-m5-progress.txt` â€” notas de milestone.

## Borealis

Borealis Studio e engine sao produtos externos ao contrato semantico da
linguagem. Planejamento ativo deve viver no pacote/produto correspondente, nao
como fonte ativa de planejamento da linguagem.

## Dependencias

Upstream:

- `docs/spec/language/README.md`
- `docs/internal/decisions/language/README.md`
- `compiler/CODE_MAP.md`

Downstream:

- implementacao no codigo;
- suites de teste;
- relatorios em `docs/internal/reports/`;
- saidas operacionais em `reports/`.

## Regra de manutencao

Quando uma entrega muda:

1. atualize specs ou decisions se o comportamento da linguagem mudou;
2. atualize o documento de planejamento ativo somente se a entrega pertencer a self-hosting, LSP ou outra trilha ainda mantida aqui;
3. registre evidencia em `docs/internal/reports/` quando a entrega fechar.

Para novas reorganizacoes documentais, crie um plano novo em vez de reabrir
`documentation-cleanup-plan-2026-05-09.md`. O plano de 2026-05-09 fica como
registro de execucao concluida.

