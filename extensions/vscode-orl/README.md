# Ori — VS Code / Cursor extension

Local language support for **Ori** (`.orl`): LSP, grammar, snippets, doctor.

**Install:** local `.vsix` only for now — **no Marketplace publish**.

Surface: **S3 `0.3.0`** + inference B **`0.3.1`** · extension version **`0.3.2`**.

## Features

- **LSP** via `ori-lsp`: diagnostics, hover, go-to-definition, completion (stdlib), rename, format, semantic tokens, inlay hints
- **Local inference (option B):** inlays for obvious local types
- **Pipe `|>`**
- **Commands:** Check, Run, Test, Format, Doctor (`ori doctor`), Project Summary
- TextMate grammar + snippets (`--` comments, S3 keywords)

## Settings

| Setting | Env | Description |
|---------|-----|-------------|
| `ori.lsp.path` | — | Path to `ori-lsp` |
| `ori.compiler.path` | — | Path to `ori` CLI |
| `ori.stdlib.root` | `ORI_STDLIB_ROOT` | Stdlib directory |
| `ori.runtime.lib` | `ORI_RUNTIME_LIB` | Native staticlib |
| `ori.runtime.cdylib` | `ORI_RUNTIME_CDYLIB` | JIT cdylib |
| `ori.useJit` | `ORI_USE_JIT=1` | Force JIT for extension terminals (default true) |
| `ori.useAot` | `ORI_USE_AOT=1` | Force AOT for extension terminals |

Binary discovery (when paths empty): `PATH`, then monorepo  
`compiler/target/{debug,release}/`, then root `target/{debug,release}/`.

## Development

```bash
# Language server + CLI (from repo root)
cd compiler && cargo build -p ori-lsp -p ori-driver

cd ../extensions/vscode-orl
npm install
npm run compile
```

F5 in VS Code → Extension Development Host.

### Local install (no Marketplace)

```bash
npm run package:vsix
# VS Code:
code --install-extension ./vscode-orl-0.3.2.vsix --force
# Cursor:
npm run install:cursor
```

Repo smoke:

```bash
./tools/smoke_vscode_extension.sh
# or PowerShell: tools/smoke_vscode_extension.ps1
```

## Doctor

**Ori: Run Doctor** or `ori doctor` — stdlib, runtime, linker, JIT readiness.
