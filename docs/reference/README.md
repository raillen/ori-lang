# Reference Docs

> Referencias consultaveis da linguagem e do ecossistema.
> Audience: package-author, contributor, advanced-user
> Surface: reference

## Objetivo

Guardar regras curtas, estaveis e faceis de consultar.

Use esta pasta para:

- referencia de sintaxe;
- referencia de tipos;
- diagnosticos;
- modelo de projeto;
- APIs estaveis;
- knowledge base.

Se a pergunta for normativa para implementacao do compilador, consulte tambem `docs/spec/language/`.
Para a RC publica, comece por `docs/spec/language/final-language-contract.md` e
`docs/spec/language/zenith-language-spec.md` antes de atualizar qualquer referencia.

## Secoes

- `language/`: referencias publicas da linguagem.
- `stdlib/`: referencias publicas da stdlib.
- `cli/`: referencia de CLI, diagnosticos e tooling.
- `diagnostics/`: destino dedicado para diagnosticos.
- `grammar/`: destino dedicado para sintaxe e gramatica curta.
- `api/`: destino reservado para documentacao gerada por ZDoc.
- `zenith-kb/`: base curta de conhecimento por area.

## Entrada rapida

| Preciso de | Leia |
| --- | --- |
| sintaxe da linguagem | `docs/reference/grammar/syntax.md` |
| tipos e genericos | `docs/reference/language/types.md` |
| `public`, `public var`, imports | `docs/reference/language/modules-and-visibility.md` |
| funcoes e controle de fluxo | `docs/reference/language/functions-and-control-flow.md` |
| `optional` e `result` | `docs/reference/language/errors-and-results.md` |
| comandos `zt` | `docs/reference/cli/zt.md` |
| comandos `zpm` | `docs/reference/cli/zpm.md` |
| diagnosticos | `docs/reference/diagnostics/README.md` |
| gramatica curta | `docs/reference/grammar/README.md` |
| modulos da stdlib | `docs/reference/stdlib/modules.md` |
| I/O e JSON | `docs/reference/stdlib/io-json.md` |
| texto, bytes e formatacao | `docs/reference/stdlib/text-bytes-format.md` |
| filesystem, OS e tempo | `docs/reference/stdlib/filesystem-os-time.md` |
| collections | `docs/reference/stdlib/collections.md` |
| math, regex, random e validate | `docs/reference/stdlib/math-random-validate.md` |
| concurrent, lazy, test e net | `docs/reference/stdlib/concurrency-lazy-test-net.md` |
