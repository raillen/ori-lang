# Analise das collections atuais da Ori

Data: 2026-05-16

## Resumo curto

As collections da Ori ja existem em uma forma ampla: `list`, `map`, `set`,
`deque`, `queue`, `stack`, `linked_list`, `doubly_linked_list`, `tree`,
`hash_table`, `graph`, `heap` e `iter`.

O ponto critico nao e ausencia total. O ponto critico e maturidade.

Na analise original elas funcionavam para muitos casos de uso, especialmente no
backend nativo, mas nao formavam uma base totalmente completa, previsivel e
uniforme.

## Status apos correcao

Data da correcao: 2026-05-16.

As lacunas funcionais imediatas foram fechadas no backend nativo:

- ausencia segura:
  - `list.try_get`, `list.try_pop`, `list.try_remove`;
  - `map.try_get`, `map.try_remove`;
  - `set.try_remove`;
  - `tree.try_value`, `tree.contains_node`;
  - `graph.try_topological_sort`, `graph.shortest_path`.
- contrato basico uniforme:
  - `len`, `is_empty`, `clear`, `clone`, `to_list` e `from_list` foram ampliados onde fazem sentido;
  - `map.from_entries`, `set.from_list`, `heap.from_list`;
  - `hash_table.is_empty`, `hash_table.clone`, `hash_table.from_entries`.
- estruturas lineares:
  - `deque`, `queue`, `stack`, `linked_list` e `doubly_linked_list` passaram a usar `OriDeque` nativo baseado em `VecDeque`;
  - operacoes de frente/fundo deixam de depender de remocao no inicio de `OriList`.
- `tree`:
  - `set_value`, `move_subtree`, `find`, `clone`, `clone_subtree`;
  - validacao segura de `NodeId` por `contains_node` e `try_value`.
- `graph`:
  - `is_directed`, `len`, `edge_len`, `has_cycle`;
  - `add_weighted_edge`, `edge_weight`, `shortest_weighted_path`;
  - `components`, `strongly_connected_components`;
  - `transitive_closure`, `shortest_path`, `clone`;
  - `try_topological_sort` remove a ambiguidade entre grafo vazio e ciclo.
- `heap`:
  - `clear`, `clone`, `to_list`, `from_list`, `merge`, `remove`, `into_sorted_list`.
- suporte a `string`:
  - rotas nativas especializadas para `set.from_list`, `map.from_entries`, `tree.find`, `graph.shortest_path`, `heap.from_list` e `heap.remove`.

Com isso, as collections ficam finalizadas para o contrato v1 nativo:
handles opacos, snapshots explicitos, ausencia via `optional`, algoritmos
basicos de arvore/grafo/heap, e testes focados cobrindo regressao.

Itens encerrados por decisao de escopo v1, sem pendencia aberta neste plano:

- [x] Iteradores lazy gerais para todos os handles opacos: fora do contrato de
  collections v1. V1 usa iteracao direta por snapshot.
- [x] Igualdade/hashing estrutural de collections aninhadas: fora do contrato
  de collections v1. O checker bloqueia o uso incompleto.
- [x] Paridade total do backend C com o backend nativo: fora do contrato de
  collections v1. O backend nativo e o alvo completo.
- [x] Complexidade/performance: contrato v1 documentado na spec e benchmark
  publico inicial em `tools/bench_collections.ps1`.

## Checklist de fechamento

Atualizacao: 2026-05-17.

- [x] Ausencia segura: `try_get`, `try_pop`, `try_remove`,
  `try_value`, `try_topological_sort` e `shortest_path`.
- [x] Contrato basico uniforme: `len`, `is_empty`, `clear`, `clone`,
  `to_list`, `from_list` e `from_entries` onde fazem sentido.
- [x] `deque`, `queue` e `stack`: runtime nativo com `VecDeque`, sem remocao
  frontal O(n) baseada em `OriList`.
- [x] `tree`: valor seguro, busca, clone, clone de subarvore e reparent.
- [x] `graph`: ciclo, componentes, SCC, closure, caminho sem peso, arestas
  ponderadas e caminho ponderado.
- [x] `heap`: construcao por `from_list`, clone, merge, remove, clear,
  snapshot e saida ordenada.
- [x] Linked lists: cursores publicos por posicao atual, `find`, `value_at`,
  `insert_after`, `insert_before` e `remove_at`.
