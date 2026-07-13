# Estabilidade e compatibilidade

Status: normativo para o ciclo de superfície **S3 / `0.3.0`** (package Cargo
pode permanecer `0.2.0` até a tag de release — ver `CHANGELOG.md` e `AGENTS.md`).

Ori ainda esta antes da versao `1.0`. Mesmo assim, o projeto deve separar o que
e contrato publico do que e experimento. Essa separacao reduz surpresas para
quem esta aprendendo a linguagem ou mantendo um projeto pequeno.

## Contrato estavel do ciclo atual

Durante o ciclo **S3 (`0.3.0` surface)**, estes pontos devem ser tratados como
contrato publico:

- arquivos `.orl` em UTF-8;
- blocos terminados por `end` (labels opcionais: `end if`, `end match`, …);
- `module` obrigatorio no topo do arquivo (`namespace` e erro);
- imports explicitos nas tres formas S3: `import path (A)`, `import path = alias`,
  `import path` (sem `as` / `only` de import);
- tipos explicitos em bindings, parametros e retornos publicos;
- tipos compostos com `[]` (`optional[T]`, `result[T, E]`, `list[T]`, …);
- ausencia via `optional[T]`;
- falha via `result[T, E]`;
- propagacao **somente** via `try expr` (postfix `expr?` e erro);
- construcao de struct por `Type { field: value }` ou `{ field: value}` quando o
  tipo esperado e conhecido;
- construcao de enum por `Enum.Variant(...)` ou `.Variant(...)` quando o tipo
  esperado e conhecido;
- traits via `apply Type` + `use Trait`;
- backend nativo como referencia semantica.

Mudancas nesses pontos devem ser documentadas no `CHANGELOG.md` e precisam de
teste de regressao.

## Contrato binario nativo (M3 + ABI-1)

O backend nativo documenta o ABI em [`19-abi.md`](19-abi.md) sob a tag
**`ori-native-abi-1`** (`ORI_ABI_VERSION` no `ori-runtime`). **ABI-1 esta em
vigor** com a janela FREEZE-1 (ver `docs/planning/freeze-and-abi-gates.md`).
Isso inclui:

- layouts de primitivos, structs, enums (tag `i32`), `optional`/`result`/tuples;
- header ARC (`OriHeapHeader`) e simbolos `ori_alloc` / `ori_arc_*`;
- layouts de colecoes runtime (`OriList`, `OriMap`, `OriSet`, …);
- mangling `ORI__*` e export de `main`;
- checagem de `abi_version` no `runtime-link.json`.

Mudanca **incompativel** com runtimes ja staged exige bump de
`ori-native-abi-N`, atualizacao do cap. 19 e re-stage staticlib+cdylib.
Simbolos `ori_*` **aditivos** nao exigem bump, mas devem ser listados no
manifesto stdlib.

O backend C/debug **nao** e referencia de ABI.

## Contrato experimental

Estes pontos podem mudar antes de `1.0`:

- formato final de pacote e lockfile;
- registry hospedado;
- limites do REPL;
- APIs marcadas como experimentais na stdlib;
- detalhes de otimizacao de generics e tamanho de binario;
- superficie publica do backend C/debug;
- detalhes finos de inferencia local alem da opcao B ja documentada
  (literais + campo/index/call/pipe); **HM global permanece fora**);
- formatos de package/registry ainda nao estabilizados;
- nomes finais de aliases de dominio na stdlib alem dos ja documentados.

> **Ja estabilizado na superficie pre-1.0 (mas ainda pre-1.0):** pipe `|>`
> (mantido na Ori), inferencia local Nim-style + opcao B (`0.3.1`+), ABI nativo
> `ori-native-abi-1` (M3), caminho de instalacao sem Rust (M1).

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
