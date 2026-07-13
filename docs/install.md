# Instalação de Ori

> **Público-alvo:** usuários finais que querem instalar Ori para desenvolver programas, **sem** clonar o repositório e **sem** a toolchain Rust.  
> **M1 (fechado):** o caminho suportado é package + linker do SO (AOT) ou package + JIT (`ori run`).

## Requisitos do sistema

Ori usa o **linker nativo do sistema** para AOT (`ori compile`, `ori test`).  
Para JIT (`ori run`), nenhum linker é necessário — apenas o runtime empacotado (`runtime/<triple>/` ao lado do `ori`).

### Windows (10/11)

**Pré-requisito:** Visual Studio Build Tools ou Visual Studio Community com a workload **"Desktop development with C++"**.

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

Ou o installer em [visualstudio.microsoft.com/downloads](https://visualstudio.microsoft.com/downloads/) com a workload **"Desktop development with C++"**.

**Por quê:** Ori usa `link.exe` (MSVC) para binários nativos.

**Não é necessário:** Rust (`rustc`, `cargo`), nem `rust-lld` (o default é o linker do sistema).

---

### Linux (Debian, Ubuntu, Fedora, Arch, …)

**Pré-requisito:** `build-essential` (ou `gcc` + `ld` + headers da libc).

**Debian / Ubuntu:**

```bash
sudo apt update
sudo apt install build-essential
```

**Fedora / RHEL:**

```bash
sudo dnf install gcc gcc-c++ make glibc-devel
```

**Arch:**

```bash
sudo pacman -S base-devel
```

**Não é necessário:** Rust.

---

### macOS (Intel e Apple Silicon)

**Pré-requisito:** Xcode Command Line Tools.

```bash
xcode-select --install
```

**Não é necessário:** Rust, Xcode completo, nem `rust-lld`.

---

## Download e instalação

### Via release package (recomendado)

1. Baixe o artefato do [GitHub Releases](https://github.com/raillen/ori-lang/releases) para o seu OS. Exemplo **v0.3.2**:
   - Windows: `ori-v0.3.2-x86_64-pc-windows-msvc.zip`
   - Linux: `ori-v0.3.2-x86_64-unknown-linux-gnu.tar.gz`
   - macOS: ainda não empacotado no release pipeline (use build a partir do source ou estenda `release.yml`)

2. Extraia em um diretório (ex.: `~/ori`, `C:\Tools\ori`).

3. Conteúdo esperado do package:

   | Caminho | Função |
   |---------|--------|
   | `ori` / `ori.exe` | CLI |
   | `ori-lsp` / `ori-lsp.exe` | servidor LSP |
   | `stdlib/` | módulos `.orl` Layer 2/3 |
   | `runtime/<triple>/` | staticlib + cdylib + `runtime-link.json` |

4. Adicione o diretório ao `PATH`:

   **Windows (PowerShell):**
   ```powershell
   [Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Tools\ori", "User")
   ```

   **Linux / macOS:**
   ```bash
   echo 'export PATH="$HOME/ori:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

5. Verifique:

```bash
ori --version
ori doctor
```

### Verificação com `ori doctor`

Esperado em instalação saudável:

- stdlib root encontrado  
- runtime estática (AOT) e cdylib (JIT)  
- triple detectado  
- estratégia de linker: **SystemLinker** (ou fallback documentado)  
- `ori run` em modo JIT disponível  

---

## Primeiro programa

`hello.orl` (superfície **S3**):

```ori
module app.hello

import ori.io = io

main()
    io.println("Hello, Ori!")
end
```

JIT (sem linker):

```bash
ori run hello.orl
```

AOT (precisa do linker do SO):

```bash
ori compile hello.orl --out hello
./hello
```

Projeto mínimo (recomendado):

```bash
ori new my_app
cd my_app
ori run main.orl
```

---

## Validação local do package (mantenedores)

Com Rust (só para **gerar** o package):

```bash
# a partir da raiz do repo
sh tools/package_native_release.sh --force
```

Sem usar Rust no smoke do package gerado (em máquina de dev com Rust no PATH):

```bash
sh tools/smoke_no_rust.sh --package-root compiler/target/dist/ori-… --allow-rust-on-path
```

Em CI, o job `smoke-no-rust` roda **sem** `rustc`/`cargo` no PATH (ver `.github/workflows/native-route.yml`).

---

## Variáveis de ambiente (override)

Normalmente **nenhuma** é necessária.

| Variável | Propósito |
|----------|-----------|
| `ORI_USE_SYSTEM_LINKER=1` | Força linker do SO |
| `ORI_SYSTEM_LINKER` | Caminho explícito do linker |
| `ORI_USE_BUNDLED_RUST_LLD=1` | Força `rust-lld` empacotado |
| `ORI_USE_RUSTC_DRIVER=1` | Driver `rustc` legacy (não para usuários finais) |
| `ORI_USE_JIT=1` / `ORI_USE_AOT=1` | Força modo de `ori run` |
| `ORI_RUNTIME_CDYLIB` / `ORI_RUNTIME_LIB` | Override de runtime |
| `ORI_STDLIB_ROOT` | Override da stdlib |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Exige só runtime empacotado (smoke/release) |

---

## Troubleshooting

### `native.link_failed` / linker not found

Instale o pré-requisito do SO e confira:

- Windows: `where link.exe`
- Linux: `ld --version`
- macOS: `xcrun --find ld`

### Runtime not found

O diretório `runtime/` deve estar ao lado do executável `ori` (layout do package).

### `ori run` ok, `ori compile` falha

JIT não precisa de linker; AOT precisa. Instale o toolchain do SO.

### Extensão VS Code / LSP

Garanta `ori-lsp` no `PATH` ou configure `ori.lsp.path` / `ori.compiler.path` / `ori.stdlib.root`.

---

## Desinstalação

Remova o diretório do package e a entrada no `PATH`.

---

## Veja também

- `docs/spec/19-abi.md` — ABI nativo (`ori-native-abi-1`)
- `AGENTS.md` — estratégia de independência do Rust (M1)
- `docs/planning/historico/rust-independence.md` — histórico técnico
- `tools/smoke_no_rust.sh` — smoke de usuário final
