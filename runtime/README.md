# Ori native runtime package

This directory is the release layout for the native runtime route.

`ori compile` first looks for the Rust `ori-runtime` static library here:

```text
runtime/{target-triple}/{runtime-artifact}
```

Examples:

```text
runtime/x86_64-pc-windows-msvc/ori_runtime.lib
runtime/x86_64-unknown-linux-gnu/libori_runtime.a
```

A complete release package should keep this shape:

```text
ori.exe                         # or `ori` on Unix
runtime/
  {target-triple}/
    {runtime-artifact}
    runtime-link.json
examples/
README.md
```

Each target directory should also contain `runtime-link.json`. That file records
the system libraries required by the Rust `staticlib` when a raw native linker is
used. It also records the Ori version and native ABI version used to stage the
runtime, so the driver can reject stale or incompatible runtime packages early.

`ori-runtime` is the source of truth for native runtime semantics. The C backend
is a debug/transpile route and must not be used as the semantic reference for
`ori compile`, `ori test`, collections, ARC, or async/concurrency behavior.

Use this command from the repository root to stage the current host runtime:

```powershell
.\tools\stage_native_runtime.ps1
```

On Linux or macOS, use:

```sh
./tools/stage_native_runtime.sh
```

Use this command to verify a clean release-style package:

```powershell
.\tools\smoke_native_release.ps1
```

On Linux or macOS, use:

```sh
sh tools/smoke_native_release.sh
```

The smoke test copies `ori`, `runtime/`, and examples into a temporary folder
and runs with `ORI_REQUIRE_PACKAGED_RUNTIME=1`, so it does not silently fall back
to the workspace runtime.

The `.github/workflows/native-route.yml` workflow runs the native route against
the five CI triples below. Each triple has a staging command; the smoke step in
CI runs `tools/smoke_native_release.{ps1,sh}` with `ORI_REQUIRE_PACKAGED_RUNTIME=1`
to validate the staged runtime end-to-end.

| Triple | Runner | Stage command |
|---|---|---|
| `x86_64-pc-windows-msvc` | `windows-latest` | `.\tools\stage_native_runtime.ps1` |
| `x86_64-pc-windows-gnu` | `windows-latest` (GNU toolchain) | `.\tools\stage_native_runtime.ps1 -Target x86_64-pc-windows-gnu` |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `./tools/stage_native_runtime.sh` |
| `x86_64-apple-darwin` | `macos-15-intel` | `./tools/stage_native_runtime.sh --target x86_64-apple-darwin` |
| `aarch64-apple-darwin` | `macos-15` | `./tools/stage_native_runtime.sh --target aarch64-apple-darwin` |

Only `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu` runtime folders are
versioned in git as the canonical development baseline. The other three triples
are staged in CI and shipped with release packages, but not committed to the
repository — see `CONTRIBUTING.md` for the pre-built artifact policy.

Use this command to verify that the compiled static library exports every native
runtime symbol required by the stdlib manifest and backend:

```powershell
.\tools\check_native_runtime_exports.ps1
```

The static library itself is generated output and is ignored by git.
