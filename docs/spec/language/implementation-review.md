# Plano de Análise Profunda da Implementação da Linguagem Zenith

Realize uma análise técnica aprofundada e progressiva da implementação da linguagem **Zenith**, cobrindo **compiler, checker, runtime, stdlib, tooling, documentação, testes e conformidade com o contrato final da linguagem**.

A análise deve ser organizada em fases. Cada fase deve conter tópicos de validação com checkboxes, que só devem ser marcados quando o item tiver sido realmente conferido, testado ou comprovado por evidência no código, nos testes ou na documentação.

A análise deve usar como base os seguintes documentos de referência:

* `final-language-contract.md`, como fonte normativa principal do que é final, executável hoje, futuro ou ainda em discussão.
* `syntax-semantics-by-topic.md`, para validar sintaxe, semântica, formas rejeitadas e comportamento esperado da linguagem.
* `runtime-model.md`, para validar ARC, valores gerenciados, semântica de valor, cleanup, panic, contratos e checks de runtime.
* `stdlib-reference-by-topic.md`, para validar superfície pública, módulos, funções e limites atuais da biblioteca padrão.
* `implementation-plan.md`, para comparar o estado real do projeto contra as fases já planejadas, itens concluídos, pendências e validação final.

---

# Fase 1 — Preparação e Mapeamento do Projeto

Objetivo: entender a estrutura atual do projeto antes de procurar bugs.

## Validações

* [ ] Identificar a estrutura geral do repositório.
* [ ] Localizar os diretórios principais do compiler.
* [ ] Localizar os diretórios principais do runtime.
* [ ] Localizar a implementação da stdlib.
* [ ] Localizar os testes positivos e negativos.
* [ ] Localizar fixtures de comportamento.
* [ ] Localizar testes de diagnóstico.
* [ ] Localizar ferramentas de build, check, run, fmt, test e doc.
* [ ] Localizar documentação pública e documentação interna.
* [ ] Identificar quais arquivos representam a fonte real da verdade para cada área.
* [ ] Confirmar se o projeto possui CI configurado.
* [ ] Confirmar se há scripts oficiais de validação, como build, smoke, conformance ou test suite.

## Resultado esperado da fase

Ao final desta fase, deve ser entregue um mapa técnico do projeto, incluindo:

* principais componentes;
* fluxo de compilação;
* fluxo de execução;
* fluxo de testes;
* arquivos críticos;
* áreas de maior risco;
* partes ainda não claramente documentadas.

---

# Fase 2 — Validação Contra o Contrato Final da Linguagem

Objetivo: verificar se a implementação atual corresponde ao contrato final da linguagem Zenith.

O contrato final define o que é decisão final, subconjunto executável atual, lacuna, implementação futura, histórico ou discussão aberta. A análise deve separar claramente esses estados.

## Validações

* [ ] Verificar se a sintaxe aceita corresponde apenas à sintaxe final.
* [ ] Verificar se sintaxes removidas são rejeitadas.
* [ ] Verificar rejeição de `group`.
* [ ] Verificar rejeição de `fmt"..."`.
* [ ] Verificar rejeição de `given`.
* [ ] Verificar rejeição de `default` em `match`.
* [ ] Verificar rejeição de `dyn`.
* [ ] Verificar rejeição de `try`, `catch` e `throw`.
* [ ] Verificar rejeição de `async` e `await`.
* [ ] Verificar rejeição de `null`.
* [ ] Verificar rejeição de imports wildcard.
* [ ] Verificar rejeição de imports seletivos.
* [ ] Verificar rejeição de inferência local ampla.
* [ ] Verificar se `tuple<T1, T2>` é a forma canônica.
* [ ] Verificar se `any<Trait>` é a forma canônica para dispatch dinâmico.
* [ ] Verificar se `case else:` é o único fallback de pattern matching.
* [ ] Verificar se `case pattern if condition:` é a forma correta de guard.
* [ ] Verificar se `f"..."` é a única forma de interpolação final.
* [ ] Verificar se os operadores customizados são limitados a `Addable`, `Subtractable` e `Comparable`.
* [ ] Verificar se qualquer expansão fora do contrato está marcada como futura ou rejeitada.

## Resultado esperado da fase

Entregar uma matriz de conformidade contendo:

| Área | Status esperado | Status encontrado | Evidência | Risco | Ação necessária |
|---|---|---|---|---|
| Sintaxe removida | Rejeitada |  |  |  |  |
| Tuplas | `tuple` canônico |  |  |  |  |
| `any<Trait>` | Canônico |  |  |  |  |
| Pattern matching | Final |  |  |  |  |
| Error model | Sem exceções |  |  |  |  |
| Runtime | ARC/value semantics |  |  |  |  |
| Stdlib | Subconjunto executável documentado |  |  |  |  |

---

# Fase 3 — Análise do Lexer, Parser e Sintaxe

Objetivo: encontrar falhas de parsing, aceitação indevida de sintaxe antiga, ambiguidade gramatical e erros de diagnóstico.

## Validações

