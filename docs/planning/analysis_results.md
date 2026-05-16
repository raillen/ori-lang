# Resultado historico da analise residual da linguagem Ori

Este arquivo substitui o relatorio antigo, que estava com mojibake e misturava
achados atuais com itens ja resolvidos.

Fonte de verdade operacional: `_reversa_sdd/plano-correcao-implementacao-linguagem.md`,
secao `P10 - Pendencias reais restantes de docs/analysis_results.md`.

Status em 2026-05-15: este arquivo e historico. Para o backlog ativo da rota
nativa 100%, use `docs/IMPLEMENTATION_CHECKLIST_2.md`. Itens abaixo podem ter
sido superados por implementacoes posteriores e nao devem ser tratados como
pendencia ativa sem revalidacao no codigo atual.

## Estado atual

### Ja resolvido ou obsoleto

- `BytesLit` no backend nativo: existe codegen e teste E2E.
- Indice de tupla fora do limite: existe diagnostico no checker.
- Constantes globais comuns: existem testes de execucao para `const` global e
  `const` importada.
- `HirExprKind::GlobalConst`: o node residual foi removido do HIR e dos backends.

### Corrigido no P10

- `==` e `!=` em `any<Trait>` agora sao proibidos pelo checker com
  `type.comparison_not_supported`.
- `is` em `any<Trait>` continua validado por teste nativo.
- `lazy<T>` agora e aceito pelo checker e tem runtime/std para `lazy.once` e
  `lazy.force`.
- Os backends C e native geram thunks de avaliacao unica e cacheiam o resultado.
- A spec, o catalogo de erros e o checklist foram atualizados para tratar
  `lazy<T>` como feature pronta.
- O backend nativo prefere `ori_string_len` e mantem `strlen` apenas como fallback
  para ponteiros C.

### Historico: planejado explicitamente na epoca da auditoria

- ARC completo com retain/release real para tipos gerenciados.
- Deteccao de ciclos em objetos com contagem de referencia.

## Como ler este arquivo

Use este arquivo como historico limpo da auditoria residual. Para saber o que
ainda precisa ser executado, consulte `docs/IMPLEMENTATION_CHECKLIST_2.md` e o
plano de correcao em `_reversa_sdd/`.
