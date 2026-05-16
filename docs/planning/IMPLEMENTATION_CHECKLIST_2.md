# Ori native route 100% implementation checklist

Status: backlog de implementacao.

Data: 2026-05-14.

Fonte operacional:

- `docs/IMPLEMENTATION_CHECKLIST.md`
- `docs/walkthrough-correcoes.md`
- `docs/planning/native-runtime-route-correction-plan.md`
- `docs/planning/async-implementation-plan.md`
- `docs/planning/native-async-state-machine-design.md`
- `docs/spec/12-stdlib.md`
- `compiler/crates/ori-types/src/stdlib.rs`
- `compiler/crates/ori-runtime/src/lib.rs`

Este arquivo nao substitui o checklist antigo. Ele registra o que ainda precisa
existir para a rota nativa ser considerada completa, consistente e pronta para
ser a unica fonte de verdade da linguagem.

## Definicao de "100% nativo"

A rota nativa so deve ser considerada 100% quando todos estes pontos forem
verdadeiros ao mesmo tempo:

- [x] `ori compile` gera executavel sem depender de runtime C embutido.
- [x] `ori test` usa a mesma rota nativa de `ori compile`.
- [x] O runtime Rust `ori-runtime` e a unica fonte canonica de semantica de runtime.
- [x] O backend C, se continuar existindo, e apenas uma rota opcional de debug/transpile.
- [x] Toda feature aceita pelo checker tem codegen nativo correto ou e rejeitada antes do backend com diagnostico claro.
- [x] Toda funcao da stdlib marcada como nativa no manifesto tem simbolo, ABI, lowering, testes e documentacao.
- [x] O pacote de release contem binario, runtime, metadados de linkagem e smoke tests fora do workspace Cargo.
- [x] Async/await possui state machine real, suspensao nao bloqueante e ARC correto entre pontos de `await`.
- [x] O lowering nativo de `await` nao chama `ori_task_block_on*`; `block_on` fica restrito a pontes sincronas explicitas.
- [x] A documentacao registra a state machine async nativa atual e separa `task.block_on` como ponte sincrona explicita.
- [x] Ownership de strings/bytes produzidos pelo runtime nativo esta auditado, documentado e coberto por testes.
- [x] Collections criadas internamente pelo runtime registram ownership de elementos managed ou documentam transferencia segura.
- [x] Collections novas estao documentadas, tipadas, testadas e cobertas pelo runtime nativo.
- [x] CI valida a rota nativa em uma matriz minima de plataformas.

## 0. Estado base confirmado

Estes pontos ja aparecem como implementados no checklist atual. Eles sao a base,
nao o fim do trabalho.

- [x] Backend nativo + Rust `ori-runtime` definidos como rota principal.
- [x] Runtime Rust tratado como fonte canonica para `ori compile` e `ori test`.
- [x] Backend C documentado como debug/transpile com paridade parcial.
- [x] Manifesto da stdlib centralizado em `compiler/crates/ori-types/src/stdlib.rs`.
- [x] `ori.list`, `ori.map`, `ori.set`, `ori.iter`, `ori.task`, `ori.channel`, `ori.atomic` e `future<T>` existem na superficie atual.
- [x] O C backend rejeita async/concurrency quando nao consegue preservar a semantica.

## 1. Rota nativa e isolamento do backend C

- [x] Auditar novamente todas as chamadas a `cc`, `clang`, `gcc`, `link.exe`, `lld` e `rust-lld`.
- [x] Separar claramente "linker nativo" de "compilador C" nas mensagens do CLI.
- [x] Garantir que nenhuma funcao de `ori compile` chame `ensure_cc_available()` ou equivalente.
- [x] Garantir que nenhuma funcao de `ori test` dependa da rota C.
- [x] Adicionar teste de regressao que falha se o caminho nativo compilar runtime C temporario.
- [x] Adicionar teste de regressao que falha se erro da rota nativa mencionar "C compiler" como requisito publico.
- [x] Definir se o C backend fica fora do pacote principal ou entra como componente opcional. Decisao atual: fica no CLI como rota explicita de debug/transpile.
- [x] Se o C backend ficar, mover o contrato publico para algo explicito como `ori build --backend c`. Decisao atual: `ori build` e o comando explicito do C debug backend.
- [x] Documentar que a falta de suporte no C backend nao bloqueia features nativas.
- [x] Adicionar uma pagina curta explicando "native route" versus "C debug backend".

## 2. Linkagem nativa e empacotamento

- [x] Validar `NativeLinker` em Windows MSVC.
- [x] Validar `NativeLinker` em Windows GNU via `.github/workflows/native-route.yml`.
- [x] Validar `NativeLinker` em Linux GNU via `.github/workflows/native-route.yml`.
- [x] Planejar e validar macOS x86_64 via `.github/workflows/native-route.yml`.
- [x] Planejar e validar macOS aarch64 via `.github/workflows/native-route.yml`.
- [x] Gerar `runtime-link.json` por target com bibliotecas de sistema exigidas pelo `staticlib`.
- [x] Validar que `runtime-link.json` e lido fora do workspace Cargo.
- [x] Versionar metadata do runtime: versao Ori, target triple, perfil, ABI version.
- [x] Criar smoke test de release que copia `ori`, `runtime/` e exemplos para uma pasta temporaria limpa.
- [x] Rodar `ori compile` nessa pasta temporaria sem usar paths do workspace.
- [x] Rodar `ori test` nessa pasta temporaria sem usar paths do workspace.
- [x] Criar erro claro para runtime ausente: caminho esperado, target atual e comando de staging.
- [x] Criar erro claro para runtime de target errado.
- [x] Criar erro claro para linker ausente.
- [x] Criar erro claro para simbolo nativo ausente.
- [x] Garantir que `tools/stage_native_runtime.ps1` cubra o target host atual.
- [x] Adicionar script equivalente ou modo cross-platform para Linux/macOS.

