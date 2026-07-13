# Ori — Zed extension

Local language support for **Ori** (`.orl`) in [Zed](https://zed.dev).

- Language config (`.orl`, `--` comments, brackets)
- **LSP** via `ori-lsp` on `PATH` (no download, no store publish)

**Not published** to the Zed extension store yet — install as a **dev extension**.

## Prerequisites

1. Build or install Ori so `ori-lsp` is on your `PATH`:

```bash
cd compiler
cargo build -p ori-lsp -p ori-driver
export PATH="$PWD/target/debug:$PATH"
```

2. Zed with extension host support.

## Install (dev)

From Zed: **zed: install dev extension** and select this directory:

```text
extensions/zed-ori
```

Or symlink into Zed’s extensions path (varies by OS):

```bash
# Linux example
mkdir -p ~/.local/share/zed/extensions/installed
ln -sfn /path/to/ori-lang/extensions/zed-ori ~/.local/share/zed/extensions/installed/ori
```

Rebuild after Rust changes:

```bash
# Zed recompiles wasm for dev extensions on reload
```

## Settings

Optional: force stdlib if auto-detect fails:

```json
// settings.json — only if you set env globally for Zed
{
  "lsp": {
    "ori-lsp": {
      // binary path overrides are extension-driven via PATH
    }
  }
}
```

The extension sets `ORI_STDLIB_ROOT` when it finds `stdlib/` in the worktree.

## Features / limits

| Feature | Status |
|---------|--------|
| Open `.orl` as language Ori | yes |
| `ori-lsp` diagnostics / hover / complete | yes (if on PATH) |
| Tree-sitter syntax colors | **not yet** (no grammar crate) |
| Marketplace / extension store | **out of scope** |

## Version

Matches Ori package line **0.3.2** (S3 + inference B).
