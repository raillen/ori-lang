#!/usr/bin/env sh
# Build a Debian package (.deb) from a staged Ori package directory
# (layout: ori, ori-lsp, stdlib/, runtime/ next to each other).
#
# Usage:
#   tools/package_deb.sh --package-root DIR [--output PATH] [--version VER] [--arch amd64]
set -eu

package_root=""
output_path=""
version=""
arch="amd64"
maintainer="Ori Language Maintainers <https://github.com/raillen/ori-lang>"

usage() {
    cat <<'USAGE'
Usage: tools/package_deb.sh --package-root DIR [--output PATH] [--version VER] [--arch amd64]

Creates ori_<version>_<arch>.deb installing:

  /usr/lib/ori/{ori,ori-lsp,stdlib,runtime,...}
  /usr/bin/ori -> /usr/lib/ori/ori
  /usr/bin/ori-lsp -> /usr/lib/ori/ori-lsp

Requires: dpkg-deb
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --package-root) package_root="${2:-}"; shift 2 ;;
        --output) output_path="${2:-}"; shift 2 ;;
        --version) version="${2:-}"; shift 2 ;;
        --arch) arch="${2:-}"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
done

if [ -z "$package_root" ] || [ ! -d "$package_root" ]; then
    echo "--package-root must be an existing package directory" >&2
    exit 2
fi
if ! command -v dpkg-deb >/dev/null 2>&1; then
    echo "dpkg-deb not found; install dpkg-dev" >&2
    exit 2
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
cargo_toml="$repo_root/compiler/Cargo.toml"
if [ ! -f "$cargo_toml" ]; then
    cargo_toml="$repo_root/Cargo.toml"
fi
if [ -z "$version" ]; then
    version=$(awk '
        /^\[workspace\.package\]/ { in_section=1; next }
        in_section && /^\[/ { exit }
        in_section && /^[[:space:]]*version[[:space:]]*=/ {
            gsub(/"/, "", $3)
            print $3
            exit
        }
    ' "$cargo_toml")
fi
if [ -z "$version" ]; then
    echo "could not determine package version" >&2
    exit 2
fi

package_root=$(CDPATH= cd -- "$package_root" && pwd)
if [ ! -x "$package_root/ori" ] && [ ! -f "$package_root/ori" ]; then
    echo "package root missing ori binary: $package_root/ori" >&2
    exit 2
fi
if [ ! -d "$package_root/stdlib" ]; then
    echo "package root missing stdlib/: $package_root" >&2
    exit 2
fi
if [ ! -d "$package_root/runtime" ]; then
    echo "package root missing runtime/: $package_root" >&2
    exit 2
fi

work="${TMPDIR:-/tmp}/ori-deb-build-$$"
rm -rf "$work"
mkdir -p "$work/DEBIAN" \
    "$work/usr/lib/ori" \
    "$work/usr/bin" \
    "$work/usr/share/doc/ori"

# Copy package contents under /usr/lib/ori (flat layout preserves runtime discovery).
# Exclude smoke-only scratch binaries if present.
for item in ori ori-lsp stdlib runtime examples runtime-link.json; do
    if [ -e "$package_root/$item" ]; then
        cp -a "$package_root/$item" "$work/usr/lib/ori/"
    fi
done
# Also copy any remaining runtime-link if nested only
if [ ! -e "$work/usr/lib/ori/runtime" ]; then
    echo "runtime not copied into deb staging" >&2
    exit 1
fi

chmod 755 "$work/usr/lib/ori/ori" 2>/dev/null || true
if [ -f "$work/usr/lib/ori/ori-lsp" ]; then
    chmod 755 "$work/usr/lib/ori/ori-lsp"
fi

ln -sfn /usr/lib/ori/ori "$work/usr/bin/ori"
if [ -f "$work/usr/lib/ori/ori-lsp" ]; then
    ln -sfn /usr/lib/ori/ori-lsp "$work/usr/bin/ori-lsp"
fi

cat > "$work/usr/share/doc/ori/copyright" <<EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: ori-lang
Source: https://github.com/raillen/ori-lang

Files: *
Copyright: Ori Language contributors
License: SEE LICENSE IN upstream repository
EOF

cat > "$work/usr/share/doc/ori/README.Debian" <<EOF
Ori language toolchain (CLI + LSP + stdlib + native runtime).

After install:
  ori --version
  ori doctor

AOT compile needs build-essential (gcc/ld). JIT run does not.
Docs: https://github.com/raillen/ori-lang
EOF

# Approximate installed size in KiB
installed_size=$(du -sk "$work" | awk '{print $1}')

cat > "$work/DEBIAN/control" <<EOF
Package: ori
Version: $version
Section: devel
Priority: optional
Architecture: $arch
Maintainer: $maintainer
Installed-Size: $installed_size
Depends: libc6
Recommends: build-essential
Homepage: https://github.com/raillen/ori-lang
Description: Ori programming language toolchain (CLI, LSP, stdlib, runtime)
 Ori is a reading-first, explicitly typed language (surface S3).
 This package installs the ori CLI, ori-lsp, the standard library, and the
 native runtime for x86_64 Linux. Use \`ori run\` (JIT) without a linker, or
 \`ori compile\` / \`ori test\` with the system linker (build-essential).
EOF

if [ -z "$output_path" ]; then
    output_path="$(dirname -- "$package_root")/ori_${version}_${arch}.deb"
fi
mkdir -p "$(dirname -- "$output_path")"
rm -f "$output_path"

if dpkg-deb --help 2>&1 | grep -q -- '--root-owner-group'; then
    dpkg-deb --build --root-owner-group "$work" "$output_path"
else
    dpkg-deb --build "$work" "$output_path"
fi
rm -rf "$work"

printf 'debian package: %s\n' "$output_path"
