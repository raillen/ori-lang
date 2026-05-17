# Analise Completa da Implementacao da Linguagem Ori

Data: 2026-05-16

Fonte principal: `docs/planning/ori-test-prompt.md`

Objetivo: comparar o prompt de teste profundo com a implementacao atual da
linguagem, incluindo lexer, parser, AST, HIR, type checker, codegen nativo,
runtime, CLI, LSP, testes e documentacao.

## Resumo Executivo

Status geral: implementacao forte, validada e com as lacunas reais desta
analise fechadas nesta rodada.

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

Lacunas reais encontradas e status de fechamento:

- [x] `generic.circular_instantiation`: implementado no checker, catalogado em
  `docs/spec/13-error-catalog.md` e coberto por teste negativo dedicado.
- [x] `range<T>` obsoleto: docs corrigidas para `range<int>` em
  `docs/spec/04-types.md` e `docs/spec/12-stdlib.md`.
- [x] `Iterable<Item>` obsoleto: docs corrigidas para `core.Iterable` marcador
  + `mut func next() -> optional<T>` em `docs/spec/11-generics.md`.
- [x] Prompt 7.2: enum de erro corrigido para payload nomeado em
  `docs/planning/ori-test-prompt.md`; exemplo normativo tambem alinhado em
  `docs/spec/09-errors.md`.
- [x] Backend C debug/partial: contrato ja estava explicito em `README.md`,
  `docs/spec/10-memory.md`, `docs/spec/12-stdlib.md` e
  `docs/planning/native-route.md`.
- [x] Testes 1:1 do prompt: adicionados testes diretos para triple string,
  f-triple string, short-circuit com side effect, divisao por zero int/float,
  ausencia de `throw`/`catch`/`try`, full pipeline e LSP open-doc/hover.

Atualizacao de fechamento: 2026-05-17.

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

Validacao de fechamento em 2026-05-17:

```powershell
cargo fmt --check
cargo test -p ori-driver --test multifile_imports
cargo test -p ori-lsp
cargo test -p ori-driver --test diagnostic_catalog
```

Resultado:

- `cargo fmt --check`: passou.
- `cargo test -p ori-driver --test multifile_imports`: passou, 235 testes.
- `cargo test -p ori-lsp`: passou, 11 testes.
- `cargo test -p ori-driver --test diagnostic_catalog`: passou.

## Matriz por Parte do Prompt

Legenda:

- OK: implementado e coberto por teste/documentacao suficiente.
- PARCIAL: implementado, mas com limite ou cobertura incompleta.
- LACUNA: divergencia real contra o prompt.

| Parte | Area | Status | Observacao |
|---|---|---:|---|
| 1 | Lexical Structure | OK | Lexer cobre comentarios, numericos, strings, bytes e triple string; baseline e f-triple string agora tem teste runtime direto. |
| 2 | Type System | OK | Primitivos, structs, enums nomeados, tuple, optional, result, igualdade e aliases estao cobertos. |
| 3 | Expressions | OK | Aritmetica, comparacao, `?`, pipe, inline `if`, anonymous struct, update, colecoes, index e slice cobertos. |
| 4 | Statements | OK | `const`/`var`, `if some`, `while some`, loop, repeat, match, using e check cobertos. |
| 5 | Functions and Closures | OK | Defaults, named args, contracts, variadic, mut methods, closures, async, panic/todo/unreachable cobertos. |
| 6 | Traits and Implement | OK | Default methods, operadores, Comparable, any<Trait>, ambiguidade e Iterable custom nativo cobertos. |
| 7 | Errors and Propagation | OK | Implementacao boa; prompt 7.2 foi corrigido para enum com payload nomeado e ha teste de ausencia de exceptions. |
| 8 | Memory and Cleanup | OK | ARC/runtime, value semantics e `using` nativo em exit paths cobertos; async+using rejeitado. |
| 9 | Generics | OK | Inferencia, where, constraints, generics, limitacoes e circular instantiation agora estao cobertos. |
| 10 | Cross-Cutting | OK | Imports, visibility, cycle, namespace mismatch, full pipeline, LSP e ferramentas cobertos. |

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

### 3. Spec de range estava obsoleta

Arquivo: `docs/spec/04-types.md`

Texto antigo dizia:

- `range<T>`;
- "inclusive range of ordered values";
- propriedades como `r.length()` e `r.contains(v)`.

Implementacao atual:

- range literal e `range<int>`;
- endpoints float falham;
- prompt ja pede `0.0..1.0` invalido;
- `for` reconhece `range<int>`.

Status: FECHADO em 2026-05-17.

Acao concluida:

- `range<T>` foi trocado por `range<int>` nesta fase da linguagem;
- `length()` e `contains()` nao sao mais apresentados como metodos atuais de
  range;
- a inclusividade do range ficou separada da regra half-open de slice.

### 4. Spec de generics citava `Iterable<Item>`

Arquivo: `docs/spec/11-generics.md`

Texto antigo dizia:

- associated types nao suportados;
- "use `Iterable<Item>` instead".

Implementacao atual:

- parser nao usa `Iterable<Item>` como contrato de `for`;
- contrato real e `core.Iterable`;
- metodo requerido: `mut func next() -> optional<T>`;
- item `T` e inferido pelo retorno de `next`.

Status: FECHADO em 2026-05-17.

Acao concluida:

- `Iterable<Item>` foi trocado por `core.Iterable` marcador;
- o contrato real agora esta descrito como `mut func next() -> optional<T>`.

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

Status: FECHADO em 2026-05-17.

Forma alinhada:

```ori
enum AppError
    Io(error: IoError)
    Validation(error: ValidationError)
end
```

