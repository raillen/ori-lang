# Analise de migracao para Rust

Data: 2026-05-09  
Escopo: compilador, runtime, backend C, design da linguagem e viabilidade de um backend/runtime em Rust.  
Entrada principal: `docs/internal/migrate-to-rust.md`.

## Resumo executivo

Vale a pena desenvolver um caminho em Rust, mas nao como reescrita total imediata.

Recomendacao objetiva:

1. Manter o backend C como oraculo oficial de comportamento.
2. Fechar melhor o contrato de ZIR estruturado.
3. Criar primeiro um backend Rust incremental que leia ZIR estruturado e emita C.
4. Usar a mesma runtime C no primeiro ciclo.
5. Migrar partes da runtime para Rust depois, por ABI C estavel e testes diferenciais.
6. Considerar um frontend/compilador totalmente em Rust apenas quando o backend Rust ja passar a suite de conformidade.

Esforco estimado:

| Caminho | Esforco | Recomendacao |
|---|---:|---|
| Backend Rust que emite C a partir de ZIR | Alto | Sim, melhor primeiro passo |
| Runtime Rust parcial via ABI C | Alto | Sim, depois do backend Rust |
| Reescrita total do compilador e runtime | Muito alto | Nao agora |
| Refatorar apenas C | Medio a alto | Util, mas nao resolve todos os riscos |

O projeto esta em um estado tecnicamente bom para iniciar uma migracao controlada: o build passou, o projeto raiz passou no `zt check`, o pacote Borealis passou no `zt check`, a sintaxe atual dos docs passou, e a suite smoke passou com 69/69.

## Evidencia usada

Validacoes executadas nesta analise:

```text
python build.py
./zt.exe check zenith.ztproj --all --ci
./zt.exe check packages/borealis/zenith.ztproj --all --ci
python tools/check_docs_current_syntax.py
python run_suite.py smoke --no-perf
```

Resultado:

```text
build: ok
root check: ok
Borealis check: ok
docs syntax: ok
smoke suite: 69/69
```

Metricas rapidas da base C atual:

| Medida | Valor |
|---|---:|
| Arquivos `.c`/`.h` em `compiler/` e `runtime/c/` | 93 |
| Linhas C/H aproximadas | 92.011 |
| Arquivos em `tests/` | 1.849 |
| Fixtures `.zt` em `tests/` | 604 |
| Ocorrencias de `malloc` | 103 |
| Ocorrencias de `calloc` | 116 |
| Ocorrencias de `realloc` | 68 |
| Ocorrencias de `free` | 1.158 |
| Ocorrencias de `snprintf` | 564 |
| Ocorrencias de `legacy` em compiler/runtime/docs relevantes | 52 |
| Ocorrencias de `expr_text` | 130 |

Maiores arquivos observados:

| Arquivo | Linhas |
|---|---:|
| `compiler/targets/c/emitter.c` | 13.799 |
| `compiler/semantic/types/checker.c` | 7.874 |
| `compiler/driver/lsp.c` | 5.976 |
| `runtime/c/zenith_rt_outcome.c` | 5.908 |
| `compiler/hir/lowering/from_ast.c` | 5.695 |
| `compiler/zir/lowering/from_hir.c` | 5.480 |
| `compiler/driver/main.c` | 4.158 |
| `compiler/frontend/parser/parser.c` | 3.053 |

## Diagnostico da implementacao C

### Arquitetura atual

O compilador ja tem uma arquitetura por estagios:

```text
source .zt
  -> lexer
  -> parser
  -> AST
  -> binder
  -> type checker
  -> HIR
  -> ZIR
  -> verifier
  -> C backend
  -> native compiler
  -> executable + runtime C
```

Arquivos centrais:

| Camada | Arquivos principais |
|---|---|
| CLI e orquestracao | `compiler/driver/main.c`, `compiler/driver/pipeline.c`, `compiler/driver/driver_internal.h` |
| Frontend | `compiler/frontend/lexer/`, `compiler/frontend/parser/`, `compiler/frontend/ast/` |
| Semantica | `compiler/semantic/binder/`, `compiler/semantic/types/`, `compiler/semantic/diagnostics/` |
| HIR | `compiler/hir/nodes/`, `compiler/hir/lowering/` |
| ZIR | `compiler/zir/model.*`, `compiler/zir/lowering/`, `compiler/zir/verifier.*` |
| Backend C | `compiler/targets/c/` |
| Runtime | `runtime/c/zenith_rt.c`, `runtime/c/zenith_rt.h`, modulos `zenith_rt_*.c` |
| Stdlib | `stdlib/std/*.zt`, `stdlib/core/`, `stdlib/platform/` |

