# Performance baseline (LANG-PERF)

> Host: Linux x86_64 · Date: 2026-07-13  
> Binary: `compiler/target/release/ori`  
> Runtime: staged under `runtime/<triple>/` (includes `runtime/bin/rust-lld`)

## Debug (pre-work, for comparison)

| Workload | Mode | Wall time |
|----------|------|-----------|
| `examples/hello` | check | ~0.02 s |
| `examples/hello` | run (JIT) | ~0.50 s |
| `examples/hello` | compile AOT | (not measured) |

## Release — before LANG-PERF changes

| Workload | Mode | Wall time (≈ 3 samples) |
|----------|------|-------------------------|
| `examples/hello` | check | ~0.00–0.00 s |
| `examples/hello` | run (JIT) | ~0.13–0.16 s |
| `examples/calculator` | run (JIT) | ~0.12–0.31 s |
| `examples/language_features` | run (JIT) | ~0.17–0.19 s |
| `examples/collections_demo` | run (JIT) | ~0.14–0.18 s |
| `examples/hello` | compile AOT (system `ld`) | ~4.0 s first / ~2.5 s second |
| AOT binary only | execute | ~0.00 s |

Default linker then: **SystemLinker** (`ld`).

## Release — after LANG-PERF (this wave)

### Changes

1. **Cranelift product flags:** `enable_verifier=false`; AOT `opt_level=speed`;
   JIT `opt_level=none` (fast lower for `ori run`).
2. **Default linker order:** discover **BundledRustLld** first when available
   (packaged `runtime/bin/rust-lld`), then SystemLinker, then rustc driver.
   Still no `rustc` for end users with package layout.

### Numbers

| Workload | Mode | Wall time (≈ samples) |
|----------|------|------------------------|
| `examples/hello` | check | ~0.00–0.02 s |
| `examples/hello` | run (JIT) | ~0.11–0.27 s (median ≈ 0.15) |
| `examples/calculator` | run (JIT) | ~0.10–0.20 s |
| `examples/language_features` | run (JIT) | ~0.12–0.23 s |
| `examples/collections_demo` | run (JIT) | ~0.09–0.15 s |
| `examples/hello` | compile AOT | **~0.93–1.03 s** |
| AOT binary only | execute | ~0.00 s |

`ori doctor` reports: `linker strategy — BundledRustLld (default)`.

### Wins

| Path | Before | After | Notes |
|------|--------|-------|-------|
| AOT `ori compile` hello | ~2.5–4 s | **~1.0 s** | ~2.5–4× faster with default packaged lld + CL flags |
| JIT `ori run` tiny | ~0.13–0.16 s | ~0.11–0.15 s | modest; dominated by Cranelift lower + cdylib |
| `ori check` tiny | already sub-20 ms | unchanged | |

### Residual (not done this wave)

- JIT cold start still ~100+ ms for hello (codegen + load runtime).
- Multi-file / ARC-heavy program microbench suite.
- Optional: mold/ld.lld discovery if present on PATH.
- Release-runtime stage in default `stage_native_runtime.sh` (currently debug artifacts).

### How to re-measure

```bash
cd compiler && cargo build -p ori-driver --release
../tools/stage_native_runtime.sh   # or package layout with runtime/bin/rust-lld
/usr/bin/time -f '%e' ./target/release/ori compile ../examples/hello --out /tmp/h
/usr/bin/time -f '%e' ./target/release/ori run ../examples/hello
```
