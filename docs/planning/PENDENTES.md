# Recursos Pendentes e Plano de Correções — Ori Language

Este documento descreve as funcionalidades pendentes, bugs conhecidos e melhorias necessárias para a maturidade da linguagem Ori. As tarefas estão estruturadas em **Etapas de Desenvolvimento** sequenciais. Para avançar de uma etapa para a outra, todos os respectivos itens da etapa atual devem estar marcados como concluídos (`[x]`).

---

## Etapa 1: Features Bloqueadoras e Consistência (Alta Prioridade)
*Esta etapa foca em resolver as limitações de fluxo assíncrono, limpeza de memória e APIs básicas de arquivos/cancelamento.*

### 1. Redesign da State Machine Async para Controles de Fluxo
- [x] Implementar suporte a `await` dentro de blocos condicionais e de repetição (`if`, `else`, `match`, `while`, `for`), permitindo pontos de suspensão em múltiplos caminhos (branching states).
- [x] Adaptar a representação HIR/MIR para mapear os novos estados assíncronos gerados por branches.
- [x] Atualizar a geração de código no backend nativo para o despacho correto no método `step` da state machine.
- [x] Escrever testes de regressão que validem variáveis locais e ARC preservados em branches que contêm suspensões.

### 2. Dispose Automático em Frames Assíncronos (`using` + `async`)
- [x] Garantir que o bloco `using` usado dentro de funções `async` invoque o método `dispose()` do recurso ao sair do escopo, mesmo em suspensões ou erros.
- [x] Injetar a chamada de destruição do recurso ARC nos caminhos terminais do frame assíncrono (retorno normal, erro via `?` ou future cancelado).
- [x] Criar testes garantindo que recursos (como conexões ou arquivos) sejam fechados de forma determinística após await.

### 3. File Handle Dedicado (`ori.fs.File`)
- [x] Criar a assinatura e estrutura de tipo opaco `File` na stdlib.
- [x] Implementar funções nativas no runtime Rust para:
  - `open_read(path: string) -> result<File, string>`
  - `open_write(path: string) -> result<File, string>`
  - `read(file: File, bytes_count: int) -> result<bytes, string>`
  - `write(file: File, data: bytes) -> result<int, string>`
  - `close(file: File) -> void`
- [x] Expor o binding no compilador (`stdlib.rs`) e adicionar testes no backend nativo.

### 4. Cancelamento Cooperativo de Tarefas
- [x] Implementar o tipo opaco `task.CancelToken` no runtime.
- [x] Expor `task.cancel(token: CancelToken) -> void` e checagem de estado `task.is_cancelled(token) -> bool`.
- [x] Integrar o token com o executor nativo para interromper o agendamento de futures associados.
- [x] Escrever casos de teste validando o cancelamento de tarefas demoradas.

### 5. Igualdade Estrutural Avançada
- [x] Estender os operadores `==` e `!=` no backend nativo para suportar structs genéricas.
- [x] Implementar comparação estrutural de mapas e conjuntos (`map<K,V>` e `set<T>`) cujos elementos/chaves implementam o trait `Equatable`.

### **Critérios de Passagem para a Etapa 2:**
- [x] Todos os 5 blocos acima implementados.
- [x] Nenhuma regressão nas suítes `concurrency_async` e `multifile_imports`.
- [x] Teste de sanidade do compilador executado com sucesso.

---

## Etapa 2: Avanços no Sistema de Tipos (Compilador)
*Esta etapa estende as capacidades semânticas e expressividade do compilador e do type-checker.*

### 1. Igualdade Dinâmica para Traits
- [ ] Implementar a igualdade estrutural para objetos dinâmicos `any<Trait>`.
- [ ] Desenhar o mecanismo de lookup via vtable no runtime nativo para invocar as funções de igualdade do tipo concreto correspondente.

### 2. Associated Types em Traits
- [ ] Modificar o parser para aceitar declarações de tipos associados em traits (ex: `type Item`).
- [ ] Atualizar o type-checker para validar e unificar tipos associados em assinaturas de funções genéricas.
- [ ] Adaptar a monomorfização no backend para resolver os tipos associados em tempo de compilação.

### 3. Const Generics e Higher-Kinded Types (HKT)
- [ ] Remover as restrições temporárias `generic.unsupported_const_generic` e `generic.unsupported_hkt`.
- [ ] Implementar a sintaxe e a semântica de checagem para parâmetros genéricos de constantes (ex. tamanhos fixos de arrays/bytes).
- [ ] Implementar tipos genéricos parametrizados por outros tipos genéricos (HKT) com suporte a constraints avançadas.

### 4. Igualdade e Propagação de Traits para Coleções
- [ ] Habilitar comparação direta `==` para tipos opacos de coleções (`Deque`, `Stack`, `Queue`, `LinkedList`, etc.).
- [ ] Implementar a propagação estática de traits (ex. permitir `list<T> is Equatable` somente se `T is Equatable`).

### 5. Iteradores Lazy Gerais
- [ ] Definir e implementar a interface lazy para estruturas opacas, evitando a necessidade de cópias completas/snapshots (`to_list()`).
- [ ] Adicionar suporte a iteradores "vivos" com políticas claras de invalidação caso a coleção subjacente seja modificada.

### 6. API de JSON Estruturado
- [ ] Substituir o mapeamento atual de `json.Value = string` por um tipo de dado real e recursivo na stdlib:
  ```ori
  enum Value
      Null
      Bool(value: bool)
      Number(value: float)
      String(value: string)
      Array(items: list<Value>)
      Object(fields: map<string, Value>)
  end
  ```