* [ ] Verificar se o lexer reconhece corretamente palavras-chave finais.
* [ ] Verificar se palavras removidas não são aceitas como sintaxe ativa.
* [ ] Verificar comentários `--`.
* [ ] Verificar comentários de bloco `--- ... ---`.
* [ ] Verificar rejeição de `//`.
* [ ] Verificar rejeição de `/* ... */`.
* [ ] Verificar rejeição de `#`.
* [ ] Verificar parsing correto de `namespace`.
* [ ] Verificar parsing correto de imports qualificados.
* [ ] Verificar parsing correto de funções.
* [ ] Verificar parsing correto de structs.
* [ ] Verificar parsing correto de enums.
* [ ] Verificar parsing correto de traits.
* [ ] Verificar parsing correto de `apply`.
* [ ] Verificar parsing correto de `match`.
* [ ] Verificar parsing correto de `using`.
* [ ] Verificar parsing correto de closures.
* [ ] Verificar parsing correto de generics.
* [ ] Verificar parsing correto de slices `start..end`, `start..`, `..end`.
* [ ] Verificar parsing correto de interpolação `f"..."`.
* [ ] Verificar erro claro para interpolação vazia `{}`.
* [ ] Verificar erro claro para interpolação não terminada.
* [ ] Verificar se entradas malformadas não causam crash do compilador.
* [ ] Rodar fuzzing no lexer.
* [ ] Rodar fuzzing no parser.
* [ ] Rodar testes com arquivos incompletos.
* [ ] Rodar testes com Unicode.
* [ ] Rodar testes com strings enormes.
* [ ] Rodar testes com nesting profundo de blocos.

## Resultado esperado da fase

Entregar:

* lista de sintaxes aceitas indevidamente;
* lista de sintaxes válidas rejeitadas indevidamente;
* bugs de parser;
* casos mínimos reproduzíveis;
* fixtures negativos sugeridos;
* melhorias de diagnóstico.

---

# Fase 4 — Análise Semântica, Checker e Tipagem

Objetivo: validar regras de tipos, escopo, mutabilidade, generics, traits, chamadas e restrições semânticas.

A semântica da linguagem exige intenção explícita, imports qualificados, mutação explícita, ausência via `optional<T>`, falhas recuperáveis via `result<T, E>` e falhas fatais via `panic(message)`.

## Validações

* [ ] Verificar resolução de nomes.
* [ ] Verificar escopo local.
* [ ] Verificar escopo de namespace.
* [ ] Verificar símbolos privados por padrão.
* [ ] Verificar `public` em declarações de topo.
* [ ] Verificar shadowing inválido.
* [ ] Verificar nomes duplicados no mesmo escopo.
* [ ] Verificar imports qualificados.
* [ ] Verificar aliases de import.
* [ ] Verificar reexports explícitos.
* [ ] Verificar `const` imutável.
* [ ] Verificar `var` mutável.
* [ ] Verificar que atribuição é statement, não expression.
* [ ] Verificar rejeição de `++` e `--`.
* [ ] Verificar que condições exigem `bool`.
* [ ] Verificar ausência de truthiness numérica.
* [ ] Verificar conversões numéricas explícitas.
* [ ] Verificar rejeição de conversões implícitas inseguras.
* [ ] Verificar overflow/underflow.
* [ ] Verificar divisão por zero.
* [ ] Verificar modulo por zero.
* [ ] Verificar field mutation somente com receiver mutável.
* [ ] Verificar index mutation somente em binding mutável.
* [ ] Verificar contratos `where`.
* [ ] Verificar parâmetros com contratos.
* [ ] Verificar structs com contratos.
* [ ] Verificar retorno de função.
* [ ] Verificar `void`.
* [ ] Verificar argumentos nomeados.
* [ ] Verificar default parameters.
* [ ] Verificar generics por inferência de argumento.
* [ ] Verificar rejeição de inferência apenas por retorno.
* [ ] Verificar constraints `where T is Trait`.
* [ ] Verificar traits genéricas.
* [ ] Verificar overlapping applies.
* [ ] Verificar método `mut func`.
* [ ] Verificar chamadas em `any<Trait>`.
* [ ] Verificar object safety de traits usadas em `any<Trait>`.
* [ ] Verificar incompatibilidades de `optional<T>` e `result<T, E>`.
* [ ] Verificar propagação com `?`.
* [ ] Verificar erro quando `?` é usado em função incompatível.
* [ ] Verificar que `?` não é safe navigation.

## Resultado esperado da fase

Entregar diagnóstico do checker:

* regras corretamente implementadas;
* regras parcialmente implementadas;
* lacunas em relação ao contrato;
* bugs de aceitação indevida;
* bugs de rejeição indevida;
* diagnósticos ruins ou confusos;
* riscos de soundness;
* testes adicionais necessários.

---

# Fase 5 — Pattern Matching e Controle de Fluxo

Objetivo: validar exaustividade, guards, bindings, casos inalcançáveis e comportamento de loops.

## Validações