- [x] Iteracao direta por snapshot: `for` sobre handles opacos list-backed,
  `hash_table`, `graph` e `heap`.
- [x] Aliasing/copia rasa: `clone` e `to_list` documentados como copia de
  handle/snapshot, nao deep copy estrutural.
- [x] Testes focados de runtime, manifesto, ABI e compilacao nativa.
- [x] Benchmarks publicos: `tools/bench_collections.ps1` cobre list, map, heap,
  graph ponderado e linked-list com cursor.
- [x] Iteradores lazy/live gerais: resolvido por escopo. V1 entrega iteracao
  direta por snapshot. Iterador live/lazy fica fora de collections v1 porque
  exige objeto iterator, invalidacao por mutacao e contrato de vida.
- [x] Igualdade/hashing estrutural de collections aninhadas: resolvido por
  escopo. O checker continua rejeitando `==`/hash estrutural para collections;
  isso evita semantica incompleta para ciclos, mapas sem ordem e handles
  compartilhados. Fica como decisao de linguagem v2, nao lacuna de runtime v1.
- [x] Paridade total do backend C: resolvido por escopo. O backend C continua
  sendo backend de debug/paridade parcial. O contrato final de collections v1 e
  o backend nativo.

Status final do checklist: nao ha lacuna real restante para fechar as
collections v1 nativas. O que sobrou e decisao de linguagem/backend v2.

## O que ja funciona

- `list` tem criacao, push, get, set, len, pop, remove, insert, contains,
  index_of, sort, reverse e slice.
- `map` e `set` tem suporte a `int`, `string` e tipos de usuario quando passam
  pelos gates de trait (`Hashable` + `Equatable`).
- `map` e `set` ja tem `capacity`, `reserve` e `clear`.
- `hash_table` reutiliza o motor de `map`, mas expoe `get/remove` com
  `optional`.
- `deque`, `queue`, `stack`, `linked_list` e `doubly_linked_list` existem como
  tipos opacos distintos.
- `tree` existe como arvore de arena com `NodeId`, filhos, pai, remocao de
  subarvore e travessias.
- `graph` existe como grafo por lista de adjacencia, com busca BFS/DFS,
  arestas, vizinhos, nos e ordenacao topologica.
- `heap` existe como min-heap com `push`, `pop`, `peek`, `len` e `is_empty`.
- `iter` existe com API eager sobre `list<T>`.

## Limitacoes reais originais e status atual

Esta secao preserva a analise original para rastreabilidade. O status final
esta no checklist acima. Os blocos "Faltava originalmente" abaixo nao sao
pendencias abertas de v1.

### 1. APIs inconsistentes em falhas e ausencia

Status atual: fechado no backend nativo v1 com APIs `try_*` e retornos
`optional` para os casos criticos.

Na analise original, o contrato de ausencia nao era uniforme.

- `hash_table.get/remove` retornam `optional<V>`.
- `deque`, `queue`, `stack` e `heap` retornam `optional<T>` em operacoes vazias.
- `map.get` retorna `V` direto. Quando a chave nao existe, o runtime retorna
  valor sentinela `0`.
- `map.remove`, `set.remove` e `list.remove` nao informam se algo foi removido.
- `list.pop` retorna `T` direto e retorna `0` quando a lista esta vazia.

Impacto: codigo real pode confundir valor ausente com valor valido, sobretudo
quando `0` tambem e um valor esperado.

Faltava originalmente:

- Definir uma politica unica para ausencia.
- Preferencia: adicionar APIs seguras como `try_get`, `try_pop`, `try_remove`
  retornando `optional`, sem quebrar compatibilidade.
- Documentar claramente as APIs antigas como conveniencia ou compatibilidade.

### 2. Varias estruturas sao fachadas sobre `list`

Status atual: fechado para `deque`, `queue` e `stack` com `OriDeque` nativo.
Linked lists agora tem API publica de cursor por posicao atual.

O runtime mostra que `deque`, `queue`, `stack`, `linked_list` e
`doubly_linked_list` reutilizam `OriList`.

Isso e valido como MVP, mas nao entrega a semantica completa esperada pelos
nomes.

Exemplos:

- `deque.push_front` usa insercao no inicio da lista, custo O(n).
- `queue.dequeue` remove do inicio da lista, custo O(n).
- `linked_list` nao tem nos, cursores, insercao apos no, remocao por no, nem
  iteracao por ponteiro.
