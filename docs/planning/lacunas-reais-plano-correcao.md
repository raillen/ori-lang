# Plano de Correcao das Lacunas Reais da Linguagem Ori

Data: 2026-05-16

Fonte principal: `docs/planning/ori-test-prompt.md`

Este plano organiza as lacunas encontradas na analise da implementacao atual da linguagem Ori.

Objetivo: alinhar implementacao, testes, diagnosticos e documentacao.

## Regras de trabalho

- Fazer uma lacuna por vez.
- Em cada lacuna, atualizar codigo, testes e docs juntos.
- Manter mensagens de erro curtas e acionaveis.
- Evitar mudancas amplas sem teste que prove o comportamento.
- Antes de mudar sintaxe da linguagem, decidir se a fonte da verdade sera o spec atual ou o prompt de testes.
- Rodar a validacao base ao fim de cada fase.

## Validacao base

Rodar sempre:

```powershell
cargo test --workspace
cargo run -q -p ori-driver -- check examples\hello_world.orl
cargo run -q -p ori-driver -- compile --out target\ori-plan-smoke.exe examples\hello_world.orl
target\ori-plan-smoke.exe
```

Quando a fase mexer em LSP:

```powershell
cargo test -p ori-lsp
```

Quando a fase mexer em diagnostics:

```powershell
cargo test -p ori-driver --test diagnostic_catalog
```

## Visao de Prioridade

| Prioridade | Lacuna | Tipo | Motivo |
|---|---|---|---|
| P0 | Anonymous struct sem validacao suficiente | Bug real | Codigo invalido pode passar |
| P1 | Sintaxe de struct update divergente | Decisao de linguagem | Parser, spec e prompt discordam |
| P1 | Range float divergente | Decisao de linguagem | Spec/prompt pedem, checker bloqueia |
| P1 | Trait method ambiguity ausente | Semantica incompleta | Prompt espera desambiguacao por trait |
| P2 | Iterable custom em `for` ausente | Semantica incompleta | Spec sugere, backend nao fecha |
| P2 | CLI sem `ori run` | UX/driver | Prompt e fluxo de uso discordam |
| P2 | Diagnostics dedicados faltando | Qualidade de erro | Erros existem, mas sao genericos |
| P3 | LSP semantico limitado | Ferramenta | Funciona, mas nao entrega hover rico |
| P3 | Conflito de slice no prompt | Alinhamento | Prompt contradiz spec/testes atuais |

## Status das Fases

| Fase | Status | Contrato |
|---|---|---|
| Fase 0 | Concluida em 2026-05-16 | Contrato congelado nesta tabela |
| Fase 1 | Concluida em 2026-05-16 | Anonymous struct exige tipo esperado e valida campos |
| Fase 2 | Concluida em 2026-05-16 | Struct update oficial: `with { ... } end` |
| Fase 3 | Concluida em 2026-05-16 | Range oficial no curto prazo: `range<int>` |
| Fase 4 | Concluida em 2026-05-16 | Metodo de trait ambiguo falha; chamada qualificada passa |
| Fase 5 | Concluida em 2026-05-16 | `core.Iterable` com `next() -> optional<T>` funciona em `for` nativo |
| Fase 6 | Concluida em 2026-05-16 | `ori run <file>` compila em temporario, executa e propaga exit code |
| Fase 7 | Concluida em 2026-05-16 | Diagnostics dedicados emitidos e catalogo alinhado |
| Fase 8 | Concluida em 2026-05-16 | Hover semantico para simbolos locais, campos, parametros e contratos |
| Fase 9 | Concluida em 2026-05-16 | Slice oficial: inicio incluso, fim exclusivo |

## Fase 0 - Congelar Contrato de Teste

Objetivo: transformar as lacunas em contrato rastreavel antes de alterar codigo.

Status: concluida em 2026-05-16.

Contrato congelado:

