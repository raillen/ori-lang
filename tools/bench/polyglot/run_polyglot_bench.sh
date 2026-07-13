#!/usr/bin/env sh
# Polyglot runtime microbench: Ori (AOT) vs Python 3 vs Rust (release).
# Measures pure execution wall time (compile timed separately for Ori/Rust).
# Timing: Python time.perf_counter (µs), not /usr/bin/time (centisecond floor).
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$root/../../.." && pwd)
if [ ! -d "$repo/tools" ]; then
  repo=$(CDPATH= cd -- "$root/../.." && pwd)
fi

ORI_BIN="${ORI_BIN:-$(command -v ori)}"
PYTHON="${PYTHON:-python3}"
samples="${SAMPLES:-5}"
out_dir="$root/results"
mkdir -p "$out_dir" "$root/bin"
stamp=$(date -Iseconds 2>/dev/null || date)
report="$out_dir/report_$(date +%Y%m%d_%H%M%S).md"
: >"$out_dir/compile_times.txt"

workloads="sum_loop fib_iter list_sum nested"

median() {
  sort -n | awk '
    { a[NR]=$1 }
    END {
      if (NR==0) { print "nan"; exit }
      if (NR%2==1) printf "%.6f", a[(NR+1)/2]
      else printf "%.6f", (a[NR/2]+a[NR/2+1])/2
    }'
}

mean() {
  awk '{s+=$1;n++} END{if(n) printf "%.6f", s/n; else print "nan"}'
}

# High-res wall time of a command; stdout of the program goes to $1, elapsed to stdout.
time_cmd() {
  out_file=$1
  shift
  "$PYTHON" - "$out_file" "$@" <<'PY'
import subprocess, sys, time
out_path = sys.argv[1]
cmd = sys.argv[2:]
t0 = time.perf_counter()
with open(out_path, "w", encoding="utf-8") as f:
    r = subprocess.run(cmd, stdout=f, stderr=subprocess.PIPE, text=True)
t1 = time.perf_counter()
if r.returncode != 0:
    sys.stderr.write(r.stderr or "")
    sys.exit(r.returncode)
print(f"{t1 - t0:.6f}")
PY
}

echo "# Polyglot bench report" >"$report"
echo "" >>"$report"
echo "- **When:** $stamp" >>"$report"
echo "- **Host:** $(uname -srm)" >>"$report"
echo "- **CPU:** $(grep -m1 'model name' /proc/cpuinfo 2>/dev/null | cut -d: -f2 | sed 's/^ //' || echo unknown)" >>"$report"
echo "- **ori:** $($ORI_BIN --version 2>/dev/null || echo "$ORI_BIN")" >>"$report"
echo "- **python:** $($PYTHON --version 2>&1)" >>"$report"
echo "- **rustc:** $(rustc --version)" >>"$report"
echo "- **samples per cell:** $samples" >>"$report"
echo "- **timer:** Python \`time.perf_counter\` (subprocess wall time)" >>"$report"
echo "" >>"$report"
echo "## Workloads" >>"$report"
echo "" >>"$report"
echo "| Workload | Description |" >>"$report"
echo "|----------|-------------|" >>"$report"
echo "| sum_loop | sum 0..10_000_000-1 |" >>"$report"
echo "| fib_iter | iterative fib **20_000_000 steps** (i64 wrap; Python masks to 64-bit) |" >>"$report"
echo "| list_sum | push 1_000_000 ints + sum |" >>"$report"
echo "| nested | nested loops 2000×2000 |" >>"$report"
echo "" >>"$report"

echo "== compile Rust release =="
for w in $workloads; do
  (cd "$root/rust_$w" && cargo build --release -q)
done

echo "== compile Ori AOT =="
export ORI_USE_SYSTEM_LINKER="${ORI_USE_SYSTEM_LINKER:-1}"
for w in $workloads; do
  src="$root/ori/${w}.orl"
  bin="$root/bin/ori_${w}"
  echo "  ori compile $w"
  t0=$("$PYTHON" -c 'import time; print(time.perf_counter())')
  "$ORI_BIN" compile "$src" --out "$bin"
  t1=$("$PYTHON" -c 'import time; print(time.perf_counter())')
  ct=$("$PYTHON" -c "print(f'{$t1 - $t0:.3f}')")
  echo "  compile_time_ori_$w ${ct}s"
  echo "ori_compile_${w}=${ct}" >>"$out_dir/compile_times.txt"
done

echo "== compile times Rust (release rebuild after clean) =="
for w in $workloads; do
  (
    cd "$root/rust_$w"
    cargo clean -q
    t0=$("$PYTHON" -c 'import time; print(time.perf_counter())')
    cargo build --release -q
    t1=$("$PYTHON" -c 'import time; print(time.perf_counter())')
    "$PYTHON" -c "print(f'rust_compile_${w}={$t1 - $t0:.3f}')"
  )
done | tee -a "$out_dir/compile_times.txt"

# re-ensure release bins
for w in $workloads; do
  (cd "$root/rust_$w" && cargo build --release -q)
done

echo "" >>"$report"
echo "## Compile time (seconds, one sample)" >>"$report"
echo "" >>"$report"
echo "| Workload | Ori \`ori compile\` | Rust \`cargo build --release\` (after clean) |" >>"$report"
echo "|----------|-------------------|-----------------------------------------------|" >>"$report"

