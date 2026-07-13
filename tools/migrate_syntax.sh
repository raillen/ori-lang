#!/usr/bin/env bash
# Thin wrapper: best-effort S3 syntax migration via `ori migrate-syntax`.
# Usage:
#   tools/migrate_syntax.sh [--dry-run] [-v] [paths...]
# Defaults to stdlib examples tests when no paths are given.
# Best-effort pre-S3 → S3 rewrite for `.orl` trees.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ARGS=()
PATHS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run|-v|--verbose)
      ARGS+=("$1")
      shift
      ;;
    *)
      PATHS+=("$1")
      shift
      ;;
  esac
done

if [[ ${#PATHS[@]} -eq 0 ]]; then
  PATHS=(stdlib examples tests)
fi

if command -v ori >/dev/null 2>&1 && [[ -z "${ORI_MIGRATE_USE_CARGO:-}" ]]; then
  exec ori migrate-syntax "${ARGS[@]}" "${PATHS[@]}"
fi

exec cargo run --manifest-path "$ROOT/compiler/Cargo.toml" -q -p ori-driver -- migrate-syntax "${ARGS[@]}" "${PATHS[@]}"
