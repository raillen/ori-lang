# Ori — VS Code / Cursor extension

Language support for **Ori** (`.orl`): LSP, grammar, snippets, doctor.

**Surface:** S3 + inference B · version **0.3.5** (matches language package).

## Install

### From GitHub Release (recommended)

1. Install Ori so `ori` / `ori-lsp` are on your `PATH`  
   ([docs/install.md](../../docs/install.md) — Windows one-liner: `irm …/get.ps1 | iex`).
2. Download **`ori-vscode-orl-0.3.5.vsix`** from  
   [GitHub Releases](https://github.com/raillen/ori-lang/releases).
3. Install:

```bash
code --install-extension ori-vscode-orl-0.3.5.vsix
# Cursor:
cursor --install-extension ori-vscode-orl-0.3.5.vsix
```

Or: VS Code → **Extensions: Install from VSIX…**

Not published to the VS Marketplace yet (local / GitHub release only).

### From this monorepo

```bash
./tools/install_vscode_extension.sh
# or:
cd extensions/vscode-orl && npm install && npm run package:vsix && npm run install:local
```

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
cd compiler && cargo build -p ori-lsp -p ori-driver
cd ../extensions/vscode-orl
npm install
npm run compile
```

F5 in VS Code → Extension Development Host.

Repo smoke: `./tools/smoke_vscode_extension.sh`

## Package for release

```bash
sh tools/package_editor_extensions.sh --force
# → compiler/target/dist/ori-vscode-orl-<ver>.vsix
```
