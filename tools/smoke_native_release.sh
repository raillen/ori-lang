#!/usr/bin/env sh
set -eu

package_root=""
skip_build=0
keep_package=0

usage() {
    cat <<'USAGE'
Usage: tools/smoke_native_release.sh [--package-root DIR] [--skip-build] [--keep-package]

Builds a release-style Ori package in a clean folder, stages the native runtime,
then verifies `ori compile` and `ori test` using only package-local files.
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --package-root)
            package_root="${2:-}"
            shift 2
            ;;
        --skip-build)
            skip_build=1
            shift
            ;;
        --keep-package)
            keep_package=1
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

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)

host_triple() {
    rustc -vV | awk -F': ' '/^host:/ { print $2; exit }'
}

output_exe_name() {
    case "$(uname -s)" in
        MINGW*|MSYS*|CYGWIN*) printf '%s.exe\n' "$1" ;;
        *) printf '%s\n' "$1" ;;
    esac
}

run_checked() {
    desc="$1"
    shift
    "$@" || {
        status=$?
        echo "$desc failed with exit code $status." >&2
        exit "$status"
    }
}

target_root="${CARGO_TARGET_DIR:-$repo_root/target}"
ori_exe=$(output_exe_name ori)
source_ori="$target_root/release/$ori_exe"
host=$(host_triple)

if [ -z "$host" ]; then
    echo "could not detect Rust host target from rustc -vV" >&2
    exit 2
fi

if [ -z "$package_root" ]; then
    package_root="${TMPDIR:-/tmp}/ori-native-release-smoke-$$"
fi

package_root=$(CDPATH= cd -- "$(dirname -- "$package_root")" && pwd)/$(basename -- "$package_root")
examples_dir="$package_root/examples"
runtime_dir="$package_root/runtime"
package_ori="$package_root/$ori_exe"

cleanup() {
    if [ "$keep_package" -eq 0 ] && [ -d "$package_root" ]; then
        rm -rf "$package_root"
    fi
}
trap cleanup EXIT

if [ "$skip_build" -eq 0 ]; then
    run_checked "cargo build -p ori-driver --release" \
        sh -c "cd '$repo_root' && cargo build -p ori-driver --release"
fi

if [ ! -f "$source_ori" ]; then
    echo "release compiler not found at $source_ori" >&2
    exit 1
fi

rm -rf "$package_root"
mkdir -p "$examples_dir"
cp "$source_ori" "$package_ori"
cp "$repo_root/examples/hello_world.orl" "$examples_dir/hello_world.orl"
cp "$repo_root/examples/async_demo.orl" "$examples_dir/async_demo.orl"

stage_args="--target $host --profile release --output-root $runtime_dir"
if [ "$skip_build" -eq 1 ]; then
    stage_args="$stage_args --skip-build"
fi
run_checked "tools/stage_native_runtime.sh" \
    sh -c "cd '$repo_root' && tools/stage_native_runtime.sh $stage_args"

cat > "$examples_dir/package_smoke_test.orl" <<'ORI'
namespace app.package_smoke

import ori.test as test
import ori.task as task

@test
func package_smoke_test()
    check 1 + 1 == 2
    test.assert(true, "package smoke test")
end

@test
async func package_async_smoke_test()
    await task.sleep(1)
    test.assert(true, "package async smoke test")
end
ORI

hello_exe="$package_root/$(output_exe_name hello_world)"
(
    cd "$package_root"
    ORI_REQUIRE_PACKAGED_RUNTIME=1 "$package_ori" compile "examples/hello_world.orl" --out "$hello_exe"
)

hello_output=$("$hello_exe")
case "$hello_output" in
    *"The answer is: 42"*) ;;
    *)
        echo "compiled hello_world executable did not print expected answer" >&2
        echo "$hello_output" >&2
        exit 1
        ;;
esac

async_exe="$package_root/$(output_exe_name async_demo)"
(
    cd "$package_root"
    ORI_REQUIRE_PACKAGED_RUNTIME=1 "$package_ori" compile "examples/async_demo.orl" --out "$async_exe"
)

async_output=$("$async_exe")
case "$async_output" in
    "42") ;;
    *)
        echo "compiled async_demo executable did not print expected async answer" >&2
        echo "$async_output" >&2
        exit 1
        ;;
esac

(
    cd "$package_root"
    ORI_REQUIRE_PACKAGED_RUNTIME=1 "$package_ori" test "examples/package_smoke_test.orl"
)

printf 'native release smoke passed: %s\n' "$package_root"