## 3. ABI nativa como contrato unico

- [x] Criar `ORI_ABI_VERSION` no runtime e no driver.
- [x] Validar ABI version no momento da linkagem ou antes dela.
- [x] Gerar ou validar declaracoes do backend nativo a partir do manifesto da stdlib.
- [x] Impedir declaracoes duplicadas de simbolos runtime fora do manifesto, salvo helpers internos documentados.
- [x] Criar teste que compara manifesto, declaracoes do backend nativo e exports reais do `ori-runtime`.
- [x] Criar teste de layout para strings.
- [x] Criar teste de layout para bytes.
- [x] Criar teste de layout para listas.
- [x] Criar teste de layout para map/set.
- [x] Criar teste de layout para option/result.
- [x] Criar teste de layout para tuples runtime-managed.
- [x] Criar teste de layout para closures e ambientes capturados.
- [x] Criar teste de layout para futures.
- [x] Criar teste de layout para task/job/channel/atomic.
- [x] Criar teste de layout para as novas collections.
- [x] Documentar quais tipos sao handles runtime e quais sao valores diretos.
- [x] Documentar regras de ownership de retorno para cada simbolo runtime.
- [x] Documentar quando o caller deve reter, liberar ou transferir ownership.
- [x] Atualizar `docs/native-abi.md` com helpers internos async da state machine e helpers legados `ori_async_spawn_*`/`ori_task_last_await_status` que nao definem mais `await` nativo.
- [x] Decidir se `ori_async_spawn_*` continua como API interna final do executor ou se vira apenas detalhe legado removido do lowering de `await`.
- [x] Auditar helpers que retornam string/bytes para eliminar buffers C crus sem dono claro ou documentar/liberar ownership explicitamente.
- [x] Criar teste ABI/ownership para strings e bytes produzidos por helpers nativos de stdlib.

## 4. ARC, ownership e memoria

- [x] Auditar todos os tipos managed usados pelo backend nativo.
- [x] Garantir retain/release em retorno de funcao managed.
- [x] Garantir retain/release em atribuicao de campo managed.
- [x] Garantir retain/release em atribuicao de indice managed.
- [x] Garantir retain/release em closures com captures managed.
- [x] Garantir retain/release em tuples contendo managed values.
- [x] Garantir retain/release em enums com payload managed.
- [x] Garantir retain/release em option/result com payload managed.
- [x] Garantir retain/release em list/map/set com elementos managed.
- [x] Garantir retain/release em novas collections com elementos managed.
- [x] Testar ciclos simples em structs.
- [x] Testar ciclos via list/map/set.
- [x] Testar ciclos via closures.
- [x] Testar ciclos via doubly linked list.
- [x] Testar ciclos via graph.
- [x] Definir regra para destructors/finalizers de handles runtime.
- [x] Definir comportamento de panic durante cleanup.
- [x] Criar modo de teste para detectar leaks obvios no runtime nativo.
- [x] Criar testes de stress para retain/release em loops.
- [x] Criar testes de stress para retain/release em concorrencia.
- [x] Auditar helpers runtime que constroem collections com valores managed sem passar pelo codegen de insercao.
- [x] Garantir registro de edge ARC, retain/release ou transferencia explicita para collections criadas por `split`, `chars`, iteradores, snapshots, `keys`, `values` e `entries`.
- [x] Adicionar testes de leak/ownership para collections retornadas por helpers runtime, nao apenas por operacoes chamadas pelo usuario.

## 5. Backend nativo: cobertura total de HIR

- [x] Criar matriz de expressao HIR -> cobertura nativa -> teste.
- [x] Criar matriz de statement HIR -> cobertura nativa -> teste.
- [x] Garantir que todo HIR valido gerado pelo checker tenha caminho nativo.
- [x] Remover fallback silencioso restante no backend nativo, se existir.
- [x] Converter todo "unsupported" interno em diagnostico antes do Cranelift quando a entrada vier do usuario.
- [x] Testar codegen de generics monomorfizados com tipos managed.
- [x] Testar codegen de trait calls com generics.
- [x] Testar codegen de `any<Trait>` com valores managed.
- [x] Testar operator overloading com valores managed.
- [x] Testar match com enum payload managed.
- [x] Testar pattern matching profundo com tuples, enums e structs.
- [x] Testar `using` com retorno normal.
- [x] Testar `using` com `?`.
- [x] Testar `using` com `panic`.
- [x] Testar `using` em loops com `break` e `continue`.
- [x] Testar top-level globals managed.
- [x] Testar imports transitiveis com generics e traits.
- [x] Testar exemplos grandes com `ori compile` e execucao real.

## 6. Async/await nativo completo