O fluxo principal em `compiler/driver/pipeline.c` passa por parse, namespace validation, binder, checker, HIR, ZIR, verifier e limite de monomorfizacao antes de entregar o modulo ZIR para emissao.

### Separacao real entre frontend, backend e runtime

A separacao existe, mas ainda nao esta perfeita.

Pontos positivos:

- O frontend, HIR, ZIR, checker e emissor C estao separados em pastas.
- Existe `zir_verify_module`.
- Existem documentos canonicos: `compiler-model.md`, `runtime-model.md`, `final-language-contract.md`, `post-v1-backend-conformance-suite.md`.
- O C backend e declarado como oraculo de comportamento para backends futuros.

Pontos fracos:

- O backend C ainda depende de texto em ZIR, como `expr_text` e `init_expr_text`.
- `compiler-model.md` diz que o backend nao deve parsear expressoes textuais como contrato principal, mas `emitter.c` ainda contem muitos caminhos textuais.
- O emissor tem caminhos "legacy" para expressao, FFI e runtime helpers.
- `driver_internal.h` junta dependencias de muitas camadas e expoe globals.
- A runtime tem modulos separados, mas `zenith_rt.c` funciona como unity source que inclui varios `.c`.

### Build e execucao

O build do compilador e simples:

- `build.py` percorre `compiler/` com `os.walk`.
- Compila `zt.exe` e `zpm.exe` com `gcc -O0 -Wall -Wextra -I.`.
- `main.c`, `zpm_main.c` e `lsp.c` sao tratados como entrypoints especiais.

O build de programas Zenith depende do backend C e de um compilador C externo:

- `ZT_CC` ou `CC` podem escolher o compilador.
- Caso contrario, usa `gcc`.
- A runtime e compilada e cacheada como `.ztc-tmp/runtime/zenith_rt.o`.
- A runtime principal usa `runtime/c/zenith_rt.c` como unidade agregadora.

Esse modelo e pragmatico e funciona, mas amarra portabilidade, instalacao e UX a um toolchain C presente no ambiente.

### Gerenciamento de memoria

Ha tres modelos misturados:

1. Arena e string pool no frontend/AST.
2. Ownership manual em HIR/ZIR, com funcoes `dispose`.
3. Runtime com header de heap, referencia contada, retain/release, deep copy e ORC parcial.

Exemplos observados:

- `zt_header` contem `rc` e `kind`.
- `zt_retain`, `zt_release` e `zt_deep_copy` formam a base dos valores gerenciados.
- A runtime documenta que valores gerenciados comuns vivem em um dominio single-isolate.
- Cross-thread deve usar copia profunda ou wrapper compartilhado dedicado.

Isso e coerente com a linguagem, mas e dificil de manter em C.

Riscos:

- `rc` nao e atomic para valores comuns.
- `retain/release` depende de disciplina manual correta no emitter e runtime.
- FFI e callbacks tornam ownership mais delicado.
- Payloads gerenciados em optional/result, enums, maps, sets e estruturas aninhadas ainda exigem cobertura cuidadosa.
- O numero alto de `free`, `malloc`, `calloc`, `realloc`, `snprintf` e caminhos de cleanup aumenta o custo de revisao.

### Modelo de tipos

O sistema de tipos ja e substancial:

- Tipagem estatica.
- Tipos explicitos para locals como decisao de linguagem.
- Generics com subconjunto executavel.
- Traits e `apply`.
- `any<Trait>` como dispatch dinamico object-safe.
- Optional/result.
- Tuples.
- Enums com payload.
- Contratos `where`.
- Restricoes para FFI.

Ponto forte:

- O design atual evita inferencia excessiva e tenta preservar legibilidade.

Ponto fraco:

- O arquivo `checker.c` concentra logica demais.
- A combinacao de generics, traits, any, closures, optional/result e managed values exige um contrato de IR muito forte.
- Algumas superficies publicas sao "typed facade", mas a runtime executavel ainda e mais estreita, por exemplo concorrencia com payload `int`.