- [ ] Implementar parser e serializador nativos em Rust no runtime para esse tipo, mantendo o suporte a *pretty print*.

### **Critérios de Passagem para a Etapa 3:**
- [ ] Traits avançados, const generics e HKT compilando e passando por testes semânticos dedicados.
- [ ] API recursiva de JSON validada com testes de parse/stringificação.

---

## Etapa 3: Robusteza do Runtime e Coleta de Memória (Runtime & ARC)
*Esta etapa foca na garantia de vazamento zero de memória e recursos.*

### 1. Destrutores Tipo-Específicos Completos
- [ ] Auditar todos os layouts de alocação de memória do backend nativo (structs, enums, tuplas, collections).
- [ ] Desenvolver geradores automáticos de funções destrutoras no backend nativo, garantindo que objetos compostos aninhados liberem seus campos recursivamente no descarte.

### 2. Cycle Collector para Referências ARC
- [ ] Implementar o Cycle Collector no runtime Rust (`ori-runtime`) baseado nos grafos de arestas registrados (`ori_arc_register_edge`).
- [ ] Integrar o coletor de ciclos com a thread principal ou dispará-lo periodicamente de forma cooperativa.
- [ ] Validar a detecção e limpeza automática de ciclos complexos órfãos (ex: grafos cíclicos de objetos, referências circulares em estruturas customizadas).

### **Critérios de Passagem para a Etapa 4:**
- [ ] Validação de Memory Leaks ativada e passando sem erros sob execução de testes de estresse cíclicos.

---

## Etapa 4: LSP Semântico e Ferramental (LSP & Tooling)
*Melhorias na experiência de desenvolvimento e diagnóstico do workspace.*

### 1. Índice Semântico Cross-Module no LSP
- [ ] Reestruturar o `ori-lsp` para gerar um modelo semântico completo de todo o projeto (workspace), resolvendo tipos e referências entre múltiplos arquivos de forma inteligente, em vez de depender da indexação textual local por arquivo.
- [ ] Implementar auto-complete de membros e métodos baseados no tipo real do objeto.

### 2. Testes E2E de LSP e Formatter
- [ ] Desenvolver testes de integração reais simulando requisições LSP (hover, go-to-definition, autocomplete) via tower-lsp.
- [ ] Garantir que o comando `ori fmt` formate corretamente construções complexas de concorrência e async.

### 3. Diagnósticos de Nível de Projeto
- [ ] Emitir mensagens de erro e avisos estruturados do compilador no LSP para problemas que abrangem múltiplos arquivos (importações circulares redundantes, namespaces divergentes, entrypoint `main` ausente).

### **Critérios de Passagem para a Etapa 5:**
- [ ] LSP indexando corretamente projetos multi-módulo complexos com hover semântico preciso em todas as referências.

---

## Etapa 5: Diagnósticos Restantes (Catálogo)
*Finalização da consistência do catálogo de diagnósticos da linguagem.*

- [ ] Implementar emissão e testes para os seguintes códigos planejados (atualmente reservados no catálogo):
  - [ ] `bind.undefined` (uso de símbolo não declarado)
  - [ ] `contract.check_failure` (falha genérica de contrato)
  - [ ] `contract.field_violation` (violação de contrato de campo)
  - [ ] `contract.param_violation` (violação de contrato de parâmetro)
  - [ ] `doc.unclosed_block` (bloco de comentário não fechado)
  - [ ] `generic.ambiguous_type_arg` (ambiguidade de tipo genérico)
  - [ ] `match.guard_not_exhaustive` (guarda de pattern matching não exaustiva)
  - [ ] `project.circular_import` (importação circular)
  - [ ] `project.entry_not_found` (arquivo de entrada principal não encontrado)
  - [ ] `project.namespace_file_mismatch` (divergência de namespace físico)
  - [ ] `project.no_proj_file` (arquivo de projeto ausente)
  - [ ] `type.ambiguous_generic` (especificação de genérico ambígua)
  - [ ] `type.annotation_required` (anotação explícita necessária)
  - [ ] `using.non_result_init` (uso de using sem inicializador do tipo result/disposable)

### **Critérios de Passagem para a Etapa 6:**
- [ ] Todos os diagnósticos acima integrados ao type-checker ou parser e cobertos por testes unitários dedicados em `compiler/crates/ori-driver/tests/diagnostic_catalog.rs`.

---

## Etapa 6: Finalização do Projeto (Release)
*Atividades finais de empacotamento, qualidade e publicação.*

- [ ] Atualizar o arquivo [CHANGELOG.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/CHANGELOG.md) descrevendo as mudanças de escopo e novas APIs de coleções nativas.
- [ ] Sincronizar todos os documentos em `docs/spec/` garantindo que o status de cada recurso reflita a realidade técnica.
- [ ] Atualizar o arquivo [AGENTS.md](file:///c:/Users/raillen.DESKTOP-99RJ5M6/Documents/Projetos/ori-lang/AGENTS.md) com o status atualizado do compilador e testes.
- [ ] Executar otimização no repositório local:
  ```powershell
  git gc --prune=now
  ```
- [ ] Enviar todas as alterações locais consolidadas para o repositório remoto:
  ```powershell
  git push origin master
  ```

### **Critério Final:**
- [ ] Workspace limpo, testes 100% integrados e passando na pipeline local e CI remota.
