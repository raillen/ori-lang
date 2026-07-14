#!/usr/bin/env sh
# Ensure diagnostic catalog consistency (compiler ↔ Spec 13).
set -eu
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$script_dir/../.." && pwd)
if [ -f "$repo/compiler/Cargo.toml" ]; then
  cd "$repo/compiler"
else
  cd "$repo"
fi
cargo test -p ori-driver --test diagnostic_catalog -- --nocapture
echo "catalog_lint: OK"
