# Recursos Implementados e Resolvidos — Ori Language

Este documento registra o estado atual de implementação da linguagem Ori (conforme verificado pelas suítes de testes automatizados e especificações normativas).

---

## 1. Estrutura Léxica e Sintática (Lexer & Parser)

- **Comentários**: Suporte a comentários de linha (`--`) e blocos de comentários reais (`--| ... |--`), com tratamento robusto para detectar blocos não fechados (`lex.unclosed_block_comment`).
- **Literais e Strings**:
  - Inteiros e floats com e sem sufixos específicos.
  - Strings simples (`"..."`), byte strings (`b"..."`), f-strings interpoladas (`f"..."`).
  - Triple strings multi-linha (`"""..."""`) com desindentação automática baseada na margem esquerda (*baseline stripping*) em runtime, tanto para strings comuns quanto para f-strings interpoladas.
  - Validação de escapes unicode em bytes string (rejeição de `\u{...}` no parser).
- **Identificadores**: Permite identificadores unicode e palavras contextualizadas (como o uso de `times` como identificador local fora de contextos especiais).
- **Atributos**: Parsing e validação de atributos em declarações de alto nível (ex. `@deprecated`, `@test`).

---

## 2. Namespaces e Importações (Imports)

- **Carregamento de Múltiplos Arquivos**: Resolução de módulos locais recursivos e transitivos a partir do caminho raiz descoberto (`DefMap`).
- **Aliases de Importação**:
  - Alias explícito: `import app.util as util`.
  - Alias implícito (último segmento): `import app.util` (disponibiliza `util`).
- **Resolução de Conflitos**: Detecção e diagnóstico claro para imports duplicados, conflitos de nomes locais vs. importados e caminhos circulares (`project.circular_import`).
- **Visibilidade**: Controle de visibilidade `pub` (público) e `priv` (privado) em imports de símbolos entre arquivos.

---

## 3. Sistema de Tipos e Verificação Semântica (Type Checker)

- **Tipos Primitivos**: `int`, `float`, `bool`, `string`, `bytes`, `void`, `never`.
- **Tipos Estruturados**: Structs, enums (com payloads nomeados), tuplas, tipo `optional<T>` e `result<T, E>`.
- **Inferência e Unificação de Tipos**: Algoritmo completo de inferência com unificação para tipos genéricos e resolução monomórfica.
- **Checagem de Funções e Métodos**:
  - Parâmetros posicionais, parâmetros com valores default (inseridos no call-site) e parâmetros variádicos (`Type...` posicionados no final da assinatura).
  - Parâmetros nomeados/rotulados.
  - Contratos em parâmetros de funções e campos de structs (`if it > 0`).
  - Métodos inerentes e métodos de traits, suportando o receptor implícito `self` e explícito.
  - Detecção e erro em recursões infinitas de instanciações genéricas (`generic.circular_instantiation`).

---

## 4. Traits, Operadores e Resolução de Métodos

- **Traits**: Definição e implementação de traits, incluindo métodos default e constraints genéricas (`where T is Trait`).
- **Restrições Negativas**: Validação de constraints negativas no type-checker (`where T is not Trait`).
- **Resolução de Métodos e Ambiguidade**:
  - Múltiplas implementações do mesmo nome de método para traits distintos são aceitas.
  - Chamadas de método ambíguas e diretas são rejeitadas (`type.ambiguous_method`).
  - Suporte a chamadas qualificadas explícitas (`Trait.metodo(valor)`).
- **Sobrecarga de Operadores**: Sobrecarga via traits nativos (`ori.core`) para operadores aritméticos e relacionais (`+`, `-`, `==`, `!=`, `<`, `<=`, `>`, `>=`).
- **Igualdade Estrutural (== / !=)**:
  - Implementado para tipos primitivos, `bytes`, `list<T>`, `optional<T>`, `result<T, E>`, tuplas, structs concretas e structs genéricas (com substituição correta de parâmetros genéricos nos campos em tempo de compilação).
  - Implementado suporte para comparação estrutural avançada em mapas (`map<K,V>`) e conjuntos (`set<T>`) cujos elementos/chaves suportam igualdade (seja via trait `Equatable` ou via igualdade estrutural).
- **Displayable**: Conversão dirigida por trait para strings (`string(value)`) e f-strings.

---

## 5. Lowering (HIR) e Compilação

- **HIR Lowering**: Desaçucaramento completo de construções como operador pipe (`|>`), atualização de structs (`x with { field: v } end`), expressões `is`, indexadores e f-strings em chamadas de runtime correspondentes.
- **using**: Lowering de blocos `using` para injeção automática de chamadas `Disposable.dispose()` na saída do escopo, incluindo caminhos de retorno normal, retornos antecipados (`?`) e panics.
- **Backend Nativo (Cranelift)**: Rota principal de compilação sem dependência direta de toolchain C. Gera binários nativos executando linkagem própria via `NativeLinker` e empacotamento do runtime para Linux e Windows.
- **Backend C (Debug/Transpile)**: Mantido apenas como rota de depuração e transpilação com paridade parcial. Rejeita ativamente features não suportadas de concorrência ou async com diagnósticos claros.

---

## 6. Runtime Nativo e Biblioteca Padrão (Stdlib)

