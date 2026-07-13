#!/usr/bin/env sh
# Install the monorepo Ori VS Code extension (local .vsix — no Marketplace).
#
# Usage:
#   tools/install_vscode_extension.sh [--skip-npm-install] [--editor code|cursor|flatpak|auto]
#
# Detects: `code` / `cursor` on PATH, or Flatpak `com.visualstudio.code`.
set -eu

skip_npm_install=0
editor="auto"

usage() {
    cat <<'USAGE'
Usage: tools/install_vscode_extension.sh [--skip-npm-install] [--editor code|cursor|flatpak|auto]

Builds extensions/vscode-orl into a .vsix and installs it into the chosen editor.
Does not publish to any marketplace.
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --skip-npm-install) skip_npm_install=1; shift ;;
        --editor)
            editor="${2:-}"
            shift 2
            ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
done

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
ext_root="$repo_root/extensions/vscode-orl"

if [ ! -f "$ext_root/package.json" ]; then
    echo "extension not found at $ext_root" >&2
    exit 1
fi

version=$(
    # Prefer node if present; fallback to sed for "version": "x.y.z"
    if command -v node >/dev/null 2>&1; then
        node -e "console.log(require('$ext_root/package.json').version)"
    else
        sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$ext_root/package.json" | head -n1
    fi
)
if [ -z "$version" ]; then
    echo "could not read extension version from package.json" >&2
    exit 1
fi
vsix_name="vscode-orl-${version}.vsix"
vsix_path="$ext_root/$vsix_name"

resolve_editor() {
    case "$editor" in
        code|cursor|flatpak) printf '%s\n' "$editor"; return ;;
        auto) ;;
        *) echo "unknown --editor value: $editor" >&2; exit 2 ;;
    esac
    if command -v code >/dev/null 2>&1; then
        printf 'code\n'
    elif command -v cursor >/dev/null 2>&1; then
        printf 'cursor\n'
    elif command -v flatpak >/dev/null 2>&1 \
        && flatpak info com.visualstudio.code >/dev/null 2>&1; then
        printf 'flatpak\n'
    else
        echo "no VS Code/Cursor found (code/cursor on PATH or Flatpak com.visualstudio.code)" >&2
        exit 1
    fi
}

chosen=$(resolve_editor)
echo "editor: $chosen"
echo "extension version: $version"

if ! command -v npm >/dev/null 2>&1; then
    echo "npm is required to build the extension" >&2
    exit 1
fi

cd "$ext_root"
if [ "$skip_npm_install" -eq 0 ]; then
    npm install
fi
npm run package:vsix

if [ ! -f "$vsix_path" ]; then
    # vsce may name without path prefix; search
    found=$(ls -1 "$ext_root"/vscode-orl-*.vsix 2>/dev/null | tail -n1 || true)
    if [ -n "$found" ]; then
        vsix_path="$found"
    else
        echo "vsix not produced at $vsix_path" >&2
        exit 1
    fi
fi
echo "vsix: $vsix_path"

case "$chosen" in
    code)
        code --install-extension "$vsix_path" --force
        ;;
    cursor)
        cursor --install-extension "$vsix_path" --force
        ;;
    flatpak)
        flatpak run com.visualstudio.code --install-extension "$vsix_path" --force
        ;;
esac

echo "installed Ori VS Code extension $version ($chosen)"
echo "reload the editor window if it was already open."
echo "ensure ori-lsp is on PATH (e.g. compiler/target/release or package install)."
