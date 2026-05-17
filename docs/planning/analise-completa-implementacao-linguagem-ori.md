# Analise Completa da Implementacao da Linguagem Ori

Data: 2026-05-16

Fonte principal: `docs/planning/ori-test-prompt.md`

Objetivo: comparar o prompt de teste profundo com a implementacao atual da
linguagem, incluindo lexer, parser, AST, HIR, type checker, codegen nativo,
runtime, CLI, LSP, testes e documentacao.

## Resumo Executivo

Status geral: implementacao forte e validada.

O compilador cobre a maior parte do prompt:

- sintaxe principal;
- tipos primitivos e genericos;
- structs, enums, traits e `implement`;
- `optional`, `result` e propagacao com `?`;
- colecoes, index, slice e range inteiro;
- contratos de campo e parametro em runtime;
- `using` com cleanup deterministico no backend nativo;
- async/await no subconjunto nativo suportado;
- `ori run`, `ori test`, `ori fmt`, `ori doc`;
- diagnostics dedicados e catalogo verificado;
- LSP com diagnostics, completion, go-to-definition e hover semantico local.

Nao ha P0 encontrado nesta analise.

Ainda existem lacunas reais:

1. `generic.circular_instantiation` ainda e planejado, nao implementado.
2. `docs/spec/04-types.md` ainda descreve `range<T>` generico, mas a
   implementacao atual aceita apenas `range<int>`.
3. `docs/spec/11-generics.md` ainda sugere `Iterable<Item>`, mas o contrato real
   e `core.Iterable` marcador + `mut func next() -> optional<T>`.
4. O prompt tem conflito interno em 7.2: usa enum com payload posicional
   `Io(IoError)`, mas a propria parte 2.3 exige variants com campos nomeados.
5. Backend C continua debug/partial; backend nativo e o alvo real validado.
6. Alguns casos do prompt existem como suporte no codigo, mas ainda nao aparecem
   como teste direto 1:1.

## Validacao Rodada

Comandos executados:

```powershell
cargo test --workspace
cargo run -q -p ori-driver -- check examples\hello_world.orl
cargo run -q -p ori-driver -- run examples\hello_world.orl
cargo test -p ori-lsp
cargo test -p ori-driver --test diagnostic_catalog
cargo run -q -p ori-driver -- --help
```

Resultado:

- `cargo test --workspace`: passou.
- `ori check examples\hello_world.orl`: passou, `no errors`.
- `ori run examples\hello_world.orl`: passou e imprimiu o programa.
- `cargo test -p ori-lsp`: passou, 9 testes.
- `cargo test -p ori-driver --test diagnostic_catalog`: passou.
- CLI lista `check`, `doc`, `test`, `fmt`, `lex`, `parse`, `compile`, `run`,
  `build`.

## Matriz por Parte do Prompt

Legenda:

- OK: implementado e coberto por teste/documentacao suficiente.
- PARCIAL: implementado, mas com limite ou cobertura incompleta.
- LACUNA: divergencia real contra o prompt.

| Parte | Area | Status | Observacao |
|---|---|---:|---|
| 1 | Lexical Structure | PARCIAL | Lexer cobre comentarios, numericos, strings, bytes e triple string; falta teste runtime 1:1 para baseline de triple quote. |
| 2 | Type System | OK | Primitivos, structs, enums nomeados, tuple, optional, result, igualdade e aliases estao cobertos. |
| 3 | Expressions | OK | Aritmetica, comparacao, `?`, pipe, inline `if`, anonymous struct, update, colecoes, index e slice cobertos. |
| 4 | Statements | OK | `const`/`var`, `if some`, `while some`, loop, repeat, match, using e check cobertos. |
| 5 | Functions and Closures | OK | Defaults, named args, contracts, variadic, mut methods, closures, async, panic/todo/unreachable cobertos. |
| 6 | Traits and Implement | OK | Default methods, operadores, Comparable, any<Trait>, ambiguidade e Iterable custom nativo cobertos. |
| 7 | Errors and Propagation | PARCIAL | Implementacao boa; prompt 7.2 tem exemplo de enum em sintaxe invalida. |
| 8 | Memory and Cleanup | OK | ARC/runtime, value semantics e `using` nativo em exit paths cobertos; async+using rejeitado. |
| 9 | Generics | PARCIAL | Inferencia, where, constraints, generics e limitacoes cobertas; circular instantiation falta. |
| 10 | Cross-Cutting | OK | Imports, visibility, cycle, namespace mismatch, full examples, LSP e ferramentas cobertos. |

## Achados Principais

### 1. Suite de testes cobre grande parte da linguagem

Evidencia:

- `compiler/crates/ori-driver/tests/multifile_imports.rs` tem 227 testes.
- `compiler/crates/ori-driver/tests/concurrency_async.rs` tem 35 testes.
- `compiler/crates/ori-driver/tests/method_resolution.rs` cobre traits/metodos.
- `compiler/crates/ori-driver/tests/diagnostic_catalog.rs` garante catalogo de
  diagnostics alinhado ao compilador.
