# Microbench de performance (polyglot)

> **Público:** usuários e contribuidores que querem um retrato honesto do custo
> de runtime da Ori em kernels pequenos.  
> **Não** é um ranking completo de linguagens.  
> **Inglês (canônico):** [performance.md](performance.md)  
> **Harness:** [`tools/bench/polyglot/`](../../tools/bench/polyglot/)  
> **Relatório da máquina:** [`tools/bench/polyglot/results/LATEST.md`](../../tools/bench/polyglot/results/LATEST.md)

## Snapshot (2026-07-13, expandido)

| Item | Valor |
|------|--------|
| Host | Linux x86_64 · Intel Core i7-3632QM @ 2.20 GHz |
| Amostras | **3** (mediana de wall time) |
| Timer | `time.perf_counter` em torno do processo (µs) |
| Ori | **0.3.4** AOT (`ori compile`) |
| Python | CPython **3.12.3** |
| Rust | **1.95.0** release |
| C | **gcc 13.3** `-O2` |
| Go | **1.22.2** |
| JavaScript | **Node v24.18** |
| TypeScript | **tsc 7.0** → Node |
| Ruby | **3.2.3** (CRuby) |
| Nim | **1.6.14** `-d:release` |

Mesmo formato de algoritmo (`while` / índices explícitos). Resultados impressos
batem em todas as linguagens em todos os kernels.

### Runtime (mediana em segundos)

| Workload | Ori | Python | Rust | C | Go | JS | TS | Ruby | Nim |
|----------|-----|--------|------|---|-----|----|----|------|-----|
| `sum_loop` Σ 0..10⁷ | **0.329** | 3.21 | 0.0015\* | 0.0013\* | 0.017 | 0.103 | 0.087 | 0.497 | 0.0066 |
| `fib_iter` 2·10⁷ passos | **0.649** | 11.2 | 0.0085 | 0.013 | 0.023 | 1.60 | 1.60 | 7.98 | 0.019 |
| `list_sum` 10⁶ push+soma | **0.017** | 0.998 | 0.010 | 0.011 | 0.014 | 0.142 | 0.191 | 0.272 | 0.030 |
| `nested` 2000×2000 | **0.123** | 1.04 | 0.0039 | 0.0016 | 0.0043 | 0.081 | 0.067 | 0.209 | 0.0018 |

\* Rust/C em `sum_loop` podem reduzir a forma fechada. Prefira **`fib_iter`** e
**`list_sum`** para comparar AOT.

### Relativo à Ori (lang / Ori; **menor é mais rápido**)

| Workload | Py | Rust | C | Go | JS | TS | Ruby | Nim |
|----------|-----|------|---|-----|----|----|------|-----|
| `sum_loop` | 9.8× | ~0.005×\* | ~0.004×\* | 0.05× | 0.31× | 0.26× | 1.5× | 0.02× |
| `fib_iter` | **17×** | 0.013× | 0.021× | 0.036× | 2.5× | 2.5× | 12× | 0.029× |
| `list_sum` | **60×** | **0.61×** | **0.65×** | **0.86×** | 8.5× | 12× | 16× | 1.8× |
| `nested` | 8.4× | 0.031× | 0.013× | 0.035× | 0.66× | 0.54× | 1.7× | 0.014× |

## Como ler

### Ori vs interpretadores

| Par | Leitura |
|-----|---------|
| **Python** | Ori cerca de **8–60×** mais rápida |
| **Ruby** | Ori cerca de **1.5–16×** mais rápida |
| **JS / TS (Node)** | Misto: Node pode ganhar em aritmética simples; Ori ganha em **`fib_iter`** (~2.5×) e sobretudo **`list_sum`** (~8–12×) |

### Ori vs AOT / sistemas

| Par | Leitura |
|-----|---------|
| **`list_sum`** | Ori só **~1.2–1.6×** atrás de Rust/C/Go — melhor sinal de lista + ARC |
| **`fib_iter`** | Ori **~30–75×** atrás de C/Go/Rust/Nim em loop inteiro tight — gap de codegen |
| **Nim / Go** | Muito mais rápidos em loops puros; mais perto em churn de lista |

### Posicionamento (pre-1.0)

- Claramente **acima de CPython e CRuby**.
- **Competitiva em push+soma de lista** frente a Rust/C/Go.
- **Atrás de AOT maduro** (C, Rust, Go, Nim) e às vezes do Node em aritmética
  tight — espaço para otimizações de mid-end/codegen.

## Justiça / limites

1. Mesmo formato de fonte nas linguagens.
2. Ori é **AOT** (`ori compile`), não JIT `ori run`.
3. Python / Ruby: máscara 64-bit no fib (sem explosão de bigint).
4. JS/TS: BigInt com wrap 64-bit.
5. Nim: `{.push overflowChecks: off.}` no fib wrapping.
6. Rust pode reduzir `sum_loop`.
7. Tempos incluem start do processo + um `print`.
8. Host é notebook; **razões importam mais que ms absolutos**.
9. **Não** mede I/O, async, FFI ou apps reais.

## Como reproduzir

```bash
SAMPLES=3 ./tools/bench/polyglot/run_polyglot_bench.sh
```

Fontes em `tools/bench/polyglot/{ori,python,rust_*,c,go,javascript,typescript,ruby,nim}/`.

## Documentos relacionados

| Documento | Papel |
|-----------|--------|
| [tools/bench/polyglot/README.md](../../tools/bench/polyglot/README.md) | Layout do harness |
| [results/LATEST.md](../../tools/bench/polyglot/results/LATEST.md) | Relatório completo |
| [language-comparison.md](language-comparison.md) | Suite PowerShell antiga (histórico) |
| [../planning/perf-baseline-2026-07-13.md](../planning/perf-baseline-2026-07-13.md) | Baseline LANG-PERF do compilador |
