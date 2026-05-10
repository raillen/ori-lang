# Contrato Final da Linguagem Zenith

> Público: mantenedores, designers da linguagem, implementadores do compilador/runtime  
> Status: tradução auxiliar do contrato final  
> Superfície: consolidação normativa explicada  
> Fonte da verdade: não; a fonte normativa é `final-language-contract.md`  
> Última atualização: 2026-05-03

Este documento é a versão em português do contrato compacto atual da linguagem Zenith após o fechamento pós-v1.

Use este arquivo para entender o contrato. Em caso de conflito, a versão normativa em inglês, `final-language-contract.md`, prevalece.

Ele responde quatro perguntas:

- o que é a forma final aprovada da linguagem;
- o que executa hoje no backend/runtime C;
- qual lacuna ainda existe;
- onde está a evidência canônica detalhada.

## Rótulos de Status

| Rótulo | Significado |
|---|---|
| Contrato Final | Forma aprovada da linguagem. É o alvo de design, salvo se uma decisão explícita mais nova substituir esse ponto. |
| Subconjunto Executável Atual | O que o backend/runtime C atual consegue verificar, compilar e executar hoje. |
| Histórico | Racional antigo ou contexto de planejamento. Não define o comportamento atual. |
| Contexto de Migração | Grafia ou comportamento antigo mantido intencionalmente por compatibilidade/depreciação. |
| Implementação Futura | Direção aceita, mas ainda não implementada no subconjunto executável atual. |
| Discussão Aberta | Ainda não finalizado. Requer uma sessão de design antes de implementação. |

## Como Ler a Matriz

Cada linha separa intenção final de realidade executável:

- **Decisão final**: o desenho aprovado da linguagem.
- **Implementação atual**: o que já funciona hoje no compilador/backend/runtime.
- **Lacuna**: o que falta, o que precisa de hardening, ou o que não deve ser ensinado como completo.
- **Documento canônico**: onde está a evidência técnica detalhada.

Isso evita confundir “aceito como design” com “já executa completamente hoje”.

## Matriz do Contrato Final