for w in $workloads; do
  src="$root/ori/${w}.orl"
  bin="$root/bin/ori_${w}"
  t0=$("$PYTHON" -c 'import time; print(time.perf_counter())')
  "$ORI_BIN" compile "$src" --out "$bin" >/dev/null
  t1=$("$PYTHON" -c 'import time; print(time.perf_counter())')
  oct=$("$PYTHON" -c "print(f'{$t1 - $t0:.3f}')")
  rct=$(grep "rust_compile_${w}=" "$out_dir/compile_times.txt" | cut -d= -f2)
  echo "| \`$w\` | ${oct} | ${rct:-?} |" >>"$report"
done

echo "" >>"$report"
echo "## Runtime wall time (seconds)" >>"$report"
echo "" >>"$report"
echo "Pure process execution (program already compiled for Ori/Rust). Median of $samples samples." >>"$report"
echo "" >>"$report"
echo "| Workload | Ori AOT (median) | Python 3 (median) | Rust release (median) | Ori/Rust | Py/Ori | Py/Rust |" >>"$report"
echo "|----------|------------------|-------------------|------------------------|----------|--------|---------|" >>"$report"

echo "== run benchmarks (samples=$samples) =="
for w in $workloads; do
  echo "-- $w --"
  : >"$out_dir/${w}_ori.times"
  : >"$out_dir/${w}_py.times"
  : >"$out_dir/${w}_rs.times"

  # warmup once each (not recorded)
  "$root/bin/ori_${w}" >/dev/null
  "$PYTHON" "$root/python/${w}.py" >/dev/null
  "$root/rust_${w}/target/release/rust_${w}" >/dev/null

  i=1
  while [ "$i" -le "$samples" ]; do
    time_cmd "$out_dir/${w}_ori.out" "$root/bin/ori_${w}" >>"$out_dir/${w}_ori.times"
    time_cmd "$out_dir/${w}_py.out" "$PYTHON" "$root/python/${w}.py" >>"$out_dir/${w}_py.times"
    time_cmd "$out_dir/${w}_rs.out" "$root/rust_${w}/target/release/rust_${w}" >>"$out_dir/${w}_rs.times"
    i=$((i + 1))
  done

  o=$(tr -d '\r\n' <"$out_dir/${w}_ori.out")
  p=$(tr -d '\r\n' <"$out_dir/${w}_py.out")
  r=$(tr -d '\r\n' <"$out_dir/${w}_rs.out")
  if [ "$o" != "$p" ] || [ "$o" != "$r" ]; then
    echo "WARN result mismatch $w ori=$o py=$p rs=$r" >&2
  else
    echo "  result ok: $o"
  fi

  om=$(median <"$out_dir/${w}_ori.times")
  pm=$(median <"$out_dir/${w}_py.times")
  rm_=$(median <"$out_dir/${w}_rs.times")
  ratio_or=$("$PYTHON" -c "a=float('$om'); b=float('$rm_'); print(f'{a/b:.2f}' if b>0 else 'n/a')")
  ratio_po=$("$PYTHON" -c "a=float('$pm'); b=float('$om'); print(f'{a/b:.2f}' if b>0 else 'n/a')")
  ratio_pr=$("$PYTHON" -c "a=float('$pm'); b=float('$rm_'); print(f'{a/b:.2f}' if b>0 else 'n/a')")
  echo "| \`$w\` | ${om} | ${pm} | ${rm_} | ${ratio_or}× | ${ratio_po}× | ${ratio_pr}× |" >>"$report"
  printf "  ori med=%s  py med=%s  rust med=%s\n" "$om" "$pm" "$rm_"
  printf "  raw ori: "; tr '\n' ' ' <"$out_dir/${w}_ori.times"; echo
  printf "  raw py:  "; tr '\n' ' ' <"$out_dir/${w}_py.times"; echo
  printf "  raw rs:  "; tr '\n' ' ' <"$out_dir/${w}_rs.times"; echo
done

echo "" >>"$report"
echo "## Notes / fairness" >>"$report"
echo "" >>"$report"
echo "- Same algorithm shape (while loops, explicit indices) across languages." >>"$report"
echo "- Rust: \`cargo build --release\` without fat LTO; \`black_box\` on final result so the value is not DCE'd." >>"$report"
echo "- Python: CPython 3, no PyPy / no numba; fib uses 64-bit mask (no bigint blow-up)." >>"$report"
echo "- Ori: AOT binary via \`ori compile\` (not JIT \`ori run\`)." >>"$report"
echo "- Times include process start + one-line stdout." >>"$report"
echo "- Fib is **N iterative steps**, not classic F(40) (too small to measure)." >>"$report"
echo "- Caveat: Rust may strength-reduce \`sum_loop\` (time does not scale with N); prefer fib/list for Ori↔Rust." >>"$report"
echo "- This is **not** a language ranking for all domains — only these microkernels." >>"$report"
echo "" >>"$report"
echo "## Raw times" >>"$report"
echo "" >>"$report"
echo "See \`tools/bench/polyglot/results/*_*.times\`." >>"$report"

echo ""
echo "Report written: $report"
cat "$report"