### IR: HIR e ZIR

O projeto tem duas IRs:

- HIR: mais proxima da linguagem, depois do AST.
- ZIR: contrato para backend.

Isso e o caminho certo para um backend Rust.

O problema principal:

- ZIR ainda contem restos textuais usados pelo C backend.
- `compiler-model.md` afirma que ZIR textual deve ser usado para debug, fixtures e golden tests, nao como contrato principal do backend.
- O emissor C ainda parseia strings em varios caminhos.

Conclusao:

Antes de um backend Rust serio, o ZIR estruturado precisa ficar forte o bastante para que um backend nao tenha de adivinhar semantica a partir de texto.

### Diagnosticos

Diagnosticos sao uma forca do projeto.

Evidencias:

- Codigos estaveis em `diagnostic-code-catalog.md`.
- `zt explain`.
- Perfis de diagnostico.
- Formato action-first.
- Acessibilidade cognitiva documentada.
- Testes de erro negativo na suite.

Esse investimento deve ser preservado em Rust.

Regra importante:

Um backend Rust pode variar em texto de diagnostico so quando a suite permitir, mas deve preservar codigo estavel, severidade, estagio e span.

### Acoplamentos excessivos

Principais acoplamentos:

- `driver_internal.h` conhece frontend, HIR, semantica, backend C, ZIR e runtime.
- `pipeline.c` mistura orquestracao, stdlib discovery, runtime cache, native compile e process execution.
- `emitter.c` mistura mapeamento de tipos, ownership, FFI, optional/result, any, closures, output C e hacks de compatibilidade.
- `checker.c` mistura catalogo de modulos, resolucao de tipos, traits, calls, generics, diagnostics e regras de transferencia.
- `lsp.c` e grande e tende a duplicar conhecimento de linguagem/tooling.

Isso nao torna a base ruim. Ela esta funcionando. Mas torna a evolucao mais cara.

## Dificuldades que parecem vir de C

Problemas que C agrava:

- Ownership e cleanup dependem de disciplina manual.
- Falhas de alocacao precisam ser tratadas em muitos pontos.
- Strings e buffers exigem `snprintf`, tamanho fixo e checagem constante.
- Union/struct manual para AST/HIR/ZIR aumenta risco de campo errado.
- Falta de tipos soma nativos deixa optional/result/enum/payloads mais verbosos.
- Erros em emissao de C so aparecem tarde, muitas vezes no compilador nativo.
- Testar componentes internos exige mais harness C.

Problemas que nao sao "culpa do C":

- ZIR ainda nao e totalmente estruturado para backend.
- Algumas regras semanticas sao complexas por decisao da linguagem.
- Generic runtime, `any`, closures, FFI e managed values sao dificeis em qualquer linguagem.
- Um backend novo precisa de uma suite diferencial forte, mesmo se for Rust.
- A runtime precisa de decisoes sobre ciclos, payloads genericos e thread safety.

## Viabilidade de um backend em Rust

### Grau tecnico necessario

Alto.

Nao porque Rust seja inadequado, mas porque Zenith ja tem muitos recursos:

- generics e monomorfizacao;
- managed values;
- optional/result;
- closures;
- any dispatch;
- FFI;
- stdlib runtime-backed;
- diagnostics com spans;
- runtime checks;
- source mapping;
- conformance suite.

### O que pode ser traduzido diretamente

Bom candidato para Rust:

- Modelos de AST/HIR/ZIR como enums e structs tipados.
- ZIR verifier.
- Buffer de emissao.
- Name mangling.
- Type mapping.
- Legalization pre-emissao.
- Serializacao de ZIR.
- Runner de conformance/diferencial.
- Catalogos de diagnostics.
- Partes de runtime com ownership local claro, como texto, bytes, optional/result e collections genericas.

### O que deve ser redesenhado

Precisa de redesenho, nao traducao linha-a-linha:

- Dependencia de `expr_text` no backend.
- Funcoes grandes do emissor.
- Runtime ABI para managed payloads genericos.
- FFI shield.
- Callback ABI com closures capturadas.
- Thread boundary e atomic/shared wrappers.
- API interna de diagnosticos, para preservar spans sem copiar string demais.
- Driver/pipeline, se a meta for um compilador Rust completo.

