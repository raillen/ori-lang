#!/usr/bin/env sh
# P1: compile Ori shared lib + run C embed harness (PLANO-CDYLIB-EMBED).
set -eu
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$script_dir/../.." && pwd)
ORI_BIN="${ORI_BIN:-ori}"
WORKDIR=$(mktemp -d)
trap 'rm -rf "$WORKDIR"' EXIT

export ORI_USE_SYSTEM_LINKER="${ORI_USE_SYSTEM_LINKER:-1}"

triple=$(uname -m)-unknown-linux-gnu
case "$(uname -s)" in
    Linux) ;;
    *)
        echo "embed_smoke: skip (Linux only for P1)"
        exit 0
        ;;
esac

runtime_dir="$repo/runtime/$triple"
if [ ! -f "$runtime_dir/libori_runtime.so" ]; then
    echo "embed_smoke: staging runtime cdylib..."
    sh "$repo/tools/stage_native_runtime.sh" --profile release
fi

src="$repo/examples/embed/add_scores.orl"
if [ ! -f "$src" ]; then
    echo "embed_smoke: missing $src" >&2
    exit 1
fi

lib="$WORKDIR/libadd_scores.so"
echo "== embed_smoke: ori compile --lib =="
"$ORI_BIN" compile --lib "$src" -o "$lib"

echo "== embed_smoke: build C harness =="
cc -O2 -o "$WORKDIR/embed_smoke" "$repo/tests/native/embed_smoke.c" -ldl \
    -Wl,-rpath,"$runtime_dir" -L"$runtime_dir"

echo "== embed_smoke: run =="
# Ensure dlopen of libadd finds libori_runtime.so via rpath on the harness
# and on the Ori lib itself (set at link time).
export LD_LIBRARY_PATH="$runtime_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
ORI_EMBED_LIB="$lib" "$WORKDIR/embed_smoke"

echo "embed_smoke: ALL OK"