| Área | Decisão final | Implementação atual | Lacuna | Documento canônico |
|---|---|---|---|---|
| Congelamento de sintaxe | A sintaxe aceita/rejeitada está congelada para este fechamento: tipos explícitos, imports qualificados, `using`, `if cond then a else b`, guards com `case ... if guard:`, pipe `|>`, acesso `@field`, `..` para slice, interpolação `f"..."`, sem `group`, sem `fmt"..."`, sem `given`, sem `unless`, sem `for` estilo C, sem variádicos e sem `{ fields }` nu. | Parser, checker e formatter cobrem o conjunto executável atual. | Recursos com muitos símbolos precisam de regras de ensino e formatação antes da reconstrução dos docs públicos. | `post-v1-syntax-freeze.md`, `post-v1-implementation-plan.md` |
| Tipos e genéricos | Anotações explícitas de tipo em variáveis locais continuam obrigatórias. Inferência de argumentos genéricos em posição de chamada é aceita. Inferência por contexto de retorno e inferência local completa continuam rejeitadas. | Inferência de argumentos genéricos e subconjunto executável de monomorfização no backend C estão implementados para chamadas genéricas diretas/aninhadas. | HOFs genéricos mais amplos, bounds compostos mais ricos e mais superfícies genéricas de runtime continuam trabalho futuro. | `post-v1-monomorphization-closure.md`, `post-v1-monomorphization-controls.md`, `post-v1-remaining-language-work.md` |
| Tuplas | `tuple<T1,T2,...>` é a forma canônica única. `group` foi removido da superfície final. Use `tuple` em docs normativos, de referência e públicos. | Literais de tupla, `tuple<...>`, structs C geradas, destructuring em `const` e match com múltiplos valores estão implementados. | Acesso posicional a campos ainda é um item de superfície separado quando não estiver coberto pelos caminhos de lowering já existentes. | `language-reference.md`, `post-v1-implementation-plan.md` |
| Traits e apply | Traits/apply são o modelo de composição de comportamento. Métodos default e lookup determinístico de apply são aceitos. Applies sobrepostos são rejeitados. | Parsing/checking de traits, métodos default, lookup de métodos, traits centrais e diagnósticos de overlap estão implementados para o subconjunto atual. | Formas genéricas de traits mais ricas e bounds compostos precisam de hardening incremental. | `post-v1-trait-stability.md`, `post-v1-remaining-language-work.md` |
| Operator overloading Level 2 | Implementado e final apenas como traits restritas de operador: `Addable` para `+`, `Subtractable` para `-`, `Comparable` para `<`, `<=`, `>`, `>=`. Overloading amplo de funções/métodos/operadores continua rejeitado. | O checker reconhece `Addable`, `Subtractable`, `Comparable`; HIR baixa operadores suportados para chamadas de métodos de trait; fixtures cobrem casos positivos e erros de trait ausente. | Não há traits para multiplicação, divisão, módulo ou bitwise. Qualquer expansão exige nova decisão explícita porque aumenta risco de comportamento oculto. | `post-v1-trait-stability.md`, `post-v1-implementation-plan.md`, `docs/internal/decisions/language/042-overload-lambdas-and-macros.md` |
| Callables e closures | Tipos callable `func(T) -> R` são aceitos. Closures usam sintaxe callable explícita e regras de captura definidas. A ABI de callable/closure armazenado está definida. | Valores callable, bindings locais, subconjunto de captura de closure, funções aninhadas, subconjunto de ABI de callback e fronteira de callback de jobs estão implementados. | Callbacks de closures capturadas atravessando FFI continuam rejeitados. Docs de closure são artefatos de evidência; este contrato é o ponto de entrada de leitura. | `post-v1-callable-closure-abi.md`, `language-reference.md`, `post-v1-implementation-plan.md` |
| Dispatch com `any` | A grafia canônica é `any<Trait>`. `dyn` é apenas contexto de migração/alias depreciado do parser. `any` é dispatch object-safe de trait, não acesso dinâmico a campos nem reflexão universal. | Parser, checker, diagnósticos, LSP e emitter usam comportamento canônico de `any`, preservando compatibilidade depreciada com `dyn`. `list<any<TextRepresentable>>` e o baseline de usuário `list<any<Trait>>` estão validados para literal, iteração, indexação, slice, `len`, `std.list.append`, atribuição por índice/list-set e dispatch por vtable. | Formatos de trait mais amplos, casos de retorno gerenciado além do subconjunto validado, `any` escalar mutável, garantias cross-thread e objetos de traits genéricos precisam de hardening futuro. | `post-v1-any-migration.md`, `post-v1-any-dispatch-stabilization.md`, `post-v1-remaining-language-work.md` |
| Pattern matching | Guards, match de payload de enum, match de optional, destructuring de const e match com múltiplos valores são aceitos para o subconjunto atual. | Implementado e coberto por fixtures para guards, destructuring de tupla, match de múltiplos valores e padrões de optional/enum. | Expansões futuras de pattern matching devem preservar exaustividade e contratos de diagnóstico. | `post-v1-pattern-matching-closure.md`, `language-reference.md` |
| Modelo de erro | `result`, `optional`, `?`, `.or_return`, `.or_wrap` e fronteiras de panic são finais. Não há `try/catch`; não há sintaxe `async/await`. | Checker/lowering atuais aplicam propagação compatível e comportamento de fronteira para o subconjunto implementado. | Hardening adicional de diagnósticos e casos-limite de FFI/jobs permanecem trabalho incremental. | `post-v1-error-model-closure.md`, `diagnostic-code-catalog.md` |
| Limpeza de recursos | `using` é o constructo público de cleanup. Cleanup é determinístico e LIFO em retornos, `?`, panic e saídas de controle de loop. | Implementado para o escopo atual do backend C. | Ownership de cleanup em cenários cross-thread/cross-FFI deve permanecer explícito conforme APIs expandirem. | `post-v1-using-cleanup-semantics.md`, `runtime-model.md` |
| Memória e ownership | Zenith mantém semântica de valor/gerenciada sem keywords de ownership. Hooks ORC e APIs de intenção de `std.mem` são nível biblioteca. | Moves ARC/ORC por último uso, hooks ORC estáveis, subconjuntos de coleções genéricas, `std.unsafe`, helpers concretos de `std.mem` para text/list e `mem.own/view/edit` para o subconjunto seguro finalizado do Appendix B estão implementados. | Coleta completa de ciclos só se torna significativa quando existirem APIs públicas capazes de formar ciclos. Enums, optional/result payloads, valores gerenciados mutáveis aninhados, chaves de set por tupla/struct, valores gerenciados de map e recursos de allocator ficam rastreados no Appendix B. | `post-v1-runtime-abi-ownership-audit.md`, `post-v1-remaining-language-work.md`, `runtime-model.md`, `implementation-plan.md` |
| Concorrência | A direção final para o usuário é jobs/channels/shared/atomic tipados com handles explícitos, `Transferable`, jobs/channels para IO assíncrono, sem scheduler oculto e sem `async/await`. | O subconjunto executável atual tem `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>` e `Atomic<int>`. APIs especializadas `_int`/`_text` continuam âncoras concretas de backend/runtime para o oráculo C atual. | Ensinar publicamente as facades tipadas quando disponíveis; tratar nomes especializados de runtime como evidência de backend. Payloads mais amplos, capacidade/backpressure, cancelamento e captura de panic mais rica continuam implementação futura. | `post-v1-concurrency-semantics-closure.md`, `post-v1-remaining-language-work.md`, `stdlib/std/jobs.zt`, `stdlib/std/channels.zt`, `stdlib/std/atomic.zt` |
| FFI | `extern c` é explícito. Callbacks e anotações de ABI são aceitos. Structs de usuário atravessando FFI precisam de representação C explícita. Valores gerenciados só atravessam por formatos ABI suportados. | Callbacks top-level primitivos, invocação C imediata, `attr name` e `attr abi("cdecl"|"stdcall")` estão implementados. | Callbacks capturados, valores gerenciados, variáveis extern, varargs e externs condicionais exigem trabalho futuro com gates, conforme especificado. | `post-v1-callable-closure-abi.md`, `post-v1-remaining-language-work.md` |
| ABI de runtime e ZIR | O backend C continua sendo o oráculo. Contratos de ZIR/runtime ABI são definidos antes de backends alternativos. | Verifier, contratos de source mapping, auditoria da ABI de runtime, contrato de conformidade do oráculo C e fixtures de closure existem. | Automatizar runner de conformidade de backend antes de ativar Zig/LLVM/WASM. Expandir fixtures golden de ZIR. | `post-v1-zir-consolidation.md`, `post-v1-backend-conformance-suite.md`, `post-v1-source-mapping-contract.md` |
| Fronteira da biblioteca padrão | Zenith usa stdlib de fundação mais pacotes oficiais. Fundações de protocolo podem viver na stdlib; frameworks/bibliotecas de domínio vivem em pacotes. | O subconjunto executável atual inclui stdlib central mais fundações implementadas de time/net/lazy/HOF/memória/concorrência. | APIs HTTP/TLS/WebSocket/server, streams/sinks genéricos, lazy genérico, HOFs cross-type e implementação da política de graduação de pacotes continuam futuro. | `post-v1-remaining-language-work.md`, `stdlib-model.md` |
| Fronteira de tooling | Tooling é LSP-first e externo. Não há plugins de compilador in-process. `zt bench` e `zt migrate` são direções aceitas. | CLI/LSP/formatter/diagnósticos atuais existem; lint de sintaxe documental existe. | Docs públicos estão em reset; LSP maduro, extensão marketplace, playground web, polimento de migrator e registry continuam futuro. | `post-v1-remaining-language-work.md`, `tooling-model.md` |

