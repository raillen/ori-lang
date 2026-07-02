# Plano de uso real — projetos pequenos e médios

> **Status:** ativo
> **Data:** 2026-07-01
> **Escopo:** levar Ori a 100% de usabilidade prática para projetos pequenos e médios.
> **Não é:** promessa de `1.0`, self-hosting, registry público completo ou linguagem universal.

Este plano é a fonte de verdade para a próxima fase de maturidade da linguagem.
Ele substitui o uso do `PLANO-MATURIDADE-COMPLETO.md` como backlog ativo.

Use este documento quando a pergunta for:

- "o que falta para usar Ori em projetos reais pequenos?"
- "qual é o próximo bloco de implementação?"
- "quais decisões ainda podem quebrar sintaxe ou semântica?"
- "o que precisa estar pronto antes de usuários externos dependerem da linguagem?"

## Definição de 100%

"100%" aqui significa uma linguagem usável, previsível e distribuível para projetos
pequenos e médios.

Um usuário deve conseguir:

1. instalar Ori sem precisar clonar o repositório;
2. criar um projeto com estrutura clara;
3. escrever código com sintaxe e semântica estáveis;
4. usar uma stdlib suficiente para tarefas comuns;
5. rodar `check`, `run`, `test`, `fmt`, `doc` e LSP sem ajustes manuais frágeis;
6. empacotar ou distribuir um binário simples;
7. entender erros comuns sem ler o código do compilador.

Isso **não** exige ainda:

- compilador self-hosted;
- registry público completo com governança;
- ABI final de longo prazo;
- zero breaking changes por seis meses;
- ecossistema grande de terceiros.

Esses pontos continuam sendo critérios de `1.0`.

## Matriz de maturidade

| Área | Estado atual | Meta de 100% | Gate de aceitação |
| --- | --- | --- | --- |
| Sintaxe e modelo mental | Contrato central auditado e sincronizado em spec/docs locais. | Sintaxe central congelada e explicada com exemplos curtos. Divergências documentais removidas. | Suite de conformidade cobre formas principais; spec/fixtures locais não se contradizem. |
| Type checker, diagnósticos e runtime nativo | Checker, runtime, JIT/AOT, leak-check e smoke empacotado têm gates verdes locais. | Diagnósticos previsíveis, runtime sem vazamentos conhecidos em casos pequenos/médios, execução empacotada confiável. | `cargo test --workspace`, leak-check, smoke de release e catálogo de diagnósticos verdes. |
| Stdlib para programas reais pequenos | Kit mínimo de CLI, arquivos, JSON, tempo, logging, config, testes, processos e coleções está documentado e testado. | Kit mínimo para CLI, arquivos, JSON, tempo, logging, config, testes e processos. | 5 exemplos reais compilam, rodam e testam usando só pacote instalado + stdlib empacotada. |
| Tooling local | CLI, LSP, formatter, doc, REPL e extensão têm smoke externo e regressões. | Fluxo local completo: criar, checar, formatar, testar, documentar, inspecionar ambiente e usar LSP. | Smoke de projeto novo + VS Code extension + formatter idempotente + doc HTML. |
| Ecossistema, distribuição e terceiros | Pacotes locais por path, cache, resolvedor de imports e release package por host/CI estão fechados. | Projeto pode declarar dependências locais, ser empacotado e instalado por terceiros. | `ori new`, manifest, path deps, cache local e installer validados em projeto fora do repo. |

## Princípios de decisão

Estas regras evitam crescimento confuso:

1. **Preferir estabilidade do núcleo.** Sintaxe nova só entra se resolver dor real e tiver gate de regressão.
2. **Não esconder breaking change.** Se mudar contrato público, registrar migração e decidir se justifica `0.3.0`.
3. **Stdlib primeiro para uso real.** Priorizar tarefas comuns de CLI, arquivos, JSON, tempo, config e testes.
4. **Ferramenta local antes de registry remoto.** Primeiro `ori new`, manifest e dependência por caminho; depois registry hospedado.
5. **Docs seguem implementação.** Spec é normativa; planning é plano; `_reversa_sdd/` é histórico.

## Fase 1 — Sintaxe e modelo mental da linguagem

**Objetivo:** congelar a experiência central da linguagem antes de expandir o ecossistema.

### Entregas