Pendencia central atual: a implementacao existente usa executor minimo. Para
100%, `await` precisa suspender sem bloquear o executor inteiro.

Status tecnico atual: runtime/executor/futures existem. O backend nativo usa
state machine para o subset v1 suportado e rejeita shapes async fora desse
subset com `backend.native_unsupported`. O lowering cobre parametros, locals
pre-`await`, bindings de `await`, payloads managed simples mantidos por edges
ARC no frame, dois ou mais estados, continuacao via `ori_future_on_ready`,
leitura por `ori_future_value_*`, propagacao de `failed/cancelled`, `?`, e
tail `if`/`while`/`for`/`match` sem `await` ou `return` interno.

### 6.1 Modelo interno

- [x] Desenhar representacao HIR/MIR para state machine async.
- [x] Definir estados gerados para cada `async func`.
- [x] Definir armazenamento de parametros entre suspensoes.
- [x] Definir armazenamento de locals vivos entre suspensoes.
- [x] Definir armazenamento de temporarios vivos entre suspensoes.
- [x] Definir regra para retorno `T` virar `future<T>`.
- [x] Definir regra para `result<T,E>` e `?` dentro de async.
- [x] Definir diagnosticos para casos async invalidos.
- [x] Atualizar spec de expressoes e funcoes com o modelo real.
- [x] Gerar corpo interno nativo para `async func` separado da funcao publica que retorna `future<T>`.
- [x] Implementar primeiro subset de state machine nativa para `async func` sem parametros no formato `await chamada(); return valor`.
- [x] Expandir o subset de state machine para `const x: T = await chamada(); return expr` com binding escalar.
- [x] Expandir o subset de state machine para sequencias de dois ou mais awaits escalares em estados distintos.
- [x] Expandir o subset de state machine para `async main`/`future<void>` com expressao final sem `return`.
- [x] Expandir o subset de state machine para parametros escalares copiados no frame async.
- [x] Expandir o subset de state machine para parametros/bindings managed simples com edge ARC no frame.
- [x] Expandir o subset de state machine para `return await chamada()`.
- [x] Expandir o subset de state machine para `const x = (await chamada())?` quando o result aguardado tem o mesmo tipo do result retornado.
- [x] Expandir o subset de state machine para locals simples declarados antes do primeiro `await` e vivos depois da suspensao.
- [x] Expandir o subset de state machine para statements finais sem `await`/`return` interno, incluindo `if`, `while`, `for` e `match`.
- [x] Materializar frame async nativo por funcao no subset sequencial atual, com `state`, future de resultado, parametros, locals vivos e temporarios vivos de `await`.
- [x] Gerar funcao interna `step` da state machine com despacho por estado no subset sequencial sem parametros.
- [x] Fazer a funcao publica `async func` alocar/inicializar frame, agendar o primeiro `step` e retornar `future<T>` imediatamente no subset sequencial sem parametros.
- [x] Definir diagnostico temporario para constructs async ainda nao suportados pela state machine, em vez de cair em panic/backend unsupported.

### 6.2 Runtime e executor

- [x] Implementar future pollable no `ori-runtime`.
- [x] Expor helper interno `ori_future_pending` para wrappers async nativos criarem o future de resultado sem depender de spawn/thread.
- [x] Implementar wake/schedule no executor nativo.
- [x] Fazer `ori_async_spawn_*` agendar o corpo async no executor nativo em vez de criar thread dedicada por chamada. Mantido apenas como helper legado/runtime, fora do lowering nativo de `await`.
- [x] Fazer chamada de `async func` retornar `future<T>` antes do primeiro `await`.
- [x] Rebaixar `await` para `ori_future_poll` + `ori_future_on_ready` no subset inicial de state machine simples.
- [x] Fazer `await` registrar continuidade em vez de bloquear thread comum no subset sequencial da state machine.
- [x] Rebaixar `await` para `ori_future_poll` + leitura tipada via `ori_future_value_*` quando o future estiver pronto no subset sequencial escalar.
- [x] Registrar continuacao com `ori_future_on_ready` quando o future estiver pendente no subset sequencial da state machine.
- [x] Salvar o proximo estado no frame antes de suspender no subset sequencial da state machine.
- [x] Propagar future failed/cancelled pela state machine sem produzir valor default silencioso no subset sequencial da state machine.
- [x] Remover chamadas a `ori_task_block_on*` do lowering de `await` em `compiler/crates/ori-codegen/src/native_backend.rs`.
- [x] Manter `task.block_on` apenas como ponte sincrona explicita.
- [x] Implementar fila de tarefas do executor.
- [x] Implementar timer nao bloqueante para `task.sleep`.
- [x] Definir se o executor e single-threaded, thread-pool ou configuravel.
- [x] Expor politica minima de executor na documentacao.
- [x] Implementar tratamento de future failed.
- [x] Implementar tratamento de future cancelled ou manter cancelamento privado e documentado.
- [x] Propagar `failed/cancelled` de `await` pela state machine nativa atual.
- [x] Se cancelamento publico entrar, adicionar `task.CancelToken`. Decisao atual: cancelamento publico fica fora do v1.
- [x] Se cancelamento publico entrar, adicionar `task.cancel`. Decisao atual: cancelamento publico fica fora do v1.
- [x] Se cancelamento publico entrar, adicionar `task.cancelled`. Decisao atual: cancelamento publico fica fora do v1.