- Fase 1: implementar. Testes negativos devem falhar para campo ausente e anonymous struct sem contexto.
- Fase 2: decidir por `with { ... } end`. Corrigir docs/prompt e melhorar erro.
- Fase 3: decidir por range apenas inteiro. Corrigir docs/prompt e manter erro direto.
- Fase 4: implementar chamada qualificada de trait e erro de ambiguidade.
- Fase 5: implementar Iterable custom depois de fechar contrato `Iterable`/`Iterator`.
- Fase 6: implementar `ori run <file>`.
- Fase 7: implementar diagnostics dedicados onde reduzem confusao; manter resto como planned.
- Fase 8: ampliar hover LSP para simbolos do usuario.
- Fase 9: manter slice half-open. Corrigir prompt.

Tarefas:

- Criar uma lista curta de testes esperados por lacuna.
- Marcar cada item como `implementar`, `decidir` ou `corrigir prompt/spec`.
- Confirmar se o comportamento esperado vem do spec atual, do prompt ou de ambos.

Arquivos provaveis:

- `docs/planning/ori-test-prompt.md`
- `docs/spec/05-expressions.md`
- `docs/spec/08-traits.md`
- `docs/spec/11-generics.md`
- `docs/spec/13-error-catalog.md`
- `compiler/crates/ori-driver/tests/diagnostic_catalog.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`

Pronto quando:

- Cada lacuna tiver um teste alvo.
- Cada divergencia spec versus prompt estiver marcada.

## Fase 1 - Corrigir Anonymous Struct

Problema:

- `.{ x: 1.0 }` pode passar mesmo quando o tipo esperado exige mais campos.
- Expressao anonymous struct sem contexto tambem pode passar.
- O catalogo ja tem codigos planejados:
  - `type.anon_struct_field_mismatch`
- `type.anon_struct_type_unknown`

Status: concluida em 2026-05-16.

Implementado:

- `.{...}` sem tipo esperado agora emite `type.anon_struct_type_unknown`.
- `.{...}` com tipo esperado de struct valida campos ausentes, extras e repetidos.
- Catalogo moveu `type.anon_struct_field_mismatch` e `type.anon_struct_type_unknown` para diagnostics emitidos.
- Testes negativos adicionados em `multifile_imports`.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Comportamento esperado:

- Anonymous struct deve precisar de tipo esperado.
- Se o tipo esperado for struct, todos os campos exigidos devem bater.
- Campo ausente, campo extra ou tipo errado deve gerar erro claro.
- Anonymous struct fora de contexto deve gerar `type.anon_struct_type_unknown`.

Tarefas:

- Localizar tratamento de `AnonStructLit` no AST, HIR e checker.
- No type checker, passar o tipo esperado para a expressao anonymous struct.
- Validar:
  - campos obrigatorios;
  - campos extras;
  - tipos dos campos;
  - duplicidade de campo;
  - uso sem contexto.
- Adicionar testes positivos e negativos.
- Ativar ou documentar codigos no catalogo de erros.

Arquivos provaveis:

- `compiler/crates/ori-ast/src/expr.rs`
- `compiler/crates/ori-hir/src/lower.rs`
- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`
- `compiler/crates/ori-driver/tests/diagnostic_catalog.rs`
- `docs/spec/05-expressions.md`
- `docs/spec/13-error-catalog.md`

Testes minimos:

```ori
struct Vec2
    x: float
    y: float
end

func main()
    const ok: Vec2 = .{ x: 1.0, y: 2.0 }
end
```

```ori
struct Vec2
    x: float
    y: float
end

func main()
    const bad: Vec2 = .{ x: 1.0 }
end
```

```ori
func main()
    .{ x: 1.0, y: 2.0 }