### Coexistencia C/Rust via FFI

Coexistencia e viavel.

Formas possiveis:

1. Rust como processo externo:
   - C compiler gera ZIR serializado.
   - Rust backend le o arquivo.
   - Rust backend emite C.
   - Menor risco inicial.

2. Rust como staticlib/dylib:
   - C driver chama funcoes Rust por ABI C.
   - Mais rapido e integrado.
   - Mais dificil no Windows e no build/distribuicao.

3. Runtime Rust com ABI C:
   - Generated C chama simbolos `zt_*`.
   - Modulos Rust exportam `extern "C"` com `no_mangle`.
   - Exige contrato de layout, ownership e panic muito rigido.

Melhor primeiro passo:

Usar Rust como processo externo para backend experimental. Depois integrar por ABI se o ganho compensar.

### Riscos tecnicos de migracao

| Risco | Impacto | Mitigacao |
|---|---:|---|
| Backend Rust interpretar ZIR textual de modo diferente | Alto | Fechar ZIR estruturado antes |
| Divergencia de comportamento em managed values | Alto | Teste diferencial C vs Rust |
| Divergencia de diagnostics | Medio/alto | Exigir codigo estavel e span equivalente |
| Build mais complexo no Windows | Medio | Comecar com CLI Rust externa |
| Runtime Rust com ABI instavel | Alto | Migrar modulos pequenos e manter C oracle |
| Full rewrite perder cobertura implicita do C | Alto | Nao fazer full rewrite agora |
| Performance pior por wrappers iniciais | Medio | Medir com benchmarks existentes |

## Melhor approach tecnico em Rust

### Arquitetura recomendada

Proposta inicial:

```text
crates/
  zenith-ir/
    ZIR estruturado, tipos, spans, serializacao
  zenith-diagnostics/
    codigos, severidade, render data, spans
  zenith-backend-api/
    traits comuns para backend
  zenith-backend-c-rs/
    backend Rust que emite C
  zenith-conformance/
    runner diferencial C vs Rust
  zenith-runtime-rs/
    runtime Rust experimental, por ABI C
tools/
  zt-rs-backend/
    CLI: le ZIR e emite C
```

Fluxo recomendado no primeiro marco:

```text
zt C compiler
  -> emit-zir estruturado
  -> zt-rs-backend
  -> generated.c
  -> gcc/clang + runtime C
  -> executable
```

Esse fluxo evita trocar frontend, checker e runtime ao mesmo tempo.

### Contratos internos

O backend Rust deve depender de:

- ZIR estruturado, nao texto.
- Tipos ja resolvidos.
- Spans preservados.
- ABI de runtime declarada em um manifesto.
- Conformance matrix como contrato executavel.

O backend Rust nao deve depender de:

- Parsing de `expr_text`.
- Heuristica de string para saber tipo.
- Nomes C internos que vazam para ZIR.
- Comportamento implicito do compilador C.

### Ownership em Rust

Sugestao:

- AST/HIR/ZIR: `Box`, `Vec`, `String`, enums e IDs estaveis.
- Interner: usar IDs, nao `&str` soltos em tudo.
- Diagnostics: spans por valor, mensagens pequenas ou catalogadas.
- Backend: `Result<T, BackendError>`, sem panics para erro esperado.
- Runtime Rust: encapsular ponteiros crus em tipos pequenos e auditados.
- FFI: todo `unsafe` deve ficar em modulos `ffi` pequenos.

Regra de design:

Rust deve reduzir a superficie `unsafe`, nao apenas mover o mesmo risco de C para `unsafe`.

## Design da linguagem

### Pontos fortes

O design atual tem boas decisoes para legibilidade e manutencao:

- Tipos locais explicitos.
- Sem `null`; ausencia usa `optional<T>`.
- Falhas recuperaveis usam `result<T,E>`.
- `panic` e fatal, nao controle de fluxo comum.
- Mutacao e visibilidade sao explicitas.
- Imports qualificados reduzem surpresa.
- `any<Trait>` e restrito a dispatch object-safe.
- FFI e explicitamente marcado.
- Public docs e specs separam contrato final, subconjunto executavel e futuro.

Isso combina bem com o objetivo de acessibilidade cognitiva.

### Pontos de atencao