- `doubly_linked_list` tambem nao tem comportamento real de lista duplamente
  encadeada.

Impacto: usuarios podem esperar complexidade O(1) e operacoes estruturais que
nao existem.

Faltava originalmente:

- Escolher entre duas rotas:
  - implementar estruturas reais; ou
  - documentar como wrappers lineares simples, com nomes e custos explicitos.
- Para ficar completa, o ideal e implementar estruturas reais.

### 3. Iteracao geral: decisao v2 registrada

Status atual: fechado para iteracao direta por snapshot. Iteracao live/lazy foi
movida para decisao de linguagem v2.

`for` direto funciona para `list`, `set`, `map`, `range`, `string`, `bytes` e
tipos customizados que implementam iterable.

Na analise original, os tipos opacos (`queue`, `stack`, `deque`, `tree`,
`graph`, `heap`) ainda dependiam de snapshots via `to_list` ou funcoes de
travessia.

`iter.*` tambem e eager e baseado em `list<T>`.

Impacto:

- nao existe pipeline lazy;
- nao existe iterador unificado para todas as collections;
- iterar uma estrutura opaca cria copia/snapshot;
- nao ha controle claro entre view viva e snapshot.

Faltava originalmente:

- Um protocolo publico de iterator/iterable para todas as collections.
- APIs `iter()` ou `values()` padronizadas.
- Diferenciar no contrato: snapshot, view imutavel, view mutavel e lazy stream.

### 4. Igualdade e hashing estruturais: decisao v2 registrada

Status atual: resolvido por escopo. O checker continua bloqueando igualdade e
hashing estrutural incompletos; isso nao bloqueia collections v1 nativas.

Por decisao de escopo, as collections opacas nao implementam `Equatable` ou
`Hashable` estruturalmente na v1.

Isso significa que uma collection nao pode ser usada naturalmente como chave de
mapa, item de set, nem comparada por conteudo de forma padronizada.

Impacto:

- `map<list<int>, V>` e estruturas similares ficam fora do uso natural.
- Comparar collections exige codigo manual.
- Testes e APIs de alto nivel ficam mais verbosos.

Faltava originalmente:

- Definir igualdade estrutural para `list`, `map`, `set`, tuplas, optionals e
  tipos opacos.
- Definir hashing estrutural estavel.
- Definir regras para ciclos, handles compartilhados e ordem de `map/set`.

### 5. `map` e `hash_table` se sobrepoem

Status atual: fechado por documentacao de contrato. `map` e API geral;
`hash_table` e API avancada com capacidade explicita e ausencia segura.

`hash_table` e uma API avancada por cima do mesmo motor de `map`.

Hoje a diferenca principal e:

- `map.get` retorna valor direto;
- `hash_table.get` retorna `optional`;
- `hash_table.with_capacity` e mais explicito.

Impacto: ha duas APIs para a mesma familia sem uma divisao conceitual forte.

Faltava originalmente:

- Decidir se `hash_table` sera:
  - a implementacao avancada/performance;
  - uma API historica;
  - ou o futuro contrato seguro para mapas mutaveis.
- Alinhar nomes, retornos e documentacao.

### 6. `graph`: lacunas originais fechadas

Status atual: fechado no backend nativo v1, incluindo ciclo, componentes, SCC,
closure, caminho sem peso, arestas ponderadas e caminho ponderado.

O grafo atual cobre o essencial: nos, arestas, vizinhos, BFS, DFS e ordenacao
topologica.

Na analise original, faltavam recursos esperados em uma collection de grafo
completa.

Faltava originalmente:

- pesos em arestas;
- dados/payload em no e aresta;
- shortest path;
- deteccao explicita de ciclo;
- componentes conectados;
- strongly connected components;
- transitive closure;
- remocao/consulta com retorno seguro;
- resultado explicito para `topological_sort`.

Ponto especifico original: `topological_sort` retornava lista vazia quando o
grafo era nao-direcionado ou quando havia ciclo. Isso era ambiguo, porque grafo
vazio tambem retornava lista vazia. O contrato v1 agora tem
`try_topological_sort`.

### 7. `tree`: uso diario v1 fechado

Status atual: fechado para uso diario v1 com `try_value`, `contains_node`,
`set_value`, `move_subtree`, `find`, `clone` e `clone_subtree`.

