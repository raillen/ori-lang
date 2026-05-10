# `zt` CLI Reference

> Surface: reference
> Status: current

## Forma geral

```text
zt <command> [input] [options]
```

## Comandos principais

| Comando | Uso |
| --- | --- |
| `help` | mostra ajuda geral ou de comando |
| `check` | valida projeto, manifesto ou arquivo `.zt` avulso |
| `build` | compila projeto ou arquivo `.zt` avulso |
| `run` | compila e executa projeto ou arquivo `.zt` avulso |
| `create` | cria scaffold de projeto |
| `test` | executa testes |
| `fmt` | formata codigo |
| `clean` | remove diretorio de build output |
| `repl` | avalia expressoes em loop compile-and-run |
| `doctor` | verifica instalacao e compilador C nativo |
| `doc check` | valida documentacao |
| `doc show` | mostra documentacao |
| `summary` | resume projeto |
| `resume` | retoma fluxo operacional |
| `perf` | comandos de performance |

## Ajuda

```powershell
.\zt.exe help
.\zt.exe help check
.\zt.exe help build
.\zt.exe help run
.\zt.exe help create
.\zt.exe help repl
.\zt.exe help doctor
```

## Single-File Mode

Voce pode usar `check`, `build` e `run` diretamente em um arquivo `.zt` avulso,
sem precisar de um projeto (`zenith.ztproj`):

```powershell
.\zt.exe check hello.zt
.\zt.exe build hello.zt
.\zt.exe run hello.zt
.\zt.exe build hello.zt -o meu_app.exe
.\zt.exe emit-c hello.zt
```

O compilador cria um manifesto sintetico em memoria. O arquivo precisa declarar
`namespace` e conter uma funcao `main`. A validacao de namespace-path e ignorada.

## Check

```powershell
.\zt.exe check zenith.ztproj
.\zt.exe check hello.zt
.\zt.exe check zenith.ztproj --all
.\zt.exe check zenith.ztproj --ci
.\zt.exe check zenith.ztproj --focus packages\borealis
```

Options accepted by help:

```text
--ci --profile <level> --all --focus <path> --since <git-ref> --lang <lang>
```

## Test

```powershell
.\zt.exe test zenith.ztproj
.\zt.exe test zenith.ztproj --filter pass_case
.\zt.exe test zenith.ztproj --filter app.tests.pass_case
.\zt.exe test zenith.ztproj --ci --filter parser
```

`--filter <name>` runs only tests whose function name, module name, qualified
name, project path, or project name contains the given text.

Use it for a small loop while editing one area. Keep the full test command for
final validation.

For `attr test` cases, `zt test` prints one compact line per test:

```text
test pass app.tests.pass_case duration=12ms
```

Failed or skipped cases also print a small source trace:

```text
stacktrace:
  at app.tests.fail_case (src/app/tests.zt:16:1)
```

## REPL

```powershell
.\zt.exe repl
.\zt.exe repl --eval "1 + 2"
.\zt.exe repl --eval "\"hello\""
```

`zt repl` accepts one expression per line.
Each expression is compiled and run as a small temporary program.

Prompt commands:

- `:help`
- `:quit`
- `:exit`

Use `--eval` for scripts and tests.

## Build

```powershell
.\zt.exe build zenith.ztproj
.\zt.exe build hello.zt
.\zt.exe build zenith.ztproj -o build\app.exe
.\zt.exe build zenith.ztproj --native-raw
.\zt.exe build zenith.ztproj --verbose
```

By default, `build` prints one compact success line.
Use `--verbose` for driver steps such as generated paths.
Use `--native-raw` only when you need the raw C compiler output.

Compiler selection:

- `ZT_CC`: preferred explicit C compiler.
- `CC`: fallback C compiler.
- default: `gcc`.

Run `zt doctor` to see what the current shell can find.

## Run

```powershell
.\zt.exe run zenith.ztproj
.\zt.exe run hello.zt
.\zt.exe run zenith.ztproj --native-raw
.\zt.exe run zenith.ztproj --verbose
```

By default, `run` prints the program output.
If the program exits with a non-zero code, `zt` prints that code.
Native compiler details stay hidden unless `--verbose` or `--native-raw` is used.

## Doctor

```powershell
.\zt.exe doctor
```

`doctor` checks:

- the Zenith CLI version;
- `ZENITH_HOME`;
- `ZT_CC` and `CC`;
- common compilers: `gcc`, `clang`, and `cl`.

Use it after installing Zenith or after changing the native toolchain.

## Create

```powershell
.\zt.exe create examples\hello --app
.\zt.exe create packages\my_lib --lib
.\zt.exe create examples\hello --app --force
```

## Compatibility Commands

The driver still exposes compatibility commands for older workflows:

```text
project-info verify emit-c build <file.zir> doc-check parse
```

Prefer the main commands for new documentation.
