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
Windows MSVC, Windows GNU, Linux GNU, macOS x86_64, and macOS aarch64.

Use this command to verify that the compiled static library exports every native
runtime symbol required by the stdlib manifest and backend:

```powershell
.\tools\check_native_runtime_exports.ps1
```

The static library itself is generated output and is ignored by git.
