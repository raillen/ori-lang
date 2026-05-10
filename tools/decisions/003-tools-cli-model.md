# Decision 003 - Modelo de tools.cli

- Status: draft
- Data: 2026-04-20
- Tipo: tools / cli / api
- Escopo: parser de CLI, comandos, flags, argumentos, help e erros

## Resumo

`tools.cli` deve permitir criar CLIs claras sem parser manual.

O modulo deve cobrir:

1. comandos
2. subcomandos
3. argumentos posicionais
4. opcoes com valor
5. flags booleanas
6. help automatico
7. validacao
8. erros tipados
9. integracao com `tools.output` e `tools.diagnostics`

## Objetivo

Criar uma API simples o bastante para CLIs pequenas e estruturada o bastante para ferramentas oficiais como:

1. `zt new`
2. `zt doctor`
3. `zt bench`
4. `zt release`
5. `zt explain`

## Modelo mental

```zt
use tools.cli

func main() -> int
    var app = cli.app("zt-new")
        .about("Cria um novo projeto Zenith")
        .version("1.0.0")

    app.command("new")
        .about("Cria projeto")
        .argument("template").required()
        .option("name").required()
        .flag("yes")
        .handler(run_new)

    return app.run()
end
```

Handler:

```zt
func run_new(ctx: cli.Context) -> int
    const template = ctx.argument("template")?
    const name = ctx.option("name")?
    const yes = ctx.flag("yes")

    return create_project(template, name, yes)
end
```

Este exemplo e conceitual.

A API final pode mudar, mas a ergonomia desejada e esta.

## Conceitos canonicos

## App

Representa a CLI inteira.

Responsabilidades:

1. nome
2. versao
3. descricao
4. comandos globais
5. flags globais
6. execucao

## Command

Representa uma acao executavel.

Exemplos:

1. `zt new`
2. `zt doctor`
3. `zt bench`

Um comando pode ter subcomandos.

## Argument

Valor posicional.

Exemplo:

```text
zt new cli
```

Aqui, `cli` pode ser o argumento `template`.

## Option

Flag com valor.

Exemplo:

```text
--name minha_app
```

## Flag

Booleano.

Exemplo:

```text
--yes
```

## Context

Objeto entregue ao handler depois do parse.

Ele deve expor:

1. argumentos parseados
2. opcoes parseadas
3. flags parseadas
4. modo global (`json`, `quiet`, `verbose`)
5. informacao do comando atual

## Regras padrao

`tools.cli` deve oferecer por padrao:

1. `--help`
2. `--version`, quando app declara versao
3. `--json`
4. `--quiet`
5. `--verbose`
6. `--no-color`

`--yes` nao precisa ser global obrigatorio.

Mas deve ser facil de adicionar em comandos interativos.

## Help padrao

O help deve ser curto e acionavel.

Formato recomendado:

```text
zt new

ACTION:
  cria um novo projeto Zenith

USAGE:
  zt new <template> --name <nome>

OPTIONS:
  --name <nome>   nome do projeto
  --json          emite JSON
  --yes           confirma perguntas automaticamente

EXAMPLES:
  zt new cli --name minha_ferramenta
  zt new api --name meu_servico --yes
```

## Erros tipados

A v1 deve incluir pelo menos:

1. `UnknownCommand`
2. `UnknownOption`
3. `MissingArgument`
4. `MissingOption`
5. `MissingOptionValue`
6. `InvalidOptionValue`
7. `ConflictingOptions`
8. `UnexpectedArgument`

Cada erro deve ser convertivel para `tools.diagnostics`.

## Exemplo de erro humano

```text
ACTION: informe --name
WHY: o comando `zt new` precisa do nome do projeto
NEXT: rode: zt new cli --name minha_ferramenta
```

## Exemplo de erro JSON

```json
{
  "status": "error",
  "code": "MissingOption",
  "option": "name",
  "action": "informe --name",
  "next": "zt new cli --name minha_ferramenta"
}
```

## Tipos de option na v1

A v1 deve suportar:

1. `text`
2. `int`
3. `float`
4. `bool`
5. `path`
6. `choice`

Fora da v1:

1. listas
2. objetos complexos
3. parsing por schema externo
4. conversoes implicitas demais

## Sugestoes

`tools.cli` deve sugerir comandos e opcoes parecidas.

Exemplo:

```text
ACTION: use `doctor`
WHY: o comando `docter` nao existe
NEXT: rode: zt doctor
```

Essa sugestao deve ter limite.

Se a distancia for grande demais, nao sugerir.

## Integracao com tools.output

`tools.cli` nao deve imprimir tudo diretamente.

Ele deve produzir eventos ou resultados que `tools.output` consegue renderizar.

Motivo:

1. manter humano e JSON separados
2. facilitar testes de snapshot
3. evitar duplicacao de formato

## Integracao com tools.diagnostics

Erros de parse devem virar diagnosticos.

Isso garante o padrao:

1. `ACTION`
2. `WHY`
3. `NEXT`

## Nao interativo

`tools.cli` deve expor se a execucao e interativa.

Isso ajuda `tools.prompt` a nao travar CI.

## Fora da v1

Nao incluir na v1:

1. autocomplete shell
2. plugins dinamicos
3. aliases complexos
4. parsing estilo git avancado
5. config automatica por env var
6. prompt automatico para opcao ausente

Esses recursos podem entrar depois, mas nao devem atrasar a base.

## Gate de aceite

`tools.cli` v1 esta aceitavel quando estas CLIs puderem existir sem parser manual:

```text
zt new cli --name demo --yes
zt doctor --json
zt bench --filter parser --verbose
```

## Racional

A primeira experiencia de muita gente com Zenith pode ser uma ferramenta CLI.

Se a CLI for confusa, a linguagem parece confusa.

`tools.cli` deve criar um padrao de experiencia simples, previsivel e acessivel.
