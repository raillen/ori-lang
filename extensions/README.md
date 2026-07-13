# Ori editor extensions

Local DX only — **no store / Marketplace publish** until language work is done.

| Path | Editor | Notes |
|------|--------|--------|
| [`vscode-orl/`](vscode-orl/) | VS Code / Cursor | LSP client, grammar, snippets; install via `.vsix` |
| [`zed-ori/`](zed-ori/) | Zed | Language config + `ori-lsp` discovery; install as **dev extension** |

Build the language tools first:

```bash
cd compiler
cargo build -p ori-lsp -p ori-driver
```