end
```

Pronto quando:

- Codigo invalido nao passa mais.
- Os codigos de erro dedicados aparecem em testes.
- `cargo test --workspace` passa.

## Fase 2 - Decidir e Alinhar Struct Update

Problema:

- Implementacao atual aceita `base with { campo: valor } end`.
- Spec/prompt mostram `base with campo: valor end`.
- A linguagem precisa de uma forma unica.

Status: concluida em 2026-05-16.

Implementado:

- Parser agora emite mensagem direta quando `with` vem sem `{`.
- Grammar/spec/prompt foram alinhados para `base with { field: value } end`.
- Teste negativo adicionado para forma sem braces.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Decisao recomendada:

- Manter braces: `with { ... } end`.

Motivo:

- Evita ambiguidade visual.
- Reaproveita parser de field init.
- Ja esta implementado e testado.
- E mais facil para leitores com TDAH/dislexia porque delimita o bloco de campos.

Tarefas se manter braces:

- Atualizar prompt/spec para `with { ... } end`.
- Adicionar teste que rejeita a forma sem braces com erro claro.
- Melhorar mensagem do parser quando `with` vier sem `{`.

Tarefas se remover braces:

- Alterar parser para aceitar lista de campos ate `end`.
- Garantir que expressoes complexas nao capturem `end` errado.
- Atualizar testes existentes.

Arquivos provaveis:

- `compiler/crates/ori-parser/src/parse_expr.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`
- `docs/spec/03-grammar.ebnf`
- `docs/spec/05-expressions.md`
- `docs/planning/ori-test-prompt.md`

Pronto quando:

- Existe uma unica sintaxe oficial.
- Parser, docs e testes concordam.
- Erro de sintaxe aponta a correcao esperada.

## Fase 3 - Decidir Range Float

Problema:

- Spec/prompt indicam `range<float>`.
- Type checker e HIR tratam range como `range<int>`.

Status: concluida em 2026-05-16.

Implementado:

- Spec/prompt agora dizem que ranges usam endpoints `int`.
- `0.0..1.0` virou caso negativo esperado.
- Teste negativo adicionado para range float.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Decisao necessaria:

- Opcao A: implementar range generico por tipo numerico.
- Opcao B: manter range apenas inteiro e corrigir spec/prompt.

Decisao recomendada para curto prazo:

- Manter range apenas inteiro.

Motivo:

- `for` sobre float exige regra de passo, comparacao e parada.
- Float range pode gerar bugs por precisao.
- `0..10` cobre o uso principal de iteracao.
- Float range pode voltar depois como `range(0.0, 1.0, step: 0.1)`.

Tarefas se manter apenas inteiro:

- Corrigir `docs/spec/05-expressions.md`.
- Corrigir `docs/planning/ori-test-prompt.md`.
- Adicionar teste negativo para `0.0..1.0`.
- Emitir erro mais direto: `range endpoints must be int`.

Tarefas se implementar range float:

- Alterar type checker para `range<T>` onde `T` e numerico aceito.
- Alterar HIR para preservar tipo do endpoint.
- Alterar codegen C/native para range float.
- Definir semantica de inclusividade e passo.
- Adicionar testes de execucao.

Arquivos provaveis:

- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-hir/src/lower.rs`
- `compiler/crates/ori-codegen/src/native_backend.rs`
- `compiler/crates/ori-codegen/src/c_backend.rs`
- `docs/spec/05-expressions.md`
- `docs/planning/ori-test-prompt.md`

Pronto quando:

- Spec, prompt e checker concordam.
- Existe teste positivo ou negativo explicito para float range.

## Fase 4 - Implementar ou Proibir Ambiguidade de Trait Method

Problema:

- Hoje dois traits com mesmo metodo para o mesmo tipo geram `name.duplicate`.
- O prompt espera:
  - permitir os dois impls;
  - rejeitar `value.method()` por ambiguidade;
  - aceitar chamada qualificada por trait.

Decisao necessaria:

- Opcao A: implementar ambiguidade e chamada qualificada.
- Opcao B: proibir metodos de trait duplicados por tipo e documentar isso.

Decisao recomendada:

- Implementar a semantica do prompt.

Motivo:

- E comportamento esperado em linguagens com traits.
- Preserva composicao.
- Evita bloquear bibliotecas independentes que escolhem o mesmo nome de metodo.

Status: concluida em 2026-05-16.

Implementado:

- Metodos de trait implementados agora usam simbolo `Tipo.Trait.metodo`.
- Dois traits diferentes podem expor o mesmo metodo para o mesmo tipo.
- Chamada simples ambigua emite `type.ambiguous_method`.
- Chamada qualificada `Trait.metodo(valor)` e baixada para o metodo correto.
- Cleanup de `using` resolve `Disposable.dispose` pelo trait implementado no backend nativo.
- Testes de metodo, operadores e `using dispose` foram atualizados.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Tarefas:

- Separar namespace de metodos inerentes e metodos de trait.
- Permitir multiplos metodos de trait com mesmo nome para o mesmo tipo.
- No method call simples, detectar mais de um candidato e emitir erro de ambiguidade.
- Implementar chamada qualificada, por exemplo `Alpha.output(value)`.
- Garantir que metodo inerente tenha precedencia clara ou gere regra documentada.
- Adicionar diagnostics dedicados.

Arquivos provaveis:

- `compiler/crates/ori-types/src/resolve.rs`
- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-hir/src/lower.rs`
- `compiler/crates/ori-codegen/src/native_backend.rs`
- `compiler/crates/ori-driver/tests/method_resolution.rs`
- `docs/spec/08-traits.md`
- `docs/spec/13-error-catalog.md`

Testes minimos:

```ori
trait Alpha
    func output(self) -> string
end

trait Beta
    func output(self) -> string
end

struct Thing
    name: string
end

impl Alpha for Thing
    func output(self) -> string
        return "alpha"
    end
end

impl Beta for Thing
    func output(self) -> string
        return "beta"
    end
end

func main()
    const item = Thing { name: "x" }
    const a = Alpha.output(item)
    const b = Beta.output(item)
end
```

Pronto quando:

- Impl duplicado por trait diferente passa.
- Chamada simples ambigua falha.
- Chamada qualificada passa.

## Fase 5 - Implementar Iterable Custom em `for`

Problema:

- Spec menciona `Iterable<Item>`.
- `for` no backend suporta apenas tipos conhecidos.

Comportamento esperado:

- Um tipo que implementa `Iterable<Item>` deve funcionar em `for`.
- O compilador deve baixar o loop para chamadas bem definidas.

Decisao necessaria:

- Definir contrato minimo do trait:
  - `iter(self) -> Iterator<Item>`?
  - `next(self) -> Option<Item>`?
  - `has_next` + `next`?

Decisao recomendada:

- Nao implementar direto sem fechar o contrato de `Iterator`.
- Primeiro escrever spec pequena de `Iterable` e `Iterator`.

Status: concluida em 2026-05-16.

Implementado:

- Contrato atual definido como `core.Iterable` marcador mais metodo `mut func next() -> optional<T>`.
- O item do `for` e inferido pelo `T` retornado por `next`.
- Tipo sem `Iterable` agora emite `type.not_iterable`.
- `Iterable` sem `next` emite `type.iterable_next_missing`.
- `next` com assinatura invalida emite `type.iterable_next_signature`.
- HIR e backend nativo baixam `for` custom para chamadas a `next`.
- Backend C segue sem suporte completo para Iterable custom neste ciclo.
- Spec, prompt e catalogo de diagnostics foram alinhados ao contrato real.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Tarefas:

- Especificar trait `Iterable<Item>`.
- Especificar trait `Iterator<Item>`.
- Atualizar type checker para reconhecer `for x in value` via trait.
- Atualizar HIR para representar loop via iterator custom.
- Atualizar codegen native/C.
- Adicionar testes com tipo custom.

Arquivos provaveis:

- `docs/spec/08-traits.md`
- `docs/spec/06-statements.md`
- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-hir/src/lower.rs`
- `compiler/crates/ori-codegen/src/native_backend.rs`
- `compiler/crates/ori-codegen/src/c_backend.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`

Pronto quando:

- `for` sobre tipo custom compila e roda.
- Tipos sem `Iterable` recebem erro claro.

## Fase 6 - Alinhar CLI com `ori run`

Problema:

- Prompt menciona `ori run`.
- CLI atual nao tem `run`.

Decisao recomendada:

- Implementar `ori run <file>` como atalho para compilar em temporario e executar.

Motivo:

- Melhora fluxo de teste manual.
- Alinha prompt e expectativa do usuario.
- Evita explicar sempre `compile --out` + executar.

Status: concluida em 2026-05-16.

Implementado:

- Subcomando `ori run <file>` adicionado ao CLI.
- `ori run` reutiliza o pipeline nativo de `compile`.
- O executavel temporario fica em `temp`, e removido apos a execucao.
- O exit code do programa e propagado.
- `--native-raw` tambem funciona no `run`.
- README e help do CLI foram atualizados.
- Validacao: teste de help, `ori run examples\hello_world.orl`, `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Tarefas:

- Adicionar subcomando `Run`.
- Reutilizar pipeline de `compile`.
- Gerar executavel temporario em pasta segura.
- Executar o binario e propagar exit code.
- Adicionar help e teste de CLI.
- Documentar em README/docs.

Arquivos provaveis:

- `compiler/crates/ori-driver/src/main.rs`
- `compiler/crates/ori-driver/src/pipeline.rs`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`
- `README.md`
- `docs/planning/ori-test-prompt.md`

Pronto quando:

- `cargo run -q -p ori-driver -- run examples\hello_world.orl` funciona.
- Help lista `run`.
- Erro de compilacao impede execucao.

## Fase 7 - Diagnostics Dedicados

Problema:

- Alguns erros existem, mas caem em mensagens genericas.
- Exemplos:
  - HKT vira parse error generico.
  - associated type vira parse error generico.
  - const generic vira parse error generico.
- `success()` onde payload e exigido vira `type.type_mismatch`.
- byte string com `\u{...}` vira `parse.invalid_escape`, mas prompt espera erro lexical.

Status: concluida em 2026-05-16.

Implementado:

- `success()` sem payload em `result<T, E>` com `T != void` agora emite `contract.success_void_mismatch`.
- HKT em parametro de tipo agora emite `generic.unsupported_hkt`.
- Associated type em trait agora emite `generic.unsupported_associated_type`.
- Const generic agora emite `generic.unsupported_const_generic`.
- Byte string com `\u{...}` agora emite `parse.byte_unicode_escape`.
- Catalogo de diagnostics foi atualizado.
- Testes negativos adicionados em `multifile_imports`; `diagnostic_catalog` passou.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Decisao recomendada:

- Criar diagnostics dedicados apenas quando ajudam o usuario.
- Nao criar codigo novo se a mensagem generica ja for melhor.

Tarefas:

- Revisar `docs/spec/13-error-catalog.md`.
- Separar codigos `emitted` de `planned`.
- Para cada codigo planejado, decidir:
  - implementar agora;
  - manter planejado;
  - remover.
- Adicionar testes negativos focados.
- Garantir que mensagens digam:
  - o que deu errado;
  - onde;
  - como corrigir.

Arquivos provaveis:

- `docs/spec/13-error-catalog.md`
- `compiler/crates/ori-diagnostics`
- `compiler/crates/ori-parser`
- `compiler/crates/ori-types/src/check.rs`
- `compiler/crates/ori-driver/tests/diagnostic_catalog.rs`

Pronto quando:

- Catalogo nao promete erro que o compilador nao emite, exceto em secao `planned`.
- Testes cobrem os diagnostics emitidos.

## Fase 8 - Melhorar LSP Semantico

Problema:

- LSP atual cobre hover basico, definicao local e completion simples.
- Prompt sugere hover semantico mais rico.

Status: concluida em 2026-05-16.

Implementado:

- Hover continua cobrindo tipos builtin.
- Hover agora indexa simbolos locais por arquivo.
- Funcoes mostram assinatura.
- Structs mostram campos indexados.
- Campos de struct mostram campo, tipo e struct dona.
- `const` e `var` locais mostram tipo declarado.
- Parametros mostram tipo declarado.
- Contratos `if it ...` mostram resumo de `it`.
- Definicao local tambem encontra campos e bindings.
- Testes LSP adicionados para funcao, campo, binding, parametro e contrato.
- Validacao: `cargo test -p ori-lsp`, `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Comportamento esperado:

- Hover em simbolos definidos pelo usuario mostra tipo.
- Hover em campo de struct mostra campo e tipo.
- Hover em funcao mostra assinatura.
- Hover em contrato mostra resumo simples.

Tarefas:

- Reutilizar parse/type info no LSP.
- Criar indice leve de simbolos por arquivo.
- Adicionar hover para:
  - tipos;
  - funcoes;
  - variaveis locais;
  - campos de struct;
  - metodos.
- Adicionar testes LSP.

Arquivos provaveis:

- `compiler/crates/ori-lsp/src/main.rs`
- `compiler/crates/ori-lsp/Cargo.toml`
- `compiler/crates/ori-lsp/tests`
- `compiler/crates/ori-driver/src/pipeline.rs`

Pronto quando:

- Hover de tipo builtin continua passando.
- Hover de simbolo do usuario passa.
- Falha de parse nao derruba o LSP.

## Fase 9 - Resolver Conflito de Slice

Problema:

- Prompt sugere `text[0..3]` com quatro caracteres.
- Spec/testes atuais indicam slice half-open: pega indices `0,1,2`.

Status: concluida em 2026-05-16.

Implementado:

- Prompt corrigido para slice half-open.
- Spec ja documentava fim exclusivo; mantido.
- Teste existente `compile_runs_index_slicing_native` cobre lista e string.
- Validacao: `cargo test --workspace`, `ori check examples\hello_world.orl`, `ori compile` + execucao passaram.

Decisao recomendada:

- Manter slice half-open.

Motivo:

- E padrao em muitas linguagens modernas.
- Evita off-by-one em comprimentos.
- Ja esta coerente com testes atuais.

Tarefas:

- Corrigir `docs/planning/ori-test-prompt.md`.
- Garantir que spec diga claramente:
  - `start..end` inclui `start`;
  - exclui `end`.
- Adicionar teste com string e lista.

Arquivos provaveis:

- `docs/planning/ori-test-prompt.md`
- `docs/spec/05-expressions.md`
- `compiler/crates/ori-driver/tests/multifile_imports.rs`

Pronto quando:

- Prompt, spec e testes concordam.

## Ordem Recomendada de Execucao

1. Fase 1: Anonymous struct.
2. Fase 2: Struct update.
3. Fase 3: Range float.
4. Fase 9: Slice.
5. Fase 7: Diagnostics dedicados.
6. Fase 4: Trait method ambiguity.
7. Fase 6: `ori run`.
8. Fase 5: Iterable custom.
9. Fase 8: LSP semantico.

## Marco de Fechamento

O trabalho pode ser considerado fechado quando:

- `cargo test --workspace` passa.
- O prompt de testes nao contem expectativa falsa.
- O spec nao contradiz o compilador.
- Cada lacuna tem:
  - teste positivo quando aplicavel;
  - teste negativo quando aplicavel;
  - diagnostic claro quando falha;
  - documentacao atualizada.

## Resultado Esperado

Depois deste plano, a linguagem deve ter:

- menos codigo invalido passando;
- menos divergencia entre prompt, spec e implementacao;
- erros mais claros;
- fluxo de execucao mais direto com `ori run`;
- base melhor para recursos grandes como `Iterable` custom e LSP semantico.
