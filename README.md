# Ori

Ori is a reading-first, explicitly typed programming language designed for clarity and accessibility.

> *ori* (אורי)  Hebrew for "my light"

## Status

Early development. Compiler being written in Rust.

## Current CLI Contract

- `ori check file.orl`: type-checks code and reports diagnostics.
- `ori fmt file.orl`: validates and prints structurally formatted Ori source.
- `ori test file.orl`: runs concrete functions marked with `@test`.
- `ori run file.orl`: compiles to a temporary native binary, runs it, and
  returns the program exit code.
- `ori build file.orl`: emits C from a debug backend. This backend has partial
  feature parity and may reject features that the native backend supports.
- `ori compile file.orl`: emits a native binary with Cranelift, then uses the
  Rust `ori-runtime` static library as the canonical native runtime.

`ori compile` and `ori test` do not use the C debug backend. For local
development, the driver finds a packaged runtime under
`runtime/{target-triple}` or builds `compiler/crates/ori-runtime` with Cargo.
Set `ORI_RUNTIME_LIB` to point at a specific runtime static library, or
`ORI_NATIVE_LINKER` to diagnose a raw native linker route. Set
`ORI_USE_RUST_LLD=1` to ask the Rust driver to use `rust-lld` when it is
available. Set `ORI_REQUIRE_PACKAGED_RUNTIME=1` when validating a release
package that must use only the packaged `runtime/` directory.

## Current Tooling Status

- `ori-lsp` implements a real Language Server Protocol entry point over
  stdin/stdout.
- The LSP publishes parser/checker diagnostics, resolves local imports, and
  provides basic hover, go-to-definition, and stdlib completion.
- `ori check file.orl` remains the shortest CLI path for CI diagnostics.

## Release Layout

A native release package is expected to contain:

```text
ori.exe
runtime/{target-triple}/ori_runtime.lib
runtime/{target-triple}/runtime-link.json
examples/
```

Validate that layout with:

```powershell
.\tools\smoke_native_release.ps1
```

On Linux or macOS:

```sh
sh tools/smoke_native_release.sh
```

Use `ORI_REQUIRE_PACKAGED_RUNTIME=1` when testing a package directory. That
forces `ori compile` to use the packaged `runtime/` folder instead of the Cargo
workspace fallback.

The `native-route` CI workflow validates the native route on Windows MSVC,
Windows GNU, Linux GNU, macOS x86_64, and macOS aarch64.

## Philosophy

Ori optimizes for reading, not writing. Code should make visible:

- where a file belongs (namespace)
- what each value is (explicit types)
- where absence and errors can happen (optional, result)
- when resources are cleaned up (using)
- when behavior comes from a trait (implement)

## Quick Example

```ori
namespace app.main

import ori.io as io

func main()
    io.print("hello from Ori")
end
```

## License

MIT OR Apache-2.0
