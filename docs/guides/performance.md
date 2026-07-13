# Performance microbench (polyglot)

> **Audience:** users and contributors who want an honest snapshot of Ori
> runtime cost on small kernels.  
> **Not** a full language ranking.  
> **Portuguese:** [performance.pt-BR.md](performance.pt-BR.md)  
> **Harness:** [`tools/bench/polyglot/`](../../tools/bench/polyglot/)  
> **Latest machine report:** [`tools/bench/polyglot/results/LATEST.md`](../../tools/bench/polyglot/results/LATEST.md)

## Snapshot (2026-07-13, expanded)

| Item | Value |
|------|--------|
| Host | Linux x86_64 · Intel Core i7-3632QM @ 2.20 GHz |
| Samples | **3** (median wall time) |
| Timer | `time.perf_counter` around the process (µs) |
| Ori | **0.3.4** AOT (`ori compile`) |
| Python | CPython **3.12.3** |
| Rust | **1.95.0** release |
| C | **gcc 13.3** `-O2` |
| Go | **1.22.2** |
| JavaScript | **Node v24.18** |
| TypeScript | **tsc 7.0** → Node |
| Ruby | **3.2.3** (CRuby) |
| Nim | **1.6.14** `-d:release` |

Same algorithm shape (`while` / explicit indices). Printed results match across
all languages on every kernel.

### Runtime (median seconds)

| Workload | Ori | Python | Rust | C | Go | JS | TS | Ruby | Nim |
|----------|-----|--------|------|---|-----|----|----|------|-----|
| `sum_loop` Σ 0..10⁷ | **0.329** | 3.21 | 0.0015\* | 0.0013\* | 0.017 | 0.103 | 0.087 | 0.497 | 0.0066 |
| `fib_iter` 2·10⁷ steps | **0.649** | 11.2 | 0.0085 | 0.013 | 0.023 | 1.60 | 1.60 | 7.98 | 0.019 |
| `list_sum` 10⁶ push+sum | **0.017** | 0.998 | 0.010 | 0.011 | 0.014 | 0.142 | 0.191 | 0.272 | 0.030 |
| `nested` 2000×2000 | **0.123** | 1.04 | 0.0039 | 0.0016 | 0.0043 | 0.081 | 0.067 | 0.209 | 0.0018 |

\* Rust/C `sum_loop` may be strength-reduced (time does not always scale with N).
Prefer **`fib_iter`** and **`list_sum`** for compiled-language comparisons.

### Relative to Ori (lang / Ori; **lower is faster**)

| Workload | Py | Rust | C | Go | JS | TS | Ruby | Nim |
|----------|-----|------|---|-----|----|----|------|-----|
| `sum_loop` | 9.8× | ~0.005×\* | ~0.004×\* | 0.05× | 0.31× | 0.26× | 1.5× | 0.02× |
| `fib_iter` | **17×** | 0.013× | 0.021× | 0.036× | 2.5× | 2.5× | 12× | 0.029× |
| `list_sum` | **60×** | **0.61×** | **0.65×** | **0.86×** | 8.5× | 12× | 16× | 1.8× |
| `nested` | 8.4× | 0.031× | 0.013× | 0.035× | 0.66× | 0.54× | 1.7× | 0.014× |

## How to read this

### Ori vs interpreters

| Peer | Takeaway |
|------|----------|
| **Python** | Ori is about **8–60×** faster on these kernels |
| **Ruby** | Ori is about **1.5–16×** faster |
| **JS / TS (Node)** | Mixed: Node can beat Ori on simple arithmetic (`sum`/`nested`); Ori wins on **`fib_iter`** (~2.5×) and especially **`list_sum`** (~8–12×) |

### Ori vs AOT / systems languages

| Peer | Takeaway |
|------|----------|
| **`list_sum`** | Ori is only **~1.2–1.6×** behind Rust/C/Go — best signal for managed lists + ARC |
| **`fib_iter`** | Ori is **~30–75×** behind C/Go/Rust/Nim on a tight integer loop — codegen / mid-end gap |
| **Nim / Go** | Much faster than Ori on pure loops; closer on list churn |

### Positioning (pre-1.0)

- Clearly **ahead of CPython and CRuby** on these microkernels.
- **Competitive on list push+sum** against Rust/C/Go.
- **Behind mature AOT** (C, Rust, Go, Nim) and sometimes Node on tight arithmetic —
  room for mid-end / codegen optimisations, not “stuck as an interpreter”.

## Fairness / caveats

1. Same source shape across languages.
2. Ori path is **AOT** (`ori compile`), not JIT `ori run`.
3. Python / Ruby fib use a **64-bit mask** so bigints do not dominate.
4. JS/TS fib use BigInt with 64-bit wrap for parity with i64.
5. Nim uses `{.push overflowChecks: off.}` for wrapping i64 fib.
6. Rust may strength-reduce simple reductions (`sum_loop`).
7. Times include process start and one-line stdout.
8. Host is a laptop CPU; **ratios matter more than absolute milliseconds**.
9. This does **not** measure I/O, async, FFI, multi-file projects, or real apps.

## How to reproduce

Requires `ori`, `python3`, `cargo`/`rustc`, `gcc`, `go`, `node`, `tsc`, `ruby`, `nim`
on `PATH` (missing langs are skipped).

```bash
SAMPLES=3 ./tools/bench/polyglot/run_polyglot_bench.sh
# or SAMPLES=5 for tighter medians
```

Sources: `tools/bench/polyglot/{ori,python,rust_*,c,go,javascript,typescript,ruby,nim}/`.

## Related docs

| Document | Role |
|----------|------|
| [tools/bench/polyglot/README.md](../../tools/bench/polyglot/README.md) | Harness layout |
| [results/LATEST.md](../../tools/bench/polyglot/results/LATEST.md) | Full machine report |
| [language-comparison.md](language-comparison.md) | Older PowerShell multi-lang suite (historical) |
| [../planning/perf-baseline-2026-07-13.md](../planning/perf-baseline-2026-07-13.md) | Compiler-side LANG-PERF baseline |
