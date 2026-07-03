# Ori Package Ecosystem Guidelines

Este documento define as convenções oficiais para o ecossistema de pacotes da linguagem Ori, especialmente para pacotes que provêm *bindings* C via FFI (`extern c`), como o `ori-imgui` e o `ori-raylib`.

## 1. Nomenclatura

### 1.1 Repositório no GitHub
Todos os pacotes criados para a linguagem Ori devem usar o prefixo `ori-` em seus repositórios no GitHub para facilitar o _discoverability_ e indicar claramente sua finalidade.
- **Formato:** `ori-<nome-da-lib>`
- **Exemplo:** `ori-imgui`, `ori-raylib`, `ori-sqlite`.

### 1.2 Nome do Pacote (`ori.pkg.toml`)
Internamente, o manifesto do pacote (`ori.pkg.toml`) não deve conter o prefixo `ori-`. O nome deve ser limpo, refletindo o namespace pelo qual a biblioteca será importada no código fonte.
- **Formato:** `name = "<nome-da-lib>"`
- **Exemplo:** `name = "imgui"`
- **Uso no código:** `import imgui.ui`

## 2. Estrutura de Diretórios Recomendada

Pacotes que lidam com bibliotecas nativas devem adotar a seguinte estrutura canônica:

```
ori-<pacote>/
+-- ori.pkg.toml           # Manifesto do pacote
+-- README.md              # Instruções claras de instalação e uso
+-- src/                   # Código fonte Ori (.orl)
¦   +-- ffi.orl            # Declarações puras de `extern c` e structs opacas
¦   +-- ui.orl             # Wrappers idiomáticos e abstractions do Ori
+-- lib/                   # Artefatos Nativos Pré-compilados (C/C++)
¦   +-- win-x64/           # .dll e .lib para Windows
¦   +-- linux-x64/         # .so e .a para Linux
¦   +-- macos-arm64/       # .dylib e .a para macOS (Apple Silicon)
+-- tools/                 # Scripts auxiliares (ex: build do C/C++ do zero)
¦   +-- build_native.ps1
+-- examples/              # Códigos demonstrando como usar a biblioteca
    +-- demo.orl
```

## 3. Diretrizes de Bibliotecas Nativas (FFI)

Como o Ori v0.2.1 ainda não possui um sistema automatizado de *build scripts* nativos (como o `build.rs` do Rust), a responsabilidade de prover os artefatos nativos recai sobre o autor do pacote.

### Regras para o diretório `lib/`:
1. **Artefatos Prontos:** O repositório deve conter as bibliotecas `.dll`, `.so` e `.dylib` pré-compiladas na pasta `lib/<target>/`. Isso garante que quando um usuário baixar o pacote e der `ori run`, a execução JIT encontre a biblioteca compartilhada imediatamente.
2. **Bibliotecas Estáticas:** Recomenda-se também fornecer as bibliotecas estáticas (`.lib` ou `.a`) nas mesmas pastas. Isso permitirá que comandos futuros como `ori compile` liguem (link) a dependência diretamente no executável final (AOT).
3. **Scripts de Build (Opcional, mas Recomendado):** Para transparência e atualizações futuras, crie um script na pasta `tools/` que baixe o código fonte C/C++ original via `git clone`, rode o `cmake` ou similar, e mova os `.dll`/`.so`/`.dylib` resultantes para a pasta `lib/` apropriada.
