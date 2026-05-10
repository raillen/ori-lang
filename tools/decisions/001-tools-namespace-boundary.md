# Decision 001 - Fronteira entre std, tools e zt

- Status: draft
- Data: 2026-04-20
- Tipo: tools / architecture
- Escopo: namespace, responsabilidade e dependencia

## Resumo

Zenith deve manter `std.*` enxuta e criar `tools.*` para bibliotecas oficiais de ferramentas.

A separacao canonica e:

```text
std.*   = base universal
tools.* = bibliotecas para criar ferramentas
zt.*    = comandos oficiais da linguagem
```

## Decisao

1. `std.*` nao deve depender de `tools.*`.
2. `tools.*` pode depender de `std.*`.
3. `zt.*` pode depender de `std.*` e `tools.*`.
4. APIs opinativas de CLI nao entram em `std.*` por padrao.
5. Recursos universais continuam em `std.*`.

## Exemplos de std.*

1. `std.fs`
2. `std.path`
3. `std.process`
4. `std.os`
5. `std.time`
6. `std.json`
7. `std.io`

## Exemplos de tools.*

1. `tools.cli`
2. `tools.output`
3. `tools.diagnostics`
4. `tools.prompt`
5. `tools.progress`
6. `tools.table`
7. `tools.template`

## Criterio de fronteira

Pergunta para `std`:

`um programa comum, nao necessariamente uma ferramenta, precisa disso?`

Pergunta para `tools`:

`isso ajuda a criar CLIs, geradores, linters, formatadores ou automacoes?`

Se a resposta for sim para a segunda pergunta e nao para a primeira, o recurso pertence a `tools`.

## Racional

A `stdlib` deve ser estavel e pequena.

Ferramentas precisam de APIs mais opinativas.

Separar `tools.*` evita transformar `std.*` em um framework de CLI.

## Consequencias

1. `tools.*` pode evoluir mais rapido que `std.*`.
2. `std.*` fica mais previsivel para todos os alvos.
3. comandos oficiais `zt.*` viram campo de prova para `tools.*`.
4. mudancas quebrantes em `tools.*` nao implicam mudanca na linguagem central.

## Fora de escopo

1. formato final de pacote `tools`
2. versionamento publico de `tools.*`
3. implementacao concreta dos modulos
4. compatibilidade binaria