## Estacionamento de Discussões Abertas

Estes itens não bloqueiam o fechamento da linguagem, mas precisam ser resolvidos antes da próxima reconstrução dos docs públicos ou antes de a implementação estrutural final depender deles.

| Tópico | Problema atual | Próximo passo necessário |
|---|---|---|
| Pressão do orçamento de símbolos | `?`, `@field`, `|>`, attributes, ângulos genéricos e sintaxe de derive são aceitos individualmente, mas ficam visualmente densos juntos. | Criar regras de formatação/ensino com exemplos mostrando quando quebrar linhas, preferir helpers nomeados ou evitar empilhar símbolos. |
| `tuple` versus `group` | `group` foi removido da superfície final para manter uma forma canônica única. | Ensinar apenas `tuple` em docs públicos e manter `group` apenas em contexto histórico/migração. |
| Tensão de operator overloading | Traits de operador Level 2 tornam código conciso, mas escondem chamadas de método atrás de símbolos. | Manter Level 2 fixo; exigir nomes/métodos de trait explícitos nos docs; rejeitar expansão salvo se o ganho de legibilidade superar o custo de comportamento oculto. |
| Nomeação do subconjunto runtime/backend | Nomes `_int` são amigáveis para implementação, mas não são elegantes para usuário final. | Tratar APIs `_int` como âncoras backend/runtime e evidência do subconjunto executável atual; ensinar facades tipadas onde implementadas; documentar diagnósticos de capacidade para payloads não suportados. |
| Reset dos docs públicos | Docs públicos existentes misturavam decisões finais, sintaxe antiga e implementação parcial. | Apagar docs públicos atuais e reconstruir a partir deste contrato mais evidência de status de implementação. |

## Precedência

1. `final-language-contract.md` é o índice normativo compacto para distinguir final/futuro/subconjunto atual.
2. Esta tradução PT-BR é auxiliar; ela não substitui o arquivo normativo em inglês.
3. Artefatos detalhados de closure continuam sendo evidência autoritativa para seu tópico específico.
4. `post-v1-remaining-language-work.md` rastreia lacunas aceitas e implementação futura.
5. Decisões antigas preservam racional, mas perdem conflitos para specs mais novos e para o contrato final.
6. Docs públicos não devem ser reconstruídos a partir de material público antigo sem verificar o contrato final primeiro.
