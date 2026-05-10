# R6 - Clarity and Accessibility Closure Report

Date: 2026-04-26
Status: fechado localmente
Scope: language surface, diagnostics, helper APIs, formatter and user-facing docs

## Decisao de fechamento

R6 esta fechado localmente como ciclo de clareza, leitura e acessibilidade.

O corte nao adiciona sintaxe pesada. Em vez disso, melhora caminhos ja
aceitos:

- mensagens de erro mais diretas;
- helpers pequenos com nomes explicitos;
- docs curtas para mapear conceitos comuns;
- formatter mais estavel para leitura;
- `zt explain <codigo>` para ajuda offline.

## O que ficou fechado

- Sintaxes rejeitadas continuam sem entrar no ciclo: `union`, `abstract`,
  `virtual` e `partial`.
- Conversoes numericas continuam explicitas por tipo, e `to_text(...)` ficou
  como builtin de intencao.
- `todo(...)`, `unreachable(...)` e `check(..., message)` tem cobertura de
  comportamento.
- `std.test`, `std.validate`, `std.text`, `list` e `map` receberam helpers
  pequenos e documentados.
- Warnings de leitura cobrem nomes parecidos, nomes visualmente confusos,
  blocos profundos, funcoes longas e `case default` em enum conhecido.
- Shadowing segue como erro duro (`name.shadowing`) porque o MVP rejeita
  shadowing. Ele nao foi rebaixado para warning.
- Formatter ganhou caso reading-first com golden e idempotencia.
- `zt explain` ganhou catalogo inicial e testes de driver.
- Diagnosticos de enum, callable e dyn agora explicam a proxima acao:
  - enum match nao exaustivo lista variants ausentes;
  - callable mostra assinatura esperada e recebida;
  - callable generico usado como valor pede wrapper nao generico;
  - dyn unsafe sugere generics com `where` quando dispatch dinamico nao e
    necessario.

## Validacao local

Gates verdes neste fechamento:

- `python build.py`
- `python tests/formatter/run_formatter_golden.py`
- `python tests/formatter/run_formatter_idempotence.py`
- `python tests/driver/test_explain_cli.py`
- `python tools/generate_code_maps.py --match Type --check`
- `python tools/generate_code_maps.py --match Diagnostics --check`
- `./zt.exe check zenith.ztproj --all --ci`
- `python run_suite.py smoke --no-perf` (`13/13`)
- `python run_suite.py pr_gate --no-perf` (`198/198`)
- `git diff --check` nos arquivos tocados nesta fatia

## Residuos conhecidos fora deste corte

Estes checks ainda falham por dividas ja existentes fora de R6:

- `python tools/check_docs_paths.py`
  - `reports/coverage/coverage-snapshot.md:16` referencia um caminho antigo de runtime/process
  - `reports/coverage/coverage-snapshot.md:17` referencia um caminho antigo de runtime/text
  - `tools/borealis-studio/README.md:65` referencia um caminho antigo de SDK
- `./zt.exe doc check zenith.ztproj`
  - ainda reporta 21 erros de ZDoc em stdlib/docs antigas.

Esses residuos nao bloqueiam o fechamento de R6 porque o gate raiz
`./zt.exe check zenith.ztproj --all --ci` e o `pr_gate` estao verdes.

## Proximo ciclo recomendado

O proximo ciclo deve focar maturidade de producao:

- estabilizar ZDoc e docs paths;
- reduzir sujeira historica de docs movidas;
- fechar criterios de release reproducivel;
- ampliar `zt explain` para mais codigos;
- transformar os gates verdes em politica de CI obrigatoria.