* [ ] Verificar `if` statement.
* [ ] Verificar `if cond then a else b` como expressão.
* [ ] Verificar obrigatoriedade de `else` em if-expression.
* [ ] Verificar compatibilidade de tipos entre branches.
* [ ] Verificar `else if`.
* [ ] Verificar rejeição de `elif`.
* [ ] Verificar rejeição de `unless`.
* [ ] Verificar `while`.
* [ ] Verificar `while true` com `break`.
* [ ] Verificar `for item in collection`.
* [ ] Verificar `for item, index in list`.
* [ ] Verificar `for key, value in map`.
* [ ] Verificar `repeat N times`.
* [ ] Verificar `repeat 0 times`.
* [ ] Verificar erro em `repeat -1 times`.
* [ ] Verificar `break` somente dentro de loop.
* [ ] Verificar `continue` somente dentro de loop.
* [ ] Verificar `range(start, end)`.
* [ ] Verificar `range(start, end, step)`.
* [ ] Verificar `range` com step negativo.
* [ ] Verificar `match` com literais.
* [ ] Verificar `match` com bindings.
* [ ] Verificar `match` com enums.
* [ ] Verificar `match` com tuple.
* [ ] Verificar `match` com optional.
* [ ] Verificar `match` com result.
* [ ] Verificar `match` com struct simples.
* [ ] Verificar exaustividade.
* [ ] Verificar que guarded cases não contam como cobertura exaustiva.
* [ ] Verificar diagnóstico de caso inalcançável.
* [ ] Verificar escopo de bindings em guards.
* [ ] Verificar rejeição de OR patterns.
* [ ] Verificar rejeição de range patterns.
* [ ] Verificar rejeição de rest/spread patterns.

## Resultado esperado da fase

Entregar:

* tabela de comportamento de controle de fluxo;
* cobertura de pattern matching;
* falhas de exaustividade;
* bugs em escopo de bindings;
* fixtures novos recomendados.

---

# Fase 6 — Runtime, ARC, Memória e Semântica de Valor

Objetivo: validar se o runtime torna verdadeiro o modelo simples exposto ao usuário.

O runtime deve preservar semântica de valor, valores gerenciados, ARC, cleanup, contratos, panic seguro e ausência de aliasing mutável observável.

## Validações

* [ ] Verificar ARC em `text`.
* [ ] Verificar ARC em `bytes`.
* [ ] Verificar ARC em `list<T>`.
* [ ] Verificar ARC em `map<K, V>`.
* [ ] Verificar ARC em `optional<T>` com payload gerenciado.
* [ ] Verificar ARC em `result<T, E>` com payload gerenciado.
* [ ] Verificar ARC em structs com campos gerenciados.
* [ ] Verificar ARC em enums com payloads gerenciados.
* [ ] Verificar ARC em closures.
* [ ] Verificar retain em criação de closure.
* [ ] Verificar release em destruição de closure.
* [ ] Verificar contexto de closure.
* [ ] Verificar captura imutável em closures.
* [ ] Verificar rejeição ou bloqueio de captura mutável indevida.
* [ ] Verificar lazy values.
* [ ] Verificar lazy force somente uma vez.
* [ ] Verificar erro ao forçar lazy consumido.
* [ ] Verificar cleanup em saída normal de escopo.
* [ ] Verificar cleanup em `return`.
* [ ] Verificar cleanup em propagação `?`.
* [ ] Verificar cleanup em `break`.
* [ ] Verificar cleanup em `continue`.
* [ ] Verificar cleanup em panic quando viável.
* [ ] Verificar temporários até o fim do statement.
* [ ] Verificar avaliação de argumentos da esquerda para direita.
* [ ] Verificar falha de construção com cleanup correto.
* [ ] Verificar copy-on-write em collections.
* [ ] Verificar que mutar cópia não altera original.
* [ ] Verificar que `const` collection não pode ser mutada.
* [ ] Verificar ausência de segfault em acesso inválido.
* [ ] Verificar panic estruturado em bounds check.
* [ ] Verificar panic em map missing key.
* [ ] Verificar panic em overflow.
* [ ] Verificar panic em contrato `where`.
* [ ] Verificar erro recuperável em APIs stdlib esperadas.
* [ ] Verificar vazamento de memória com sanitizers.
* [ ] Verificar double free.
* [ ] Verificar use-after-free.
* [ ] Verificar leak em ciclos ARC.
* [ ] Verificar se APIs que podem formar ciclos estão documentadas ou bloqueadas.
* [ ] Verificar se `optional` e `result` escalares evitam heap quando possível.
* [ ] Verificar se wrappers heap-first estão marcados como dívida de performance.

## Resultado esperado da fase

Entregar relatório de runtime contendo:

* segurança de memória;
* vazamentos encontrados;
* violações de semântica de valor;
* locais com aliasing mutável observável;
* pontos de cleanup incorreto;
* problemas em panic;
* problemas em contratos;
* dívidas de performance;
* riscos com ciclos ARC;
* recomendações para hardening.

---

# Fase 7 — Error Model, Panic e Resource Cleanup

Objetivo: validar a separação entre falha recuperável, ausência esperada e falha fatal.

## Validações

