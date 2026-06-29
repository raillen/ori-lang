# Ori Standard Library

The stdlib has two layers: a Rust manifest + native runtime (Layer 1), and
`.orl` source modules (Layer 2+). Spec contracts in `docs/spec/12-stdlib.md`.

## Current architecture (v1.x + Stdlib Phase 0)

### Layer 1 — Rust runtime (manifest-only, never ported to `.orl`)

Low-level primitives implemented as `extern "C"` functions in the native
runtime. These are the stable ABI contract between generated object code and
the runtime.

- **Manifest:** `compiler/crates/ori-types/src/stdlib.rs`
  - `STDLIB_RUNTIME_FUNCTIONS` is the single source of truth for stdlib
    path -> runtime symbol mapping, type signatures, and native ABI metadata.
  - `is_implemented_stdlib_module()` and `implemented_stdlib_modules()` derive
    the importable `ori.*` module set from the manifest plus
    `STDLIB_MODULE_ONLY_PATHS` (a small allowlist for modules without runtime
    entries: `ori`, `ori.core`, `ori.Error`, `ori.mem`, `ori.concurrent`).
- **Runtime:** `compiler/crates/ori-runtime/src/lib.rs`
  - `extern "C"` functions that implement each manifest symbol
  (`ori_io_print`, `ori_bytes_len`, etc.).
- **Spec:** `docs/spec/12-stdlib.md` documents the public API contract.

### Layer 2 — `.orl` safe wrappers (Stdlib Phase 0+)

Higher-level functions implemented in `.orl` that call Layer 1 primitives via
the normal `import` mechanism. These are loaded as a prelude by the compiler
when a user (or another `.orl` module) imports them.

- **Location:** `stdlib/**/*.orl` (convention: `ori.X.Y` -> `stdlib/X/Y.orl`)
- **Loading:** `ori-driver/src/pipeline.rs` discovers `.orl` source modules
  in `classify_stdlib_import` (new `StdlibSource(PathBuf)` status) and loads
  them via the same `load_source_recursive` path as user files.
- **Stdlib root resolution:** `ORI_STDLIB_ROOT` env var ->
  `CARGO_MANIFEST_DIR/../../../stdlib` (dev) -> `<ori.exe dir>/stdlib`
  (release package).
- **Visibility:** functions in `.orl` stdlib modules must be declared
  `public` to be callable from other namespaces (same rule as user code).
- **First module (Stdlib Phase 0):** `stdlib/string/utils.orl`
  - `namespace ori.string.utils`
  - `import ori.string as str` (Layer 1)
  - `public func is_empty(s: string) -> bool` — uses `str.len`
  - `public func blank(s: string) -> bool` — uses `str.trim` + `is_empty`
    (Layer 2 calling Layer 2)
  - `public func replicate(s: string, n: int) -> string` — `while` loop +
    `str.concat`

### Layer 3 — `.orl` algorithms (future, long-term)

Pure-Ori algorithms and data structures on top of Layer 1+2 (e.g. `ori.tree`
traversals). Not yet started.

## Why a hybrid Layer 1 + Layer 2 approach

Layer 1 (Rust runtime) stays for low-level operations that need direct memory
access, FFI, or performance (allocators, ARC, async executor, string
primitives like `concat`/`slice` that allocate). Layer 2 (`.orl`) lets the
stdlib grow in Ori itself — demonstrating the language is self-sufficient and
keeping higher-level logic readable and auditable in `.orl`. The boundary is:
if it can be expressed in Ori on top of Layer 1 primitives, it goes in Layer 2.

## Adding a new stdlib function

### Layer 1 (runtime FFI)

1. Add an entry to `STDLIB_RUNTIME_FUNCTIONS` in `stdlib.rs` (canonical path,
   aliases, runtime symbol, `c_backend` flag).
2. Add the semantic type signature to `stdlib_func_sig()`.
3. Add the native ABI metadata to `stdlib_native_abi()`.
4. Implement the `extern "C" fn` in `ori-runtime/src/lib.rs`.
5. Add a regression test in `compiler/crates/ori-driver/tests/`.

The parity tests in `stdlib.rs` (`manifest_runtime_entries_have_type_and_native_abi_metadata`,
`manifest_paths_and_aliases_are_unique`) fail fast if steps 1-3 diverge.

### Layer 2 (`.orl` wrapper)

1. Create or extend a file under `stdlib/<module path>.orl` matching the
   namespace (e.g. `ori.string.utils` -> `stdlib/string/utils.orl`).
2. Declare `namespace ori.X.Y` at the top.
3. `import ori.<layer1_module> as <alias>` to access Layer 1 primitives.
4. Define functions with `public func ...` so they are callable cross-namespace.
5. Add a regression test in `compiler/crates/ori-driver/tests/multifile_imports.rs`
   that imports the new module and validates behavior end-to-end.

No manifest changes are needed for Layer 2 modules — the compiler discovers
them by scanning `stdlib/` at compile time.