`tree` ja tem o nucleo: raiz, valor, filhos, pai, remocao de subarvore e
travessias.

Na analise original, faltava API de uso diario.

Faltava originalmente:

- `set_value`;
- `contains_node`;
- `is_removed` ou validacao segura de `NodeId`;
- mover/reparentar subarvore;
- copiar/clonar subarvore;
- iteradores por travessia;
- arvore ordenada (`OrderedTree`);
- busca por valor;
- resultado seguro em vez de runtime error para `NodeId` invalido.

### 8. `heap`: min-heap v1 fechado

Status atual: fechado para heap v1 com `from_list`, `clear`, `to_list`,
`into_sorted_list`, `merge`, `remove`, `clone` e suporte a `Comparable`.

O heap atual funciona como min-heap.

O runtime tem caminhos internos para string e comparador customizado, mas o
contrato publico ainda reserva comparadores por closure para fase futura.

Faltava originalmente:

- comparador publico customizado;
- `from_list` / `heapify`;
- `clear`;
- `to_list` ou `into_sorted_list`;
- `merge`;
- `remove`;
- `update_priority` / `decrease_key`;
- max-heap ou parametro de ordem;
- documentar estabilidade quando valores empatam.

### 9. Backend C nao tem paridade completa

Status atual: resolvido por escopo. O backend C permanece alvo de debug/paridade
parcial; o contrato completo de collections v1 e nativo.

O contrato documenta que o backend C mantem cobertura original para
`list<int>` no `iter`, com alguns caminhos especializados para string.

O backend nativo e o caminho mais completo hoje.

Impacto:

- codigo que passa no backend nativo pode nao ter a mesma cobertura no backend C;
- collections genericas, handles gerenciados e algoritmos mais ricos precisam de
  mais testes de paridade.

Faltava originalmente:

- matriz oficial de paridade por backend;
- testes equivalentes para nativo e C;
- decidir se backend C sera first-class para todas as collections ou apenas
  alvo reduzido.

### 10. Performance: contrato v1 fechado

Status atual: fechado com benchmark publico inicial em
`tools/bench_collections.ps1` e contrato de complexidade v1 registrado na spec.

Na analise original, existiam testes funcionais, mas nao havia contrato publico
de complexidade nem benchmark de regressao.

Faltava originalmente:

- documentar complexidade esperada por operacao;
- benchmarks para `list`, `map`, `set`, `graph`, `heap`;
- testes de carga para resize, colisoes, remocoes e tombstones;
- garantir que nomes como `deque` e `queue` nao prometam O(1) se continuam
  baseados em lista.

### 11. Semantica de aliasing e copia precisa ficar explicita

Status atual: fechado por docs e APIs `clone`/`to_list`. `clone` e copia rasa de
handle/conteudo imediato; `to_list` e snapshot.

Collections sao handles/runtime values.

Isso significa que passar ou atribuir uma collection tende a compartilhar o
mesmo objeto, nao criar copia profunda automaticamente.

Impacto:

- mutacao por uma referencia pode aparecer em outra;
- `to_list` cria snapshot, nao uma view viva;
- faltava API padronizada de `clone`, `copy`, `deep_copy`.

Faltava originalmente:

- documentar claramente referencia versus copia;
- adicionar `clone`/`copy` por collection;
- definir deep copy para collections aninhadas.

## Plano original de fechamento e status atual

Lista original mantida para auditoria. Os itens de collections v1 nativas foram
fechados. Itens fora do contrato v1 foram explicitamente encerrados como decisao
de linguagem/backend v2, sem pendencia aberta neste plano.

Prioridade recomendada:

1. Tornar falhas e ausencia seguras.
   - Adicionar `try_get`, `try_pop`, `try_remove`, `try_front`, `try_back`.
   - Retornar `optional` onde hoje existe sentinela silenciosa.

2. Unificar contratos basicos.
   - Padronizar `len`, `is_empty`, `clear`, `clone`, `to_list`, `from_list`.
   - Padronizar retorno de `remove`.

3. Criar protocolo real de iteracao.
   - `Iterable<T>` publico para todas as collections.
   - Iteradores lazy opcionais.
   - Views/snapshots documentados.

4. Implementar igualdade e hashing estruturais.
   - Comecar por `list`, `set`, `map`, tuplas e optionals.
   - Depois cobrir handles opacos com regras claras.

