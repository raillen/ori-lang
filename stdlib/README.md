# Stdlib Next

A stdlib do Next e Zenith-first.

Regras:

- contratos em Zenith sempre que possivel
- adaptacoes target-specific apenas nas bordas
- nenhum atalho para Lua na trilha nova

## Atualizacao de linguagem (2026-04-22)

`public var` agora existe em escopo de namespace.

Para stdlib, a politica e:

- usar `public const` por padrao
- usar `public var` apenas quando estado compartilhado for parte real do contrato
- permitir leitura externa qualificada
- bloquear escrita externa fora do namespace dono
- deixar claro em docs que `public` nao significa `global`

Primeiro modulo com essa adocao:

- `std.random` (`seeded`, `last_seed`, `draw_count`, `stats()`)
