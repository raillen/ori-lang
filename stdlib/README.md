# Ori Standard Library

Not yet written as `.orl` source. Spec contracts in `docs/spec/12-stdlib.md`.

## Current architecture (v1.x)

The stdlib is currently implemented as a Rust manifest plus a native runtime,
not as separate `.orl` modules. This is intentional for the v1.x timeframe:

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

## Why not `.orl` today

Bootstrap, performance, and link simplicity. The native backend (Cranelift)
links user code against a pre-compiled `libori_runtime.a`; the `extern "C"`
ABI is the stable contract between generated object code and the runtime.
Migrating stdlib to `.orl` is a v2 backlog item, not a v1 release gate; see
`docs/planning/PLANO-MATURIDADE-COMPLETO.md` Etapa 8.1.

## Adding a new stdlib function

1. Add an entry to `STDLIB_RUNTIME_FUNCTIONS` in `stdlib.rs` (canonical path,
   aliases, runtime symbol, `c_backend` flag).
2. Add the semantic type signature to `stdlib_func_sig()`.
3. Add the native ABI metadata to `stdlib_native_abi()`.
4. Implement the `extern "C" fn` in `ori-runtime/src/lib.rs`.
5. Add a regression test in `compiler/crates/ori-driver/tests/`.

The parity tests in `stdlib.rs` (`manifest_runtime_entries_have_type_and_native_abi_metadata`,
`manifest_paths_and_aliases_are_unique`) fail fast if steps 1-3 diverge.
