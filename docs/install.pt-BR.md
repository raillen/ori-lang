# Instalação de Ori

> **Público-alvo:** usuários finais que querem desenvolver em Ori **sem** clonar
> o repositório e **sem** toolchain Rust.  
> **English:** [install.md](install.md)  
> **Superfície:** S3 · package **v0.3.4** · M1 (instalação sem Rust) fechada · FREEZE-1 em 0.3.x

## Requisitos do sistema

Ori usa o **linker nativo do SO** para AOT (`ori compile`, `ori test`).  
Para JIT (`ori run`), nenhum linker é necessário — só o runtime empacotado em
`runtime/<triple>/` ao lado do binário `ori`.

### Windows (10/11)

**Pré-requisito:** Visual Studio Build Tools ou Community com a workload
**"Desktop development with C++"**.

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

**Não é necessário:** Rust nem `rust-lld` (default = SystemLinker).

### Linux

**Pré-requisito:** `build-essential` (ou `gcc` + `ld` + headers da libc).

```bash
# Debian / Ubuntu
sudo apt update && sudo apt install build-essential
```

### macOS

**Pré-requisito:** Xcode Command Line Tools (`xcode-select --install`).

---

## Download e instalação

> **Política de distribuição (2026-07-14):** packages oficiais de **release** para
> **Linux, Windows (MSVC) e macOS** (Apple Silicon + Intel) via GitHub Actions
> (`.github/workflows/release.yml`). Assets no tag `v*` em
> [GitHub Releases](https://github.com/raillen/ori-lang/releases).

1. Baixe em [GitHub Releases](https://github.com/raillen/ori-lang/releases)
   (ex. **v0.3.5**):

   | Plataforma | Arquivo |
   |------------|---------|
   | Linux x86_64 | `ori-v0.3.5-x86_64-unknown-linux-gnu.tar.gz` |
   | Linux deb | `ori_0.3.5_amd64.deb` |
   | Windows MSVC x86_64 | `ori-v0.3.5-x86_64-pc-windows-msvc.zip` |
   | macOS Apple Silicon | `ori-v0.3.5-aarch64-apple-darwin.tar.gz` |
   | macOS Intel | `ori-v0.3.5-x86_64-apple-darwin.tar.gz` |

**Windows (recomendado):** extraia o zip MSVC e rode o instalador (PATH do
usuário permanente):

```powershell
# Clique duplo em install.cmd, ou:
pwsh -ExecutionPolicy Bypass -File .\install.ps1
```

Instala em `%LOCALAPPDATA%\Programs\Ori` por padrão.  
Sistema inteiro: `.\install.ps1 -System` (como Administrador).  
Desinstalar: `pwsh -File "$env:LOCALAPPDATA\Programs\Ori\uninstall.ps1"`.  
Detalhes: [`tools/windows/README.md`](../tools/windows/README.md).

**Tarball / zip (manual):** extraia (ex. `~/ori` ou `C:\ori`), layout
`ori`/`ori.exe` + `ori-lsp` + `stdlib/` + `runtime/<triple>/`, coloque no `PATH`.

**Debian/Ubuntu:**

```bash
sudo dpkg -i ori_0.3.5_amd64.deb
# AOT: sudo apt install build-essential
```

Verifique: `ori --version` e `ori doctor`.

Esperado: stdlib, runtime AOT + JIT, triple, linker (BundledRustLld ou SystemLinker), JIT para `ori run`.

---

## Primeiro programa

```ori
module app.hello

import ori.io = io

main()
    io.println("Hello, Ori!")
end
```

```bash
ori run hello.orl
ori new my_app && cd my_app && ori run main.orl
```

Próximo: [Tour da linguagem](language/tour.pt-BR.md) ·
[Primeiro projeto](guides/first-project.pt-BR.md) ·
[Exemplos](../examples/) · Editores: [VS Code](../extensions/vscode-orl/) ·
[Zed](../extensions/zed-ori/).

---

## Variáveis de ambiente (opcional)

Normalmente **nenhuma** é necessária.

| Variável | Propósito |
|----------|-----------|
| `ORI_USE_SYSTEM_LINKER=1` | Forçar linker do SO |
| `ORI_USE_JIT=1` / `ORI_USE_AOT=1` | Forçar modo de `ori run` |
| `ORI_STDLIB_ROOT` | Raiz da stdlib |
| `ORI_RUNTIME_LIB` / `ORI_RUNTIME_CDYLIB` | Runtime nativo |

---

## Troubleshooting

| Sintoma | Ação |
|---------|------|
| `native.link_failed` | Instale o linker do SO |
| Runtime not found | `runtime/` deve ficar ao lado de `ori` |
| Só `ori run` funciona | AOT precisa do linker; JIT não |
| LSP no VS Code / Zed | `ori-lsp` no PATH (ou settings `ori.*.path` no VS Code) |

## Veja também

- [spec/19-abi.md](spec/19-abi.md) — ABI `ori-native-abi-1`
- [AGENTS.md](../AGENTS.md) — independência do Rust (M1)
- [BACKLOG.md](planning/BACKLOG.md) — o que falta implementar
