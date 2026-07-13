# Microbench de performance (Ori vs Python vs Rust)

> **Público:** usuários e contribuidores que querem um retrato honesto do custo
> de runtime da Ori em kernels pequenos.  
> **Não** é um ranking completo de linguagens.  
> **Inglês (canônico):** [performance.md](performance.md)  
> **Harness:** [`tools/bench/polyglot/`](../../tools/bench/polyglot/)  
> **Relatório da máquina:** [`tools/bench/polyglot/results/LATEST.md`](../../tools/bench/polyglot/results/LATEST.md)

## Snapshot (2026-07-13)

| Item | Valor |
|------|--------|
| Host | Linux x86_64 · Intel Core i7-3632QM @ 2.20 GHz |
| Ori | **0.3.4** AOT (`ori compile`) |
| Python | CPython **3.12.3** |
| Rust | **1.95.0** `cargo build --release` (sem fat LTO) |
| Amostras | 5 (mediana de wall time) |
| Timer | `time.perf_counter` em torno do processo (µs) |

Mesmo formato de algoritmo (`while` + índices explícitos) nas três linguagens.
Resultados impressos batem entre Ori / Python / Rust.

### Runtime (mediana em segundos)

| Workload | Ori AOT | Python 3 | Rust release | Py / Ori | Ori / Rust |
|----------|---------|----------|--------------|----------|------------|
| `sum_loop` — Σ i para i ∈ [0, 10⁷) | **0.95** | 7.41 | 0.005\* | **7.8×** | 184×\* |
| `fib_iter` — 2·10⁷ passos fib i64 | **1.16** | 25.1 | 0.012 | **21.7×** | **98×** |
| `list_sum` — 10⁶ push + soma | **0.030** | 1.41 | 0.020 | **46×** | **1.54×** |
| `nested` — 2000×2000 incrementos | **0.485** | 1.84 | 0.006 | **3.8×** | 86× |

\* **Rust `sum_loop` não é loop honesto:** o tempo fica ~5 ms para N = 10⁷ e
N = 10⁸ (forma fechada / *strength reduction* no LLVM). Para Ori↔Rust, prefira
**`fib_iter`** e **`list_sum`**.

### Tempo de compilação (1 amostra, programas minúsculos)

| Workload | Ori `ori compile` | Rust `cargo build --release` (após clean) |
|----------|-------------------|-------------------------------------------|
| `sum_loop` | ~1.8 s | ~0.7 s |
| `fib_iter` | ~1.9 s | ~0.9 s |
| `list_sum` | ~2.6 s | ~2.3 s |
| `nested` | ~1.7 s | ~1.0 s |

Python não tem etapa separada de compilação.

## Como ler

### Ori vs Python (comparação justa)

| Kernel | Leitura |
|--------|---------|
| Loops aritméticos | Ori cerca de **4–22×** mais rápido que CPython nestes formatos |
| Push + soma em lista | Ori cerca de **46×** mais rápido (lista tipada nativa vs objetos Python) |

Ori fica claramente à frente do CPython nestes microkernels — esperado para AOT
nativo frente a interpretador de bytecode.

### Ori vs Rust (parcialmente justa)

| Kernel | Leitura |
|--------|---------|
| **`list_sum`** | Ori só **~1.5×** atrás do Rust release — melhor sinal de lista gerenciada + ARC vs `Vec` |
| **`fib_iter`** | Ori **~100×** atrás em loop inteiro dependente — espaço para codegen / opts de mid-end |
| **`sum_loop` / `nested`** | O mid-end do Rust pode reescrever reduções simples; a Ori ainda executa o loop como escrito |

### Posicionamento (pre-1.0)

- **Acima do CPython** nestes kernels.
- **Competitiva em churn de lista** frente ao Rust release.
- **Gap grande em loops aritméticos tight** — não é “lenta como interpretador”,
  e sim **falta de otimizações** em relação a um pipeline LLVM maduro.

## Justiça / limites

1. Mesmo formato de fonte nas três linguagens.
2. Caminho Ori é **AOT** (`ori compile`), não JIT `ori run`.
3. Python é só CPython (sem PyPy / Numba). Fib usa máscara 64-bit para não
   explodir em bigint.
4. Rust usa `black_box` no valor final; o mid-end ainda pode reescrever
   reduções simples (`sum_loop`).
5. Tempos incluem start do processo e um `print` de uma linha.
6. Host é CPU de notebook; **razões importam mais que milissegundos absolutos**.
7. **Não** mede I/O, async, FFI, projetos multi-arquivo ou apps reais.

## Como reproduzir

Precisa de `ori` no `PATH`, `python3` e `cargo`/`rustc`.

```bash
SAMPLES=5 ./tools/bench/polyglot/run_polyglot_bench.sh
```

Fontes em `tools/bench/polyglot/{ori,python,rust_*}/`.  
Relatórios em `tools/bench/polyglot/results/`.

## Documentos relacionados

| Documento | Papel |
|-----------|--------|
| [tools/bench/polyglot/README.md](../../tools/bench/polyglot/README.md) | Layout e comandos do harness |
| [results/LATEST.md](../../tools/bench/polyglot/results/LATEST.md) | Relatório completo da máquina |
| [language-comparison.md](language-comparison.md) | Suite multi-linguagem mais antiga (histórico) |
| [../planning/perf-baseline-2026-07-13.md](../planning/perf-baseline-2026-07-13.md) | Baseline LANG-PERF do compilador |
| [benchmarks/language-comparison/](../../benchmarks/language-comparison/) | Suite PowerShell alternativa (C/Node/…) |