* [ ] Verificar `optional<T>` para ausência esperada.
* [ ] Verificar `some(value)`.
* [ ] Verificar `none`.
* [ ] Verificar `result<T, E>` para erro recuperável.
* [ ] Verificar `success(value)`.
* [ ] Verificar `success()` para `result<void, E>`.
* [ ] Verificar `error(value)`.
* [ ] Verificar propagação com `?`.
* [ ] Verificar ausência de conversão automática entre optional e result.
* [ ] Verificar ausência de conversão automática entre tipos de erro.
* [ ] Verificar rejeição de exceptions.
* [ ] Verificar `panic(message)`.
* [ ] Verificar que panic não é capturado por result/optional.
* [ ] Verificar contratos fatais.
* [ ] Verificar APIs `try_create_*` quando validação recuperável for apropriada.
* [ ] Verificar `using`.
* [ ] Verificar `using var`.
* [ ] Verificar `using then cleanup_expr`.
* [ ] Verificar cleanup LIFO.
* [ ] Verificar Disposable automático.
* [ ] Verificar mutating dispose exige `using var`.
* [ ] Verificar recurso não escapa indevidamente do escopo.
* [ ] Verificar recursos FFI com `Disposable`.
* [ ] Verificar recursos em caminhos de erro.

## Resultado esperado da fase

Entregar:

* relatório de consistência do modelo de erro;
* APIs que usam panic quando deveriam usar result;
* APIs que usam result quando deveriam panic;
* APIs que usam sentinel em vez de optional;
* problemas de cleanup;
* recomendações de design.

---

# Fase 8 — Standard Library

Objetivo: validar se a stdlib corresponde ao contrato público e se as limitações atuais estão documentadas.

A stdlib usa imports explícitos, ausência via `optional<T>`, falhas via `result<T, E>`, estado público com parcimônia e separação entre `core`, `std.*` e `platform`.

## Validações

* [ ] Verificar `std.bool`.
* [ ] Verificar `std.int`.
* [ ] Verificar `std.float`.
* [ ] Verificar `std.text`.
* [ ] Verificar `std.bytes`.
* [ ] Verificar `std.list`.
* [ ] Verificar `std.map`.
* [ ] Verificar `std.set`.
* [ ] Verificar `std.collections`.
* [ ] Verificar `std.math`.
* [ ] Verificar `std.random`.
* [ ] Verificar `std.time`.
* [ ] Verificar `std.format`.
* [ ] Verificar `std.encoding`.
* [ ] Verificar `std.hash`.
* [ ] Verificar `std.json`.
* [ ] Verificar `std.regex`.
* [ ] Verificar `std.io`.
* [ ] Verificar `std.console`.
* [ ] Verificar `std.fs`.
* [ ] Verificar `std.os`.
* [ ] Verificar `std.os.process`.
* [ ] Verificar `std.validate`.
* [ ] Verificar `std.test`.
* [ ] Verificar `std.lazy`.
* [ ] Verificar `std.concurrent`.
* [ ] Verificar `std.jobs`.
* [ ] Verificar `std.channels`.
* [ ] Verificar `std.shared`.
* [ ] Verificar `std.atomic`.
* [ ] Verificar `std.orc`.
* [ ] Verificar `std.mem`.
* [ ] Verificar `std.unsafe`.
* [ ] Verificar `std.net`.
* [ ] Verificar `std.http`.
* [ ] Verificar se APIs públicas usam `optional` em vez de sentinel quando apropriado.
* [ ] Verificar se APIs públicas usam `result` para falhas esperadas.
* [ ] Verificar se operações diretas estritas panicam conforme contrato.
* [ ] Verificar se módulos avançados estão marcados como avançados/low-level.
* [ ] Verificar se helpers internos underscore-prefixed não são ensinados como superfície principal.
* [ ] Verificar divergência entre documentação e implementação.
* [ ] Verificar funções documentadas mas não implementadas.
* [ ] Verificar funções implementadas mas não documentadas.
* [ ] Verificar comportamento dos limites atuais do backend.
* [ ] Verificar itens explicitamente deferidos.

## Resultado esperado da fase

Entregar matriz da stdlib:

| Módulo         | Implementado | Testado | Documentado | Limitações | Risco |
| -------------- | -----------: | ------: | ----------: | ---------- | ----- |
| std.text       |              |         |             |            |       |
| std.list       |              |         |             |            |       |
| std.map        |              |         |             |            |       |
| std.concurrent |              |         |             |            |       |
| std.mem        |              |         |             |            |       |
| std.http       |              |         |             |            |       |

---

# Fase 9 — Concurrency, Jobs, Channels e Transferable

Objetivo: validar o modelo de concorrência baseado em fronteiras explícitas, sem estado mutável compartilhado implícito.

O runtime atual usa caminho single-isolate por padrão, ARC não-atômico e transferência por cópia profunda nas fronteiras. Concorrência deve evoluir por jobs, workers e channels, não por compartilhamento implícito.

## Validações

