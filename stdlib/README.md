# Ori Standard Library

The stdlib has three layers: a Rust manifest + native runtime (Layer 1),
`.orl` compositional wrappers (Layer 2), and `.orl` pure algorithms (Layer 3).
Spec contracts in `docs/spec/12-stdlib.md`.

## Current architecture (v1.x + Stdlib Phase 0)

### Layer 1 — Rust runtime (manifest-only, never ported to `.orl`)

Low-level primitives implemented as `extern "C"` functions in the native
runtime. These are the stable ABI contract between generated object code and
the runtime. **Frozen** — hot path (ARC, async executor, I/O, string allocators,
collections FFI) stays here permanently.

- **Manifest:** `compiler/crates/ori-types/src/stdlib.rs`
- **Runtime:** `compiler/crates/ori-runtime/src/lib.rs`
- **Spec:** `docs/spec/12-stdlib.md`

### Layer 2 — `.orl` safe wrappers (Stdlib Phase 0+)

Higher-level functions implemented in `.orl` that call Layer 1 primitives via
the normal `import` mechanism.

| Module | File | Functions |
|--------|------|-----------|
| `ori.validate` | `stdlib/validate.orl` | `between`, `positive`, `non_negative`, `one_of`, `not_empty`, `min_length`, `max_length`, … |
| `ori.path` | `stdlib/path.orl` | `join`, `normalize`, `base_name`, `extension`, `parent`, `is_absolute`, … |
| `ori.string.utils` | `stdlib/string/utils.orl` | `is_empty`, `blank`, `replicate`, `default`, `equals_ignore_case`, `center`, `count`, `reverse`, `capitalize`, `title`, `swap_case`, `lines`, `left`, `right`, `words`, `trim_all`, `last_index_of`, `is_digits`, `has_whitespace`, `limit`, `replace_all`, `has_prefix`, `has_suffix` |
| `ori.list.utils` | `stdlib/list/utils.orl` | `get_or`, `first_or`, `last_or`, `singleton` |
| `ori.convert.utils` | `stdlib/convert/utils.orl` | `parse_int_or`, `parse_float_or`, `parse_bool_or` |
| `ori.map.utils` | `stdlib/map/utils.orl` | `get_or`, `get_or_string`, `contains_key`, `has_key`, `is_empty` |
| `ori.set.utils` | `stdlib/set/utils.orl` | `contains_all`, `from_list`, `is_subset`, `contains_all_int` |
| `ori.bytes.utils` | `stdlib/bytes/utils.orl` | `is_empty`, `equals`, `from_hex_or`, `empty_bytes`, `starts_with`, `ends_with`, `contains`, `join`, `from_list`, `to_list` |
| `ori.math.utils` | `stdlib/math/utils.orl` | `sign`, `approx_eq`, `clamp_int`, `lerp`, `deg_to_rad`, `rad_to_deg`, `trunc_float`, `log10`, `abs_float` |
| `ori.json.utils` | `stdlib/json/utils.orl` | `read`, `write`, `write_pretty` |
| `ori.io.utils` | `stdlib/io/utils.orl` | `print_line`, `try_read_line`, `write` |
| `ori.fs.utils` | `stdlib/fs/utils.orl` | `read_text_or`, `write_text_result`, `create_dir_all`, `exists_result`, `remove_file`, `move_path` |
| `ori.time.utils` | `stdlib/time/utils.orl` | `milliseconds`, `seconds`, `minutes`, `hours`, `since`, `until`, `sleep_ms`, … |
| `ori.test.utils` | `stdlib/test/utils.orl` | `is_true`, `is_false`, `equal_int`, `equal_text`, … |
| `ori.process.utils` | `stdlib/process/utils.orl` | `exit_code`, `stdout`, `stderr` |
| `ori.concurrent.utils` | `stdlib/concurrent/utils.orl` | `copy_*` helpers for `Transferable` types |

Path convention: `ori.X.Y` → `stdlib/X/Y.orl`. Functions must be `public`.

**Known limitations:** map/set/graph wrappers use concrete key types (`string`,
`int`) — generic `K`/`N` type parameters are rejected until the `Hashable` +
`Equatable` trait gate is implemented for user-defined keys.

### Layer 3 — `.orl` algorithms (Stdlib Phase 0+)

Pure-Ori algorithms on top of Layer 1+2. No runtime traversal shortcuts —
implemented with stacks/queues in `.orl` where possible.

| Module | File | Functions |
|--------|------|-----------|
| `ori.list.algorithms` | `stdlib/list/algorithms.orl` | `sum_int`, `binary_search_int`, `all_equal_int` |
| `ori.tree.algorithms` | `stdlib/tree/algorithms.orl` | `is_leaf`, `values_preorder`, `leaf_count`, `max_depth_from` |
| `ori.graph.algorithms` | `stdlib/graph/algorithms.orl` | `has_path`, `reachable_count`, `is_reachable`, `has_path_int` |

**Known limitations:** recursive generic functions are rejected by the
typechecker (`generic.circular_instantiation`) — Layer 3 tree algorithms use
iterative stacks instead of recursive generic helpers.

## Why a hybrid Layer 1 + Layer 2 + Layer 3 approach

Layer 1 (Rust runtime) stays for low-level operations that need direct memory
access, FFI, or performance. Layer 2 (`.orl`) grows the stdlib with readable
compositional helpers. Layer 3 (`.orl`) proves the language can express
non-trivial algorithms without new runtime symbols.

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
5. Prefer indexed iteration over `for item in list<string>` until ARC loop
   binding is fully hardened (see `emit_for_element_binding` in native backend).
6. Add regression test in `multifile_imports.rs`.

No manifest changes needed for Layer 2/3 — the compiler discovers modules by
scanning `stdlib/` at compile time.

## Stdlib root resolution

1. `ORI_STDLIB_ROOT` env var (override for tests/packaging)
2. `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode)
3. `<ori.exe dir>/stdlib` (release package)