- **Runtime de Referência**: `ori-runtime` implementado em Rust. Gerencia o modelo de memória de contagem de referências atômica (ARC) e bounds-checking para coleções, strings e bytes.
- **Módulos Centrais da Stdlib**:
  - `ori.core`: Declaração de traits essenciais (`Displayable`, `Equatable`, `Comparable`, `Hashable`, `Disposable`, `Default`, `Error`, `Cloneable`).
  - `ori.io`: Operações de entrada e saída, incluindo `print` e `read_line`.
  - `ori.fs`: Leitura e escrita de texto/bytes síncronas e assíncronas (`read_text_async`, `write_text_async`), além de utilitários como `exists`, `delete`, `list_dir`, `create_dir`, etc.
  - `ori.string`: Manipulação e conversão de strings (`trim_start`, `trim_end`, `index_of`, `join`, `repeat`, `pad_left`, `pad_right`, `parse_int`, `parse_float`).
  - `ori.bytes`: Manipulação de bytes crus.
  - `ori.math`: Funções trigonométricas, exponenciais e constantes matemáticas (`pi`, `e`, `nan`, etc.).
  - `ori.convert`: Conversão explícita entre floats, strings, ints e bools.
  - `ori.time` / `ori.os` / `ori.random` / `ori.lazy` / `ori.test`.
  - `ori.iter`: Operações *eager* sobre listas (`map`, `filter`, `any`, `all`, `reduce`, `find`, `zip`, etc.).

---

## 7. Estruturas de Dados Avançadas (Collections v1)

Todas as coleções listadas abaixo estão integradas com o type-checker, possuem ABI estável no runtime nativo e tratamento seguro de valores vazios/nulos com `optional`:

- `ori.list`: Operações de ordenação, fatiamento e manipulação direta.
- `ori.map` / `ori.set`: Implementações de tabelas hash com chaves do tipo `int`, `string` ou customizados com `Hashable + Equatable`.
- `ori.deque` / `ori.queue` / `ori.stack`: Estruturas de dados lineares otimizadas e implementadas sobre buffers circulares no runtime nativo (via `VecDeque`).
- `ori.linked_list` / `ori.doubly_linked_list`: Listas encadeadas expostas através de cursores por posição (`insert_after`, `insert_before`, `remove_at`), evitando vazamentos de ciclos ARC internos.
- `ori.tree`: Árvores gerais baseadas em arenas com `NodeId` e suporte a travessias e reparenting de subárvores.
- `ori.hash_table`: API de tabela hash explícita voltada a performance (capacidade e reservas de memória).
- `ori.graph`: Implementação de grafos direcionados e não-direcionados, cobrindo caminhos sem peso (BFS/DFS), arestas ponderadas, caminhos de custo mínimo (Dijkstra) e ordenação topológica.
- `ori.heap`: Min-heap com ordenação dirigida por `Comparable`.

---

## 8. Concorrência e Async (v1)

- **Concorrência**:
  - Trait marcador `Transferable` validado no checker para tipos que podem cruzar tasks/canais.
  - Spawning e joining de tarefas em threads nativas (`ori.task`).
  - Canais de comunicação sincronizados e seguros para concorrência (`ori.channel`).
  - Inteiros atômicos (`ori.atomic`).
- **Async/Await**:
  - Tipo primitivo `future<T>` integrado.
  - Executor nativo em thread dedicada com timers não-bloqueantes (`task.sleep`).
  - Sintaxe `async func` e expressão `await` controladas no parser, HIR e type checker.
  - Entrada assíncrona principal: `async main()`.
  - **State Machine Async (v1)**: Geração nativa de frame e state machine com despacho por estado para fluxos sequenciais. Preserva a contagem de referências ARC para locals vivos através de suspensões sequenciais simples.

---

## 9. Ferramentas, CLI e LSP

- **Comandos do CLI**:
  - `ori check` (verificação estática rápida).
  - `ori compile` (compilação para executável nativo).
  - `ori run` (atalho para compilar e rodar arquivo temporário).
  - `ori test` (execução automática de suítes de testes `@test`).
  - `ori fmt` (preserva indentação e regras de formatação).
  - `ori doc` (extração de documentação).
- **LSP (`ori-lsp`)**:
  - Emissão de diagnósticos de parser e checker por arquivo.
  - Preenchimento automático (*autocomplete*) para stdlib.
  - Hover semântico detalhado mostrando assinaturas de funções, tipos de variáveis, campos de structs e restrições de contratos.
  - Navegação de ir para a definição (*go-to-definition*) para escopos locais.

---

## 10. Bugs e Ajustes Críticos Resolvidos (Fase 0)

- **Bug 0.1 (Heap)**: Resolvido o problema de ordenação incorreta de elementos no custom min-heap devido ao registro tardio de arestas ARC (`ori_arc_register_edge` invocado corretamente antes de `heap_push_raw`).
- **Bug 0.2 (Iterable)**: Corrigido o avanço incorreto de iteradores customizados em loops `for` nativos (injetando retenção e liberação corretas no cabeçalho do loop).
- **Bug 0.3 (Git)**: Limpeza de objetos inacessíveis no repositório executada com sucesso.