* [ ] Verificar `Transferable`.
* [ ] Verificar deep-copy em fronteiras de job.
* [ ] Verificar rejeição de valores não transferíveis.
* [ ] Verificar `Job<T>`.
* [ ] Verificar `jobs.spawn`.
* [ ] Verificar `jobs.join`.
* [ ] Verificar payloads atualmente suportados.
* [ ] Verificar diagnóstico para payloads não suportados.
* [ ] Verificar `Channel<T>`.
* [ ] Verificar capacidade/backpressure atual.
* [ ] Verificar `Shared<T>`.
* [ ] Verificar `Atomic<T>`.
* [ ] Verificar restrição de `Atomic<int>`, se aplicável.
* [ ] Verificar ausência de compartilhamento implícito de managed values.
* [ ] Verificar ausência de passagem indevida de closures capturadas.
* [ ] Verificar comportamento de panic em jobs.
* [ ] Verificar itens futuros: payload não-int, cancelamento, backpressure, panic capture.
* [ ] Verificar se nomes `_int` não aparecem como API pública recomendada.

## Resultado esperado da fase

Entregar:

* status real da concorrência;
* limites do runtime atual;
* riscos de segurança e memória;
* APIs estáveis;
* APIs experimentais;
* itens futuros obrigatórios.

---

# Fase 10 — FFI e Fronteiras Nativas

Objetivo: encontrar falhas de ABI, recursos nativos mal encapsulados, callbacks inseguros e vazamentos.

## Validações

* [ ] Verificar parsing de `extern c`.
* [ ] Verificar chamada de função C.
* [ ] Verificar `attr name("symbol")`.
* [ ] Verificar `attr abi("cdecl")`.
* [ ] Verificar `attr abi("stdcall")`.
* [ ] Verificar callback primitivo top-level.
* [ ] Verificar rejeição de closure capturada como callback FFI.
* [ ] Verificar rejeição de extern var.
* [ ] Verificar rejeição de variadic extern.
* [ ] Verificar managed args em chamadas FFI.
* [ ] Verificar retain/release em fronteiras FFI.
* [ ] Verificar structs atravessando FFI.
* [ ] Verificar recursos nativos com `Disposable`.
* [ ] Verificar `using` com recurso FFI.
* [ ] Verificar escape de recurso nativo.
* [ ] Verificar falhas de ABI por plataforma.
* [ ] Verificar comportamento em Windows/Linux/macOS, se suportado.

## Resultado esperado da fase

Entregar:

* riscos de ABI;
* violações de ownership;
* APIs FFI inseguras;
* casos de crash;
* recomendações para tipos C explícitos;
* testes negativos adicionais.

---

# Fase 11 — Backend, ZIR, Runtime ABI e Conformance

Objetivo: validar se o backend C continua sendo o oracle de comportamento e se a IR preserva semântica, spans e ownership.

O contrato final afirma que o backend C é o oracle atual e que ZIR/runtime ABI devem estar definidos antes de backends alternativos como Zig, LLVM ou WASM.

## Validações

* [ ] Verificar lowering para HIR/ZIR.
* [ ] Verificar verifier de ZIR.
* [ ] Verificar spans preservados.
* [ ] Verificar source mapping.
* [ ] Verificar emissão C.
* [ ] Verificar chamadas runtime geradas.
* [ ] Verificar temporários gerados.
* [ ] Verificar retain/release emitidos.
* [ ] Verificar cleanup emitido.
* [ ] Verificar panic emitido.
* [ ] Verificar contratos emitidos.
* [ ] Verificar bounds checks emitidos.
* [ ] Verificar overflow checks emitidos.
* [ ] Verificar conformance runner.
* [ ] Verificar fixtures golden.
* [ ] Verificar divergências entre checker e emitter.
* [ ] Verificar código C gerado mal formatado, inseguro ou duplicado.
* [ ] Verificar código morto no backend.
* [ ] Verificar se backends futuros estão claramente deferidos.

## Resultado esperado da fase

Entregar:

* relatório de conformidade backend;
* inconsistências entre front-end e backend;
* bugs de lowering;
* bugs de lifetime;
* riscos para backends futuros;
* recomendações para estabilização de ABI.

---

# Fase 12 — Tooling, Formatter, LSP, Test Runner e Documentação

Objetivo: validar a experiência do usuário e a confiabilidade das ferramentas.

## Validações

* [ ] Verificar `zt check`.
* [ ] Verificar `zt build`.
* [ ] Verificar `zt run`.
* [ ] Verificar `zt test`.
* [ ] Verificar `zt fmt`.
* [ ] Verificar `zt fmt --check`.
* [ ] Verificar `zt doc`.
* [ ] Verificar `zpm install --locked`.
* [ ] Verificar parsing de `zenith.ztproj`.
* [ ] Verificar erro para chaves desconhecidas no manifest.
* [ ] Verificar projetos `app`.
* [ ] Verificar projetos `lib`.
* [ ] Verificar LSP diagnostics.
* [ ] Verificar LSP completions.
* [ ] Verificar formatter com indentação de 4 espaços.
* [ ] Verificar rejeição de tabs.
* [ ] Verificar alinhamento de `end`.
* [ ] Verificar alinhamento de `case`.
* [ ] Verificar uma linha em branco entre declarações top-level.
* [ ] Verificar long signatures.
* [ ] Verificar long calls.
* [ ] Verificar ausência de vertical alignment.
* [ ] Verificar docs públicas contra sintaxe final.
* [ ] Verificar documentação com exemplos executáveis.
* [ ] Verificar se docs não prometem funcionalidades futuras como atuais.