O design tambem tem complexidade real:

- Muitos simbolos fortes: `?`, `|>`, `..`, `<T>`, `{}`.
- `{}` tem multiplos papeis: map, set, struct.
- Generics, traits, `any`, closures e FFI interagem de forma dificil.
- Algumas superficies publicas sao finais, mas a runtime atual ainda tem subconjuntos.
- O backend C precisa de muitos helpers especializados.
- A linguagem depende muito de diagnostics bons para continuar acessivel.

Conclusao:

A linguagem e promissora, mas precisa de uma especificacao executavel e de uma suite de conformidade antes de qualquer backend alternativo ser chamado de real.

### Sintaxe

Estado atual:

- A sintaxe esta mais coerente depois do contrato final.
- A decisao de remover `group`, `fmt"..."`, `given`, `unless`, variadics e C-style `for` reduz ambiguidade.
- A exigencia de tipos explicitos ajuda o parser, o checker e o leitor.

Risco:

- A densidade de simbolos ainda exige guias, formatter e diagnosticos muito bons.

Recomendacao:

- Manter o contrato sintatico congelado.
- Nao adicionar novas abreviacoes durante a migracao para Rust.
- Usar a migracao para fortalecer o parser e o formatter, nao para ampliar a linguagem.

### Semantica

Partes bem encaminhadas:

- Escopo, imports, mutabilidade, optional/result, panic, `using`, match e any tem documentos.
- Ordem de avaliacao e cleanup sao tratados como contratos.

Partes que exigem cuidado para Rust:

- Cleanup em `return`, `?`, panic e loop control.
- Managed values em payloads.
- FFI shield.
- Concurrency payloads.
- Any object safety.
- Monomorfizacao e limites.

### Sistema de tipos

Classificacao:

- Zenith e majoritariamente estatico.
- Ha dispatch dinamico controlado via `any<Trait>`.
- Ha generics e traits.
- Inferencia e limitada por design.

Isso e bom para backend Rust.

O desafio nao e "descobrir tipos"; e transportar tipos resolvidos ate ZIR/backend sem cair em texto ou heuristica.

### Modelo de memoria

Contrato atual:

- Usuario ve semantica de valor.
- Implementacao pode usar RC/ORC, clone, copy-on-write e moves internos.
- Nao ha keywords de ownership na linguagem.
- Runtime comum e single-isolate.
- Cross-thread exige copia profunda ou wrapper dedicado.

Rust ajuda muito aqui, mas nao resolve tudo sozinho.

Se a runtime Rust exportar ABI C, ainda havera:

- ponteiros crus;
- layout C;
- ownership em fronteira;
- `unsafe`;
- regras de panic atraves de FFI;
- compatibilidade com generated C.

### Modelo de execucao

Hoje Zenith compila para C e depois para executavel nativo.

Runtime e necessario para:

- texto, bytes e colecoes;
- optional/result especializado;
- panic e runtime errors;
- checks;
- stdlib host-backed;
- IO, fs, os, process, net, http;
- any/dyn dispatch;
- closures/lazy/concurrency;
- Borealis.

Nao ha evidencia de uma VM bytecode como caminho atual.

### Diagnosticos e acessibilidade

Este e um diferencial do projeto.

O backend Rust deve preservar:

- codigos estaveis;
- spans;
- formato action-first;
- perfis de diagnostico;
- mensagens pequenas;
- sugestoes acionaveis;
- `zt explain`.

Para TDAH e dislexia, a migracao nao pode piorar a saida do compilador. A prioridade e manter mensagens curtas, consistentes e com proximo passo claro.

### Especificacao formal

Ha muitos documentos bons, mas a especificacao ainda precisa virar contrato executavel para backend.

Antes de marcar Rust como backend real, e necessario:

- formato estruturado de ZIR;
- fixtures golden de ZIR;
- matriz de conformidade por feature;
- fixtures negativas com codigos de diagnostico;
- testes de runtime para ownership;
- testes diferenciais C vs Rust;
- regras de variancia aceita.

## Comparacao C vs Rust