### 6.3 ARC e cleanup em async

- [x] Preservar ARC de valores vivos antes e depois de cada `await` na state machine nativa.
- [x] Liberar corretamente valores que deixam de estar vivos apos um `await` na state machine nativa.
- [x] Garantir cleanup em retorno normal de async na state machine nativa.
- [x] Garantir cleanup em erro propagado por `?`.
- [x] Garantir cleanup em future failed.
- [x] Garantir cleanup em future cancelled, se cancelamento publico existir.
- [x] Implementar ou rejeitar `using` dentro de `async func` com regra definitiva.
- [x] Testar recurso aberto antes de `await` e liberado depois. N/A no v1: `using` em async e rejeitado.
- [x] Testar recurso aberto antes de `await` e liberado em erro. N/A no v1: `using` em async e rejeitado.
- [x] Testar recurso aberto antes de `await` e liberado em cancelamento. N/A no v1: `using` em async e rejeitado.
- [x] Fazer `future<T>` reter payloads managed armazenados em resultados por ponte para a state machine.
- [x] Registrar edges ARC para parametros e bindings managed simples copiados para o frame no subset sequencial da state machine.
- [x] Calcular liveness de locals e temporarios managed atravessando cada ponto de `await` na state machine.
- [x] Reter valores managed copiados para o frame antes da suspensao no subset sequencial da state machine.
- [x] Liberar valores managed quando deixam de estar vivos apos retomada.
- [x] Garantir cleanup do frame em retorno normal, `?`, future failed e future cancelled no subset sequencial da state machine.
- [x] Testar ARC da state machine com branches, loops, `match`, closures capturadas e payloads managed. Coberto por testes de collections/struct/enum/closure atravessando `await` e por `compile_runs_async_state_machine_tail_control_flow_native`.

### 6.4 Testes async

- [x] Testar `await` de future pendente sem bloquear o executor inteiro.
- [x] Testar que chamada de `async func` retorna antes do primeiro `await`.
- [x] Testar runtime async nativo propagando future failed/cancelled.
- [x] Testar dois futures pendentes alternando progresso.
- [x] Testar `async main` com dois awaits.
- [x] Testar async com `result<T,E>` e `?`.
- [x] Testar async com string/list vivos atravessando `await`.
- [x] Testar async com map/set vivos atravessando `await`.
- [x] Testar async com struct managed vivo atravessando `await`.
- [x] Testar async com enum payload managed atravessando `await`.
- [x] Testar async com closure capture managed atravessando `await`.
- [x] Testar async test runner com future pendente real.
- [x] Testar erro de `await` fora de async.
- [x] Testar erro de `await` em valor nao future.
- [x] Testar diagnostico de recurso async nao suportado no C backend.
- [x] Adicionar teste de regressao que falha se `emit_await` chamar `ori_task_block_on*`.
- [x] Testar `await` pendente com continuacao registrada sem ocupar worker/thread ate o future ficar pronto. Coberto por `pending_future_continuation_does_not_block_executor_queue` e pelos testes de state machine com `task.sleep`.
- [x] Testar state machine async com dois `await` em estados diferentes.
- [x] Testar state machine async com `if`, `while`, `for`, `match` e `?`. Coberto por `compile_runs_managed_collections_across_await_native`, `compile_runs_async_state_machine_tail_control_flow_native` e testes `compile_runs_async_result_question_mark*`.
- [x] Testar cleanup da state machine em retorno antecipado, future failed e future cancelled. Coberto por `simple_async_state_machine_cleans_frame_on_terminal_paths` e regressao `compile_runs_async_result_question_mark_error_state_machine_native`.
- [x] Testar que `task.block_on` continua funcionando apenas como API sincrona explicita.

## 7. Stdlib nativa: lacunas planejadas

- [x] Definir se helpers planejados de optional/result continuam fora do v1 ou entram agora.
- [x] Manter `optional.or` fora do v1 e documentar uso de `?`, `if some` ou `match`.
- [x] Manter `optional.or_return` fora do v1 e documentar uso de `?`, `if some` ou `match`.
- [x] Manter `result.or_wrap` fora do v1 e documentar uso de `?` ou `match`.
- [x] Implementar conversao ampla via `ori.core.Displayable` ou documentar limite como pos-v1.
- [x] Implementar `ori.core.Error` com metodos reais ou documentar limite como pos-v1.
- [x] Documentar cause chaining em `ori.Error` como pos-v1.
- [x] Documentar que APIs atuais de erro continuam com `string` ate o contrato rico de `ori.Error`.
- [x] Documentar `mem.size_of<T>()` e `mem.align_of<T>()` como bloqueados ate sintaxe de type args; usar value witness no v1.
- [x] Implementar `ori.fs.read_bytes`.
- [x] Implementar `ori.fs.write_bytes`.
- [x] Declarar `ori.fs.open_read` fora de escopo ate existir `ori.fs.File`.
- [x] Declarar `ori.fs.open_write` fora de escopo ate existir `ori.fs.File`.
- [x] Implementar `ori.fs.read_all`.
- [x] Implementar tipo `ori.fs.File` ou declarar explicitamente fora de escopo.
- [x] Implementar JSON estruturado com object/array reais ou manter `json.Value = string` como decisao documentada.
- [x] Implementar pretty print em `ori.json`.
- [x] Melhorar `ori.map` com `clear`.
- [x] Melhorar `ori.map` com `reserve`.
- [x] Melhorar `ori.map` com `capacity`.
- [x] Melhorar `ori.set` com `clear`.
- [x] Melhorar `ori.set` com `reserve`.
- [x] Melhorar `ori.set` com `capacity`.
- [x] Definir se `map.get` deve continuar retornando valor direto ou migrar para `optional<V>` em uma versao futura.

