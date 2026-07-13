# Performance microbench (Ori vs Python vs Rust)

> **Audience:** users and contributors who want an honest snapshot of Ori
> runtime cost on small kernels.  
> **Not** a full language ranking.  
> **Portuguese:** [performance.pt-BR.md](performance.pt-BR.md)  
> **Harness:** [`tools/bench/polyglot/`](../../tools/bench/polyglot/)  
> **Latest machine report:** [`tools/bench/polyglot/results/LATEST.md`](../../tools/bench/polyglot/results/LATEST.md)

## Snapshot (2026-07-13)

| Item | Value |
|------|--------|
| Host | Linux x86_64 · Intel Core i7-3632QM @ 2.20 GHz |
| Ori | **0.3.4** AOT (`ori compile`) |
| Python | CPython **3.12.3** |
| Rust | **1.95.0** `cargo build --release` (no fat LTO) |
| Samples | 5 (median wall time) |
| Timer | `time.perf_counter` around the process (µs) |

Same algorithm shape (`while` + explicit indices) in all three languages.
Printed results match across Ori / Python / Rust.

### Runtime (median seconds)

| Workload | Ori AOT | Python 3 | Rust release | Py / Ori | Ori / Rust |
|----------|---------|----------|--------------|----------|------------|
| `sum_loop` — Σ i for i ∈ [0, 10⁷) | **0.95** | 7.41 | 0.005\* | **7.8×** | 184×\* |
| `fib_iter` — 2·10⁷ i64 fib steps | **1.16** | 25.1 | 0.012 | **21.7×** | **98×** |
| `list_sum` — 10⁶ push + sum | **0.030** | 1.41 | 0.020 | **46×** | **1.54×** |
| `nested` — 2000×2000 increments | **0.485** | 1.84 | 0.006 | **3.8×** | 86× |

\* **Rust `sum_loop` is not an honest loop:** wall time stays ~5 ms for both
N = 10⁷ and N = 10⁸ (LLVM closed-form / strength reduction). Prefer
**`fib_iter`** and **`list_sum`** when comparing Ori to Rust.

### Compile time (one sample, tiny programs)

| Workload | Ori `ori compile` | Rust `cargo build --release` (after clean) |
|----------|-------------------|--------------------------------------------|
| `sum_loop` | ~1.8 s | ~0.7 s |
| `fib_iter` | ~1.9 s | ~0.9 s |
| `list_sum` | ~2.6 s | ~2.3 s |
| `nested` | ~1.7 s | ~1.0 s |

Python has no separate compile step.

## How to read this

### Ori vs Python (fair comparison)

| Kernel | Takeaway |
|--------|----------|
| Arithmetic loops | Ori is about **4–22×** faster than CPython on these shapes |
| List push + sum | Ori is about **46×** faster (typed native list vs Python objects) |

Ori is clearly ahead of CPython on these microkernels — expected for AOT native
code versus a bytecode interpreter.

### Ori vs Rust (partly fair)

| Kernel | Takeaway |
|--------|----------|
| **`list_sum`** | Ori is only **~1.5×** behind Rust release — best signal for managed lists + ARC vs `Vec` |
| **`fib_iter`** | Ori is **~100×** behind on a tight dependent integer loop — room for codegen / mid-end opts |
| **`sum_loop` / `nested`** | Rust’s mid-end can rewrite simple reductions; Ori still runs the loop as written |

### Positioning (pre-1.0)

- **Above CPython** on these kernels.
- **Competitive on list churn** against Rust release.
- **Large gap on tight arithmetic loops** — not “interpreter-slow”, but
  **missing optimisations** relative to a mature LLVM pipeline.

## Fairness / caveats

1. Same source shape across languages.
2. Ori path is **AOT** (`ori compile`), not JIT `ori run`.
3. Python is CPython only (no PyPy / Numba). Fib uses a 64-bit mask so bigints
   do not dominate.
4. Rust uses `black_box` on the final value; mid-end may still rewrite simple
   reductions (`sum_loop`).
5. Times include process start and one-line stdout.
6. Host is a laptop CPU; **ratios matter more than absolute milliseconds**.
7. This does **not** measure I/O, async, FFI, multi-file projects, or real apps.

## How to reproduce

Requires `ori` on `PATH`, `python3`, and `cargo`/`rustc`.

```bash
SAMPLES=5 ./tools/bench/polyglot/run_polyglot_bench.sh
```

Sources live under `tools/bench/polyglot/{ori,python,rust_*}/`.  
Reports land in `tools/bench/polyglot/results/`.

## Related docs

| Document | Role |
|----------|------|
| [tools/bench/polyglot/README.md](../../tools/bench/polyglot/README.md) | Harness layout and commands |
| [results/LATEST.md](../../tools/bench/polyglot/results/LATEST.md) | Full machine report |
| [language-comparison.md](language-comparison.md) | Older multi-language suite (historical) |
| [../planning/perf-baseline-2026-07-13.md](../planning/perf-baseline-2026-07-13.md) | Compiler-side LANG-PERF baseline |
| [benchmarks/language-comparison/](../../benchmarks/language-comparison/) | Alternate PowerShell suite (C/Node/…) |
