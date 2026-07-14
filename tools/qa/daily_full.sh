#!/usr/bin/env sh
# Ori language QA — full daily/weekly (S0–S8).
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
"$script_dir/daily_fast.sh"

repo=$(CDPATH= cd -- "$script_dir/../.." && pwd)
if [ -f "$repo/compiler/Cargo.toml" ]; then
  comp="$repo/compiler"
elif [ -f "$repo/Cargo.toml" ]; then
  comp="$repo"
else
  echo "workspace not found" >&2
  exit 2
fi
cd "$comp"

echo "== S4 multifile_imports =="
cargo test -p ori-driver --test multifile_imports -- --quiet

echo "== S3 concurrency_async full =="
cargo test -p ori-driver --test concurrency_async -- --quiet

echo "== S5 cargo test --workspace =="
cargo test --workspace -- --quiet

echo "== S6 examples smoke =="
if [ -x "$script_dir/examples_smoke.sh" ]; then
  "$script_dir/examples_smoke.sh"
fi

echo "== S6b packages web SEC8 =="
if [ -x "$script_dir/web_sec8.sh" ]; then
  "$script_dir/web_sec8.sh"
fi

echo "== S7 perf =="
if [ -x "$script_dir/perf_daily.sh" ]; then
  "$script_dir/perf_daily.sh" || true
fi

echo "daily_full: OK"