| C atual | Rust proposto |
|---|---|
| Base ja funcional e validada | Mais seguro para evoluir, mas precisa provar paridade |
| Toolchain simples para quem ja tem GCC | Cargo adiciona dependencia nova |
| Controle baixo nivel direto | Controle baixo nivel com `unsafe` localizado |
| Mais risco de memory bugs | Menos risco por ownership e tipos soma |
| Strings/buffers manuais | `String`, `Vec`, `Result`, enums |
| Mais facil integrar com C gerado | Precisa ABI bem definida |
| Runtime C ja e oraculo | Runtime Rust exige prova de equivalencia |
| Backend C grande e dificil de refatorar | Backend Rust pode nascer modular |
| Debug de generated C ja conhecido | Rust backend precisa observabilidade nova |

Conclusao:

Rust e melhor para o proximo backend e para a evolucao da runtime. C deve continuar como oraculo enquanto Rust prova equivalencia.

## Estrategias avaliadas

### 1. Manter C e apenas refatorar

Vantagens:

- Menor risco imediato.
- Aproveita a suite atual.
- Nao muda toolchain.
- Pode reduzir arquivos grandes.

Desvantagens:

- Continua com ownership manual.
- Nao elimina a fragilidade de strings/buffers.
- Pode consumir muito tempo sem abrir caminho para backends modernos.

Quando faz sentido:

- Para fechar ZIR estruturado.
- Para reduzir `expr_text`.
- Para separar emissor em modulos menores.

### 2. Criar backend Rust incremental

Vantagens:

- Melhor equilibrio risco/beneficio.
- Mantem frontend/checker C.
- Usa C backend como oraculo.
- Permite testes diferenciais.
- Cria base para LLVM/WASM depois.

Desvantagens:

- Exige serializacao de ZIR.
- Exige runner de conformidade.
- Inicialmente ainda depende do runtime C.

Quando faz sentido:

- Agora, depois de fechar melhor o contrato de ZIR.

### 3. Reescrever partes criticas em Rust

Vantagens:

- Pode atacar ownership, optional/result e collections.
- Reduz risco de memory bugs em componentes isolados.

Desvantagens:

- FFI pode introduzir novo risco.
- Build e distribuicao ficam mais complexos.
- Sem contrato ABI rigido, pode virar acoplamento novo.

Quando faz sentido:

- Depois de uma prova pequena via ABI C, por exemplo um modulo runtime bem delimitado.

### 4. Criar runtime Rust separada

Vantagens:

- Ataca o ponto onde Rust tem mais valor.
- Pode manter generated C chamando simbolos `zt_*`.
- Permite migracao modulo a modulo.

Desvantagens:

- Runtime e o ponto mais sensivel.
- ABI, layout e ownership precisam ser precisos.
- Panic Rust nao pode cruzar FFI sem regra explicita.

Quando faz sentido:

- Depois que o backend Rust emitindo C ja passar a suite basica.

### 5. Arquitetura hibrida C/Rust

Vantagens:

- Caminho mais realista.
- C continua oraculo.
- Rust entra onde traz ganho claro.

Desvantagens:

- Duas toolchains.
- Pontos FFI precisam de revisao.
- Mais complexidade de CI.

Quando faz sentido:

- Recomendado como caminho principal.

### 6. Reescrita total

Vantagens:

- Arquitetura limpa desde o inicio.
- Rust pode modelar a linguagem de forma muito mais segura.

Desvantagens:

- Muito alto risco.
- Longo tempo ate paridade.
- Perda de comportamento implícito validado pela suite atual.
- Muitas features atuais precisariam ser reimplementadas juntas.

Quando faz sentido:

- So depois de backend/runtime Rust parciais provarem valor.

## Plano recomendado

### Marco 0: congelar o oraculo C

Objetivo:

Ter uma base verde e reproduzivel antes de qualquer backend alternativo.

Tarefas:

- Registrar comandos oficiais de gate.
- Fixar relatorios de smoke/nightly.
- Garantir que `post-v1-backend-conformance-suite.md` seja a regra de entrada.
- Separar testes por classes: parse, semantica, runtime, diagnostics, ZIR, generated C.

Criterio de sucesso:

- C backend passa smoke/nightly.
- Relatorio do oraculo C fica facil de comparar.

### Marco 1: fechar ZIR estruturado

Objetivo:

Remover dependencia de backend em texto.

Tarefas:

- Inventariar todos os usos de `expr_text` e `init_expr_text`.
- Trocar caminhos do emissor para `zir_expr`.
- Adicionar verifier para impedir nomes alvo-especificos.
- Expandir golden ZIR fixtures.
- Criar output ZIR estruturado estavel, possivelmente JSON ou formato proprio versionado.

Criterio de sucesso:

- Backend C nao precisa parsear expressao textual para comportamento principal.
- Textual ZIR fica restrito a debug/golden.

### Marco 2: backend Rust que emite C

Objetivo:

Provar que Rust consegue consumir ZIR e produzir comportamento equivalente sem mexer na runtime.

Tarefas:

- Criar `crates/zenith-ir`.
- Criar `crates/zenith-backend-c-rs`.
- Criar CLI `zt-rs-backend`.
- Emitir C para subconjunto pequeno: literals, functions, calls, if, result/optional simples.
- Comparar generated C e comportamento com C backend.

Criterio de sucesso:

- Passa um subset de smoke.
- Falhas sao classificadas por feature faltante, nao por divergencia desconhecida.

### Marco 3: suite diferencial

Objetivo:

Comparar C backend e Rust backend em comportamento observavel.

Classes de teste:

- run-pass;
- run-fail;
- check-pass;
- check-fail;
- diagnostics code;
- runtime panic;
- memory/ownership;
- FFI;
- any dispatch;
- closures;
- generics;
- stdlib runtime-backed;
- generated C warnings.

Criterio de sucesso:

- O Rust backend so e considerado real quando passa a suite obrigatoria e documenta variancias aceitas.

### Marco 4: runtime Rust parcial

Objetivo:

Migrar a runtime onde Rust reduz risco real.

Ordem recomendada:

1. Diagnostics/runtime error formatting.
2. Text/bytes.
3. Optional/result wrappers.
4. Generic collections.
5. Host API simples.
6. Any/vtable.
7. FFI shield.
8. Concurrency/shared/atomic.
9. Borealis e modulos pesados.

Criterio de sucesso:

- Cada modulo Rust exporta ABI C.
- Cada modulo tem teste C/Rust.
- Nenhum panic Rust cruza FFI.

### Marco 5: avaliar frontend Rust

Objetivo:

Decidir se vale migrar parser/checker.

So fazer quando:

- backend Rust ja passa a conformidade;
- ZIR e runtime ABI estao fechados;
- diagnostics tem contrato reproduzivel;
- a equipe aceita o custo de reimplementar o checker.

## O que atacar primeiro

Prioridade tecnica:

1. `expr_text` e `init_expr_text` no backend.
2. `emitter.c` modularizado por dominio.
3. ZIR estruturado serializavel.
4. Backend conformance runner.
5. Runtime ABI manifest mais forte.
6. Rust backend experimental emitindo C.
7. Runtime Rust por modulos pequenos.

Nao atacar primeiro:

- Reescrita total do parser.
- Reescrita total do checker.
- Runtime Rust completa.
- LLVM backend direto.
- Mudancas novas de sintaxe durante a migracao.

## Conclusao

Sim, vale a pena desenvolver um caminho Rust.

Conditicoes:

- C backend permanece oraculo.
- ZIR vira contrato estruturado real.
- Rust entra primeiro como backend incremental.
- Paridade e medida por suite diferencial, nao por impressao.
- Runtime Rust entra modulo a modulo, por ABI C.

Problemas que Rust resolveria bem:

- reduziria risco de memoria no backend novo;
- deixaria IR/backend mais tipados;
- melhoraria modularidade;
- facilitaria testes unitarios;
- reduziria fragilidade de buffers e strings;
- abriria caminho para LLVM/WASM no futuro.

Problemas que Rust nao resolve automaticamente:

- regra semantica ambigua;
- ZIR textual usado como contrato;
- conformance incompleta;
- FFI insegura;
- ciclo de RC;
- thread safety de valores gerenciados;
- diagnostico ruim;
- complexidade de linguagem.

Melhor approach final:

Criar uma arquitetura hibrida e incremental:

```text
C frontend/checker/oracle
  -> ZIR estruturado
  -> Rust backend experimental
  -> C output
  -> runtime C
  -> suite diferencial
  -> runtime Rust parcial por ABI C
```

Esse caminho preserva o que ja funciona, cria uma base Rust real e evita o maior risco: trocar compilador, backend e runtime ao mesmo tempo.