- [x] Criar um contrato de estabilidade da linguagem em `docs/spec/`.
- [x] Auditar `docs/spec/`, site/export e fixtures contra a implementação atual.
- [x] Corrigir divergências de sintaxe já identificadas:
  - literais inteiros com largura explícita usam sufixo no literal, como `12i8`;
  - instanciação de structs deve refletir a forma implementada e recomendada;
  - variantes de enum devem usar a forma realmente aceita pelo parser;
  - exemplos antigos com chaves, `let` ou `=>` devem sair de docs ativas.
- [x] Decidir e registrar se `.Variant{...}` será removido da spec/site ou implementado.
- [x] Formalizar a forma curta de construção de dados, por exemplo quando `Book(...)` pode ser inferido ou abreviado.
- [x] Atualizar a EBNF depois da auditoria.
- [x] Criar uma suite de conformidade para sintaxe/semântica central.

### Recomendação atual

Manter o contrato pequeno:

- literal tipado por sufixo: `12i8`, `42u64`, `3.14f32`;
- tipo declarado no binding quando o dado precisa carregar intenção pública;
- construção nominal clara para structs;
- atalhos só quando o type checker já tem contexto inequívoco.

Evitar adicionar `.Variant{...}` agora, a menos que a implementação e a documentação
mostrem ganho claro de legibilidade. Se a forma não existir hoje, remover da spec/site
é menos arriscado que expandir sintaxe antes do congelamento.

### Gate

- [x] `cargo test -p ori-driver --test ori_spec`
- [x] testes de parser/checker para exemplos canônicos;
- [x] busca documental sem exemplos obsoletos;
- [x] site/export atualizado após a spec.

## Fase 2 — Type checker, diagnósticos e runtime nativo

**Objetivo:** fazer o compilador falhar bem e executar com confiança em programas reais pequenos.

### Entregas

- [x] Revisar gaps deferidos do catálogo de diagnósticos e decidir o que entra no ciclo atual:
  - `contract.*` runtime-only;
  - `generic.ambiguous_type_arg`;
  - `match.guard_not_exhaustive`.
- [x] Corrigir o caso conhecido de `await` em loops profundamente aninhados. — `compile_runs_async_await_in_deeply_nested_bodies_native` agora roda na suite normal; o backend recarrega valores do frame apos retomada e evita reutilizar valores de blocos nao-dominantes.
- [x] Fortalecer leak-check e stress tests de ARC/cycle collector.
- [x] Revisar contratos de ownership no runtime/FFI.
- [x] Padronizar mensagens de erro de build/link/JIT/AOT.
- [x] Garantir que `ori run` JIT e `ori compile` AOT tenham smoke empacotado.
- [x] Documentar limites de cada backend: nativo, JIT e C debug.

### Gate

- [x] `cargo check --workspace`
- [x] `cargo test --workspace`
- [x] `cargo test -p ori-driver --test diagnostic_catalog`
- [x] `ORI_TEST_LEAK_CHECK=1` em testes de memória relevantes;
- [x] smoke de release com runtime empacotado;
- [x] smoke JIT e AOT fora do checkout do repositório.

## Fase 3 — Stdlib para programas reais pequenos

**Objetivo:** permitir programas úteis sem dependências externas e sem bindings manuais.

### Kit mínimo

- [x] `fs`: arquivos, diretórios, metadados, cópia, remoção, caminhos.
- [x] `io`: stdin/stdout/stderr, leitura incremental, escrita incremental.
- [x] `path`: normalização, join, basename, dirname, extensão, relativo.
- [x] `json`: parse, stringify, leitura/escrita de arquivo JSON.
- [x] `env`: variáveis de ambiente.
- [x] `process`: execução de comando e captura de saída.
- [x] `net`: TCP cliente, TLS (`connect_tls`), servidor (`listen`/`accept`), UDP síncrono; I/O blocking com `task.run_blocking` em código async.
- [x] `time`: `Instant`, `Duration`, conversões e medição.
- [x] `random`: geração determinística e não determinística.
- [x] `string`, `bytes`, `list`, `map`, `set`: helpers estáveis e documentados.
- [x] `test`: assertions, skip, fixtures simples.
- [x] `log`: logging mínimo para CLI (`error_message` evita a keyword `error`).
- [x] `config`: padrão simples para config local via texto/JSON.
- [x] `args`: helpers básicos de argumentos de CLI.

### Entregas prioritárias

- [x] Implementar `time.Instant` e `Duration`.
- [x] Projetar `io.Input` e `io.Output` antes de codar streams — ver `docs/planning/io-streams-design.md`.
- [x] Criar exemplos reais:
  - organizador de arquivos;
  - validador de JSON;
  - analisador de logs;
  - CLI de tarefas;
  - runner simples de processos;
  - `examples/http_get.orl` (GET HTTPS mínimo).