## 8. Nova stdlib de collections

Regra geral: cada collection nova precisa entrar no manifesto, no checker, no
runtime nativo, no backend nativo, na documentacao e nos testes. O C backend
pode rejeitar a collection com `backend.c_unsupported` enquanto nao houver
paridade segura.

Arvore apareceu duas vezes no pedido original. Este checklist consolida isso
em um modulo `ori.tree`.

Progresso atual: `ori.deque`, `ori.queue`, `ori.stack`, `ori.linked_list`,
`ori.doubly_linked_list`, `ori.tree`, `ori.hash_table`, `ori.graph` e `ori.heap` estao implementados
como tipos opacos da stdlib. As colecoes lineares reutilizam o handle nativo de
`list<T>`; `tree` usa arena nativa propria com `tree.NodeId`; `hash_table`
reutiliza o motor de `map<K,V>` e expõe `get/remove -> optional<V>`; `graph`
usa adjacency list nativa com suporte dirigido/nao dirigido; `heap` usa
min-heap nativo com `Comparable`. Isso
entrega a API publica inicial, `optional<T>` para operacoes vazias e snapshots
por copia, sem permitir que esses tipos sejam confundidos com `list<T>` no
checker. As listas encadeadas nao expoem nodes publicos no v1, evitando ciclos
ARC internos.

### 8.1 Fundacao comum de collections

- [x] Definir namespace final: `ori.queue`, `ori.stack`, `ori.deque`, `ori.linked_list`, `ori.doubly_linked_list`, `ori.tree`, `ori.hash_table`, `ori.graph`, `ori.heap`.
- [x] Definir se cada collection vira tipo built-in, tipo opaco da stdlib ou wrapper sobre handles runtime.
- [x] Definir representacao generica no sistema de tipos para tipos opacos da stdlib.
- [x] Adicionar suporte de type display para novos tipos genericos.
- [x] Adicionar suporte de import/resolution para os novos modulos implementados.
- [x] Adicionar assinaturas em `stdlib_func_sig` para os novos modulos implementados.
- [x] Adicionar simbolos em `STDLIB_RUNTIME_FUNCTIONS` para os novos modulos implementados.
- [x] Adicionar ABI em `stdlib_native_abi` para os novos modulos implementados.
- [x] Adicionar declaracoes no backend nativo para os novos modulos implementados.
- [x] Adicionar lowering HIR para chamadas runtime para os novos modulos implementados.
- [x] Definir politica de ownership para valores inseridos e removidos nos modulos implementados.
- [x] Definir regra `Transferable` para cada collection implementada.
- [x] Definir regra `Equatable` para comparacao de collections, se existir.
- [x] Definir regra `Hashable` para collections, se existir.
- [x] Definir regra de iteracao para `for`.
- [x] Definir conversao `to_list` para collections onde fizer sentido nos modulos implementados.
- [x] Adicionar diagnostico para tipos sem trait exigido.
- [x] Adicionar diagnostico para operacao invalida em collection vazia quando nao retornar `optional`.
- [x] Atualizar `docs/spec/12-stdlib.md` para os novos modulos implementados.
- [x] Adicionar exemplos em `examples/collections_demo.orl` ou arquivo novo.

### 8.2 `ori.deque` - fila dupla

Implementacao atual: wrapper sobre `list<T>`; buffer circular fica como
otimizacao posterior se a API precisar de `push_front` O(1).

- [x] Definir tipo `deque.Deque<T>`.
- [x] Implementar `deque.new<T>() -> deque.Deque<T>`.
- [x] Implementar `deque.push_front<T>(d, value) -> void`.
- [x] Implementar `deque.push_back<T>(d, value) -> void`.
- [x] Implementar `deque.pop_front<T>(d) -> optional<T>`.
- [x] Implementar `deque.pop_back<T>(d) -> optional<T>`.
- [x] Implementar `deque.front<T>(d) -> optional<T>`.
- [x] Implementar `deque.back<T>(d) -> optional<T>`.
- [x] Implementar `deque.len<T>(d) -> int`.
- [x] Implementar `deque.is_empty<T>(d) -> bool`.
- [x] Implementar `deque.clear<T>(d) -> void`.
- [x] Implementar `deque.to_list<T>(d) -> list<T>`.
- [x] Testar crescimento de capacidade.
- [x] Testar wrap-around do buffer circular nao aplicavel: `deque` atual e wrapper sobre `list<T>`.
- [x] Testar valores managed.
- [x] Testar `Transferable` quando `T is Transferable`.

### 8.3 `ori.queue` - fila FIFO

Implementacao recomendada: wrapper sobre `ori.deque`.

