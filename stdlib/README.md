# Ori Standard Library

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
through selective imports, for example `import ori.string only (is_empty)`.
Current flattened parents:

| Module | File | Notes |
|--------|------|-------|
| `ori.string` | `stdlib/string.orl` | Flattened text helpers and string algorithms; old `ori.string.utils`/`ori.string.algorithms` stay valid |
| `ori.list` | `stdlib/list.orl` | Flattened list helpers and integer list algorithms; old `ori.list.utils`/`ori.list.algorithms` stay valid |
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
| `ori.format.utils` | `stdlib/format/utils.orl` | Wrappers `number`, `hex`, `bytes_size`, `date`, … |
| `ori.iter.utils` | `stdlib/iter/utils.orl` | `sum_int`, `contains_int`, `unique_count_int`, … |
| `ori.net.utils` | `stdlib/net/utils.orl` | `connect`, `connect_tls`, `read_text`, `write_text`, `listen_local`, `connect_*_in_background` |
| `ori.os.utils` | `stdlib/os/utils.orl` | `env_or`, `is_windows`/`linux`/`macos`, `current_dir_or` |
| `ori.random.utils` | `stdlib/random/utils.orl` | `seeded_int`, `pick_*`, `shuffle_*` |
| `ori.string.utils` | `stdlib/string/utils.orl` | Text helpers (see gap parity doc) |
| `ori.list.utils` | `stdlib/list/utils.orl` | `get_or`, `first_or`, `last_or`, `singleton` |
| `ori.convert.utils` | `stdlib/convert/utils.orl` | `parse_*_or` |
| `ori.map.utils` | `stdlib/map/utils.orl` | `get_or`, `has_key`, `is_empty` |
| `ori.set.utils` | `stdlib/set/utils.orl` | `contains_all`, `from_list`, `is_subset` |
| `ori.bytes.utils` | `stdlib/bytes/utils.orl` | Prefix/suffix, hex, list conversion |
| `ori.math.utils` | `stdlib/math/utils.orl` | `approx_eq`, `lerp`, trig helpers |
| `ori.json.utils` | `stdlib/json/utils.orl` | `read`, `write`, `write_pretty` |
| `ori.io.utils` | `stdlib/io/utils.orl` | `print_line`, `try_read_line` |
| `ori.fs.utils` | `stdlib/fs/utils.orl` | `result` wrappers over Layer 1 FS |
| `ori.time.utils` | `stdlib/time/utils.orl` | Durations ms, `since`/`until`, `sleep_ms` |
| `ori.test.utils` | `stdlib/test/utils.orl` | Assertion helpers |
| `ori.process.utils` | `stdlib/process/utils.orl` | Parse `run_capture` map |
| `ori.concurrent.utils` | `stdlib/concurrent/utils.orl` | `copy_*`, `transfer_*` aliases |
| `ori.queue.utils` | `stdlib/queue/utils.orl` | `from_list_*`, `peek_or_*` |
| `ori.stack.utils` | `stdlib/stack/utils.orl` | Idem |
| `ori.deque.utils` | `stdlib/deque/utils.orl` | Idem |
| `ori.heap.utils` | `stdlib/heap/utils.orl` | `from_list_int`, `into_sorted_int` |
| `ori.hash_table.utils` | `stdlib/hash_table/utils.orl` | `get_or`, `from_map_string_int` |
| `ori.linked_list.utils` | `stdlib/linked_list/utils.orl` | List ↔ linked list |
| `ori.doubly_linked_list.utils` | `stdlib/doubly_linked_list/utils.orl` | Idem |

Path convention: `ori.X.Y` → `stdlib/X/Y.orl`. Functions must be `public`.

**Known limitations:** map/set/graph/algorithms wrappers use concrete key types
(`string`, `int`) until the `Hashable` + `Equatable` trait gate ships.
`repeat` is a keyword — use `string.algorithms.repeated` instead.

### Layer 3 — `.orl` algorithms (Stdlib Phase 0 — completo)

| Module | File | Functions |
|--------|------|-----------|
| `ori.list.algorithms` | `stdlib/list/algorithms.orl` | `sum_int`, `binary_search_int`, `all_equal_int` |
| `ori.tree.algorithms` | `stdlib/tree/algorithms.orl` | `is_leaf`, `values_preorder`, `leaf_count`, `max_depth_from` |
| `ori.graph.algorithms` | `stdlib/graph/algorithms.orl` | BFS `has_path`, `reachable_count`, … |
| `ori.map.algorithms` | `stdlib/map/algorithms.orl` | `merge_string_int`, `values_sum_int`, `invert_string_int` |
| `ori.set.algorithms` | `stdlib/set/algorithms.orl` | `union_*`, `intersection_*`, `is_disjoint_string` |
| `ori.string.algorithms` | `stdlib/string/algorithms.orl` | `join_non_empty`, `repeated`, `equals_any`, `truncate` |
| `ori.bytes.algorithms` | `stdlib/bytes/algorithms.orl` | `compare_lex`, `is_prefix_of` |
| `ori.math.algorithms` | `stdlib/math/algorithms.orl` | `hypot`, `clamp_float`, `is_approx_zero` |

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
2. `import ori.<layer1_module> as <alias>` for Layer 1 primitives.
3. Declare `public func ...` for cross-namespace visibility.
4. Avoid local variable name `len` (collides with `ori_len` runtime symbol).
5. Avoid keywords as function names (`repeat`, `and`, …).
6. Prefer indexed iteration over `for item in list<string>` when unsure about ARC loops.
7. Add regression test in `multifile_imports.rs`.

No manifest changes needed for Layer 2/3 — the compiler discovers modules by
scanning `stdlib/` at compile time. Opaque types used in Layer 2 signatures
must be registered in `ori-types/src/lower.rs` (e.g. `ori.net.Connection`,
`ori.net.Listener`, `ori.net.UdpSocket`, `ori.io.Input`, `ori.io.Output`).

## Stdlib root resolution

1. `ORI_STDLIB_ROOT` env var (override for tests/packaging)
2. `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode)
3. `<ori.exe dir>/stdlib` (release package)
