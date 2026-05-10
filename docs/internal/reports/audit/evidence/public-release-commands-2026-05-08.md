# Evidencia - Etapa 6 - Comandos publicos de release

Data: 2026-05-08

## Resultado

Status da etapa: aprovado para prosseguir.

Os comandos publicos definidos para a RC foram executados no Windows/PowerShell.
Todos os comandos do contrato publico passaram depois das correcoes desta etapa.

## Comandos base

```powershell
python build.py
```

Resultado: `SUCCESS`, com `zt.exe` e `zpm.exe` gerados.

```powershell
.\zt.exe check zenith.ztproj --all --ci
```

Resultado: `check ok`

```powershell
.\zt.exe test zenith.ztproj --ci
```

Resultado: `test ok (pass=1 skip=0)`

```powershell
.\zt.exe fmt zenith.ztproj --check
```

Resultado: `fmt check ok`

```powershell
.\zt.exe doc check zenith.ztproj
```

Resultado: `doc check ok`

## Ajuda e diagnostico de instalacao

```powershell
.\zt.exe help
.\zt.exe help zpm
.\zt.exe zpm help
.\zpm.exe help
.\zt.exe doctor
```

Resultado:

- `zt help`: passou.
- `zt help zpm`: passou e agora lista somente comandos do contrato publico da RC.
- `zt zpm help`: passou.
- `zpm help`: passou.
- `zt doctor`: passou; `gcc` e `clang` encontrados, `cl` ausente, status final `native compiler configured`.

## Suite oficial

```powershell
python run_suite.py release
```

Resultado:

- Status: `pass`
- Total: `365`
- Pass: `365`
- Fail: `0`
- Relatorio: `reports/suites/release__20260508T044114Z.json`
- Link atual: `reports/suites/release__latest.json`

## Pacotes, install e distribuicao

Fluxo validado em `.ztc-tmp/rc-public-commands/zpm-project`:

- `zpm init`
- `zpm add math@^1.2.3`
- `zpm list`
- `zpm install`
- `zpm install --locked`
- `zpm update`
- `zt zpm install --locked`
- `zt pkg install --locked`
- `zpm publish .`
- `zt zpm publish .`
- `zt pkg publish .`

Resultado: todos com codigo de saida `0`.

## Correcoes feitas durante a etapa

1. `zpm install` em ambiente sem registry local emitia erro falso de `cannot open .../.zenith/registry.ztproj`.
   - Correcao: `compiler/driver/zpm.c` agora consulta o cache de registry somente se o arquivo existir.
   - Cobertura: `tests/driver/test_zpm_lockfile.py` falha se `zpm install` voltar a imprimir esse erro falso.

2. `zt help zpm` anunciava comandos que nao existem no contrato atual: `login`, `search` e `info`.
   - Correcao: `compiler/driver/main.c` agora lista `init`, `install`, `add`, `remove`, `update`, `list`, `find` e `publish`.
   - Cobertura: `tests/driver/test_cli_output_clean.py`.

3. `update-registry` aceitava resposta HTTP `404` como sucesso e gravava cache invalido.
   - Correcao: `compiler/driver/zpm.c` usa `curl -f` e remove cache parcial se o download falhar.
   - Decisao de contrato: registry remoto nao faz parte da RC publica. Por isso, `update-registry` foi removido da ajuda publica e de `docs/reference/cli/zpm.md`.

## Lacunas registradas

- Registry remoto: fora do contrato publico da RC.
- Instalador nativo: fora do contrato publico da RC.
- Comandos `login`, `search` e `info`: nao fazem parte do contrato publico atual e foram removidos da ajuda da RC.

## Decisao

Etapa 6 concluida. Nao ha comando publico de release quebrado sem decisao.