- [x] Garantir que exemplos usem apenas stdlib empacotada.
- [x] Marcar APIs experimentais quando ainda não forem contrato estável.

### Fora do alvo imediato

- rede async nativa (`net.*_async` integrado ao executor);
- TLS avançado (certificados customizados, pinning);
- UDP avançado (multicast);
- drivers de banco de dados;
- framework web.

TLS cliente, UDP datagramas e servidor TCP **síncronos** já estão no kit mínimo
(`ori.net`, `docs/planning/net-v2-design.md`). Os itens acima permanecem backlog
e não bloqueiam o 100% de projetos pequenos/médios.

### Gate

- [x] 5 exemplos reais em `examples/` com `check`, `run` e `test` quando aplicável;
- [x] testes E2E em `compiler/crates/ori-driver/tests/`;
- [x] docs do cap. 12 sincronizadas com manifesto, `.orl` e runtime;
- [x] `CHANGELOG.md` atualizado para APIs públicas.

## Fase 4 — Tooling local

**Objetivo:** tornar Ori confortável de usar sem conhecimento interno do repositório.

### CLI

- [x] Fechar semântica de `ori build` conforme `c-backend-redefinition.md` — `ori build` usa a rota nativa; C debug fica em `ori emit c`.
- [x] Implementar `ori new` — cria `ori.proj`, `src/main.orl` ou `src/lib.orl`, e `docs/api` sem sobrescrever diretorio nao vazio.
- [x] Implementar `ori repl` com recorte minimo:
  - literais;
  - expressões;
  - chamadas stdlib;
  - `const`/`var` simples;
  - mensagens de erro legíveis.
- [x] Melhorar `ori test` para filtros, nomes de teste e saída mais escaneável. — `ori test <arquivo> --filter <texto>` seleciona por nome completo ou curto e informa descobertos/selecionados.
- [x] Consolidar `ori doc file` e `ori doc export`.
- [x] Validar `ori doc file --format html` em projeto real.

### Formatter

- [x] Corrigir bug conhecido de formatação de `trait`.
- [x] Garantir idempotência em:
  - async;
  - match;
  - traits;
  - generics;
  - imports seletivos;
  - construção de dados.

### LSP e editor

- [x] Empacotar `ori-lsp` junto com release.
- [x] Fechar smoke da extensão VS Code fora do workspace de desenvolvimento.
- [x] Garantir hover/goto/completion para stdlib empacotada.
- [x] Melhorar diagnósticos de projeto com ações sugeridas quando possível.

### Gate

- [x] criar projeto novo fora do repo;
- [x] abrir no VS Code;
- [x] rodar check/run/test/fmt/doc;
- [x] confirmar LSP funcional;
- [x] executar tudo sem `ORI_STDLIB_ROOT` manual.

## Fase 5 — Ecossistema, distribuição e terceiros

**Objetivo:** permitir que outra pessoa use Ori sem depender do estado interno deste checkout.

### Pacotes

- [x] Fechar formato inicial `ori.pkg.toml`.
- [x] Implementar parser e validacao do manifest.
- [x] Implementar dependencias locais por caminho.
- [x] Criar cache local de pacotes.
- [x] Definir lockfile ou registrar explicitamente por que sera adiado. — Adiado ate resolver registry remoto; path deps usam o proprio `ori.pkg.toml` como fonte explicita.
- [x] Implementar `ori install` com semantica real para pacotes locais (`--path`).
- [x] Manter `ori publish` como stub validado até existir registry remoto; fluxo local está sólido sem upload.

### Distribuição

- [x] Gerar pacote de release para Windows, Linux e macOS via matriz CI `native-route`.
- [x] Criar script de empacotamento por host apos smoke (`tools/package_native_release.ps1` / `.sh`).
- [x] Incluir `ori`, `ori-lsp`, stdlib, runtime e exemplos.
- [x] Incluir smoke automatizado para pacote isolado.
- [x] Documentar instalação e upgrade.
- [x] Definir política de compatibilidade para `0.2.x`.

### Adoção

- [x] Criar cookbook curto com tarefas reais.
- [x] Criar guia "primeiro projeto".
- [x] Criar guia "migrando entre versões 0.x".
- [x] Publicar exemplos pequenos com testes.
- [x] Definir como reportar bugs de linguagem, stdlib e tooling.

