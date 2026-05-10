# Decision 004 - Modelo de tools.output

- Status: draft
- Data: 2026-04-20
- Tipo: tools / output / cli
- Escopo: formatos de saida, streams, modos globais e contrato para scripts

## Resumo

`tools.output` deve padronizar como ferramentas Zenith escrevem resultados.

O objetivo e evitar que cada CLI invente sua propria forma de imprimir mensagens, JSON, logs e erros.

Uma ferramenta Zenith deve ser boa para:

1. pessoas
2. scripts
3. CI
4. logs
5. diagnosticos

## Decisao

`tools.output` deve oferecer um modelo unico de saida para CLIs.

Modos canonicos:

1. `human`
2. `json`
3. `ndjson`
4. `quiet`
5. `verbose`

Streams canonicos:

1. `stdout` para resultado principal
2. `stderr` para logs, avisos e diagnosticos humanos

Regra principal:

Quando `--json` estiver ativo, `stdout` deve conter apenas JSON valido.

## Por que isso importa

CLIs quebram automacoes quando misturam texto humano com dados estruturados.

Exemplo ruim:

```text
Criando projeto...
{"status":"ok"}
```

Isso parece legivel para humanos, mas quebra scripts que esperam JSON puro.

Exemplo bom:

```json
{"status":"ok","message":"projeto criado"}
```

Se houver log humano, ele deve ir para `stderr` ou ser omitido em modo JSON.

## Conceitos canonicos

## OutputMode

Representa o modo de saida atual.

Valores iniciais:

1. `human`
2. `json`
3. `ndjson`

## Verbosity

Representa o nivel de detalhe.

Valores iniciais:

1. `quiet`
2. `normal`
3. `verbose`

## OutputEvent

Representa algo que a ferramenta quer comunicar.

Tipos iniciais:

1. `Success`
2. `Info`
3. `Warning`
4. `Error`
5. `StepStarted`
6. `StepFinished`
7. `Data`

## OutputWriter

Objeto responsavel por renderizar eventos no formato correto.

Ele deve saber:

1. modo de saida
2. nivel de detalhe
3. uso de cor
4. largura do terminal
5. stream correto

## API mental

Exemplo conceitual:

```zt
use tools.output

func run() -> int
    var out = output.writer()
        .mode(output.human)
        .verbosity(output.normal)

    out.success("projeto criado")
    out.next("rode: zt run")

    return 0
end
```

Com JSON:

```zt
var out = output.writer().mode(output.json)
out.data({ status: "ok", message: "projeto criado" })
```

A API final pode mudar, mas a separacao deve permanecer.

## Formato humano

O formato humano deve ser curto, legivel e orientado a acao.

Exemplo:

```text
OK: projeto criado

Criado:
- zenith.toml
- src/main.zt
- tests/main_test.zt

NEXT: rode: zt run
```

Regras:

1. usar blocos curtos
2. evitar parede de texto
3. mostrar proximo passo quando fizer sentido
4. nao depender apenas de cor
5. truncar tabelas largas via `tools.table`

## Formato JSON

O formato JSON deve ser estavel.

Exemplo:

```json
{
  "status": "ok",
  "message": "projeto criado",
  "created": ["zenith.toml", "src/main.zt"],
  "next": "zt run"
}
```

Regras:

1. `stdout` deve ser JSON valido
2. campos devem ter nomes estaveis
3. erros devem ter `status`, `code`, `message`
4. nao imprimir texto humano em `stdout`
5. logs devem ir para `stderr` ou serem desativados

## Formato NDJSON

NDJSON deve ser usado para saida em streaming.

Exemplo:

```jsonl
{"type":"step_started","name":"parse"}
{"type":"step_finished","name":"parse","ms":12}
{"type":"success","message":"ok"}
```

Uso recomendado:

1. comandos longos
2. watchers
3. benchmarks
4. CI com stream de eventos

## Quiet

`quiet` reduz ruido.

Em modo quiet:

1. nao imprimir sucesso comum
2. imprimir apenas erro essencial
3. manter exit code significativo
4. permitir JSON se `--json` tambem estiver ativo

## Verbose

`verbose` aumenta detalhe.

Em modo verbose:

1. mostrar etapas
2. mostrar caminhos relevantes
3. mostrar duracao quando disponivel
4. mostrar contexto tecnico sem virar parede de texto

## Cores

Cor e opcional.

Regras:

1. `--no-color` desativa cor
2. CI pode desativar cor por padrao
3. cor nao pode ser a unica fonte de significado
4. simbolos e texto devem carregar a informacao

## Exit codes

`tools.output` nao decide sozinho o exit code.

Mas deve padronizar constantes recomendadas:

1. `0`: sucesso
2. `1`: erro geral
3. `2`: erro de uso ou argumentos
4. `3`: erro de ambiente
5. `4`: erro de dependencia externa

Esses valores podem ser refinados por decision futura.

## Integracao com tools.cli

`tools.cli` deve configurar `tools.output` a partir de flags globais.

Exemplos:

1. `--json` ativa `OutputMode.json`
2. `--quiet` ativa `Verbosity.quiet`
3. `--verbose` ativa `Verbosity.verbose`
4. `--no-color` desativa cor

## Integracao com tools.diagnostics

`tools.diagnostics` deve produzir estruturas renderizaveis por `tools.output`.

Isso evita duplicar logica de render.

## Testes obrigatorios

Todo modulo que usa `tools.output` deve permitir snapshots de output.

Casos minimos:

1. sucesso humano
2. erro humano
3. sucesso JSON
4. erro JSON
5. quiet
6. verbose
7. no-color

## Fora da v1

Nao incluir na v1:

1. temas customizaveis complexos
2. HTML output
3. TUI completa
4. logs remotos
5. telemetry
6. internacionalizacao completa

## Gate de aceite

`tools.output` v1 esta aceitavel quando:

1. `zt doctor --json` emite JSON puro em stdout
2. `zt doctor` emite saida humana clara
3. `zt bench --verbose` mostra etapas e tempos
4. `zt new --quiet` nao imprime ruido desnecessario
5. snapshots de output passam no CI

## Racional

A UX da Zenith nao deve parar na sintaxe da linguagem.

Ferramentas oficiais tambem precisam ser previsiveis, legiveis e acessiveis.

`tools.output` e a base para isso.