## Resultado esperado da fase

Entregar:

* avaliação da experiência de desenvolvimento;
* problemas no formatter;
* problemas no LSP;
* divergências nos docs;
* lacunas de tooling;
* recomendações para release público.

---

# Fase 13 — Segurança, Robustez e Fuzzing

Objetivo: encontrar bugs graves, crashes, corrupção de memória, DoS e entradas maliciosas.

## Validações

* [ ] Fuzzing do lexer.
* [ ] Fuzzing do parser.
* [ ] Fuzzing do checker.
* [ ] Fuzzing de interpolação de texto.
* [ ] Fuzzing de pattern matching.
* [ ] Fuzzing de generics.
* [ ] Fuzzing de `any<Trait>`.
* [ ] Fuzzing de stdlib text/bytes.
* [ ] Fuzzing de JSON.
* [ ] Fuzzing de regex.
* [ ] Fuzzing de encoding hex/base64.
* [ ] Fuzzing de paths de filesystem.
* [ ] Fuzzing de FFI declarations.
* [ ] Testes com arquivos gigantes.
* [ ] Testes com nesting profundo.
* [ ] Testes com símbolos enormes.
* [ ] Testes com Unicode inválido.
* [ ] Testes com bytes inválidos.
* [ ] Testes com números extremos.
* [ ] Testes com overflow.
* [ ] Testes com recursão profunda.
* [ ] Testes de stack overflow.
* [ ] Testes de consumo de memória.
* [ ] Testes de tempo de compilação.
* [ ] Testes de tempo de execução.
* [ ] Testes contra crash do compilador.
* [ ] Testes contra crash do runtime.
* [ ] Testes com sanitizers.
* [ ] Testes com valgrind ou ferramenta equivalente.
* [ ] Testes de leak.
* [ ] Testes de use-after-free.
* [ ] Testes de double-free.
* [ ] Testes de data race, se houver threads.

## Resultado esperado da fase

Entregar:

* bugs críticos;
* CVEs potenciais;
* crashes reproduzíveis;
* PoCs;
* entradas maliciosas;
* classificação por severidade;
* plano de correção.

---

# Fase 14 — Qualidade de Código, Manutenibilidade e Dívida Técnica

Objetivo: encontrar código ruim, duplicado, morto, mal formatado, mal arquitetado ou difícil de evoluir.

## Validações

* [ ] Identificar código morto.
* [ ] Identificar código duplicado.
* [ ] Identificar funções grandes demais.
* [ ] Identificar arquivos grandes demais.
* [ ] Identificar responsabilidades misturadas.
* [ ] Identificar nomes confusos.
* [ ] Identificar APIs internas inconsistentes.
* [ ] Identificar comentários obsoletos.
* [ ] Identificar TODOs críticos.
* [ ] Identificar lógica repetida entre checker e emitter.
* [ ] Identificar lógica repetida entre stdlib e runtime.
* [ ] Identificar erros tratados de forma inconsistente.
* [ ] Identificar diagnostics hardcoded.
* [ ] Identificar strings mágicas.
* [ ] Identificar números mágicos.
* [ ] Identificar funções sem testes.
* [ ] Identificar módulos sem documentação.
* [ ] Identificar dependências cíclicas.
* [ ] Identificar acoplamento excessivo.
* [ ] Identificar abstrações prematuras.
* [ ] Identificar partes difíceis de portar para backends futuros.
* [ ] Identificar partes difíceis de testar.
* [ ] Identificar dívida técnica bloqueante para v1.
* [ ] Identificar dívida técnica aceitável pós-v1.

## Resultado esperado da fase

Entregar:

| Problema           | Local | Tipo | Severidade | Impacto | Correção sugerida |
| ------------------ | ----- | ---- | ---------- | ------- | ----------------- |
| Código duplicado   |       |      |            |         |                   |
| Código morto       |       |      |            |         |                   |
| Arquitetura frágil |       |      |            |         |                   |
| Diagnóstico ruim   |       |      |            |         |                   |

---

# Fase 15 — Performance e Escalabilidade

Objetivo: avaliar desempenho do compiler, runtime e stdlib.

## Validações

* [ ] Medir tempo de build.
* [ ] Medir tempo de check.
* [ ] Medir tempo de run.
* [ ] Medir tempo de formatter.
* [ ] Medir tempo de test runner.
* [ ] Medir tempo de parsing.
* [ ] Medir tempo de checking.
* [ ] Medir tempo de monomorphization.
* [ ] Medir tempo de emissão C.
* [ ] Medir tempo de compilação C.
* [ ] Medir runtime de listas.
* [ ] Medir runtime de maps.
* [ ] Medir runtime de sets.
* [ ] Medir runtime de text.
* [ ] Medir runtime de bytes.
* [ ] Medir overhead de ARC.
* [ ] Medir overhead de closures.
* [ ] Medir overhead de `any<Trait>`.
* [ ] Medir overhead de `using`.
* [ ] Medir overhead de panic checks.
* [ ] Medir overhead de bounds checks.
* [ ] Medir consumo de memória do compiler.
* [ ] Medir consumo de memória do runtime.
* [ ] Comparar benchmarks antes/depois.
* [ ] Identificar algoritmos O(n²) acidentais.
* [ ] Identificar cópias desnecessárias.
* [ ] Identificar retain/release excessivo.
* [ ] Identificar heap allocation evitável.
* [ ] Identificar wrappers heap-first em optional/result.
* [ ] Identificar pontos de otimização seguros.