- [x] Definir tipo `queue.Queue<T>`.
- [x] Implementar `queue.new<T>() -> queue.Queue<T>`.
- [x] Implementar `queue.enqueue<T>(q, value) -> void`.
- [x] Implementar `queue.dequeue<T>(q) -> optional<T>`.
- [x] Implementar `queue.peek<T>(q) -> optional<T>`.
- [x] Implementar `queue.len<T>(q) -> int`.
- [x] Implementar `queue.is_empty<T>(q) -> bool`.
- [x] Implementar `queue.clear<T>(q) -> void`.
- [x] Implementar `queue.to_list<T>(q) -> list<T>`.
- [x] Testar ordem FIFO.
- [x] Testar dequeue em fila vazia.
- [x] Testar valores managed.
- [x] Testar uso com `channel` ou `task` quando `T is Transferable`.

### 8.4 `ori.stack` - pilha LIFO

Implementacao recomendada: wrapper sobre `list<T>` ou `ori.deque`.

- [x] Definir tipo `stack.Stack<T>`.
- [x] Implementar `stack.new<T>() -> stack.Stack<T>`.
- [x] Implementar `stack.push<T>(s, value) -> void`.
- [x] Implementar `stack.pop<T>(s) -> optional<T>`.
- [x] Implementar `stack.peek<T>(s) -> optional<T>`.
- [x] Implementar `stack.len<T>(s) -> int`.
- [x] Implementar `stack.is_empty<T>(s) -> bool`.
- [x] Implementar `stack.clear<T>(s) -> void`.
- [x] Implementar `stack.to_list<T>(s) -> list<T>`.
- [x] Testar ordem LIFO.
- [x] Testar pop em pilha vazia.
- [x] Testar valores managed.

### 8.5 `ori.linked_list` - lista ligada simples

Implementar apenas se houver necessidade real. Ela e menos eficiente que
`list<T>` para a maioria dos casos, mas pode ser util para APIs com insercao
local estavel.

- [x] Definir tipo `linked_list.LinkedList<T>`.
- [x] Evitar expor ponteiros de node sem uma regra segura de ownership.
- [x] Preferir handles opacos ou indices estaveis se nodes publicos forem necessarios.
- [x] Implementar `linked_list.new<T>() -> linked_list.LinkedList<T>`.
- [x] Implementar `linked_list.push_front<T>(list, value) -> void`.
- [x] Implementar `linked_list.push_back<T>(list, value) -> void`.
- [x] Implementar `linked_list.pop_front<T>(list) -> optional<T>`.
- [x] Implementar `linked_list.front<T>(list) -> optional<T>`.
- [x] Implementar `linked_list.len<T>(list) -> int`.
- [x] Implementar `linked_list.is_empty<T>(list) -> bool`.
- [x] Implementar `linked_list.clear<T>(list) -> void`.
- [x] Implementar `linked_list.to_list<T>(list) -> list<T>`.
- [x] Testar ownership de nodes removidos.
- [x] Testar valores managed.
- [x] Testar que nao ha ciclo ARC por acidente.

### 8.6 `ori.doubly_linked_list` - lista duplamente ligada

Maior risco: ciclos internos entre `prev` e `next`. A implementacao atual evita
esse risco no v1 usando handle opaco list-backed e nao expondo nodes publicos.

- [x] Definir tipo `doubly_linked_list.DoublyLinkedList<T>`.
- [x] Definir se nodes publicos existem ou se a API e apenas por posicao/handle opaco.
- [x] Implementar `doubly_linked_list.new<T>() -> doubly_linked_list.DoublyLinkedList<T>`.
- [x] Implementar `push_front`.
- [x] Implementar `push_back`.
- [x] Implementar `pop_front`.
- [x] Implementar `pop_back`.
- [x] Implementar `front`.
- [x] Implementar `back`.
- [x] Implementar `len`.
- [x] Implementar `is_empty`.
- [x] Implementar `clear`.
- [x] Implementar `to_list`.
- [x] Testar insercao e remocao nas extremidades.
- [x] Testar limpeza de lista com muitos nodes.
- [x] Testar deteccao ou ausencia de ciclo ARC.
- [x] Testar valores managed.

### 8.7 `ori.tree` - arvores

Recomendacao: comecar por arvore generica baseada em arena/ids, nao por
ponteiros publicos. Depois adicionar arvore ordenada se `Comparable` estiver
estavel para esse uso.

- [x] Definir tipo `tree.Tree<T>`.
- [x] Definir tipo `tree.NodeId`.
- [x] Implementar `tree.new<T>(root: T) -> tree.Tree<T>`.
- [x] Implementar `tree.root<T>(tree) -> tree.NodeId`.
- [x] Implementar `tree.value<T>(tree, node) -> T`.
- [x] Implementar `tree.add_child<T>(tree, parent, value) -> tree.NodeId`.
- [x] Implementar `tree.children<T>(tree, node) -> list<tree.NodeId>`.
- [x] Implementar `tree.parent<T>(tree, node) -> optional<tree.NodeId>`.
- [x] Implementar `tree.remove_subtree<T>(tree, node) -> void`.
- [x] Implementar `tree.len<T>(tree) -> int`.
- [x] Implementar `tree.depth<T>(tree, node) -> int`.
- [x] Implementar traversal pre-order.
- [x] Implementar traversal post-order.
- [x] Implementar traversal breadth-first.
- [x] Definir se existira `tree.OrderedTree<T> where T is Comparable` - decisao v1: nao entra agora.
- [x] Se existir, implementar insert/search/remove ordenado - N/A no v1 pela decisao acima.
- [x] Testar remocao de subarvore com valores managed.
- [x] Testar node id invalido com diagnostico/runtime error claro.

