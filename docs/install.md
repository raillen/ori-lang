# Installing Ori

> **Audience:** end users who want to write Ori programs **without** cloning this
> repository and **without** a Rust toolchain.  
> **Portuguese:** [install.pt-BR.md](install.pt-BR.md)  
> **Surface:** S3 · package **v0.3.2** · M1 (Rust-free install path) complete

## System prerequisites

Ori uses the **OS native linker** for AOT (`ori compile`, `ori test`).  
For JIT (`ori run`), no linker is required — only the packaged runtime next to
the `ori` binary (`runtime/<triple>/`).

### Windows (10/11)

**Requirement:** Visual Studio Build Tools or Visual Studio Community with the
**"Desktop development with C++"** workload.

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

Or the installer at [visualstudio.microsoft.com/downloads](https://visualstudio.microsoft.com/downloads/).

**Why:** AOT uses MSVC `link.exe`.

**Not required:** Rust (`rustc`, `cargo`), or `rust-lld` (SystemLinker is the default).

### Linux

**Requirement:** `build-essential` (or `gcc` + `ld` + libc headers).

```bash
# Debian / Ubuntu
sudo apt update && sudo apt install build-essential

# Fedora / RHEL
sudo dnf install gcc gcc-c++ make glibc-devel

# Arch
sudo pacman -S base-devel
```

**Not required:** Rust.

### macOS

**Requirement:** Xcode Command Line Tools.

```bash
xcode-select --install
```

**Not required:** full Xcode, Rust, or `rust-lld`.

---

## Download and install

### Release package (recommended)

> **Shipping policy (2026-07-13):** official **release packages are Linux-only**
> (`x86_64-unknown-linux-gnu`). Windows/macOS packages are deferred (BACKLOG
> DIST-1/2). On those OSes, use **build from source** below.

1. Download from [GitHub Releases](https://github.com/raillen/ori-lang/releases).
   Example **v0.3.2**:
   - **Linux (shipped):** `ori-v0.3.2-x86_64-unknown-linux-gnu.tar.gz`
   - Windows / macOS: build from source (release zip/tar not published yet)

2. Extract to a directory (e.g. `~/ori`, `C:\Tools\ori`).

3. Expected layout:

   | Path | Role |
   |------|------|
   | `ori` / `ori.exe` | CLI |
   | `ori-lsp` / `ori-lsp.exe` | LSP server |
   | `stdlib/` | Layer 2/3 `.orl` modules |
   | `runtime/<triple>/` | staticlib + cdylib + `runtime-link.json` |

4. Put the directory on your `PATH`.

5. Verify:

```bash
ori --version
ori doctor
```

Healthy install: stdlib found, AOT + JIT runtime present, target triple detected,
linker strategy **SystemLinker** (or documented fallback), JIT available for
`ori run`.

---

## First program

`hello.orl` (S3):

```ori
module app.hello

import ori.io = io

main()
    io.println("Hello, Ori!")
end
```

```bash
ori run hello.orl                 # JIT — no linker
ori compile hello.orl --out hello # AOT — needs system linker
./hello
```

Recommended project skeleton:

```bash
ori new my_app
cd my_app
ori run main.orl
```

Next: [Language tour](language/tour.md) · [First project](guides/first-project.md) ·
[Examples](../examples/) · Editors: [VS Code](../extensions/vscode-orl/) ·
[Zed](../extensions/zed-ori/).

---

## Environment overrides

Usually **none** are needed.

| Variable | Purpose |
|----------|---------|
| `ORI_USE_SYSTEM_LINKER=1` | Force OS linker |
| `ORI_SYSTEM_LINKER` | Explicit linker path |
| `ORI_USE_BUNDLED_RUST_LLD=1` | Force bundled `rust-lld` |
| `ORI_USE_RUSTC_DRIVER=1` | Legacy `rustc` driver (not for end users) |
| `ORI_USE_JIT=1` / `ORI_USE_AOT=1` | Force `ori run` mode |
| `ORI_RUNTIME_CDYLIB` / `ORI_RUNTIME_LIB` | Runtime path overrides |
| `ORI_STDLIB_ROOT` | Stdlib path override |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Package-only runtime (smoke/release) |

---

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| `native.link_failed` / linker not found | Install OS linker prereqs; check `link.exe` / `ld` / `xcrun --find ld` |
| Runtime not found | Keep `runtime/` beside `ori` |
| `ori run` works, `ori compile` fails | Install system linker (AOT only) |
| VS Code / LSP | Put `ori-lsp` on `PATH` or set `ori.lsp.path` / `ori.compiler.path` / `ori.stdlib.root` |

---

## Maintainer package smoke

```bash
sh tools/package_native_release.sh --force
sh tools/smoke_no_rust.sh --package-root compiler/target/dist/ori-… --allow-rust-on-path
```

See [AGENTS.md](../AGENTS.md) and [spec/19-abi.md](spec/19-abi.md) (`ori-native-abi-1`).
