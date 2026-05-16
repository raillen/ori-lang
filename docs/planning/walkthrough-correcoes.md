# Walkthrough das Correcoes da Implementacao Ori

> Status: atualizado em 2026-05-14.
> Fonte operacional: `docs/IMPLEMENTATION_CHECKLIST.md`.
> Escopo: orientar a leitura das correcoes ja feitas e deixar claro o que ainda
> nao deve ser tratado como concluido.

Este arquivo substitui o relatorio antigo que estava com mojibake e misturava
achados atuais com problemas que ja foram corrigidos. Use este walkthrough como
guia de contexto. Para decidir o proximo trabalho, use sempre o checklist.

---

## Resumo curto

A implementacao da linguagem ja recebeu varias rodadas de correcao em parser,
checker, HIR, backend nativo, runtime, stdlib, diagnosticos e testes.

Hoje, os grandes blocos abaixo estao fechados no checklist:

- rota do runtime nativo como fonte de verdade;
- linkagem e empacotamento do runtime nativo;
- contratos de CLI e documentacao da rota nativa;
- stdlib principal;
- `ori.task`, `ori.channel`, `ori.atomic`;
- `future<T>` e executor minimo;
- `async func`, `await`, `async main`;
- `ori.fs.read_text_async` e `ori.fs.write_text_async`;
- `@test async func`;
- suporte de formatacao para `async func` e `await`;
- diagnosticos de async/concurrency;
- rejeicao clara de async/concurrency pelo C backend quando nao ha paridade.

O C backend permanece como rota de depuracao/transpilacao parcial. A rota
principal de compilacao e execucao e o backend nativo.

---

## Correcoes que nao devem voltar para a lista de bugs abertos

Os pontos abaixo ja foram tratados em rodadas anteriores ou ficaram obsoletos
por mudanca de arquitetura. Nao reabra estes itens sem uma nova reproducao.

### Parser e sintaxe

- Atribuicao em campo, como `obj.value = 2`, e parseada como lvalue real.
- Lvalue invalido emite diagnostico, em vez de descartar a instrucao durante
  recuperacao de parser.
- `mut func` foi alinhado com a semantica atual de metodos.
- Parametros variadicos validam posicao e forma documentada.
- Parametros com default antes de parametros obrigatorios sao rejeitados.
- Campos duplicados em `struct` e variantes duplicadas em `enum` sao rejeitados.
- Diagnosticos em expressoes dentro de f-string apontam para o span correto.

### Resolucao de nomes e semantica

- Nomes desconhecidos falham no checker, em vez de chegarem ao backend.
- Chamadas desconhecidas falham no checker.
- Caminhos qualificados desconhecidos emitem diagnostico de nome/caminho.
- `panic`, `todo` e `unreachable` interagem com analise de retorno via `never`.
- `and`, `or` e `not` validam operandos booleanos.
- Closures rejeitam captura de `var` conforme a regra atual.
- Descarte de `result<T, E>` em statement emite diagnostico.
- `==` e `!=` em `any<Trait>` sao rejeitados.

### Literais e lexer

- `--|` dentro de string, byte string e texto de f-string nao abre comentario.
- Comentario de bloco real sem fechamento mantem diagnostico dedicado.
- Literais invalidos nao devem virar crash de usuario; quando um caso chega ao
  HIR, a mensagem e tratada como ICE interno.

### Runtime e backend

- Indice fora do limite em lista, string e bytes panica em runtime.
- Slice invalido segue a regra documentada de bounds.
- `repeat` com contagem negativa panica ou e rejeitado conforme cobertura atual.
- O backend nativo valida HIR antes de chegar ao verificador do Cranelift.
- `for` em `bytes` tem caminho no backend nativo.
- O runtime Rust e a rota C coberta compartilham comportamento de bounds para
  os recursos suportados.

### Stdlib e contratos centrais

