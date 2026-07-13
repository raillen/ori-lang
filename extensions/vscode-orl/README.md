# Ori — VS Code Extension

Language support for [Ori](https://github.com/ori-lang/ori) (`.orl` files).

## Features

- **LSP** via `ori-lsp`: diagnostics, hover, go-to-definition, completion (Layer 1 + Layer 2 stdlib), rename, format, semantic tokens, inlay hints
- **Local inference (0.3.1 + option B):** inlay/hover for omitted local `const`/`var` types when the RHS is obvious (literals, field, index, call, pipe)
- **Pipe `|>`:** first-class Ori operator (not removed in S3)
- **Stdlib-aware completion**: `import ori.string` + dot-complete on aliases
- **Go to stdlib source** for Layer 2 `.orl` functions
- **Incremental document sync**
- **Commands**: Check, Run, Test, Format, **Doctor** (`ori doctor`)
- TextMate grammar + snippets

## Settings

| Setting | Env var | Description |
|---------|---------|-------------|
| `ori.lsp.path` | — | Path to `ori-lsp` binary |
| `ori.compiler.path` | — | Path to `ori` CLI |
| `ori.stdlib.root` | `ORI_STDLIB_ROOT` | Stdlib directory |
| `ori.runtime.lib` | `ORI_RUNTIME_LIB` | Native runtime static lib |
| `ori.runtime.cdylib` | `ORI_RUNTIME_CDYLIB` | JIT cdylib |
| `ori.useJit` | `ORI_USE_JIT=1` | Prefer JIT for run tasks |

## Development

```bash
cd extensions/vscode-orl
npm install
npm run compile
```

Press F5 in VS Code to launch the Extension Development Host.

Repository-level smoke:

```powershell
.\tools\smoke_vscode_extension.ps1
```

The smoke compiles the extension, validates extension JSON files, runs LSP E2E
tests, and checks a temporary Ori project outside the repository.

Build the language server first:

```bash
cargo build -p ori-lsp -p ori-driver
```

## Doctor

Run **Ori: Run Doctor** from the command palette or `ori doctor` in the terminal to verify stdlib root, native runtime, linker strategy, and JIT availability.
