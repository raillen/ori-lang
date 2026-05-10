# Contributing to Zenith

Obrigado por contribuir.

Este arquivo cobre 3 pontos:

1. Como enviar contribuicao
2. Licenciamento de contribuicoes
3. Regras de terceiros (codigo copiado/portado)

## 1) Fluxo curto

1. Abra issue (bug/feature) com contexto minimo.
2. Faca branch com foco unico.
3. Adicione testes obrigatorios:
   - bugfix: teste de regressao;
   - feature: teste positivo e teste negativo.
4. Rode os gates do projeto antes de abrir PR.
5. Atualize docs no mesmo PR quando houver mudanca de comportamento.
6. Abra PR com descricao curta e objetiva.

Gate minimo oficial (local):

- `python run_all_tests.py`
- `./zt.exe fmt tests/behavior/tooling_gate_smoke --check`
- `./zt.exe doc check tests/behavior/tooling_gate_smoke`

## 1.1) Regras continuas obrigatorias (R2)

- todo bug novo deve incluir teste de regressao;
- toda feature nova deve incluir teste positivo e negativo;
- toda mudanca de comportamento deve atualizar docs no mesmo PR;
- regressao critica de performance bloqueia merge sem override documentado;
- divergencia spec x codigo deve ser classificada em P0/P1/P2.

## 1.2) Evidencia minima de fechamento

Todo PR de fechamento deve registrar:

- comando executado + resultado;
- arquivo de teste novo ou alterado;
- commit/PR de fechamento;
- risco residual (se houver).

## 2) Licenca de contribuicoes

Ao enviar codigo para este repositorio, voce concorda que sua contribuicao e licenciada como:

- Apache-2.0 OR MIT

Regra padrao (inspirada no ecossistema Rust):

- A menos que voce declare o contrario explicitamente,
- toda contribuicao submetida intencionalmente para o projeto,
- entra no projeto sob licenca dupla Apache-2.0 OR MIT,
- sem termos adicionais.

## 3) Regras para codigo de terceiros

Antes de copiar/portar codigo de outro projeto:

- confirme compatibilidade de licenca com Apache-2.0 OR MIT;
- preserve avisos de copyright exigidos;
- cite a origem no PR;
- evite qualquer codigo sob licenca incompativel.

Nao enviar:

- codigo GPL que exija relicenciamento do Zenith;
- conteudo sem autorizacao clara;
- codigo com termos adicionais restritivos.

## 4) Sign-off (DCO simples)

Recomendado em commits de contribuicao:

Signed-off-by: Seu Nome <seu-email>

Exemplo de comando:

`git commit -s -m "fix: corrigir parser edge case"`

## 5) Checklist de PR

- [ ] problema esta claro
- [ ] testes adicionados/atualizados
- [ ] documentacao atualizada se houve mudanca de comportamento
- [ ] sem regressao nos comandos de gate
- [ ] origem/licenca de codigo externo validada
