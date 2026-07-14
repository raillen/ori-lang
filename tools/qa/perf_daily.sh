#!/usr/bin/env sh
# Performance / microbench daily (non-fatal unless performance_guard fails).
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$script_dir/../.." && pwd)
if [ -f "$repo/compiler/Cargo.toml" ]; then
  comp="$repo/compiler"
  tools="$repo/tools"
elif [ -f "$repo/Cargo.toml" ]; then
  comp="$repo"
  tools="$repo/tools"
else
  exit 2
fi

cd "$comp"
echo "== performance_guard =="
cargo test -p ori-driver --test performance_guard -- --quiet

if [ -x "$tools/microbench_lang_perf.sh" ]; then
  echo "== microbench_lang_perf =="
  "$tools/microbench_lang_perf.sh" || echo "microbench: non-zero (logged)"
fi

echo "perf_daily: done"
