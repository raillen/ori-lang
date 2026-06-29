# Stdlib Gap Parity — Referência `std.*` vs Ori `ori.*`

> Status: plano de implementação ativo (`[Unreleased]`, congelado em `0.2.x`)
> Última revisão: 2026-06-29

## Objetivo

Fechar gaps funcionais entre a stdlib de referência (módulos `std.io`, `std.text`,
`std.fs.path`, etc.) e a stdlib Ori, **sem violar a arquitetura de 3 camadas**:

| Camada | Papel | Regra |
|--------|-------|-------|
| **Layer 1** | Manifesto Rust + FFI | Hot path, I/O de SO, rede, metadados FS |
| **Layer 2** | `.orl` composicional | Wrappers, validação, paths, erros uniformes |
| **Layer 3** | `.orl` algoritmos | Lógica pura sobre Layer 1+2 |

## Mapa de equivalência

| Referência | Ori | Estratégia |
|------------|-----|------------|
| `std.io` | `ori.io` + `ori.io.utils` | Layer 1 print/read_line; Layer 2 `try_read_line` |
| `std.text` | `ori.string` + `ori.string.utils` | Layer 1 primitivas; Layer 2 helpers |
| `std.bytes` | `ori.bytes` + `ori.bytes.utils` | Idem |
| `std.json` | `ori.json` + `ori.json.utils` | Layer 1 parse/stringify; Layer 2 read/write |
| `std.format` | `ori.format` + futuro `format.utils` | Layer 1 existente |
| `std.fs` | `ori.fs` + `ori.fs.utils` | Layer 1 + metadados; Layer 2 aliases result |
| `std.fs.path` | `ori.path` | **Layer 2 puro** |
| `std.os` | `ori.os` | Layer 1 + `current_dir`/`change_dir` |
| `std.os.process` | `ori.process` + `ori.process.utils` | Layer 1 subprocess; Layer 2 parse |
| `std.time` | `ori.time` + `ori.time.utils` | Layer 1 ms int; Layer 2 durações |
| `std.list` / `std.map` | `ori.list` / `ori.map` + utils | Ori **supera** a referência |
| `std.collections` | `ori.queue`, `stack`, `heap`, `tree`, `graph`… | Modelo opaco (diferente, mais capaz) |
| `std.math` | `ori.math` + `ori.math.utils` | Layer 1 trig/log; Layer 2 helpers |
| `std.random` | `ori.random` | Layer 1 + `seed` |
| `std.validate` | `ori.validate` | **Layer 2 puro** |
| `std.concurrent` | `ori.concurrent` + `task`/`channel` | Contrato `Transferable` (não copy_*) |
| `std.lazy` | `ori.lazy` + `is_consumed` | Codegen inline |
| `std.test` | `ori.test` + `ori.test.utils` | Layer 1 + `skip` |
| `std.net` | `ori.net` | Layer 1 TCP opaco |

## Lacunas fechadas neste ciclo

### Layer 2 (`.orl`)

- `stdlib/validate.orl` — validação declarativa
- `stdlib/path.orl` — manipulação de paths (sem FFI)
- `stdlib/json/utils.orl` — `read`/`write` arquivo
- `stdlib/io/utils.orl` — `try_read_line`, `print_line`
- `stdlib/fs/utils.orl` — helpers `result`, `create_dir_all` documentado
- `stdlib/time/utils.orl` — durações e conversões Unix
- `stdlib/test/utils.orl` — `is_true`/`is_false`
- Expansões: `string.utils`, `bytes.utils`, `math.utils`, `map.utils`

### Layer 1 (runtime + manifesto)

- `fs.file_size`, `fs.modified_at`, `fs.created_at`
- `fs.create_dir_all` (alias semântico; runtime já usa `create_dir_all`)
- `os.current_dir`, `os.change_dir`
- `random.seed`
- `process.run`, `process.run_capture`
- `net.connect`, `net.read_some`, `net.write_all`, `net.close`, `net.is_closed`
- `test.skip` (exit 77)
- `lazy.is_consumed` (codegen inline)
- `math`: `trunc`, `ln`, `exp`, `asin`, `acos`, `atan`, `atan2`, `is_finite`

## Lacunas remanescentes (backlog)

| Item | Motivo | Rastreabilidade |
|------|--------|-----------------|
| `grid2d`/`grid3d`/`circbuf`/`btree` | Sem demanda concreta; coleções opacas cobrem filas/heaps | — |
| `format.date_pattern` / locale | Requer parser de pattern ou ICU | Backlog v2 |
| `io.Input`/`Output` stream types | Redesign de I/O; spec futura | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §4 |
| Uniformizar **todos** os `bool` FS → `result` | Breaking change; wrappers Layer 2 mitigam | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §2 |
| `read_line` → `optional<string>` em Layer 1 | Breaking change; `try_read_line` mitiga hoje | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §2 |
| `time.Instant`/`Duration` tipados | Evolução de `ori.time` v2 | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §4 |
| `net` TLS/UDP/async | Fora do escopo v1 TCP síncrono | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §4 |
| Genéricos `map`/`set`/`graph` na stdlib `.orl` | Aguarda trait gate `Hashable`+`Equatable` | — |
| Toolchain `explain` / `doctor` / `repl` / `summary` | Gap DX vs referência | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §1–§3 |
| Guia pedagógico Errors/Null/Void | Documentação do modelo mental | [`PENDENTES.md`](PENDENTES.md) § Backlog v2 §1 |

## Filosofia Ori preservada

1. **Erros explícitos** — preferir `result<T, string>` em Layer 1; Layer 2 converte `bool` legado.
2. **Reading-first** — APIs nomeadas por intenção (`blank`, `parse_int_or`, `has_path`).
3. **Sem null** — `optional`/`result` only.
4. **Hot path no Rust** — rede, subprocess, metadados FS não viram `.orl`.
5. **Composição em Ori** — validate, path, wrappers JSON/FS.

## Testes

Um teste end-to-end por módulo novo em `compiler/crates/ori-driver/tests/multifile_imports.rs`.

## Referências

- `docs/spec/12-stdlib.md` — contratos públicos
- `docs/spec/15-stdlib-maintenance.md` — workflow Layer 2/3
- `stdlib/README.md` — inventário de módulos `.orl`