### 8.8 `ori.hash_table` - tabela hash explicita

`map<K,V>` e `set<T>` ja cobrem o caso comum. Este modulo so deve existir se
for uma API avancada para controle de capacidade, load factor ou hasher.

- [x] Decidir se `ori.hash_table` sera modulo publico ou apenas detalhe interno de `ori.map`.
- [x] Se for publico, definir tipo `hash_table.HashTable<K, V>`.
- [x] Reusar a implementacao hash de `ori.map` em vez de duplicar motor.
- [x] Exigir `K is Hashable and K is Equatable`.
- [x] Implementar `hash_table.new<K, V>()`.
- [x] Implementar `hash_table.with_capacity<K, V>(capacity: int)`.
- [x] Implementar `hash_table.set`.
- [x] Implementar `hash_table.get -> optional<V>`.
- [x] Implementar `hash_table.remove -> optional<V>`.
- [x] Implementar `hash_table.contains`.
- [x] Implementar `hash_table.len`.
- [x] Implementar `hash_table.capacity`.
- [x] Implementar `hash_table.reserve`.
- [x] Implementar `hash_table.clear`.
- [x] Implementar `hash_table.keys`.
- [x] Implementar `hash_table.values`.
- [x] Implementar `hash_table.entries`.
- [x] Testar colisoes.
- [x] Testar resize.
- [x] Testar chaves string e int.
- [x] Testar chaves user-defined `Hashable`/`Equatable`.
- [x] Testar valores managed.

### 8.9 `ori.graph` - grafos

Recomendacao: implementar como biblioteca sobre map/list/set, com representacao
por adjacency list. Evitar ponteiros de node publicos.

- [x] Definir tipo `graph.Graph<N> where N is Hashable and N is Equatable`.
- [x] Definir se o grafo e direcionado, nao direcionado, ou ambos por flag.
- [x] Definir tipo de edge simples.
- [x] Definir se peso entra agora ou depois.
- [x] Implementar `graph.new<N>(directed: bool) -> graph.Graph<N>`.
- [x] Implementar `graph.add_node<N>(g, node) -> void`.
- [x] Implementar `graph.remove_node<N>(g, node) -> void`.
- [x] Implementar `graph.add_edge<N>(g, from, to) -> void`.
- [x] Implementar `graph.remove_edge<N>(g, from, to) -> void`.
- [x] Implementar `graph.has_node<N>(g, node) -> bool`.
- [x] Implementar `graph.has_edge<N>(g, from, to) -> bool`.
- [x] Implementar `graph.neighbors<N>(g, node) -> list<N>`.
- [x] Implementar `graph.nodes<N>(g) -> list<N>`.
- [x] Implementar `graph.edges<N>(g) -> list<tuple<N, N>>`.
- [x] Implementar BFS.
- [x] Implementar DFS.
- [x] Implementar topological sort para grafo direcionado aciclico.
- [x] Planejar shortest path com peso para fase posterior.
- [x] Testar grafo direcionado.
- [x] Testar grafo nao direcionado.
- [x] Testar ciclos.
- [x] Testar node ausente.
- [x] Testar valores string/int/user-defined.
- [x] Testar valores managed e limpeza.

### 8.10 `ori.heap` / `ori.priority_queue` - recomendacao adicional

Este modulo nao estava na lista inicial do pedido, mas e mais util que uma
arvore crua para muitos algoritmos.

- [x] Decidir se entra como `ori.heap` ou `ori.priority_queue` - decisao v1: `ori.heap`.
- [x] Definir tipo `heap.Heap<T> where T is Comparable`.
- [x] Implementar `heap.new<T>()`.
- [x] Implementar `heap.push<T>(heap, value)`.
- [x] Implementar `heap.pop<T>(heap) -> optional<T>`.
- [x] Implementar `heap.peek<T>(heap) -> optional<T>`.
- [x] Implementar `heap.len<T>(heap) -> int`.
- [x] Implementar `heap.is_empty<T>(heap) -> bool`.
- [x] Implementar comparador customizado em fase posterior, se closures comparadoras forem estaveis - decisao v1: `Comparable` nativo por tipo, sem closure comparator publico.
- [x] Testar min-heap ou max-heap conforme decisao documentada.
- [x] Testar user-defined `Comparable`.

## 9. Diagnosticos e UX do CLI

- [x] Revisar todos os erros do caminho nativo para linguagem simples e acao clara.
- [x] Garantir que diagnostico interno nao vaze como panic Rust para erro de usuario.
- [x] Adicionar codigo de erro para runtime ausente.
- [x] Adicionar codigo de erro para linker ausente.
- [x] Adicionar codigo de erro para ABI mismatch.
- [x] Adicionar codigo de erro para simbolo runtime ausente.
- [x] Adicionar codigo de erro para collection sem trait exigido.
- [x] Adicionar codigo de erro para collection module unavailable, se algum modulo for documentado antes de estar pronto.
- [x] Atualizar `docs/spec/13-error-catalog.md`.
- [x] Garantir que `ori --help`, `ori compile --help`, `ori test --help` e `ori build --help` deixem a rota nativa clara.
- [x] Adicionar flag de diagnostico como `--native-raw` ou equivalente para detalhes de linker quando necessario.