### Gate

- [x] usuário instala Ori a partir de pacote;
- [x] roda `ori new`;
- [x] adiciona dependência local;
- [x] roda `ori check`, `ori test`, `ori run`;
- [x] gera docs;
- [x] empacota projeto;
- [x] não precisa clonar `ori-lang`.

## Decisões que devemos fechar juntos

| Decisão | Recomendação | Motivo |
| --- | --- | --- |
| Forma de variantes `.Variant{...}` | Remover da spec/site por enquanto, se não estiver implementada. | Menor risco antes do congelamento sintático. |
| Forma curta de construção de structs | Permitir só com contexto de tipo inequívoco. | Mantém leitura explícita e evita inferência mágica. |
| `ori build` | Tornar nativo/Cranelift o caminho principal; mover C para `ori emit c` ou equivalente. | O backend C não deve parecer backend de produção se for debug/parcial. |
| Registry | Fazer path deps e cache local antes de registry hospedado. | Usuário real ganha valor antes de infraestrutura pública. |
| Rede avançada | TLS/UDP/servidor TCP síncronos entregues; async nativo e TLS avançado ficam no backlog. | CLI, arquivos, JSON e processos cobrem a maioria dos casos iniciais; HTTP(S) básico já é possível. |
| Breaking changes | Só promover para `0.3.0` quando houver quebra real e documentada. | Mantém a política `0.2.x` honesta. |

## Sequência recomendada

### Sprint 1 — Congelar contrato e limpar divergências

- [x] Auditar spec/docs/examples contra parser/checker.
- [x] Corrigir docs sobre literais, structs, enum variants e atalhos.
- [x] Criar contrato de estabilidade.
- [x] Criar primeiros testes de conformidade.

### Sprint 2 — Runtime/checker confiáveis

- [x] Resolver decisão sobre `await` em loops aninhados.
- [x] Fortalecer leak-check.
- [x] Melhorar diagnósticos de backend e runtime.
- [x] Validar JIT/AOT em pacote isolado.

### Sprint 3 — Stdlib mínima de uso real

- [x] Fechar `time.Instant`/`Duration`.
- [x] Projetar streams.
- [x] Implementar `log`, `args` e config mínima se ainda não existirem.
- [x] Criar exemplos reais com testes.

### Sprint 4 — Tooling local completo

- [x] `ori new`.
- [x] `ori repl`.
- [x] `ori build` redefinido.
- [x] Formatter completo para traits/generics/imports.
- [x] LSP/extensão em smoke externo.

### Sprint 5 — Pacotes e distribuição

- [x] `ori.pkg.toml`.
- [x] path dependencies.
- [x] cache local.
- [x] installer/release package multiplataforma.
- [x] docs de instalação e migração.

## Decisões futuras sobre 1.0

> Esta seção registra os critérios, decisões arquiteturais e timeline para a versão `1.0` da linguagem Ori.  
> Ela é uma continuação natural deste plano: quando o "100% de usabilidade" for atingido, o foco shifta para maturidade de longo prazo.

### Definição de "independência do Rust" para 1.0

A independência total do Rust é entendida em **dois níveis distintos**:

1. **Independência para usuários finais (pré-requisito para 1.0):**  
   Um usuário que instala Ori via release package deve conseguir `check`, `run`, `test`, `compile`, `fmt`, `doc` e usar o LSP **sem ter `rustc`, `cargo` ou qualquer toolchain Rust instalada**. Isso já é parcialmente verdade (Phase 2 + 3 de Rust removal) e será fechado com `SystemLinker` como default + JIT como default para `ori run`.

2. **Self-hosting do compilador (não é pré-requisito para 1.0):**  
   O compilador pode continuar sendo escrito em Rust indefinidamente. Self-hosting é um *sinal* de maturidade, não um *critério* de utilidade. Python, Ruby, Lua, JavaScript — nenhuma foi self-hosted. Zig está em `0.14` após ~10 anos e ainda não é `1.0`. **Self-hosting será reconsiderada apenas quando houver usuários reais estáveis e recursos dedicados.**

### Critérios de 1.0 (6 itens)

