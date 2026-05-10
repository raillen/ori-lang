# Language Decisions

Este diretorio registra decisoes da linguagem.

Cada arquivo deve responder:

- o que foi decidido;
- por que foi decidido;
- qual comportamento fica normativo;
- quais efeitos colaterais precisam ser lembrados.

## Como ler

Os arquivos sao numerados em ordem historica.

Use a ordem crescente quando quiser entender a evolucao da linguagem.
Use `docs/spec/language/README.md` quando quiser a regra atual consolidada.

## Decisoes recentes de alto impacto

- `086-namespace-public-var-and-controlled-mutation.md`
  Define `public var`, mutacao controlada por namespace e diferenca entre publico e global.
- `087-concurrency-workers-and-transfer-boundaries.md`
  Define limites de concorrencia e transferencia.
- `088-dyn-dispatch-minimum-subset.md`
  Define subconjunto minimo de dispatch dinamico.
- `089-callable-delegates-v1.md`
  Define delegates/callables.
- `090-closures-v1.md`
  Define closures.
- `091-defer-concurrency-full-surface.md`
  Adia a superficie completa de concorrencia.
- `096-generic-runtime-capability-gates.md`
  Define gates internos de capacidade para formas genericas materializadas no runtime.

## Regra de manutencao

Quando uma decisao mudar comportamento da linguagem:

1. atualize ou adicione a decision;
2. atualize `docs/spec/language/`;
3. atualize exemplos e testes;
4. atualize roadmap/checklist se houver impacto de entrega.

Nao use uma decision antiga como fonte unica quando a spec atual contradiz o texto historico.
