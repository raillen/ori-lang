# Codebase Map

> Indice de documentacao tecnica colocalizada com codigo.
> Audience: contributor, maintainer
> Surface: internal
> Status: current

## Regra

Documentacao tecnica pode ficar ao lado do subsistema quando ela e usada durante manutencao do codigo.

Use este arquivo como mapa unico para encontrar esses documentos.

## Compiler

- `compiler/CODE_MAP.md`: visao geral do compilador.
- `compiler/CODE_MAPS_GUIDE.md`: regra para manter mapas.
- `compiler/frontend/lexer/LEXER_MAP.md`: lexer.
- `compiler/frontend/parser/PARSER_MAP.md`: parser.
- `compiler/frontend/ast/AST_MAP.md`: AST.
- `compiler/semantic/binder/BINDER_MAP.md`: binder.
- `compiler/semantic/types/TYPE_SYSTEM_MAP.md`: typechecker.
- `compiler/semantic/symbols/SYMBOLS_MAP.md`: tabela de simbolos.
- `compiler/semantic/diagnostics/DIAGNOSTICS_MAP.md`: diagnosticos.
- `compiler/hir/lowering/HIR_LOWERING_MAP.md`: lowering HIR.
- `compiler/zir/ZIR_MODEL_MAP.md`: modelo ZIR.
- `compiler/zir/ZIR_PARSER_MAP.md`: parser ZIR.
- `compiler/zir/ZIR_VERIFIER_MAP.md`: verifier ZIR.
- `compiler/targets/c/EMITTER_MAP.md`: backend C.
- `compiler/driver/DRIVER_MAP.md`: driver CLI.
- `compiler/project/PROJECT_MAP.md`: modelo de projeto.
- `compiler/tooling/FORMATTER_MAP.md`: formatter.
- `compiler/utils/UTILS_MAP.md`: utilitarios.

## Runtime, stdlib e testes

- `runtime/c/README.md`: runtime C.
- `runtime/c/RUNTIME_MAP.md`: mapa do runtime.
- `stdlib/README.md`: organizacao da stdlib.
- `stdlib/STDLIB_MAP.md`: mapa da stdlib.
- `tests/README.md`: organizacao de testes.
- `tests/behavior/README.md`: testes de comportamento.
- `tests/runtime/c/README.md`: testes do runtime C.
- `tests/perf/README.md`: testes de performance.

## Packages e ferramentas

- `packages/README.md`: workspace de packages.
- `packages/borealis/README.md`: Borealis.
- `packages/borealis/decisions/`: decisoes do Borealis.
- `tools/decisions/README.md`: decisoes de ferramentas.
- `tools/vscode-zenith/README.md`: extensao VSCode.
- Borealis Studio: planejado como superficie separada; nao ha pasta ativa neste checkout.

## Relatorios

- Relatorios curados: `docs/internal/reports/`.
- Saidas operacionais locais: `reports/`.