- `compiler/crates/ori-lsp/src/main.rs` tem testes de LSP.

Impacto: bom. A linguagem nao esta so "documentada"; ela tem validacao real.

### 2. `ori-test-prompt.md` esta mais alinhado que antes

O prompt atual ja reflete varias decisoes recentes:

- range float e invalido hoje;
- struct update oficial usa `with { ... } end`;
- slice e half-open;
- `core.Iterable` usa `next() -> optional<T>`;
- `ori run` existe;
- LSP tem hover semantico para simbolos locais.

Impacto: baixo risco. Prompt virou boa fonte de testes.

### 3. Spec de range ainda esta obsoleta

Arquivo: `docs/spec/04-types.md`

Texto atual ainda diz:

- `range<T>`;
- "inclusive range of ordered values";
- propriedades como `r.length()` e `r.contains(v)`.

Implementacao atual:

- range literal e `range<int>`;
- endpoints float falham;
- prompt ja pede `0.0..1.0` invalido;
- `for` reconhece `range<int>`.

Status: LACUNA de documentacao.

Acao recomendada:

- mudar `range<T>` para `range<int>` nesta fase da linguagem;
- remover ou marcar `length()`/`contains()` como planejados se nao existirem;
- explicar inclusividade do range separada de slice half-open.

### 4. Spec de generics ainda cita `Iterable<Item>`

Arquivo: `docs/spec/11-generics.md`

Texto atual diz:

- associated types nao suportados;
- "use `Iterable<Item>` instead".

Implementacao atual:

- parser nao usa `Iterable<Item>` como contrato de `for`;
- contrato real e `core.Iterable`;
- metodo requerido: `mut func next() -> optional<T>`;
- item `T` e inferido pelo retorno de `next`.

Status: LACUNA de documentacao.

Acao recomendada:

- trocar `Iterable<Item>` por `core.Iterable` marcador;
- apontar para `docs/spec/06-statements.md` e `docs/spec/08-traits.md`.

### 5. Prompt 7.2 tem conflito interno de enum

Prompt parte 2.3 diz:

- variants devem ter campos nomeados;
- payload posicional deve ser erro.

Prompt parte 7.2 mostra:

```ori
enum AppError
    Io(IoError)
    Validation(ValidationError)
end
```

Isto conflita com a regra de campos nomeados.

Implementacao atual:

- `parse_enum_variant` exige `name: Type` dentro de payload;
- parser rejeita payload posicional;
- pattern de enum aceita shorthand de campo, por exemplo `Done(code)` quando o
  campo real chama `code`.

Status: LACUNA do prompt, nao do compilador.

Forma alinhada:

```ori
enum AppError
    Io(error: IoError)
    Validation(error: ValidationError)
end
```

### 6. Circular generic instantiation ainda falta

Prompt parte 9.6 pede erro para:

```ori
func recurse<T>(value: T) -> T
    return recurse(value)
end
```

Evidencia:

- `docs/spec/13-error-catalog.md` lista `generic.circular_instantiation` como
  planned.
- Busca no codigo nao encontrou emissao real deste diagnostic.
- Existem testes de monomorphizacao generica normal, mas nao de ciclo infinito.

Status: LACUNA real.

Risco: medio. Pode virar recursao de especializacao, output incorreto ou erro
tardio dependendo do formato do programa.

Acao recomendada:

- adicionar detector de stack de instanciacao em `ori-hir/src/monomorph.rs`;
- emitir `generic.circular_instantiation` ou erro equivalente;
- criar teste negativo dedicado.

### 7. Backend C e parcial por desenho

O prompt foca linguagem que compila nativo. O backend nativo atual passa. O
backend C ainda e debug/partial.

Exemplo:

- custom `Iterable` em `for` funciona no backend nativo;
- `c_backend` ainda tem teste que reporta unsupported para alguns iterables;
- async/concurrency tambem rejeitam C backend.

Status: PARCIAL, mas nao bloqueia backend principal.

Acao recomendada:

- documentar C backend como debug/partial em qualquer checklist publica;
- nao tratar divergencia C como falha de linguagem enquanto native e contrato.

### 8. Cobertura exata 1:1 do prompt ainda pode melhorar

O suite cobre comportamento amplo. Nem todo item do prompt aparece como teste
nomeado e isolado.

Casos que merecem teste direto:

- triple quote com baseline stripping em runtime;
- multi-line interpolated triple string;
- short-circuit com side effect explicito;
- integer division by zero e float division by zero em teste runtime dedicado;
- `throw`, `catch`, `try` como palavras nao existentes;
- full pipeline program unico cobrindo todos itens de 10.1;
- LSP diagnostic em fluxo real de documento aberto, nao so helpers internos.

