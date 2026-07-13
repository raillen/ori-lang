#!/usr/bin/env sh
# M1: validate a release package as an end user without using Rust tooling.
# Requires only the packaged ori binary + system linker (for AOT).
set -eu

package_root=""
allow_rust_on_path=0

usage() {
    cat <<'USAGE'
Usage: tools/smoke_no_rust.sh --package-root DIR [--allow-rust-on-path]

Runs ori doctor / run / compile / test against an already-built package tree
(the layout produced by tools/smoke_native_release.sh --keep-package or a
release archive extract).

By default fails if rustc or cargo is on PATH (CI mode). Pass
--allow-rust-on-path on developer machines that have Rust installed but still
want to exercise the packaged binary only.
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --package-root)
            package_root="${2:-}"
            shift 2
            ;;
        --allow-rust-on-path)
            allow_rust_on_path=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [ -z "$package_root" ]; then
    echo "--package-root is required" >&2
    usage >&2
    exit 2
fi

package_root=$(CDPATH= cd -- "$package_root" && pwd)

output_exe_name() {
    case "$(uname -s)" in
        MINGW*|MSYS*|CYGWIN*) printf '%s.exe\n' "$1" ;;
        *) printf '%s\n' "$1" ;;
    esac
}

ori_bin="$package_root/$(output_exe_name ori)"
if [ ! -f "$ori_bin" ]; then
    # Archive extract may keep a top-level directory (ori-VERSION-TRIPLE/)
    found=$(find "$package_root" -maxdepth 2 -type f \( -name ori -o -name ori.exe \) 2>/dev/null | head -n 1)
    if [ -n "$found" ]; then
        package_root=$(CDPATH= cd -- "$(dirname -- "$found")" && pwd)
        ori_bin="$found"
    fi
fi

if [ ! -f "$ori_bin" ]; then
    echo "ori binary not found under $package_root" >&2
    exit 1
fi

if [ "$allow_rust_on_path" -eq 0 ]; then
    if command -v rustc >/dev/null 2>&1; then
        echo "ERROR: rustc is on PATH — smoke-no-rust would be invalid" >&2
        echo "Hint: use --allow-rust-on-path on developer machines, or run in CI without Rust." >&2
        exit 1
    fi
    if command -v cargo >/dev/null 2>&1; then
        echo "ERROR: cargo is on PATH — smoke-no-rust would be invalid" >&2
        exit 1
    fi
fi

workdir="${TMPDIR:-/tmp}/ori-smoke-no-rust-$$"
mkdir -p "$workdir"
cleanup() { rm -rf "$workdir"; }
trap cleanup EXIT

cat > "$workdir/hello.orl" <<'ORI'
module app.smoke_hello

import ori.io = io

main()
    io.println("hello from smoke-no-rust")
end
ORI

cat > "$workdir/test.orl" <<'ORI'
module app.smoke_test

import ori.test = test

@test
smoke_test()
    test.assert(true, "smoke test passes")
end
ORI

echo "== doctor =="
ORI_REQUIRE_PACKAGED_RUNTIME=1 "$ori_bin" doctor

echo "== run (JIT) =="
out=$(ORI_REQUIRE_PACKAGED_RUNTIME=1 "$ori_bin" run "$workdir/hello.orl")
case "$out" in
    *"hello from smoke-no-rust"*) ;;
    *)
        echo "ori run failed or unexpected output: $out" >&2
        exit 1
        ;;
esac

echo "== compile (AOT) =="
hello_out="$workdir/$(output_exe_name hello)"
ORI_REQUIRE_PACKAGED_RUNTIME=1 "$ori_bin" compile "$workdir/hello.orl" --out "$hello_out"
compiled=$("$hello_out")
case "$compiled" in
    *"hello from smoke-no-rust"*) ;;
    *)
        echo "compiled binary unexpected output: $compiled" >&2
        exit 1
        ;;
esac

echo "== test =="
ORI_REQUIRE_PACKAGED_RUNTIME=1 "$ori_bin" test "$workdir/test.orl"

printf 'smoke-no-rust passed for package: %s\n' "$package_root"
