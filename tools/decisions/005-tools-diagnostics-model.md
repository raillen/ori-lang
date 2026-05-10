# Decision 005 - Modelo de tools.diagnostics

- Status: draft
- Data: 2026-04-20
- Tipo: tools / diagnostics / cli / ux
- Escopo: diagnosticos de ferramentas, erros de CLI, mensagens de `zt.*`, integracao com manual/ztman

## Resumo

`tools.diagnostics` deve padronizar mensagens de erro, aviso e ajuda para ferramentas Zenith.

Ele nao substitui os diagnosticos do compilador.

Ele aplica a mesma filosofia das decisions da linguagem para ferramentas como:

1. `zt new`
2. `zt doctor`
3. `zt bench`
4. `zt explain`
5. `zt package`
6. futuras CLIs criadas com `tools.cli`

A regra principal:

```text
A mensagem deve dizer o que fazer, por que aconteceu e qual e o proximo passo.
```

## Base herdada da linguagem

Esta decision usa como base:

1. `docs/internal/decisions/language/038-diagnostics-and-warning-philosophy.md`
2. `docs/internal/decisions/language/039-diagnostic-rendering-and-error-codes.md`
3. `docs/internal/decisions/language/054-cli-conceptual-model.md`
4. `docs/spec/language/tooling-model.md`
5. o modelo de manual textual usado por `zman`/`ztman`

Da linguagem, herdamos estes principios:

1. mensagens reading-first
2. texto explicito
3. codigos estaveis
4. saida previsivel
5. sem depender de cor
6. ajuda acionavel
7. poucos erros por vez
8. suporte futuro a JSON, IDE e automacao

## Diferenca entre diagnostico da linguagem e diagnostico de tools

Diagnosticos da linguagem explicam problemas no codigo Zenith.

Exemplo:

```text
error[type.mismatch]
Expected int, but found text.
```

Diagnosticos de `tools.*` explicam problemas no uso de uma ferramenta.

Exemplo:

```text
error[cli.missing_option]
Missing required option --name.
```

Os dois devem parecer parte do mesmo produto.

Mas eles pertencem a dominios diferentes.

## Decisao

`tools.diagnostics` sera o modelo oficial para erros de ferramentas Zenith.

Ele deve ser usado por:

1. `tools.cli`, para erros de argumentos e comandos
2. `tools.output`, para renderizar erro humano, JSON e CI
3. `tools.prompt`, para falhas em modo nao interativo
4. `zt.*`, para comandos oficiais da linguagem
5. ferramentas futuras fora da stdlib

## Modelo mental

Todo diagnostico deve responder estas perguntas:

1. O que aconteceu?
2. O que fazer agora?
3. Por que isso aconteceu?
4. Qual e o proximo passo?
5. Onde posso ler mais?

Formato humano recomendado:

```text
error[cli.missing_option]
Missing required option --name.

ACTION
  informe --name

WHY
  o comando `zt new` precisa do nome do projeto

NEXT
  rode: zt new cli --name minha_ferramenta

HELP
  ztman explain cli.missing_option
```

## Por que manter ACTION, WHY e NEXT

Esse formato reduz carga mental.

A pessoa nao precisa inferir o que fazer.

Cada bloco tem uma funcao:

1. `ACTION`: acao imediata
2. `WHY`: motivo em linguagem simples
3. `NEXT`: comando ou passo seguinte

A ordem deve ser estavel.

Isso ajuda especialmente em:

1. erros de CLI
2. onboarding
3. usuarios iniciantes
4. pessoas com TDAH
5. pessoas com dislexia
6. leitura rapida em terminal

## Relacao com formato classico da linguagem

A linguagem usa formato como:

```text
error[type.mismatch]
Expected int, but found text.

where
  src/app/main.zt:8:21

help
  Convert the value explicitly or change the variable type.
```

`tools.diagnostics` pode usar `ACTION / WHY / NEXT` como camada mais orientada a tarefa.

Quando houver arquivo, trecho de codigo ou local exato, ele tambem pode usar blocos da linguagem:

1. `where`
2. `code`
3. `help`
4. `note`
5. `related`
6. `value`

Exemplo misto:

```text
error[project.not_found]
Could not find zenith.ztproj.

ACTION
  entre em uma pasta de projeto Zenith

WHY
  `zt build` precisa encontrar um arquivo zenith.ztproj

NEXT
  rode: zt new app --name minha_app

HELP
  ztman explain project.not_found
```

