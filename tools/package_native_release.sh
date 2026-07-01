#!/usr/bin/env sh
set -eu

package_root=""
archive_path=""
skip_build=0
force=0

usage() {
    cat <<'USAGE'
Usage: tools/package_native_release.sh [--package-root DIR] [--archive PATH] [--skip-build] [--force]

Builds and validates a release-style Ori package, then writes a .tar.gz archive.
The package is created through tools/smoke_native_release.sh, so compile/test/JIT
smoke checks must pass before the archive is produced.
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --package-root)
            package_root="${2:-}"
            shift 2
            ;;
        --archive)
            archive_path="${2:-}"
            shift 2
            ;;
        --skip-build)
            skip_build=1
            shift
            ;;
        --force)
            force=1
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
version=$(awk -F'"' '/^[[:space:]]*version[[:space:]]*=/ { print $2; exit }' "$repo_root/Cargo.toml")
host=$(rustc -Vv | awk -F': ' '/^host:/ { print $2; exit }')

if [ -z "$version" ]; then
    echo "could not find workspace version in Cargo.toml" >&2
    exit 2
fi
if [ -z "$host" ]; then
    echo "could not detect Rust host target from rustc -Vv" >&2
    exit 2
fi

dist_root="$repo_root/target/dist"
if [ -z "$package_root" ]; then
    package_root="$dist_root/ori-$version-$host"
fi
if [ -z "$archive_path" ]; then
    archive_path="$dist_root/ori-$version-$host.tar.gz"
fi

mkdir -p "$(dirname -- "$archive_path")"

if [ "$skip_build" -eq 1 ]; then
    "$script_dir/smoke_native_release.sh" --package-root "$package_root" --keep-package --skip-build
else
    "$script_dir/smoke_native_release.sh" --package-root "$package_root" --keep-package
fi

if [ -e "$archive_path" ] && [ "$force" -eq 0 ]; then
    echo "archive already exists at $archive_path; pass --force to replace it" >&2
    exit 2
fi
if [ -e "$archive_path" ]; then
    rm -f "$archive_path"
fi

tar -czf "$archive_path" -C "$(dirname -- "$package_root")" "$(basename -- "$package_root")"

printf 'native release package: %s\n' "$package_root"
printf 'native release archive: %s\n' "$archive_path"
