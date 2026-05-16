# Plano: assincronicidade em Ori

Status: proposta de desenho e implementacao.

Data: 2026-05-14.

## Objetivo

Adicionar assincronicidade ao Ori sem quebrar a filosofia da linguagem:

- leitura primeiro;
- tipos explicitos;
- comportamento previsivel;
- erros visiveis com `result<T, E>`;
- sintaxe pequena;
- backend nativo como alvo principal.

## Decisao principal

`async/await` deve existir em Ori, mas nao deve ser a primeira peca isolada.

Ordem recomendada:

1. Corrigir a rota nativa de runtime/linkagem.
2. Implementar primitivas nativas de tarefa e futuro.
3. Implementar `ori.task` e `ori.channel`.
4. Depois adicionar `async func` e `await`.

## Modelo mental

Separar tres conceitos:

| Conceito | O que significa | Exemplo |
| --- | --- | --- |
| `future<T>` | valor que sera produzido depois | leitura de arquivo async |
| `task.Job<T>` | trabalho executando em paralelo | calculo em outra thread |
| `channel.Channel<T>` | comunicacao entre tarefas/jobs | produtor e consumidor |

`async` nao deve significar "criar thread".

`spawn` deve significar "executar trabalho".

`await` deve significar "esperar um `future<T>` sem bloquear o executor inteiro".

## Sintaxe proposta

Adicionar `async` e `await` como palavras contextuais.

Isso reduz quebra de codigo existente, porque elas so ganham significado em
posicoes especificas:

- `async func` em declaracao de funcao;
- `await expr` em expressao.

### Funcao async

```ori
import ori.fs as fs

async func read_config(path: string) -> result<string, ori.Error>
    const text: string = (await fs.read_text_async(path))?
    return success(text)
end
```

Regra:

```text
async func f(...) -> T
```

produz uma chamada com tipo:

```text
future<T>
```

O tipo depois de `->` continua sendo o valor que a funcao entrega quando
aguardada.

### Uso em `main`

Permitir `async func main()`:

```ori
import ori.fs as fs
import ori.io as io

async func main() -> result<void, ori.Error>
    const text: string = (await fs.read_text_async("config.json"))?
    io.print(text)
    return success()
end
```

O driver deve transformar `async main` em chamada ao executor nativo.

Tambem deve existir uma API explicita para codigo sincrono:

```ori
func main() -> result<void, ori.Error>
    const done: result<void, ori.Error> = ori.task.block_on(run())
    return done
end

async func run() -> result<void, ori.Error>
    return success()
end
```

### `await` e `?`

Para a primeira versao, usar forma sem ambiguidade:

```ori
const value: int = (await compute())?
```

Evitar aceitar esta forma antes da precedencia estar muito bem testada:

```ori
const value: int = await compute()?
```

Depois, o formatter pode aceitar e normalizar a forma curta, se a spec decidir
que ela e clara o bastante.

## Biblioteca padrao proposta

### `ori.task`

```ori
import ori.task as task

task.spawn<T>(work: func() -> T) -> task.Job<T>
task.join<T>(job: task.Job<T>) -> result<T, task.JoinError>
task.detach<T>(job: task.Job<T>) -> void
task.block_on<T>(future: future<T>) -> T
task.sleep(ms: int) -> future<void>
task.cancel(token: task.CancelToken) -> void
task.cancelled(token: task.CancelToken) -> bool
```

Notas:

- `spawn` e thread-backed no runtime nativo inicial.
- `join` retorna `result` porque a tarefa pode falhar no nivel de runtime.
- `block_on` e ponte entre codigo sincrono e async.
- `sleep` async nao deve bloquear a thread principal do executor.

### `ori.channel`

```ori
import ori.channel as channel

channel.create<T>() -> channel.Channel<T>
channel.send<T>(ch: channel.Channel<T>, value: T) -> result<void, channel.SendError>
channel.receive<T>(ch: channel.Channel<T>) -> result<T, channel.ReceiveError>
channel.close<T>(ch: channel.Channel<T>) -> void
```

Regra:

```text
T precisa ser Transferable quando atravessa thread.
```

### `ori.concurrent`

```ori
import ori.concurrent as concurrent
```

Tipos e traits:

```ori
trait Transferable
end
```

Tipos inicialmente transferiveis:

- `bool`
- `int` e inteiros explicitos
- `float` e floats explicitos
- `string`
- `bytes`
- `list<T>` quando `T is Transferable`
- `map<K, V>` quando `K is Hashable`, `K is Equatable`, `K is Transferable`,
  e `V is Transferable`
- `set<T>` quando `T is Hashable`, `T is Equatable`, e `T is Transferable`
- structs cujos campos sao todos `Transferable`

### `ori.atomic`

Primeira versao pequena:

```ori
import ori.atomic as atomic

atomic.AtomicInt
atomic.new(value: int) -> atomic.AtomicInt
atomic.load(value: atomic.AtomicInt) -> int
atomic.store(value: atomic.AtomicInt, next: int) -> void
atomic.add(value: atomic.AtomicInt, delta: int) -> int
```

Evitar `Atomic<T>` generico ate existir uma razao real.

## Regras semanticas

### Capturas

Manter a regra atual das closures:

- `const` pode ser capturado por copia;
- `var` nao pode ser capturado.

Para `spawn`, toda captura precisa ser `Transferable`.

Para `async func`, capturas so aparecem quando `async do` existir. Isso deve
ficar para fase posterior.

### Compartilhamento de memoria

Nao compartilhar valores mutaveis comuns entre threads.

O caminho seguro inicial e:

- copiar valores transferiveis;
- comunicar por canal;
- usar atomicos para contadores simples;
- adiar `Mutex<T>` e `Shared<T>` ate a base estar estavel.

### Erros

Erros de dominio continuam usando:

```ori
result<T, E>
```

Falhas de execucao de jobs usam:

```ori
result<T, task.JoinError>
```

`await` nao deve esconder erro de dominio. Se a funcao async retorna
`result<T, E>`, o usuario ainda precisa usar `?`:

```ori
const text: string = (await read_config("config.json"))?
```

### `using`

`using` dentro de `async func` precisa de regra explicita:

- recurso aberto antes de um `await` continua vivo depois do `await`;
- cleanup roda quando a funcao async termina;
- se o futuro for cancelado, cleanup tambem deve rodar.

Se isso nao puder ser garantido no primeiro slice, o checker deve rejeitar
`using` em `async func` com diagnostico claro.

## Fases de implementacao

### Fase 0: spec e diagnosticos reservados

- Documentar `future<T>`, `async func` e `await`.
- Adicionar codigos de erro:
  - `async.await_outside_async`
  - `async.await_non_future`
  - `async.main_invalid_return`
  - `async.capture_not_transferable`
  - `async.using_not_supported`
  - `backend.async_unsupported`
- Adicionar exemplos oficiais usando `const`, `var`, `func`, `result` e `end`.

### Fase 1: primitivas nativas sem sintaxe nova

- Implementar `ori.task.spawn`.
- Implementar `ori.task.join`.
- Implementar `ori.channel`.
- Implementar `ori.atomic.AtomicInt`.
- Implementar trait/conceito `Transferable` no checker.
- C backend pode rejeitar essas APIs se nao houver suporte seguro.

Essa fase entrega concorrencia real antes de `async/await`.

### Fase 2: `future<T>` e executor nativo

- Adicionar `future<T>` ao sistema de tipos.
- Implementar runtime de future no `ori-runtime`.
- Implementar `task.block_on`.
- Implementar `task.sleep`.
- Adicionar testes nativos de future completo, future pendente e sleep.

### Fase 3: parser, AST e checker para async

- Adicionar `async func` ao parser.
- Adicionar `await` como expressao.
- Rejeitar `await` fora de funcao async.
- Rejeitar `await` em valor que nao seja `future<T>`.
- Fazer chamada de `async func` ter tipo `future<T>`.
- Permitir `async func main()`.

### Fase 4: lowering/HIR

- Representar `async func` no HIR.
- Representar `await` no HIR.
- Comecar com `async func` sem ponto de suspensao como future ja completo.
- Em seguida, implementar suspensao real em pontos de `await`.

### Fase 5: native backend

- Gerar estado de maquina para `async func`.
- Gerar chamadas ao runtime de future/executor.
- Preservar ARC corretamente em valores vivos atraves de `await`.
- Garantir cleanup de escopo em retorno, erro com `?` e cancelamento.

### Fase 6: stdlib async

- `ori.time.sleep_async` ou `ori.task.sleep`.
- `ori.fs.read_text_async`.
- `ori.fs.write_text_async`.
- Futuramente: rede, processo e streams.

### Fase 7: testes e tooling

- `ori check` para todos os erros async.
- `ori compile` para exemplos async pequenos.
- `ori test` com suporte a `@test async func`.
- formatter preservando `async func` e `await`.
- LSP futuro com diagnosticos de async.

## Criterios de aceite

- Exemplos usam sintaxe real do Ori, sem `let`.
- `async func` chama como `future<T>`.
- `await` so funciona em contexto async.
- `?` continua explicito para `result<T, E>`.
- `task.spawn` nao aceita capturas nao transferiveis.
- C backend nao bloqueia o desenvolvimento nativo; ele pode rejeitar async com
  `backend.async_unsupported`.
- O runtime nativo executa pelo menos:
  - `spawn` + `join`;
  - `channel.send` + `channel.receive`;
  - `block_on`;
  - `async main`;
  - `await` de future completo;
  - `await` de sleep.

## Exemplo alvo completo

```ori
namespace app.main

import ori.io as io
import ori.task as task

func cpu_work() -> int
    return 21 * 2
end

async func delayed_message() -> result<string, ori.Error>
    await task.sleep(10)
    return success("done")
end

async func main() -> result<void, ori.Error>
    const job: task.Job<int> = task.spawn(cpu_work)
    const value: int = task.join(job)?

    const message: string = (await delayed_message())?

    io.print(string(value))
    io.print(message)
    return success()
end
```

## Ordem recomendada

1. Fechar a rota nativa de runtime/linkagem.
2. Implementar `ori.task`, `ori.channel`, `ori.atomic` e `Transferable`.
3. Implementar `future<T>` e executor.
4. Implementar `async func`.
5. Implementar `await`.
6. Adicionar stdlib async.