## Estrutura canonica

Modelo conceitual:

```zt
type Diagnostic = struct
    code: text
    severity: Severity
    title: text
    action: optional<text>
    why: optional<text>
    next: optional<text>
    help: optional<HelpLink>
    origin: DiagnosticOrigin
    category: DiagnosticCategory
    location: optional<Location>
    details: list<DiagnosticDetail>
    suggestions: list<Suggestion>
    related: list<RelatedLocation>
    metadata: map<text, text>
end
```

A API final pode mudar.

Mas estes dados precisam existir no modelo.

## Severity

Niveis iniciais:

1. `info`: informacao util, sem falha
2. `warning`: algo suspeito, mas a ferramenta pode continuar
3. `error`: falha recuperavel, comando termina com erro
4. `fatal`: falha grave, comando nao consegue continuar com seguranca

Regra:

```text
fatal deve ser raro.
```

Erros comuns de uso, como flag ausente, devem ser `error`, nao `fatal`.

## Categorias

Categorias iniciais:

1. `cli`: comando, flag, argumento, help
2. `project`: manifesto, estrutura do projeto, descoberta de raiz
3. `environment`: sistema, PATH, compilador C, variaveis de ambiente
4. `filesystem`: arquivos, permissoes, diretorios
5. `network`: download, registry, timeout, proxy
6. `package`: pacote, versao, lockfile, dependencias
7. `template`: scaffold, arquivos gerados, placeholders
8. `prompt`: entrada interativa, modo CI, confirmacao
9. `internal`: bug da propria ferramenta

## Codigos estaveis

Codigos devem ser estaveis para:

1. testes
2. documentacao
3. busca no ztman
4. JSON
5. automacao
6. suporte tecnico

Formato recomendado:

```text
categoria.nome_do_erro
```

Exemplos:

```text
cli.unknown_command
cli.unknown_option
cli.missing_argument
cli.missing_option
cli.invalid_option_value
project.not_found
project.invalid_manifest
environment.missing_tool
filesystem.permission_denied
network.timeout
package.version_conflict
prompt.non_interactive_required
internal.unexpected_failure
```

Evitar codigos numericos como identificador principal na v1.

Motivo:

1. nomes sao mais faceis de lembrar
2. combinam com diagnosticos da linguagem
3. funcionam bem com `ztman explain`

Codigos numericos podem existir depois como alias publico, mas nao devem ser obrigatorios agora.

## Saida humana

A saida humana deve ser curta.

Formato padrao:

```text
error[cli.unknown_command]
Unknown command `docter`.

ACTION
  use `doctor`

WHY
  `docter` nao e um comando conhecido

NEXT
  rode: zt doctor

HELP
  ztman explain cli.unknown_command
```

Regras:

1. uma ideia por bloco
2. blocos curtos
3. exemplos concretos
4. sem parede de texto
5. sem depender de cor
6. sem stack trace por padrao
7. sem detalhes internos no modo normal

## Saida CI

O modo CI deve ser deterministico e curto.

Formato recomendado:

```text
error[cli.unknown_command] Unknown command `docter`. NEXT: zt doctor
```

Regras:

1. uma linha por diagnostico
2. sem prompts
3. sem animacoes
4. sem progresso interativo
5. sem cor por padrao
6. caminhos e mensagens estaveis

Esse modo atende `--ci`.

## Saida JSON

Quando `--json` estiver ativo, `stdout` deve conter apenas JSON valido.

Exemplo:

```json
{
  "status": "error",
  "diagnostic": {
    "code": "cli.missing_option",
    "severity": "error",
    "title": "Missing required option --name.",
    "action": "informe --name",
    "why": "o comando `zt new` precisa do nome do projeto",
    "next": "zt new cli --name minha_ferramenta",
    "help": "ztman explain cli.missing_option",
    "category": "cli"
  }
}
```

Regras:

1. nao misturar texto humano em `stdout`
2. diagnosticos humanos podem ir para `stderr` apenas se o modo permitir
3. campos devem ser estaveis
4. `code` e obrigatorio
5. `severity` e obrigatorio
6. `title` e obrigatorio
7. `action`, `why`, `next` e `help` sao opcionais, mas recomendados

## Perfis de detalhe

`tools.diagnostics` deve suportar os mesmos perfis planejados para a linguagem.

Perfis:

1. `beginner`
2. `balanced`
3. `full`

