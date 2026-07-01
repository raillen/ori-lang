# Estabilidade e compatibilidade

Status: normativo para o ciclo `0.2.x`.

Ori ainda esta antes da versao `1.0`. Mesmo assim, o projeto deve separar o que
e contrato publico do que e experimento. Essa separacao reduz surpresas para
quem esta aprendendo a linguagem ou mantendo um projeto pequeno.

## Contrato estavel do ciclo atual

Durante o ciclo `0.2.x`, estes pontos devem ser tratados como contrato publico:

- arquivos `.orl` em UTF-8;
- blocos terminados por `end`;
- `namespace` obrigatorio no topo do arquivo;
- imports explicitos, inclusive `as` e `only`;
- tipos explicitos em bindings, parametros e retornos publicos;
- ausencia via `optional<T>`;
- falha via `result<T, E>`;
- propagacao via `try expr` ou `expr?`;
- construcao de struct por `Type(field: value)` ou `.{field: value}` quando o
  tipo esperado e conhecido;
- construcao de enum por `Enum.Variant(...)` ou `.Variant(...)` quando o tipo
  esperado e conhecido;
- backend nativo como referencia semantica.

Mudancas nesses pontos devem ser documentadas no `CHANGELOG.md` e precisam de
teste de regressao.

## Contrato experimental

Estes pontos podem mudar antes de `1.0`:

- formato final de pacote e lockfile;
- registry hospedado;
- limites do REPL;
- APIs marcadas como experimentais na stdlib;
- detalhes de otimizacao de generics e tamanho de binario;
- superficie publica do backend C/debug.

## Regra de documentacao

A spec deve descrever o que o parser, checker, runtime e tooling aceitam hoje.
Ideias futuras pertencem a `docs/planning/`.

Quando uma feature sair de planejamento para implementacao, a alteracao deve
atualizar na mesma entrega:

- spec normativa;
- exemplos ou fixtures;
- testes;
- `CHANGELOG.md`;
- docs de planejamento, marcando o item como entregue ou alterado.
