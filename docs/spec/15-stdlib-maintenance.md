# Stdlib maintenance flow

> Status: current as of 2026-06-29.
> Audience: stdlib implementers, compiler implementers
> See also: [`12-stdlib.md`](12-stdlib.md) — Implementation Architecture (v1.x)

Goal: adding a stdlib function should not require guessing four different places.

## Source of truth

The v1.x stdlib is implemented as a Rust manifest plus a native runtime, not
as separate `.orl` source modules. `compiler/crates/ori-types/src/stdlib.rs`
is the single source of truth for the stdlib contract surface.

Main entries:

- `STDLIB_RUNTIME_FUNCTIONS`: canonical path, aliases, runtime symbol and backend flags.
- `STDLIB_MODULE_ONLY_PATHS`: importable `ori.*` modules without runtime entries
  (`ori`, `ori.core`, `ori.Error`, `ori.mem` (inline intrinsics), `ori.concurrent`
  (umbrella)).
- `stdlib_func_sig`: semantic type signature used by typecheck and HIR lowering.
- `stdlib_native_abi`: native ABI used by the Cranelift backend.
- `is_implemented_stdlib_module` / `implemented_stdlib_modules`: the importable
  module set, derived from the manifest plus `STDLIB_MODULE_ONLY_PATHS`.
- `stdlib_runtime_symbol`: canonical-path → runtime symbol lookup (used by
  `ori-hir::lower::stdlib_c_name` and the C backend).

Downstream crates must not keep parallel hardcoded lists. The driver
(`pipeline::classify_stdlib_import`, `pipeline::append_stdlib_documentation`)
and HIR lowering (`lower::stdlib_c_name`) delegate to the manifest.

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
- [ ] If it owns an importable module with no other runtime entries, add the
      module path to `STDLIB_MODULE_ONLY_PATHS` (e.g. `ori.mem`).
- [ ] Keep type rules in `stdlib_func_sig` or the typechecker helper that owns that family.
- [ ] Keep lowering in HIR or codegen, not both.
- [ ] Add a test proving the path does not need a runtime symbol.

## Drift checks

These tests must stay green (in `ori-types::stdlib::tests` and
`ori-driver::pipeline::tests`):

Manifest integrity:

- `manifest_paths_and_aliases_are_unique`
- `manifest_resolves_aliases_to_runtime_symbols`
- `manifest_runtime_entries_have_type_and_native_abi_metadata`
- `stdlib_manifest_paths_lower_to_declared_runtime_symbols`
- `native_backend_declares_manifest_runtime_symbols`

Module classification (Etapa 8.1 consolidation):

- `manifest_module_prefixes_are_all_implemented`
- `implemented_stdlib_modules_covers_legacy_hardcoded_list`
- `unknown_stdlib_modules_are_rejected`
- `collection_stdlib_doc_signatures_reference_implemented_modules` (pipeline)

Spec parity:

- `spec_c_backend_matrix_matches_manifest_flags`
- `spec_fs_and_json_contracts_match_stdlib_sig`

If one fails, fix the manifest instead of patching the failing backend locally.

## Future: `.orl` modules

Migrating stdlib modules to `.orl` source is a v2 backlog item, not a v1
release gate. Hot-path modules (collections, async, I/O, ARC) will remain
runtime intrinsics; only cold compositional modules (e.g. `ori.format`
helpers, `ori.iter` combinators) are candidates for `.orl` facades over
intrinsics. See `docs/planning/PLANO-MATURIDADE-COMPLETO.md` Etapa 8.1
and Apêndice C.

## Current cleanup left

- Reduce old fallback helpers in `compiler/crates/ori-types/src/check.rs`.
- Keep compatibility aliases only when a test or public doc needs them.
- Prefer one shared table over duplicating signatures in typecheck, HIR and codegen.
