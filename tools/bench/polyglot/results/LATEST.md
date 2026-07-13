# Polyglot microbench — Ori vs Python vs Rust

> **Canonical user docs:** [docs/guides/performance.md](../../../../docs/guides/performance.md)
> · [performance.pt-BR.md](../../../../docs/guides/performance.pt-BR.md)  
> **GitHub README snapshot:** [README.md § Performance](../../../../README.md#performance-snapshot)

- **When:** 2026-07-13T20:15:49-03:00  
- **Host:** Linux 6.14.0-33-generic x86_64  
- **CPU:** Intel(R) Core(TM) i7-3632QM CPU @ 2.20GHz  
- **ori:** 0.3.4 (AOT `ori compile`, system linker)  
- **python:** CPython 3.12.3  
- **rustc:** 1.95.0 (`cargo build --release`, no fat LTO)  
- **samples:** 5 (median wall time)  
- **timer:** `time.perf_counter` around subprocess (µs resolution)  
- **raw report:** `report_20260713_201549.md`

Re-run:

```bash
SAMPLES=5 ./tools/bench/polyglot/run_polyglot_bench.sh
```

## Workloads

| Workload | Work |
|----------|------|
| `sum_loop` | Σ i for i ∈ [0, 10⁷) |
| `fib_iter` | 2·10⁷ iterative fib steps (i64 wrap; Python masks to 64-bit) |
| `list_sum` | push 10⁶ ints + sequential sum |
| `nested` | 2000×2000 nested increments |

Result parity checked across Ori / Python / Rust for every kernel.

## Compile time (s, one sample)

| Workload | Ori `ori compile` | Rust `cargo build --release` (after clean) |
|----------|-------------------|--------------------------------------------|
| `sum_loop` | 1.79 | 0.67 |
| `fib_iter` | 1.93 | 0.87 |
| `list_sum` | 2.57 | 2.34 |
| `nested` | 1.74 | 1.01 |

Ori and Rust are the same order of magnitude on these tiny programs. Python has no separate compile step.

## Runtime (median wall seconds)

| Workload | Ori AOT | Python 3 | Rust release | Ori/Rust | Py/Ori | Py/Rust |
|----------|---------|----------|--------------|----------|--------|---------|
| `sum_loop` | **0.950** | 7.412 | 0.005* | 184×* | **7.8×** | 1434×* |
| `fib_iter` | **1.159** | 25.108 | 0.012 | **98×** | **21.7×** | 2128× |
| `list_sum` | **0.030** | 1.413 | 0.020 | **1.54×** | **46×** | 71× |
| `nested` | **0.485** | 1.840 | 0.006 | 86× | **3.8×** | 328× |

\* **Rust `sum_loop` is not an honest loop:** times stay ~5 ms for both N=10⁷ and N=10⁸ (LLVM closed-form / strength reduction). Prefer **`fib_iter`** and **`list_sum`** for Ori↔Rust comparison.

## How to read the numbers

### Ori vs Python (fair)

Same source shape; CPython vs Ori AOT binary.

| Kernel | Takeaway |
|--------|----------|
| pure arithmetic loops | Ori **~4–22×** faster than CPython |
| list push + sum | Ori **~46×** faster (big win: typed native list vs Python objects) |

Ori is clearly ahead of CPython on these microkernels — as expected for an AOT compiled language.

### Ori vs Rust (partly fair)

| Kernel | Takeaway |
|--------|----------|
| **`list_sum`** | Ori only **~1.5×** behind Rust release — best signal for “managed list + ARC” vs `Vec` |
| **`fib_iter`** | Ori **~100×** behind Rust on a tight dependent integer loop — room for codegen / regalloc / mid-end opts |
| **`sum_loop` / `nested`** | Rust benefits from aggressive mid-end opts; Ori runs the loop literally |

### Compile

`ori compile` on small programs: **~1.7–2.6 s**.  
`cargo build --release` after clean for equivalent Rust: **~0.7–2.3 s**.  
Not a full project compile comparison — just “hello-sized” kernels.

## Fairness / caveats

1. Same algorithm shape (`while`, explicit indices) in all three languages.  
2. Ori: AOT binary (`ori compile`), **not** JIT `ori run`.  
3. Python: CPython only (no PyPy/numba). Fib uses 64-bit mask so bigints do not dominate.  
4. Rust: `black_box` on the final value; mid-end may still rewrite simple reductions.  
5. Times include process start + one-line print.  
6. Host is a laptop CPU (i7-3632QM @ 2.2 GHz); ratios matter more than absolute ms.  
7. **Not** a language ranking for I/O, async, FFI, or real apps — only these microkernels.

## Suggested next benches (optional)

- Force Rust anti-opt with `black_box` **inside** the loop (fairer loop body).  
- Memory: peak RSS on `list_sum` growth.  
- Throughput: binary size of Ori vs Rust release.  
- JIT path: `ori run` vs AOT for short scripts.  
- Multi-file / stdlib-heavy programs (more realistic than tight loops).

## Artifacts

| Path | Role |
|------|------|
| `tools/bench/polyglot/run_polyglot_bench.sh` | runner |
| `tools/bench/polyglot/{ori,python,rust_*}/` | sources |
| `tools/bench/polyglot/results/*_*.times` | raw samples |
| `tools/bench/polyglot/results/report_*.md` | auto reports |