## Resultado esperado da fase

Entregar:

* benchmark report;
* hotspots;
* regressões;
* custos de ARC;
* custos de monomorphization;
* custos de stdlib;
* plano de otimização.

---

# Fase 16 — Validação Final do Projeto

Objetivo: executar a bateria final de validação e determinar o estado real do projeto.

O plano de implementação possui uma fase final de validação com itens como `python build.py`, fixtures positivos e negativos, `zt fmt --check`, verificação de documentação, `git diff --check`, ausência de sintaxe removida e cobertura das áreas da matriz final.

## Validações obrigatórias

* [ ] `python build.py` passa.
* [ ] Todos os fixtures positivos passam em `zt check`.
* [ ] Todos os fixtures positivos passam em `zt build`.
* [ ] Todos os fixtures positivos passam em `zt run`.
* [ ] Todos os fixtures negativos falham como esperado.
* [ ] Todos os fixtures negativos retornam diagnóstico esperado.
* [ ] `zt fmt --check` passa em todos os arquivos `.zt`.
* [ ] `python tools/check_docs_current_syntax.py` passa.
* [ ] `git diff --check` passa.
* [ ] Não há referências ativas a `group`.
* [ ] Não há referências ativas a `fmt"..."`.
* [ ] Não há referências ativas a `given`.
* [ ] Não há referências ativas a `default` como fallback de match.
* [ ] Não há referências ativas a `dyn`.
* [ ] Todas as áreas do contrato final têm pelo menos um teste ou evidência.
* [ ] Todos os itens futuros estão explicitamente rastreados.
* [ ] Todas as deferrals estão documentadas.
* [ ] Nenhum documento público promete como atual algo que é futuro.
* [ ] Nenhum comportamento implementado contradiz o contrato final.

---

# Diagnóstico Final Obrigatório

Ao concluir todas as fases, retornar um diagnóstico completo e detalhado do estado atual do projeto.

O diagnóstico deve conter obrigatoriamente:

## 1. Resumo executivo

* estado geral do projeto;
* nível de maturidade;
* prontidão para release;
* principais riscos;
* principais bloqueadores;
* recomendação: liberar, segurar, corrigir antes de liberar ou classificar como experimental.

## 2. Classificação geral

Classificar o projeto em uma das categorias:

* **Crítico** — não recomendado para uso; bugs graves ou insegurança estrutural.
* **Instável** — funciona parcialmente, mas há alto risco de falhas.
* **Alpha técnico** — bom para experimentação controlada.
* **Beta** — bom para usuários iniciais, com limitações documentadas.
* **Release Candidate** — quase pronto, com poucos bloqueadores.
* **Estável** — pronto para uso público conforme o contrato atual.

## 3. Estado por área

| Área             | Estado | Severidade | Observações |
| ---------------- | ------ | ---------: | ----------- |
| Lexer/parser     |        |            |             |
| Checker          |        |            |             |
| Type system      |        |            |             |
| Generics         |        |            |             |
| Traits/apply     |        |            |             |
| Pattern matching |        |            |             |
| Runtime          |        |            |             |
| ARC/memória      |        |            |             |
| Error model      |        |            |             |
| Resource cleanup |        |            |             |
| Stdlib           |        |            |             |
| Concurrency      |        |            |             |
| FFI              |        |            |             |
| Backend C        |        |            |             |
| ZIR/ABI          |        |            |             |
| Tooling          |        |            |             |
| Formatter        |        |            |             |
| LSP              |        |            |             |
| Docs             |        |            |             |
| Testes           |        |            |             |
| Segurança        |        |            |             |
| Performance      |        |            |             |

## 4. Problemas encontrados

Para cada problema:

```md
## Problema N — Título

- Severidade: crítica | alta | média | baixa | informativa
- Área:
- Arquivo:
- Linha/função:
- Descrição:
- Impacto:
- Evidência:
- Caso mínimo reproduzível:
- Comando usado para reproduzir:
- Resultado esperado:
- Resultado atual:
- Causa provável:
- Correção recomendada:
- Teste que deve ser adicionado:
- Prioridade:
```

## 5. Falhas de segurança

Separar explicitamente:

* corrupção de memória;
* use-after-free;
* double-free;
* vazamento de memória;
* segfault;
* panic indevido;
* ausência de bounds check;
* bypass de checker;
* execução arbitrária;
* FFI inseguro;
* DoS por input malicioso;
* consumo excessivo de memória;
* recursão ou nesting sem limite;
* falhas em Unicode/bytes;
* falhas de sandbox, se aplicável.