## beginner

Mostra mais contexto e exemplos.

Uso:

1. usuario novo
2. tutorial
3. erro comum
4. primeira experiencia com a ferramenta

Exemplo:

```text
error[cli.missing_option]
Missing required option --name.

ACTION
  informe o nome do projeto com --name

WHY
  o template `cli` precisa saber qual pasta e qual nome usar

NEXT
  rode: zt new cli --name minha_ferramenta

EXAMPLE
  zt new cli --name backup_tool

HELP
  ztman explain cli.missing_option
```

## balanced

Perfil padrao.

Mostra o essencial sem excesso.

Exemplo:

```text
error[cli.missing_option]
Missing required option --name.

ACTION
  informe --name

NEXT
  zt new cli --name minha_ferramenta
```

## full

Mostra detalhes tecnicos.

Uso:

1. debug
2. CI detalhado
3. suporte
4. desenvolvimento da propria ferramenta

Exemplo:

```text
error[cli.missing_option]
Missing required option --name.

ACTION
  informe --name

WHY
  o schema do comando `zt new` marcou a option `name` como obrigatoria

NEXT
  zt new cli --name minha_ferramenta

DETAILS
  command: zt new
  option: name
  parser: tools.cli
```

## Integracao com ztman

Todo diagnostico estavel deve poder apontar para uma pagina de ajuda.

Formato recomendado:

```text
HELP
  ztman explain cli.missing_option
```

`ztman` deve explicar:

1. o que o erro significa
2. causas comuns
3. como corrigir
4. exemplos bons
5. exemplos ruins quando ajudar
6. links para topicos relacionados

Exemplo de topico:

```text
ztman explain project.not_found
```

Saida esperada:

```text
project.not_found

WHAT
  O comando nao encontrou zenith.ztproj.

WHY
  O Zenith usa esse arquivo para saber onde esta o projeto.

FIX
  Entre na pasta do projeto ou passe --project <path>.

EXAMPLES
  zt check
  zt check --project apps/api/zenith.ztproj
```

## Integracao com tools.cli

Erros de `tools.cli` devem virar diagnosticos.

Mapeamento inicial:

1. `UnknownCommand` -> `cli.unknown_command`
2. `UnknownOption` -> `cli.unknown_option`
3. `MissingArgument` -> `cli.missing_argument`
4. `MissingOption` -> `cli.missing_option`
5. `MissingOptionValue` -> `cli.missing_option_value`
6. `InvalidOptionValue` -> `cli.invalid_option_value`
7. `ConflictingOptions` -> `cli.conflicting_options`
8. `UnexpectedArgument` -> `cli.unexpected_argument`

`tools.cli` nao deve imprimir erro por conta propria.

Ele deve criar um diagnostico e entregar para `tools.output` renderizar.

## Integracao com tools.output

`tools.diagnostics` define o conteudo.

`tools.output` define como esse conteudo aparece.

Separacao:

1. `tools.diagnostics`: codigo, severidade, action, why, next, help
2. `tools.output`: human, json, ndjson, quiet, verbose, stderr, stdout

Isso evita duplicacao.

Tambem facilita snapshots.

## Diagnosticos de ambiente

`zt doctor` deve ser o principal consumidor de diagnosticos de ambiente.

Exemplo:

```text
error[environment.missing_tool]
C compiler was not found.

ACTION
  instale um compilador C suportado

WHY
  o backend atual precisa de um compilador C para gerar executaveis nativos

NEXT
  rode: zt doctor --fix-guide

HELP
  ztman explain environment.missing_tool
```

Regra:

```text
Nao diga apenas "gcc not found".
```

A mensagem deve explicar o impacto no Zenith.

## Diagnosticos de projeto

Exemplo:

```text
error[project.invalid_manifest]
zenith.ztproj is invalid.

ACTION
  corrija o campo `app.entry`

WHY
  o arquivo aponta para uma entrada que nao existe

NEXT
  verifique se `src/main.zt` existe ou atualize `app.entry`

HELP
  ztman explain project.invalid_manifest
```

## Diagnosticos de prompt

Prompts nao podem travar CI.

Exemplo:

```text
error[prompt.non_interactive_required]
This command needs confirmation, but the terminal is not interactive.

ACTION
  passe --yes ou informe todos os valores por flag

WHY
  em CI, a ferramenta nao pode esperar input do usuario

NEXT
  rode: zt new cli --name minha_ferramenta --yes
```

