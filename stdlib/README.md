# Ori Standard Library

> Surface: **S3 (`0.3.0`)** — modules use `module ori.…`, no declaration `func`,
> types with `[]`, imports `import path = alias` / `import path (names)`.
> Auk9 is not a product; Ori owns the living syntax.


The stdlib has three layers: a Rust manifest + native runtime (Layer 1),
`.orl` compositional wrappers (Layer 2), and `.orl` pure algorithms (Layer 3).
Spec contracts in `docs/spec/12-stdlib.md`.

Gap parity vs reference `std.*`: see `docs/planning/stdlib-gap-parity.md`.

## Current architecture (v1.x + Stdlib Phase 0 — completo)

### Layer 1 — Rust runtime (manifest-only, never ported to `.orl`)

Low-level primitives implemented as `extern "C"` functions in the native
runtime. These are the stable ABI contract between generated object code and
the runtime. **Frozen** — hot path (ARC, async executor, I/O, string allocators,
collections FFI) stays here permanently.

- **Manifest:** `compiler/crates/ori-types/src/stdlib.rs`
- **Runtime:** `compiler/crates/ori-runtime/src/lib.rs`
- **Spec:** `docs/spec/12-stdlib.md`

### Layer 2 — `.orl` safe wrappers (Stdlib Phase 0 — completo)

Parent modules can expose `.orl` helpers beside native Layer 1 functions
through selective imports, for example `import ori.string (is_empty)`.
Current flattened parents:

| Module | File | Notes |
|--------|------|-------|
| `ori.string` | `stdlib/string.orl` | Flattened text helpers and string algorithms |
| `ori.list` | `stdlib/list.orl` | Flattened list helpers and integer list algorithms |
| `ori.fs` | `stdlib/fs.orl` | Flattened FS convenience wrappers; old `ori.fs.utils` stays valid |
| `ori.time` | `stdlib/time.orl` | Typed `Instant`/`Duration` helpers over millisecond runtime primitives |

| Module | File | Notes |
|--------|------|-------|
| `ori.io` | `stdlib/io.orl` | Flattened stream helpers over `ori.io.Input`/`Output` |
| `ori.net` | `stdlib/net.orl` | Flattened TLS/TCP server/UDP Layer 1 symbols |
| `ori.args` | `stdlib/args.orl` | Thin CLI argument helpers over `ori.os.args` |
| `ori.config` | `stdlib/config.orl` | Text/JSON config helpers over `ori.fs` and `ori.json` |
| `ori.log` | `stdlib/log.orl` | Minimal CLI logging helpers |
| `ori.validate` | `stdlib/validate.orl` | `between`, `even`, `blank`, `one_of`, string length checks, … |
| `ori.path` | `stdlib/path.orl` | `join`, `normalize`, `relative`, `parent`, `extension`, … |
| `ori.format` | `stdlib/format.orl` | Wrappers `number`, `hex`, `bytes_size`, `date`, … |
| `ori.iter` | `stdlib/iter.orl` | `sum_int`, `contains_int`, `unique_count_int`, … |
| `ori.net` | `stdlib/net.orl` | `connect`, `connect_tls`, `read_text`, `write_text`, `listen_local`, `connect_*_in_background` |
| `ori.os` | `stdlib/os.orl` | `env_or`, `is_windows`/`linux`/`macos`, `current_dir_or` |
| `ori.random` | `stdlib/random.orl` | `seeded_int`, `pick_*`, `shuffle_*` |
| `ori.string` | `stdlib/string.orl` | Text helpers (see gap parity doc) |
| `ori.list` | `stdlib/list.orl` | `get_or`, `first_or`, `last_or`, `singleton` |
| `ori.convert` | `stdlib/convert.orl` | `parse_*_or` |
| `ori.map` | `stdlib/map.orl` | `get_or`, `has_key`, `is_empty` |
| `ori.set` | `stdlib/set.orl` | `contains_all`, `from_list`, `is_subset` |
| `ori.bytes` | `stdlib/bytes.orl` | Prefix/suffix, hex, list conversion |
| `ori.math` | `stdlib/math.orl` | `approx_eq`, `lerp`, trig helpers |
| `ori.json` | `stdlib/json.orl` | `read`, `write`, `write_pretty` |
| `ori.io` | `stdlib/io.orl` | `print_line`, `try_read_line` |
| `ori.fs` | `stdlib/fs.orl` | `result` wrappers over Layer 1 FS |
| `ori.time` | `stdlib/time.orl` | Durations ms, `since`/`until`, `sleep_ms` |
| `ori.test` | `stdlib/test.orl` | Assertion helpers |
| `ori.process` | `stdlib/process.orl` | Parse `run_capture` map |
| `ori.concurrent` | `stdlib/concurrent.orl` | `copy_*`, `transfer_*` aliases |
| `ori.queue` | `stdlib/queue.orl` | `from_list_*`, `peek_or_*` |
| `ori.stack` | `stdlib/stack.orl` | Idem |
| `ori.deque` | `stdlib/deque.orl` | Idem |
| `ori.heap` | `stdlib/heap.orl` | `from_list_int`, `into_sorted_int` |
| `ori.hash_table` | `stdlib/hash_table.orl` | `get_or`, `from_map_string_int` |
| `ori.linked_list` | `stdlib/linked_list.orl` | List ↔ linked list |
| `ori.doubly_linked_list` | `stdlib/doubly_linked_list.orl` | Idem |

