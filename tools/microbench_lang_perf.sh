#!/usr/bin/env sh
# LANG-PERF microbench: check / run (JIT) / compile (AOT) wall times.
# Usage: tools/microbench_lang_perf.sh [--skip-stage] [--ori PATH]
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ori=""
skip_stage=0
samples=3

while [ "$#" -gt 0 ]; do
    case "$1" in
        --skip-stage) skip_stage=1; shift ;;
        --ori) ori="${2:-}"; shift 2 ;;
        --samples) samples="${2:-3}"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--skip-stage] [--ori PATH] [--samples N]"
            exit 0
            ;;
        *) echo "unknown arg: $1" >&2; exit 2 ;;
    esac
done

if [ -z "$ori" ]; then
    if [ -x "$repo_root/compiler/target/release/ori" ]; then
        ori="$repo_root/compiler/target/release/ori"
    else
        echo "release ori not found; build with: (cd compiler && cargo build -p ori-driver --release)" >&2
        exit 1
    fi
fi

if [ "$skip_stage" -eq 0 ]; then
    echo "== stage runtime (release) =="
    "$repo_root/tools/stage_native_runtime.sh" --profile release
fi

# Workloads: tiny, multi-module, collections, language tour, concurrency, ARC churn
examples="hello multi_module collections_demo language_features concurrency"
arc_bench="$repo_root/tools/bench/arc_list_churn.orl"

time_cmd() {
    # print wall seconds only on stderr line "TIME <label> <sec>"
    label=$1
    shift
    if command -v /usr/bin/time >/dev/null 2>&1; then
        /usr/bin/time -f "TIME $label %e" "$@" >/dev/null
    else
        start=$(date +%s.%N)
        "$@" >/dev/null
        end=$(date +%s.%N)
        # awk for float sub
        sec=$(awk -v s="$start" -v e="$end" 'BEGIN { printf "%.3f", e - s }')
        echo "TIME $label $sec" >&2
    fi
}

echo "== binary: $ori =="
"$ori" doctor 2>/dev/null | grep -E 'linker strategy|ori run mode|native runtime' || true
echo "== samples=$samples =="

for ex in $examples; do
    path="$repo_root/examples/$ex"
    if [ ! -f "$path/main.orl" ] && [ ! -f "$path/ori.proj" ]; then
        echo "skip missing $ex"
        continue
    fi
    i=1
    while [ "$i" -le "$samples" ]; do
        time_cmd "check.$ex.$i" "$ori" check "$path" || true
        time_cmd "run.$ex.$i" "$ori" run "$path" || true
        out="/tmp/ori_bench_${ex}_$$"
        time_cmd "compile.$ex.$i" "$ori" compile "$path" --out "$out" || true
        rm -f "$out"
        i=$((i + 1))
    done
done

if [ -f "$arc_bench" ]; then
    i=1
    while [ "$i" -le "$samples" ]; do
        time_cmd "check.arc_list_churn.$i" "$ori" check "$arc_bench" || true
        time_cmd "run.arc_list_churn.$i" "$ori" run "$arc_bench" || true
        out="/tmp/ori_bench_arc_$$"
        time_cmd "compile.arc_list_churn.$i" "$ori" compile "$arc_bench" --out "$out" || true
        rm -f "$out"
        i=$((i + 1))
    done
fi

echo "== done (grep TIME lines above; lower is better) =="
