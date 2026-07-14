#!/usr/bin/env sh
# Smoke for packages/ori-web-auth (TOTP + recovery codes).
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$script_dir/../.." && pwd)

if [ -x "$repo/compiler/target/debug/ori" ]; then
  ORI_BIN="${ORI_BIN:-$repo/compiler/target/debug/ori}"
elif [ -x "$repo/compiler/target/release/ori" ]; then
  ORI_BIN="${ORI_BIN:-$repo/compiler/target/release/ori}"
else
  ORI_BIN="${ORI_BIN:-ori}"
fi

export ORI_STDLIB_ROOT="${ORI_STDLIB_ROOT:-$repo/stdlib}"
if [ -z "${ORI_RUNTIME_CDYLIB:-}" ] && [ -f "$repo/runtime/x86_64-unknown-linux-gnu/libori_runtime.so" ]; then
  export ORI_RUNTIME_CDYLIB="$repo/runtime/x86_64-unknown-linux-gnu/libori_runtime.so"
fi

cd "$repo/packages/ori-web-auth/examples/smoke"
rm -rf "${HOME}/.ori/packages/web" "${HOME}/.ori/packages/web_auth" 2>/dev/null || true

echo "== web_auth_smoke: $ORI_BIN run main.orl =="
out=$("$ORI_BIN" run main.orl 2>&1) || {
  echo "$out" >&2
  echo "web_auth_smoke: FAIL (run error)" >&2
  exit 1
}
echo "$out"
echo "$out" | grep -q "WEB_AUTH SMOKE PASSED" || {
  echo "web_auth_smoke: FAIL (missing WEB_AUTH SMOKE PASSED)" >&2
  exit 1
}
echo "web_auth_smoke: OK"