Path convention: `ori.X.Y` → `stdlib/X/Y.orl`. Functions must be `public`.

**Known limitations:** map/set/graph/algorithms wrappers use concrete key types
(`string`, `int`) until the `Hashable` + `Equatable` trait gate ships.
`repeat` is a keyword — use `string.algorithms.repeated` instead.

### Layer 3 — `.orl` algorithms (Stdlib Phase 0 — completo)

| Module | File | Functions |
|--------|------|-----------|
| `ori.list` | `stdlib/list.orl` | `sum_int`, `binary_search_int`, `all_equal_int` |
| `ori.tree` | `stdlib/tree.orl` | `is_leaf`, `values_preorder`, `leaf_count`, `max_depth_from` |
| `ori.graph` | `stdlib/graph.orl` | BFS `has_path`, `reachable_count`, … |
| `ori.map` | `stdlib/map.orl` | `merge_string_int`, `values_sum_int`, `invert_string_int` |
| `ori.set` | `stdlib/set.orl` | `union_*`, `intersection_*`, `is_disjoint_string` |
| `ori.string` | `stdlib/string.orl` | `join_non_empty`, `repeated`, `equals_any`, `truncate` |
| `ori.bytes` | `stdlib/bytes.orl` | `compare_lex`, `is_prefix_of` |
| `ori.math` | `stdlib/math.orl` | `hypot`, `clamp_float`, `is_approx_zero` |

**Known limitations:** recursive generic functions are rejected by the
typechecker — tree algorithms use iterative stacks.

## What still blocks “production-ready” language use

See `docs/planning/stdlib-gap-parity.md` § Lacunas remanescentes and
`docs/planning/PENDENTES.md` § Backlog v2 (self-hosting, genéricos map/set em `.orl`,
rede async nativa).

## Adding a new stdlib function

### Layer 1 (runtime FFI)

1. Add entry to `STDLIB_RUNTIME_FUNCTIONS` in `stdlib.rs`.
2. Add type signature to `stdlib_func_sig()` and ABI to `stdlib_native_abi()`.
3. Implement `extern "C" fn` in `ori-runtime/src/lib.rs`.
4. Add regression test in `compiler/crates/ori-driver/tests/`.

### Layer 2 or Layer 3 (`.orl`)

1. Create or extend `stdlib/<module path>.orl` matching the namespace.
2. `import ori.<layer1_module> = <alias>` for Layer 1 primitives.
3. Declare `public ...` for cross-namespace visibility.
4. Avoid local variable name `len` (collides with `ori_len` runtime symbol).
5. Avoid keywords as function names (`repeat`, `and`, …).
6. Prefer indexed iteration over `for item in list[string]` when unsure about ARC loops.
7. Add regression test in `multifile_imports.rs`.
8. Add a sidecar `.oridoc` (same name, `.oridoc` extension) documenting the
   module (`doc module self`) and each `public` with `summary`/`param`/`returns`.
   Validate with `ori doc check stdlib/<module>.orl`. Layer 1 runtime symbols
   (no `.orl`) stay documented in `docs/spec/12-stdlib.md` + `ori doc export`.

No manifest changes needed for Layer 2/3 — the compiler discovers modules by
scanning `stdlib/` at compile time. Opaque types used in Layer 2 signatures
must be registered in `ori-types/src/lower.rs` (e.g. `ori.net.Connection`,
`ori.net.Listener`, `ori.net.UdpSocket`, `ori.io.Input`, `ori.io.Output`).

## Stdlib documentation (`.oridoc`)

Every Layer 2/3 `.orl` ships a sidecar `.oridoc` (40 files total) following
the sidecar-first philosophy of `docs/spec/17-project-and-docs.md`. Each file
documents the module (`doc module self`) and all `public` symbols with
`summary`/`param`/`returns` in English. The sidecars are consumed by:

- `ori doc check stdlib/<m>.orl` — validates syntax, symbol existence, and
  parameter names against the loaded `.orl` (exit 0 for all 40 modules).
- `ori doc file <m>.orl` — renders Markdown/HTML including sidecar entries.
- The LSP hover for stdlib Layer 2/3 symbols.
- Release packages (`stdlib/*.oridoc` are copied into the dist archive).

Layer 1 runtime symbols (no `.orl` parent) are not covered by `.oridoc` —
`ori doc check` only knows symbols defined in loaded `.orl`. Their contract
lives in `docs/spec/12-stdlib.md` (normative) and `ori doc export` (JSON).

## Stdlib root resolution

1. `ORI_STDLIB_ROOT` env var (override for tests/packaging)
2. `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode)
3. `<ori.exe dir>/stdlib` (release package)

## Surface S3

All in-repo `.orl` modules follow S3. When editing, do not reintroduce pre-S3
forms. Prefer public domain aliases for long `result[…]` returns (style 1.3).

External packages (`ori-game`, `ori-imgui`) may still be on pre-S3 until migrated.
