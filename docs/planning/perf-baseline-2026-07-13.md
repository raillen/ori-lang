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

### Residual (wave 1 — closed in wave 2 where noted)

- JIT cold start still ~100+ ms for hello (codegen + load runtime) — **open**.
- Multi-file / ARC-heavy program microbench suite — **added** `tools/microbench_lang_perf.sh`.
- Optional: mold/ld.lld discovery if present on PATH — **done** (SystemLinker Linux).
- Release-runtime stage default — **done** (`stage_native_runtime.*` default release).

## Wave 2 (2026-07-13) — linker PATH + stage release + microbench

### Changes

1. **Linux SystemLinker:** PATH order `mold` → `ld.lld` → `ld`, then
   `cc -print-prog-name=ld`. Override: `ORI_SYSTEM_LINKER`.
2. **Stage scripts** default **`--profile release`** (`ORI_STAGE_PROFILE` or
   `--profile debug` for runtime iteration).
3. **Harness:** `tools/microbench_lang_perf.sh` samples check/run/compile on
   `hello`, `multi_module`, `collections_demo`, `language_features`, `concurrency`.

### Numbers (release `ori` + release staged runtime + BundledRustLld)

Host Linux x86_64 · warm process · 2026-07-13 afternoon remeasure:

| Workload | Mode | Wall time |
|----------|------|-----------|
| `hello` | check | ~0.00 s |
| `hello` | run (JIT) | **~0.04–0.05 s** |
| `hello` | compile AOT (BundledRustLld) | **~0.19–0.20 s** |
| `hello` | compile AOT (`ORI_USE_SYSTEM_LINKER=1`, BFD `ld`) | ~0.36 s |
| `multi_module` | run / compile | ~0.05 s / ~0.19 s |
| `collections_demo` | run / compile | ~0.06 s / ~0.21 s |
| `language_features` | run / compile | ~0.07 s / ~0.22 s |
| `concurrency` | run / compile | ~0.04 s / ~0.19 s |

### Wins vs wave 1 baseline

| Path | Wave 1 | Wave 2 | Notes |
|------|--------|--------|-------|
| AOT `compile` hello | ~1.0 s | **~0.20 s** | release runtime + warm; still BundledRustLld |
| JIT `run` hello | ~0.11–0.15 s | **~0.04–0.05 s** | release cdylib dominates residual |
| SystemLinker vs BFD | n/a | ~0.36 s | PATH mold/lld preferred when installed |

### Residual after wave 2 (closed in wave 3 / living)

- JIT cold start (Cranelift lower + cdylib load) still dominates `ori run` tiny
  relative to AOT binary exec (~0 s) — **living**, not a v1 gate (~40–50 ms today).
- Deeper ARC end-to-end bench — **done** (`tools/bench/arc_list_churn.orl`).
- Prefer mold over BundledRustLld by default — **wontfix** for now: BundledRustLld
  is the measured product default and does not require mold installed.

## Wave 3 (2026-07-13) — ARC bench + LANG-PERF closed

### Changes

1. **`tools/bench/arc_list_churn.orl`** — 200 rounds × 500 `list.push` + nested list
   (ARC retain/release pressure).
2. **`tools/microbench_lang_perf.sh`** includes the ARC workload.
3. **BACKLOG:** `LANG-PERF` → **done**. Further JIT gains are Cranelift-bound
   living work, not an open implementation item.

### Numbers (release + staged release runtime)

| Workload | Mode | Wall time |
|----------|------|-----------|
| `arc_list_churn` | run (JIT) | ~0.05 s |
| `arc_list_churn` | compile AOT | ~0.20 s |
| `arc_list_churn` AOT binary | execute | ~instant (workload itself is light at N=200×500) |

### How to re-measure

```bash
cd compiler && cargo build -p ori-driver --release
../tools/stage_native_runtime.sh          # release runtime + rust-lld by default
../tools/microbench_lang_perf.sh --skip-stage
# or one-shot:
/usr/bin/time -f '%e' ./target/release/ori compile ../examples/hello --out /tmp/h
/usr/bin/time -f '%e' ./target/release/ori run ../examples/hello
/usr/bin/time -f '%e' ./target/release/ori run ../tools/bench/arc_list_churn.orl
```