- `ori.core` expoe traits centrais do contrato atual.
- `using` resolve `Disposable` pelo contrato escolhido, nao apenas por nome local.
- Assinaturas genericas da stdlib preservam relacoes entre parametros e retorno.
- Mismatches como `ori.list.contains(list<int>, "x")` falham no checker.
- `ori.mem` esta consistente entre spec, checklist e implementacao.
- O exemplo rapido do README compila com a superficie atual.

---

## Async e concorrencia: estado atual

O bloco de async/concurrency ja tem uma primeira versao funcional:

- `async func f(...) -> T` tem tipo de chamada `future<T>`.
- `await expr` exige `future<T>` e produz `T`.
- `await` fora de `async func` e erro.
- `await` em valor que nao e `future<T>` e erro.
- `async func main()` e aceito no backend nativo.
- `task.sleep(ms)` retorna `future<void>`.
- `ori.fs.read_text_async` retorna `future<result<string, string>>`.
- `ori.fs.write_text_async` retorna `future<result<string, string>>`.
- `ori test` aceita `@test async func` sem parametros e com retorno `void`.
- O formatter preserva blocos `async func` e linhas com `await`.

Limite importante: a implementacao atual usa o executor minimo e pode bloquear a
thread nativa no `await`. Ela ainda nao gera uma state machine real com pontos
de suspensao nao bloqueantes.

Mais precisamente, a chamada de `async func` ja retorna um `future<T>` antes do
corpo terminar. O corpo roda por uma ponte nativa executor-backed e propaga
`failed/cancelled`, mas o `await` dentro desse corpo ainda depende de
`task.block_on`.

---

## Pendencias reais ainda abertas

Neste momento, os itens abertos do bloco de async/concurrency sao:

- gerar frame async nativo com `state`, parametros, locals vivos e temporarios;
- substituir `await` baseado em `task.block_on` por `poll` + continuacao;
- preservar ARC retain/release corretamente para valores vivos atraves de
  pontos de suspensao reais;
- atualizar o contrato ABI dos helpers async internos;
- auditar ownership de strings/bytes produzidos pelo runtime;
- auditar collections criadas internamente pelo runtime com valores managed.

Esses dois itens devem ser implementados juntos ou em uma sequencia muito
controlada. Marcar qualquer um deles como concluido sem state machine real
criaria uma falsa sensacao de seguranca.

---

## Como continuar

1. Consulte `docs/IMPLEMENTATION_CHECKLIST.md`.
2. Escolha o primeiro item ainda aberto.
3. Antes de editar, confirme o comportamento atual com teste pequeno.
4. Implemente codigo, docs e regressao juntos.
5. Rode validacao focada.
6. Marque o checklist somente depois da validacao.

Para a proxima etapa grande, a ordem recomendada e:

1. desenhar a representacao HIR/MIR da state machine;
2. definir quais valores precisam sobreviver entre `await`s;
3. aplicar regras de ARC para esses valores;
4. gerar a state machine no backend nativo;
5. adicionar testes com valores gerenciados vivos atraves de `await`;
6. revisar a documentacao de `async func`, `await`, ABI e runtime.

---

## Validacao recomendada

Use estes comandos como base para conferir as areas tocadas por async e runtime:

```powershell
cargo check -p ori-types -p ori-hir -p ori-codegen -p ori-driver -p ori-runtime
cargo test -p ori-driver --test concurrency_async -- --nocapture
cargo test -p ori-runtime -- --nocapture
cargo test -p ori-types -- --nocapture
cargo test -p ori-driver --test diagnostic_catalog -- --nocapture
```

Quando `compiler/crates/ori-runtime` mudar, atualize o runtime empacotado:

```powershell
.\tools\stage_native_runtime.ps1
```

---

## Regra de manutencao

Este arquivo nao deve virar outro backlog paralelo. Ele serve para explicar o
historico das correcoes e evitar retrabalho. O backlog executavel continua em
`docs/IMPLEMENTATION_CHECKLIST.md`.
