# Stdlib Core

## Objetivo

`stdlib/core/` existe para organizar contratos internos e documentacao de fronteira.

Nao e uma camada publica concorrente de `core.*`.

## Regras

- `core.*` continua implicito e pertence ao nucleo da linguagem.
- Usuario final nao importa `core`.
- Esta pasta pode conter:
  - notas de arquitetura
  - contratos internos
  - utilitarios de teste interno
- Esta pasta nao deve virar uma nova API publica duplicando tipos e semantica do core da linguagem.

## Relacao com Stdlib Publica

APIs publicas ficam em `stdlib/std/*`.

Exemplos de ownership publico:

- `std.io`
- `std.fs`
- `std.os`
- `std.os.process`

## Referencias

- `docs/internal/decisions/language/050-core-stdlib-boundary.md`
- `docs/internal/decisions/language/070-prelude-and-stdlib-architecture.md`
- `docs/internal/decisions/language/085-core-and-platform-layering-contract.md`