Status: PARCIAL de cobertura, nao necessariamente bug.

## Analise por Camada

### Lexer

Estado: forte.

Cobre:

- comentarios de linha;
- block comments;
- unclosed block comment;
- BOM;
- tipos primitivos;
- numericos com suffix;
- strings, fstrings, bytes e triple strings;
- contextual `times` como identificador.

Ponto fraco:

- triple quote baseline existe no parser, mas precisa de teste runtime dedicado
  com output esperado.

### Parser

Estado: forte.

Cobre:

- namespace/import;
- structs, enums nomeados, traits, impls;
- alias;
- funcs sync/async/mut;
- params default, variadic, contracts;
- `if some`, `while some`, `repeat`, `using`, `check`;
- struct update com braces;
- unsupported HKT, associated type e const generic com diagnostics dedicados.

Ponto fraco:

- prompt 7.2 precisa sintaxe de enum nomeada.

### Type Checker

Estado: forte.

Cobre:

- tipos primitivos e inferencia basica;
- collections;
- optional/result/proper `?`;
- anonymous struct com tipo esperado;
- traits e ambiguity;
- where constraints e negative constraints;
- `using` exige Disposable;
- async restrictions;
- diagnostics dedicados.

Ponto fraco:

- circular generic instantiation nao aparece como erro emitido.

### HIR e Monomorphization

Estado: bom.

Cobre:

- lowering de generics;
- chamadas qualificadas de trait;
- default methods;
- operator methods;
- async state-machine subset;
- custom Iterable para native.

Ponto fraco:

- precisa detector explicito de ciclo/infinite instantiation.

### Native Codegen e Runtime

Estado: forte.

Cobre:

- runtime ABI;
- ARC e cycle collection;
- collections;
- strings/bytes;
- index/slice bounds;
- field/param contracts;
- check/panic/todo/unreachable;
- using cleanup LIFO e em traps;
- async executor/futures/task/channel/atomic;
- stdlib extensa.

Ponto fraco:

- backend nativo e o contrato real; C nao deve ser vendido como equivalente.

### CLI e Ferramentas

Estado: bom.

Cobre:

- `ori check`;
- `ori compile`;
- `ori run`;
- `ori test`;
- `ori fmt`;
- `ori doc`;
- `ori lex`;
- `ori parse`;
- `ori build` C debug backend.

### LSP

Estado: bom para MVP.

Cobre:

- diagnostics por arquivo;
- completion stdlib;
- go-to-definition local;
- hover builtin;
- hover para funcao, struct field, binding, parametro e contrato `it`.

Ponto fraco:

- ainda e indice textual leve, nao semantic model completo do checker.

## Lacunas Priorizadas

### P1 - Implementar `generic.circular_instantiation`

Motivo:

- prompt exige;
- catalogo ja reserva diagnostic;
- generics estao grandes o bastante para precisar protecao.

Arquivos provaveis:

- `compiler/crates/ori-hir/src/monomorph.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`
- `docs/spec/13-error-catalog.md`

### P1 - Corrigir `docs/spec/04-types.md` sobre range

Motivo:

- spec contradiz prompt e checker;
- pode gerar testes falsos de `range<float>` ou APIs inexistentes.

Arquivos:

- `docs/spec/04-types.md`
- talvez `docs/spec/05-expressions.md`

### P1 - Corrigir `docs/spec/11-generics.md` sobre Iterable

Motivo:

- contrato real ja mudou;
- docs de generics ainda sugerem forma nao suportada.

Arquivo:

- `docs/spec/11-generics.md`

### P2 - Corrigir prompt 7.2 de Error Enum

Motivo:

- prompt contradiz sua propria regra de enum nomeado;
- gerador de testes pode criar caso invalido por engano.

Arquivo:

- `docs/planning/ori-test-prompt.md`

### P2 - Adicionar testes 1:1 para pontos ainda indiretos

Motivo:

- suite e forte, mas prompt e mais granular;
- testes nomeados tornam regressao mais facil de localizar.

Casos:

- triple string baseline;
- f-triple string;
- short-circuit com side effect;
- division by zero int vs float;
- no exceptions keywords;
- full pipeline program unico.

### P3 - Explicitar contrato do C backend

Motivo:

- evita confundir "debug backend" com backend completo.

Arquivos:

- `README.md`
- `docs/spec` ou `docs/planning/native-route.md`

## Veredito

Ori esta em estado bom para continuar evolucao por testes.

Nao vejo regressao grande contra `ori-test-prompt.md`.

O trabalho mais importante agora nao e "corrigir tudo do zero". E fechar as
poucas divergencias:

1. detector de circular generic instantiation;
2. docs antigas de `range<T>` e `Iterable<Item>`;
3. exemplo invalido de enum em 7.2;
4. testes 1:1 para os casos ainda indiretos.

Depois disso, o prompt pode virar uma suite oficial de aceitacao da linguagem.