## 6. Lacunas contra o contrato final

Criar tabela:

| Item do contrato    | Implementado? | Testado? | Documentado? | Lacuna | Ação |
| ------------------- | ------------: | -------: | -----------: | ------ | ---- |
| Sintaxe final       |               |          |              |        |      |
| Tuplas              |               |          |              |        |      |
| `any<Trait>`        |               |          |              |        |      |
| ARC/value semantics |               |          |              |        |      |
| `using` cleanup     |               |          |              |        |      |
| Stdlib boundary     |               |          |              |        |      |

## 7. Dívida técnica

Classificar dívida em:

* bloqueante para v1;
* aceitável para v1;
* pós-v1 obrigatória;
* melhoria opcional.

## 8. Evolução futura necessária

A análise deve indicar quais evoluções são necessárias para o projeto avançar, incluindo:

* maturação de generic HOFs;
* expansão segura de stdlib genérica;
* payloads não-int em jobs/channels/shared;
* backpressure em channels;
* cancelamento de jobs;
* melhor captura de panic em jobs;
* política de ciclos ARC;
* suporte mais amplo de managed values;
* melhorias em LSP;
* web playground;
* package registry;
* `zt bench`;
* `zt migrate`;
* backends Zig/LLVM/WASM;
* ampliação de golden ZIR fixtures;
* conformance runner mais completo;
* sanitizers no CI;
* fuzzing contínuo;
* benchmarks oficiais;
* documentação de limitações atuais.

## 9. Recomendação final

Encerrar com uma recomendação clara:

```md
# Recomendação Final

Status recomendado: [Crítico / Instável / Alpha / Beta / RC / Estável]

O projeto atualmente está em estado: ...

Antes de avançar para a próxima fase, é obrigatório corrigir:

1. ...
2. ...
3. ...

Pode ser tratado como pós-v1:

1. ...
2. ...
3. ...

Riscos residuais:

1. ...
2. ...
3. ...

Conclusão:
...
```

---

# Prompt final consolidado para usar na auditoria

```md
Faça uma análise aprofundada, faseada e exaustiva da implementação da linguagem Zenith, cobrindo compiler, runtime, stdlib, tooling, documentação, testes, segurança, performance e conformidade com os documentos de referência do projeto.

Use como fontes principais:

- final-language-contract.md
- syntax-semantics-by-topic.md
- runtime-model.md
- stdlib-reference-by-topic.md
- implementation-plan.md

A análise deve ser dividida em fases. Cada fase deve conter tópicos de validação com checkboxes `- [ ]`, que só poderão ser marcados como `- [x]` quando tiverem sido realmente conferidos por inspeção, teste, fixture, diagnóstico, execução ou evidência objetiva.

As fases mínimas são:

1. Preparação e mapeamento do projeto
2. Validação contra o contrato final da linguagem
3. Lexer, parser e sintaxe
4. Semântica, checker e tipagem
5. Pattern matching e controle de fluxo
6. Runtime, ARC, memória e semântica de valor
7. Error model, panic e resource cleanup
8. Standard library
9. Concurrency, jobs, channels e Transferable
10. FFI e fronteiras nativas
11. Backend, ZIR, Runtime ABI e conformance
12. Tooling, formatter, LSP, test runner e documentação
13. Segurança, robustez e fuzzing
14. Qualidade de código, manutenibilidade e dívida técnica
15. Performance e escalabilidade
16. Validação final do projeto

Em cada fase, procure:

- erros de implementação;
- bugs;
- falhas de segurança;
- falhas de memória;
- inconsistências com o contrato final;
- divergências entre documentação e código;
- comportamento aceito indevidamente;
- comportamento válido rejeitado indevidamente;
- código morto;
- código duplicado;
- código mal formatado;
- arquitetura frágil;
- APIs inconsistentes;
- diagnósticos ruins;
- testes ausentes;
- cobertura insuficiente;
- lacunas de stdlib;
- limitações não documentadas;
- regressões;
- problemas de performance.

Para cada problema encontrado, retorne:

- severidade;
- área;
- arquivo;
- função ou linha;
- explicação técnica;
- impacto;
- evidência;
- caso mínimo reproduzível;
- comando usado para reproduzir;
- resultado esperado;
- resultado atual;
- causa provável;
- correção recomendada;
- teste que deve ser adicionado;
- prioridade.

Ao final, entregue um diagnóstico completo e detalhado do estado atual do projeto, incluindo:

- resumo executivo;
- classificação geral de maturidade;
- estado por área;
- lista priorizada de problemas;
- falhas de segurança;
- lacunas contra o contrato final;
- dívida técnica;
- cobertura de testes;
- qualidade da documentação;
- riscos de release;
- evolução futura necessária;
- recomendação final clara sobre o estado do projeto.

A análise não deve apenas dizer se algo parece correto. Ela deve comprovar com evidências, testes, comandos executados, fixtures, diagnóstico do compilador, comportamento observado e comparação direta com os documentos de referência.
```
