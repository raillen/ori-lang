#!/usr/bin/env sh
# Build editor extension release artifacts (VS Code .vsix + Zed source/wasm zip).
# Usage (from repo root):
#   sh tools/package_editor_extensions.sh
#   sh tools/package_editor_extensions.sh --out compiler/target/dist
set -eu

out_dir=""
force=0

usage() {
    cat <<'USAGE'
Usage: tools/package_editor_extensions.sh [--out DIR] [--force]

Produces:
  ori-vscode-orl-<ver>.vsix
  ori-zed-<ver>.zip          (dev-installable Zed extension + prebuilt wasm)

Version is read from extensions/vscode-orl/package.json (keep in sync with
extensions/zed-ori/extension.toml).
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --out)
            out_dir="${2:-}"
            shift 2
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
vscode_dir="$repo_root/extensions/vscode-orl"
zed_dir="$repo_root/extensions/zed-ori"

version=$(python3 -c "import json; print(json.load(open('$vscode_dir/package.json'))['version'])")
zed_ver=$(awk -F'"' '/^version *=/ {print $2; exit}' "$zed_dir/extension.toml")
if [ "$version" != "$zed_ver" ]; then
    echo "error: version mismatch vscode-orl=$version zed-ori=$zed_ver" >&2
    exit 2
fi

if [ -z "$out_dir" ]; then
    out_dir="$repo_root/compiler/target/dist"
fi
mkdir -p "$out_dir"
out_dir=$(CDPATH= cd -- "$out_dir" && pwd)

vsix_out="$out_dir/ori-vscode-orl-${version}.vsix"
zed_out="$out_dir/ori-zed-${version}.zip"

if [ -e "$vsix_out" ] && [ "$force" -eq 0 ]; then
    echo "exists: $vsix_out (pass --force to replace)" >&2
    exit 2
fi
if [ -e "$zed_out" ] && [ "$force" -eq 0 ]; then
    echo "exists: $zed_out (pass --force to replace)" >&2
    exit 2
fi

echo "== VS Code: compile + package .vsix =="
(
    cd "$vscode_dir"
    npm run package:vsix
)
built_vsix="$vscode_dir/vscode-orl-${version}.vsix"
if [ ! -f "$built_vsix" ]; then
    echo "missing $built_vsix" >&2
    exit 1
fi
cp -f "$built_vsix" "$vsix_out"
# keep a copy next to package.json for local install scripts
printf 'vscode vsix: %s (%s)\n' "$vsix_out" "$(wc -c < "$vsix_out" | tr -d ' ') bytes"

echo "== Zed: wasm32-wasip1 release + zip =="
(
    cd "$zed_dir"
    rustup target add wasm32-wasip1 >/dev/null 2>&1 || true
    cargo build --release --target wasm32-wasip1
)
wasm_src="$zed_dir/target/wasm32-wasip1/release/zed_ori.wasm"
if [ ! -f "$wasm_src" ]; then
    echo "missing $wasm_src" >&2
    exit 1
fi

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
pkg="$stage/ori-zed-${version}"
mkdir -p "$pkg/languages/ori" "$pkg/src" "$pkg/wasm"

cp "$zed_dir/extension.toml" "$pkg/"
cp "$zed_dir/Cargo.toml" "$pkg/"
cp "$zed_dir/Cargo.lock" "$pkg/"
cp "$zed_dir/README.md" "$pkg/"
cp "$zed_dir/src/ori.rs" "$pkg/src/"
cp "$zed_dir/languages/ori/config.toml" "$pkg/languages/ori/"
cp "$wasm_src" "$pkg/wasm/zed_ori.wasm"
cp "$wasm_src" "$pkg/extension.wasm"

# Install notes for end users
cat > "$pkg/INSTALL.md" <<EOF
# Ori Zed extension ${version}

## Install as dev extension (recommended)

1. Install Ori so \`ori-lsp\` is on your PATH  
   (Windows: \`irm …/tools/windows/get.ps1 | iex\`, or a release package).
2. In Zed: command palette → **zed: install dev extension**
3. Select this folder (extracted \`ori-zed-${version}/\`).

Zed will compile the extension from source (or use the prebuilt wasm when applicable).

## Symlink (Linux)

\`\`\`bash
mkdir -p ~/.local/share/zed/extensions/installed
ln -sfn "\$PWD" ~/.local/share/zed/extensions/installed/ori
\`\`\`

Prebuilt wasm: \`wasm/zed_ori.wasm\` / \`extension.wasm\`.
EOF

rm -f "$zed_out"
(
    cd "$stage"
    zip -r -q "$zed_out" "ori-zed-${version}"
)

printf 'zed zip: %s (%s)\n' "$zed_out" "$(wc -c < "$zed_out" | tr -d ' ') bytes"
printf 'package_editor_extensions: OK version=%s\n' "$version"
