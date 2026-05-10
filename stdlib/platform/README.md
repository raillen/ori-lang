# Stdlib Platform

## Objetivo

`stdlib/platform/` e uma camada interna de adaptacao entre host/target e modulos `std.*`.

Nao e uma API publica para aplicacoes.

## Regra Principal

Nao competir com:

- `std.os` (estado do processo atual)
- `std.os.process` (processos filhos)

Se uma funcao pertence claramente a esses modulos, ela nao deve aparecer em `platform`.

## Quando Implementar de Verdade

Manter placeholder ate existir necessidade concreta.

Gates para ativacao:

- logica cross-platform duplicada em 2+ modulos `std.*`
- mapeamento de erro de plataforma repetido e inconsistente
- regressao recorrente entre sistemas por falta de camada comum
- deteccao de capability duplicada e dificil de manter

## Escopo Esperado (Interno)

Exemplos de responsabilidade valida:

- normalizacao interna de codigos/erros nativos
- adaptadores internos por target para suportar `std.*`
- helpers internos de deteccao de capability de runtime

## Escopo Nao Permitido

- API publica alternativa para `std.os`
- API publica alternativa para `std.os.process`
- namespace publico para uso direto em app

## Referencias

- `docs/internal/decisions/language/067-stdlib-os.md`
- `docs/internal/decisions/language/068-stdlib-os-process.md`
- `docs/internal/decisions/language/085-core-and-platform-layering-contract.md`
