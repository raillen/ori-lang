# Stdlib maintenance flow

Status: current as of 2026-05-17.

Goal: adding a stdlib function should not require guessing four different places.

## Source of truth

Use `compiler/crates/ori-types/src/stdlib.rs` first.

Main entries:

- `STDLIB_RUNTIME_FUNCTIONS`: canonical path, aliases, runtime symbol and backend flags.
- `stdlib_func_sig`: semantic type signature used by typecheck and HIR lowering.
- `stdlib_native_abi`: native ABI used by the Cranelift backend.
- Manifest tests at the bottom of the file: drift checks.

## Add a new runtime-backed function

- [ ] Add the canonical `ori.module.name` path to `STDLIB_RUNTIME_FUNCTIONS`.
- [ ] Add aliases only when older code or ergonomic short names need them.
- [ ] Set `native_runtime: true` through the `stdlib!` macro.
- [ ] Set `c_backend_runtime: true` only when the C/debug backend really supports the call.
- [ ] Add the semantic signature in `stdlib_func_sig`.
- [ ] Add the native ABI in `stdlib_native_abi`.
- [ ] Export the runtime symbol from `compiler/crates/ori-runtime/src/lib.rs`.
- [ ] Declare or route the symbol in `compiler/crates/ori-codegen/src/native_backend.rs` only when the generic manifest path is not enough.
- [ ] Add one typecheck test.
- [ ] Add one native execution test.
- [ ] Add a C/debug test only if `c_backend_runtime` is true.

## Add a pure compiler intrinsic

- [ ] Do not add it to `STDLIB_RUNTIME_FUNCTIONS` unless it calls a runtime symbol.
- [ ] Keep type rules in `stdlib_func_sig` or the typechecker helper that owns that family.
- [ ] Keep lowering in HIR or codegen, not both.
- [ ] Add a test proving the path does not need a runtime symbol.

## Drift checks

These tests must stay green:

- `manifest_paths_and_aliases_are_unique`
- `manifest_resolves_aliases_to_runtime_symbols`
- `manifest_runtime_entries_have_type_and_native_abi_metadata`
- `stdlib_manifest_paths_lower_to_declared_runtime_symbols`
- `native_backend_declares_manifest_runtime_symbols`

If one fails, fix the table instead of patching the failing backend locally.

## Current cleanup left

- Reduce old fallback helpers in `compiler/crates/ori-types/src/check.rs`.
- Keep compatibility aliases only when a test or public doc needs them.
- Prefer one shared table over duplicating signatures in typecheck, HIR and codegen.