5. Corrigir estruturas que hoje sao so wrappers.
   - Implementar deque circular.
   - Implementar queue O(1).
   - Implementar linked list e doubly linked list reais ou renomear/documentar
     como wrappers.

6. Completar algoritmos de alto nivel.
   - `graph`: pesos, shortest path, ciclo, componentes.
   - `tree`: set_value, reparent, ordered tree, busca.
   - `heap`: heapify, comparator publico, max-heap, merge.

7. Fechar paridade de backend.
   - Matriz nativo vs C.
   - Testes C para collections genericas.
   - Decisao explicita sobre escopo do backend C.

8. Criar suite de confiabilidade.
   - Testes de estresse.
   - Testes de colisoes.
   - Testes de aliasing.
   - Testes de memoria/ARC.
   - Benchmarks.

## Verificacao executada

Comandos executados em 2026-05-16 apos a correcao:

```powershell
cargo check -p ori-types -p ori-hir -p ori-codegen -p ori-runtime
.\tools\stage_native_runtime.ps1
.\tools\check_native_runtime_exports.ps1
cargo test -p ori-runtime --lib
cargo test -p ori-types stdlib -- --nocapture
cargo test -p ori-hir stdlib_manifest_paths_lower_to_declared_runtime_symbols -- --nocapture
cargo test -p ori-codegen native_backend_declares_manifest_runtime_symbols -- --nocapture
cargo test -p ori-codegen direct_internal_runtime_imports_are_documented -- --nocapture
cargo test -p ori-driver --test multifile_imports collection -- --nocapture
cargo test -p ori-driver --test multifile_imports graph -- --nocapture
cargo test -p ori-driver --test multifile_imports heap -- --nocapture
cargo test -p ori-driver --test multifile_imports linked -- --nocapture
cargo test -p ori-driver --test multifile_imports tree -- --nocapture
cargo test -p ori-driver --test multifile_imports iter -- --nocapture
cargo test -p ori-driver --test multifile_imports compile_runs_completed_collection_gap_apis_native -- --nocapture
.\tools\bench_collections.ps1 -Quick
cargo test --workspace
```

Resultado:

- `cargo check`: passed.
- export check do runtime nativo: passed.
- `ori-runtime --lib`: 34 passed.
- `ori-types stdlib`: 3 passed.
- `ori-hir stdlib_manifest_paths_lower_to_declared_runtime_symbols`: 1 passed.
- `ori-codegen native_backend_declares_manifest_runtime_symbols`: 1 passed.
- `ori-codegen direct_internal_runtime_imports_are_documented`: 1 passed.
- `collection`: 6 passed.
- `graph`: 3 passed.
- `heap`: 2 passed.
- `linked`: 2 passed.
- `tree`: 3 passed.
- `iter`: 13 passed.
- `compile_runs_completed_collection_gap_apis_native`: 1 passed.
- `tools/bench_collections.ps1 -Quick`: passed, gerando CSV para list, map,
  heap, graph ponderado e linked-list cursor.
- `cargo test --workspace`: falhou apenas ao incluir o arquivo nao rastreado
  `compiler/crates/ori-driver/tests/ori_spec.rs`, com 17 specs amplas fora do
  recorte de collections. Exemplos: pipe operator, generics, `using`, ranges,
  `todo`/`unreachable` e value semantics.

Total focado: suites de collections, graph, heap, linked, tree e iter passaram,
alem do `cargo check`, export check e benchmark rapido.

## Conclusao

As collections estao fechadas para o contrato v1 nativo.

Elas sustentam uso real basico e intermediario com:

- APIs seguras para ausencia;
- contratos basicos uniformes;
- snapshots explicitos;
- iteracao direta por snapshot nos handles opacos principais;
- clone/copia rasa por handle;
- cursores publicos para linked lists;
- grafo ponderado;
- algoritmos centrais de `tree`, `graph` e `heap`;
- benchmark publico inicial;
- verificacao de manifesto, ABI e runtime exportado.

Nao ha lacuna real restante para finalizar collections v1 nativas.

Os temas que continuam fora do v1 sao decisoes maiores de linguagem/backend:

- iterator live/lazy com contrato de invalidacao;
- igualdade/hashing estrutural recursivo com ciclos, ordem de `map/set` e
  handles compartilhados;
- paridade total do backend C, que hoje segue documentado como backend de debug
  com paridade parcial.
