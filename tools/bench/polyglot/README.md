# Polyglot microbench — Ori / Python / Rust

Small equivalent kernels used for an honest **runtime** comparison of Ori AOT
against CPython and Rust release.

**User-facing write-up:**  
[docs/guides/performance.md](../../../docs/guides/performance.md) (EN) ·
[docs/guides/performance.pt-BR.md](../../../docs/guides/performance.pt-BR.md) (PT)

**Latest results on this machine:** [results/LATEST.md](results/LATEST.md)

## Layout

```text
polyglot/
  ori/                 Ori sources (.orl)
  python/              CPython sources
  rust_*/              one tiny Cargo package per workload
  bin/                 Ori AOT binaries (generated)
  results/             times, stdout, reports
  run_polyglot_bench.sh
```

## Workloads

| Name | What it does |
|------|----------------|
| `sum_loop` | sum `0 .. N-1` (N = 10_000_000) |
| `fib_iter` | N iterative fib steps with i64 wrap (N = 20_000_000) |
| `list_sum` | push N ints then sum (N = 1_000_000) |
| `nested` | nested loops N×N increments (N = 2000) |

Python fib masks to 64-bit so bigints do not dominate. Rust uses
`black_box` on the final printed value.

## Run

Requirements: `ori` on `PATH`, `python3`, `cargo` / `rustc`.

```bash
# from repo root
SAMPLES=5 ./tools/bench/polyglot/run_polyglot_bench.sh
```

Environment:

| Variable | Default | Meaning |
|----------|---------|---------|
| `SAMPLES` | `5` | runs per language per workload |
| `ORI_BIN` | `ori` on PATH | Ori compiler / driver |
| `PYTHON` | `python3` | Python interpreter |
| `ORI_USE_SYSTEM_LINKER` | `1` | AOT link strategy for Ori |

The script:

1. Builds Rust release binaries.
2. Compiles Ori sources with `ori compile`.
3. Times pure process execution with `time.perf_counter`.
4. Checks that stdout matches across languages.
5. Writes `results/report_YYYYMMDD_HHMMSS.md`.

Copy or refresh the human summary when numbers change:

```bash
cp tools/bench/polyglot/results/report_*.md tools/bench/polyglot/results/LATEST.md
# then edit LATEST.md / docs/guides/performance*.md with the new medians
```

## Fairness notes

- Measures **process wall time** (includes process start + one print).
- Ori path is **AOT**, not JIT `ori run`.
- Rust mid-end may strength-reduce `sum_loop` (time does not scale with N).
  Prefer `fib_iter` and `list_sum` for Ori↔Rust.
- Not a ranking for I/O, async, FFI, or real applications.

## Related

- Older multi-language suite: [`benchmarks/language-comparison/`](../../../benchmarks/language-comparison/)
- Compiler LANG-PERF baseline: [`docs/planning/perf-baseline-2026-07-13.md`](../../../docs/planning/perf-baseline-2026-07-13.md)