| # | Critério | Status atual | O que falta | Estimativa |
|---|----------|--------------|-------------|------------|
| 1 | **Rust dependency removida para usuários finais** | Phase 1, 2, 3 completas; SystemLinker default implementado | Smoke em máquinas sem Rust instalado; CI job sem Rust; `docs/install.md` | Semanas |
| 2 | **Stdlib portada em `.orl` (Layer 2+3 substantivas)** | Layer 2/3 entregues; Layer 1 permanece Rust por design | Mais módulos Layer 2 cold-path (`format.utils`, `iter.utils`); trait gate para genéricos em map/set/graph | Meses |
| 3 | **Compiler self-hosting ou bootstrapping documentado** | Não iniciado | Adiado indefinidamente; bootstrapping documentado é alternativa aceitável | Anos (se self-hosting) / Semanas (se documentação) |
| 4 | **ABI estável documentada** | Parcial (FFI C existe, não formalizada) | Documentar layout de structs, calling convention, name mangling, versão ABI | Meses |
| 5 | **Usuários reais** | Zero conhecidos fora dos mantenedores | Primeiros projetos externos; feedback; casos de uso documentados | Imprevisível |
| 6 | **Sem breaking changes por ≥6 meses** | Não atingido | Congelar sintaxe e semântica por 6 meses após estabilização do contrato central | 6+ meses após fechamento do contrato |

### Decisões arquiteturais fechadas (irreversíveis até 1.0)

1. **Self-hosting adiado.** Não será iniciado antes de haver usuários reais e demanda comprovada. O modelo de distribuição binária (compilador pré-compilado + runtime empacotada) é suficiente para 1.0.
2. **Runtime Layer 1 permanece Rust.** ARC, async executor, FFI, I/O e rede são hot paths que beneficiam da memory safety do Rust. A ABI C é o contrato público; a implementação interna pode ser reescrita no futuro sem quebrar compatibilidade.
3. **SystemLinker é o default para AOT.** A partir de 2026-07-02, `NativeLinker::discover()` prefere o linker do sistema ao `rust-lld`. Isso elimina a dependência de um binário da toolchain Rust para compilação AOT.
4. **Modelo de 3 camadas da stdlib é permanente.** Layer 1 (Rust runtime, hot path), Layer 2 (safe wrappers `.orl`), Layer 3 (algoritmos puros `.orl`). A fronteira Layer 1/2/3 é o contrato de ABI.
5. **Versionamento congelado em `0.2.x`.** `0.3.0` só quando houver breaking change real que usuários precisem saber. Patch versions (`0.2.1`, `0.2.2`) para correções e small additive features. `1.0` é critério de maturidade, não marketing.

### Próximos passos táticos para 1.0

#### Fase A — Validação de independência (próximas semanas)
- [ ] Smoke em máquina Windows sem Rust instalado (apenas VS Build Tools)
- [ ] Smoke em máquina Linux sem Rust instalado (apenas build-essential)
- [ ] Smoke em máquina macOS sem Rust instalado (apenas Xcode CLT)
- [ ] CI job que valida release package em runner sem Rust toolchain
- [ ] Escrever `docs/install.md` com prereqs do sistema por OS

#### Fase B — Estabilização de contrato (próximos meses)
- [ ] Documentar ABI C completa (layout, calling convention, name mangling)
- [ ] Congelar sintaxe central (decisão já tomada; precisa de 6 meses sem breaking changes)
- [ ] Portar mais módulos Layer 2 cold-path para `.orl`
- [ ] Implementar trait gate `Hashable`+`Equatable` para genéricos em map/set/graph

#### Fase C — Adoção (imprevisível)
- [ ] Primeiros usuários externos reportando issues
- [ ] Projetos reais documentados em `examples/` ou repositórios externos
- [ ] Decisão sobre self-hosting baseada em evidência de adoção

### Timeline estimada

| Marco | Condição | Estimativa |
|-------|----------|------------|
| Fase A completa | Smoke sem Rust em 3 OSes + CI job verde | 2026-07 |
| Fase B completa | ABI documentada + 6 meses sem breaking changes | 2026-Q4 a 2027-Q1 |
| Fase C iniciada | Primeiros usuários reais | Imprevisível |
| **1.0** | Todos os 6 critérios atendidos | Anos, não dias |

---

## Critério de fechamento deste plano

Este plano estará completo quando:

- [x] as cinco áreas da matriz tiverem gate verde;
- [x] houver pelo menos 5 exemplos reais mantidos no repositório;
- [x] o pacote de release funcionar sem checkout do repo;
- [x] docs/site/spec não divergirem da implementação;
- [x] um usuário externo conseguir criar e manter um projeto pequeno com Ori.

Depois disso, o próximo plano deve ser de `1.0`, com foco em self-hosting,
ABI estável, estabilidade por tempo e usuários reais.