## Diagnosticos internos

Falhas internas devem ser humanas, mas honestas.

Exemplo:

```text
error[internal.unexpected_failure]
The tool failed unexpectedly.

ACTION
  rode novamente com --verbose

WHY
  ocorreu uma falha interna que a ferramenta nao conseguiu recuperar

NEXT
  abra um issue com o comando usado e a versao do Zenith
```

Em modo normal, nao mostrar stack trace grande.

Em modo `full` ou `--verbose`, pode mostrar detalhes tecnicos.

## Sugestoes

Diagnosticos podem carregar sugestoes.

Exemplo:

```text
error[cli.unknown_command]
Unknown command `docter`.

ACTION
  use `doctor`

WHY
  `docter` parece um erro de digitacao

NEXT
  rode: zt doctor
```

Regras:

1. sugerir apenas quando houver confianca razoavel
2. nao listar dez alternativas
3. preferir uma sugestao boa
4. se a sugestao for fraca, omitir

## Controle de ruido

Ferramentas devem evitar avalanche de erros.

Regras:

1. mostrar erro raiz primeiro
2. agrupar erros repetidos
3. limitar repeticoes iguais
4. evitar warnings subjetivos
5. usar `summary` quando houver muitos problemas

Exemplo:

```text
error[template.write_failed]
Could not create project files.

ACTION
  verifique permissao da pasta

WHY
  4 arquivos nao puderam ser criados

NEXT
  rode novamente em uma pasta com permissao de escrita
```

Detalhes podem aparecer no perfil `full`.

## Localizacao e arquivos

Quando houver local fisico, usar `where`.

Exemplo:

```text
error[filesystem.permission_denied]
Could not write file.

where
  C:/workspace/app/src/main.zt

ACTION
  verifique a permissao de escrita

NEXT
  tente rodar em uma pasta do usuario
```

## Exit codes recomendados

`tools.diagnostics` nao decide sozinho o exit code.

Mas recomenda:

1. `0`: sucesso
2. `1`: erro geral
3. `2`: erro de uso de CLI
4. `3`: erro de ambiente
5. `4`: erro de dependencia externa
6. `5`: erro interno

Mapeamento inicial:

1. `cli.*` -> `2`
2. `project.*` -> `1`
3. `environment.*` -> `3`
4. `network.*` -> `4`
5. `package.*` -> `4` ou `1`, dependendo do caso
6. `internal.*` -> `5`

## Testes obrigatorios

Cada diagnostico estavel deve ter snapshot.

Casos minimos:

1. human balanced
2. human beginner quando fizer sentido
3. human full quando houver detalhes
4. JSON
5. CI one-line
6. no-color
7. quiet

Exemplo de gate:

```text
zt new cli
```

Deve gerar:

1. `cli.missing_option`
2. mensagem humana com `ACTION`
3. mensagem JSON valida com `code`
4. CI one-line deterministico
5. link de ajuda via `ztman explain cli.missing_option`

## Fora da v1

Nao incluir na v1:

1. traducao completa para multiplos idiomas
2. catalogo completo de todos os erros futuros
3. quick fixes automaticos
4. hyperlinks clicaveis especificos por terminal
5. telemetria de diagnosticos
6. protocolo LSP completo para tools
7. sistema de temas complexo

## Gates de aceite

`tools.diagnostics` v1 esta aceitavel quando:

1. erros de `tools.cli` viram diagnosticos estruturados
2. `tools.output` renderiza human, JSON e CI
3. `zt doctor` consegue emitir diagnosticos de ambiente claros
4. `zt new` consegue emitir diagnosticos de template claros
5. todo diagnostico tem `code`, `severity` e `title`
6. diagnosticos principais tem `ACTION`, `WHY` ou `NEXT`
7. nenhum diagnostico depende so de cor
8. snapshots cobrem os formatos principais
9. cada codigo estavel pode ser explicado por `ztman explain <code>`

## Racional

A UX da linguagem nao termina no compilador.

Se `zt` e as ferramentas oficiais forem confusas, a linguagem parece confusa.

`tools.diagnostics` cria uma ponte entre:

1. diagnosticos tecnicos da linguagem
2. mensagens acionaveis de CLI
3. documentacao explicativa via `ztman`
4. saida estavel para scripts

Essa camada torna as ferramentas mais profissionais sem colocar peso dentro da `std`.