### 6. Circular generic instantiation faltava

Prompt parte 9.6 pede erro para:

```ori
func recurse<T>(value: T) -> T
    return recurse(value)
end
```

Evidencia original:

- `docs/spec/13-error-catalog.md` lista `generic.circular_instantiation` como
  planned.
- Busca no codigo nao encontrou emissao real deste diagnostic.
- Existem testes de monomorphizacao generica normal, mas nao de ciclo infinito.

Status: FECHADO em 2026-05-17.

Risco: medio. Pode virar recursao de especializacao, output incorreto ou erro
tardio dependendo do formato do programa.

Acao concluida:

- detector conservador adicionado no checker para autochamada generica sem
  instanciacao concreta;
- diagnostic `generic.circular_instantiation` agora e emitido;
- catalogo movido de planejado para emitido;
- teste `check_reports_circular_generic_instantiation` adicionado.

### 7. Backend C e parcial por desenho

O prompt foca linguagem que compila nativo. O backend nativo atual passa. O
backend C ainda e debug/partial.

Exemplo:

- custom `Iterable` em `for` funciona no backend nativo;
- `c_backend` ainda tem teste que reporta unsupported para alguns iterables;
- async/concurrency tambem rejeitam C backend.

Status: FECHADO como contrato/documentacao. Continua parcial por desenho, mas
nao e lacuna do backend principal.

Acao confirmada:

- documentar C backend como debug/partial em qualquer checklist publica;
- nao tratar divergencia C como falha de linguagem enquanto native e contrato.

### 8. Cobertura exata 1:1 do prompt foi ampliada

O suite cobre comportamento amplo. Nem todo item do prompt aparece como teste
nomeado e isolado.

Casos que ganharam teste direto:

- triple quote com baseline stripping em runtime;
- multi-line interpolated triple string;
- short-circuit com side effect explicito;
- integer division by zero e float division by zero em teste runtime dedicado;
- `throw`, `catch`, `try` como palavras nao existentes;
- full pipeline program unico cobrindo itens de 10.1;
- LSP diagnostic em fluxo de fonte aberta/unsaved e hover de campo com
  contrato.

Status: FECHADO para os pontos listados nesta analise.

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

Fechado nesta rodada:

- triple quote baseline agora tem teste runtime dedicado com output esperado:
  `compile_runs_triple_string_baseline_and_f_triple_string`.

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

Fechado nesta rodada:

- prompt 7.2 foi reescrito com sintaxe de enum nomeada.

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

Fechado nesta rodada:

- `generic.circular_instantiation` agora aparece como erro emitido.

### HIR e Monomorphization

Estado: bom.

Cobre:

- lowering de generics;
- chamadas qualificadas de trait;
- default methods;
- operator methods;
- async state-machine subset;
- custom Iterable para native.

Fechado nesta rodada:

- detector explicito de autochamada generica sem instanciacao concreta foi
  adicionado no checker.

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

Contrato fechado:

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
- hover para funcao, struct field, binding, parametro, contrato `it` e campo
  com contrato.

Limite conhecido:

- ainda e indice textual leve, nao semantic model completo do checker.

## Lacunas Priorizadas

### P1 - Implementar `generic.circular_instantiation` - FECHADO

Motivo original:

- prompt exige;
- catalogo ja reserva diagnostic;
- generics estao grandes o bastante para precisar protecao.

Arquivos alterados:

- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`
- `docs/spec/13-error-catalog.md`

### P1 - Corrigir `docs/spec/04-types.md` sobre range - FECHADO

Motivo original:

- spec contradiz prompt e checker;
- pode gerar testes falsos de `range<float>` ou APIs inexistentes.

Arquivos alterados:

- `docs/spec/04-types.md`
- `docs/spec/12-stdlib.md`

### P1 - Corrigir `docs/spec/11-generics.md` sobre Iterable - FECHADO

Motivo original:

- contrato real ja mudou;
- docs de generics ainda sugerem forma nao suportada.

Arquivo alterado:

- `docs/spec/11-generics.md`

### P2 - Corrigir prompt 7.2 de Error Enum - FECHADO

Motivo original:

- prompt contradiz sua propria regra de enum nomeado;
- gerador de testes pode criar caso invalido por engano.

Arquivos alterados:

- `docs/planning/ori-test-prompt.md`
- `docs/spec/09-errors.md`

### P2 - Adicionar testes 1:1 para pontos ainda indiretos - FECHADO

Motivo original:

- suite e forte, mas prompt e mais granular;
- testes nomeados tornam regressao mais facil de localizar.

Casos:

- triple string baseline;
- f-triple string;
- short-circuit com side effect;
- division by zero int vs float;
- no exceptions keywords;
- full pipeline program unico;
- LSP open-doc diagnostic e hover de campo com contrato.

### P3 - Explicitar contrato do C backend - FECHADO

Motivo original:

- evita confundir "debug backend" com backend completo.

Arquivos confirmados:

- `README.md`
- `docs/spec/10-memory.md`
- `docs/spec/12-stdlib.md`
- `docs/planning/native-route.md`

## Veredito

Ori esta em estado bom para continuar evolucao por testes.

Nao vejo regressao grande contra `ori-test-prompt.md`.

As divergencias reais encontradas nesta analise foram fechadas:

1. detector de circular generic instantiation;
2. docs antigas de `range<T>` e `Iterable<Item>`;
3. exemplo invalido de enum em 7.2;
4. testes 1:1 para os casos indiretos;
5. confirmacao documental do backend C como debug/partial.

Proximo passo recomendado: promover o prompt para uma suite oficial de
aceitacao, mantendo os casos novos como base rastreavel.