## 10. Tooling

- [x] Transformar `ori-lsp` de placeholder em servidor minimo real.
- [x] LSP: publicar diagnosticos de parser/checker.
- [x] LSP: resolver imports locais.
- [x] LSP: hover de tipos basicos.
- [x] LSP: go-to-definition para funcoes e tipos.
- [x] LSP: autocomplete para stdlib.
- [x] LSP: autocomplete para novas collections.
- [x] Formatter: garantir que exemplos novos de collections nao mudem semantica.
- [x] Formatter: cobrir async state machine syntax surface, se houver nova sintaxe publica.
- [x] `ori doc`: listar modulos da stdlib.
- [x] `ori doc`: listar assinaturas das novas collections.
- [x] `ori doc`: documentar constraints como `where T is Comparable`.

## 11. Testes de aceitacao para 100%

- [x] `cargo check -p ori-types -p ori-hir -p ori-codegen -p ori-driver -p ori-runtime`.
- [x] `cargo test -p ori-types`.
- [x] `cargo test -p ori-hir`.
- [x] `cargo test -p ori-codegen`.
- [x] `cargo test -p ori-runtime`.
- [x] `cargo test -p ori-driver`.
- [x] `cargo test -p ori-driver --test multifile_imports`.
- [x] `cargo test -p ori-driver --test concurrency_async`.
- [x] `cargo test -p ori-driver --test diagnostic_catalog`.
- [x] Teste de release fora do workspace Cargo.
- [x] Teste de `ori compile` para todos os exemplos oficiais.
- [x] Teste de execucao real dos exemplos oficiais.
- [x] Teste de `ori test` com testes sync.
- [x] Teste de `ori test` com testes async.
- [x] Teste de stress de async executor.
- [x] Teste de stress de channel.
- [x] Teste de stress de atomic.
- [x] Teste de stress de map/set/hash_table.
- [x] Teste de stress de graph com ciclos.
- [x] Teste de stress de doubly linked list com muitos nodes.
- [x] Teste de memoria para valores managed em todas as collections.
- [x] Teste de regressao que impede fallback para C no caminho nativo.
- [x] Teste de regressao que impede novo simbolo stdlib sem ABI nativa.
- [x] Rodar a suite completa apos a state machine async remover o fallback sincronizado de `await`.
- [x] Rodar `tools/check_native_runtime_exports.ps1` apos atualizar helpers internos async.
- [x] Rodar smoke de release fora do workspace com exemplo async de suspensao real.
- [x] Adicionar gate que impede novo `await` nativo baseado em `block_on`.

## 12. Documentacao obrigatoria

- [x] Atualizar `README.md` com contrato nativo final.
- [x] Atualizar `runtime/README.md` com layout de release final.
- [x] Atualizar `docs/spec/10-memory.md` com regras finais de ARC/runtime.
- [x] Atualizar `docs/spec/12-stdlib.md` com collections novas.
- [x] Atualizar `docs/spec/05-expressions.md` com `await` real.
- [x] Atualizar `docs/spec/07-functions.md` com `async func` real.
- [x] Atualizar `docs/spec/13-error-catalog.md` com novos diagnosticos.
- [x] Criar exemplo oficial de `ori.queue`.
- [x] Criar exemplo oficial de `ori.stack`.
- [x] Criar exemplo oficial de `ori.deque`.
- [x] Criar exemplo oficial de `ori.tree`.
- [x] Criar exemplo oficial de `ori.graph`.
- [x] Criar exemplo oficial de `ori.heap`.
- [x] Criar exemplo oficial de async com suspensao real.
- [x] Criar exemplo oficial de release/compile fora do workspace.
- [x] Revisar `docs/spec/05-expressions.md` para remover texto obsoleto sobre failed/cancelled virarem valor default.
- [x] Revisar `docs/spec/07-functions.md` para remover texto obsoleto sobre `async func` retornar ready future.
- [x] Revisar `docs/spec/10-memory.md` para registrar ARC do frame da state machine nativa.
- [x] Revisar `docs/spec/12-stdlib.md` para diferenciar executor minimo, state machine nativa e `task.block_on` explicito.
- [x] Atualizar `docs/native-hir-coverage.md` para registrar que `Await` so fica completo quando usar poll/continuation.
- [x] Atualizar `docs/walkthrough-correcoes.md` com a lista residual real: state machine, ABI async docs, strings/bytes ownership e collections runtime-created.
- [x] Marcar `docs/analysis_results.md` como historico ou reconciliar entradas ja superadas pela rota nativa atual.

## 13. Definition of done por item

Um item deste checklist so pode ser marcado como concluido quando:

- O comportamento esta implementado no codigo.
- O comportamento esta coberto por teste automatizado.
- O comportamento esta documentado na spec ou no README apropriado.
- A rota nativa passa nos testes focados.
- O C backend foi atualizado ou rejeita o recurso com diagnostico claro.
- Nenhuma nova API usa exemplos com `let`; exemplos devem usar a sintaxe real do Ori.
- A mensagem de erro e curta, acionavel e legivel.
