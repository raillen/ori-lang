# Ori Language — Project Context

> Ori is a reading-first, explicitly typed programming language. Compiler written in Rust.

## Skills Globais (sempre ativas)

Este projeto segue as skills universais de qualidade:
- **`clean-code`** — Nomenclatura, funções, tratamento de erros, organização
- **`project-documentation`** — Documentação incremental (implementação + uso)
- **`semantic-html-a11y`** — HTML semântico e acessibilidade (quando aplicável)

## Architecture

```
ori-lang/
├── compiler/crates/           # Rust compiler (Cargo workspace)
│   ├── ori-lexer/             #   Lexer + tokenizer (logos-based)
│   ├── ori-ast/               #   AST definitions (all nodes)
│   ├── ori-parser/            #   Parser (recursive descent)
│   ├── ori-types/             #   Type system (Ty, TyKind, TypeChecker trait)
│   ├── ori-hir/               #   High-level IR (lowered from AST)
│   ├── ori-codegen/           #   Code generation (Cranelift native + C debug)
│   ├── ori-runtime/           #   Native runtime library (Rust staticlib)
│   ├── ori-diagnostics/       #   Diagnostic codes + rendering
│   ├── ori-lsp/               #   LSP server (tower-lsp)
│   └── ori-driver/            #   CLI driver + integration tests
├── runtime/                   # Pre-built runtime static libs per target triple
│   ├── x86_64-unknown-linux-gnu/
│   └── x86_64-pc-windows-msvc/
├── stdlib/                    # Ori standard library (.orl source)
├── docs/
│   ├── spec/                  #   Language specification (normative)
│   └── planning/              #   Implementation plans
├── tests/                     # End-to-end Ori test programs (.orl)
├── _reversa_sdd/              # Historical audit documents (reverse engineering)
├── branding/                  # Logo and brand assets
├── examples/                  # Example Ori programs
├── extensions/vscode-orl/       # VS Code extension (LanguageClient → ori-lsp)
├── tools/                     # Auxiliary tools
└── (vendor/ — reserved for future vendored deps; not created yet)
```

## Convention Matrix

| Aspect | Convention |
|--------|------------|
| **Docs** | Portuguese (Brazil) |
| **Code + comments** | English |
| **Compiler design** | Follow best practices |
| **Testing** | Always use `ori-testing` skill for new features |
| **Changelog** | Always update `CHANGELOG.md` with changes |
| **Bug fixes** | Always add regression test in `compiler/crates/ori-driver/tests/` |
| **Pre-implementation** | Check `docs/planning/PLANO-MATURIDADE-COMPLETO.md` and `docs/planning/PENDENTES.md` |
| **Stdlib changes** | Update `stdlib.rs`, `lower.rs` (stdlib_c_name + stdlib_c_func_ty), and changelog |
| **Documentation** | Keep `spec/` (normative), `planning/` (plans), `_reversa_sdd/` (historical) |
| **Dedup** | Consolidate documents of same scope, avoid duplication |

## Key Files

| File | Role |
|------|------|
| `compiler/crates/ori-runtime/src/lib.rs` | Canonical native runtime (Rust) |
| `compiler/crates/ori-driver/src/main.rs` | CLI entry point |
| `CHANGELOG.md` | All notable changes (Keep a Changelog format) |
| `Cargo.toml` | Workspace root (10 crates) |
| `docs/spec/13-error-catalog.md` | Diagnostic code registry |
| `.cargo/config.toml` | relocation-model=pic for PIE-compatible runtime |

## Build & Test Commands

```bash
# Check entire workspace
cargo check --workspace

# Run all tests
cargo test --workspace

# Run specific test suite
cargo test -p ori-driver --test ori_spec

# Build runtime (for native backend staging)
cargo build -p ori-runtime --lib
cp target/debug/libori_runtime.a runtime/x86_64-unknown-linux-gnu/

# Run diagnostic catalog consistency test
cargo test -p ori-driver --test diagnostic_catalog

# Ori CLI (from workspace root)
cargo run -p ori-driver -- check <file.orl>
cargo run -p ori-driver -- compile <file.orl>
cargo run -p ori-driver -- run <file.orl>
```

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `ORI_RUNTIME_LIB` | Override path to runtime static library |
| `ORI_NATIVE_LINKER` | Diagnose raw native linker route |
| `ORI_USE_RUST_LLD=1` | Use rust-lld instead of system linker (still via `rustc` driver) |
| `ORI_USE_BUNDLED_RUST_LLD=1` | Force bundled `rust-lld` (hard fail if discovery fails). By default, bundled lld is tried automatically before `rustc`. |
| `ORI_USE_RUSTC_DRIVER=1` | Opt back into the legacy `rustc` link driver when bundled/system linkers are available |
| `ORI_RUST_LLD` | Explicit path to `rust-lld[.exe]` for the bundled strategy (else discovered from `<ori.exe dir>` or `rustc` sysroot) |
| `ORI_USE_SYSTEM_LINKER=1` | Bypass `rustc` and `rust-lld` — invoke the platform system linker directly (`link.exe`/`ld`) with compiler-side CRT discovery (Rust removal Phase 2: Windows MSVC via `vswhere.exe` + `link.exe` discovery, Linux GNU via `cc -print-prog-name=ld`, macOS via `xcrun --find ld`) |
| `ORI_SYSTEM_LINKER` | Explicit path to the system linker (`link.exe`, `ld`, etc.) for the `SystemLinker` strategy |
| `ORI_USE_JIT=1` | Force JIT for `ori run` — execute Cranelift code in-process via `JITModule` with runtime symbols resolved from the staged cdylib through `libloading` (Rust removal Phase 3: no `.o` file, no linker, no subprocess). When unset, JIT is the default whenever a runtime cdylib is available. `ori compile` and `ori test` remain AOT. |
| `ORI_USE_AOT=1` | Force AOT compile+link for `ori run` even when a runtime cdylib is available (opt-out of JIT default). |
| `ORI_RUNTIME_CDYLIB` | Explicit path to the runtime cdylib (`ori_runtime.dll`/`libori_runtime.so`/`libori_runtime.dylib`) for the JIT path. When unset, resolves via packaged runtime → cargo fallback (same search order as `ORI_RUNTIME_LIB` but for the cdylib artifact). |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Validate release package uses only packaged runtime |
| `UPDATE_EXPECT=1` | Update expected diagnostic outputs in tests |
| `ORI_TEST_LEAK_CHECK=1` | When set, `ori.test.assert_no_leaks(label)` aborts with a stderr diagnostic if live ARC allocations remain after running the cycle collector. Use in E2E tests to fail fast on memory leaks. |
| `ORI_COOPERATIVE_COLLECT_THRESHOLD=N` | Number of managed allocations between cooperative cycle collections in the async executor (default 256). Set to a small value in tests to force frequent collection. |
| `ORI_STDLIB_ROOT` | Override path to the `stdlib/` directory containing `.orl` source modules (Stdlib Phase 0). When unset, resolves to `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode) or `<ori.exe dir>/stdlib` (release package). |
| `ori.lsp.path` / `ori.compiler.path` / `ori.stdlib.root` | VS Code extension settings (`extensions/vscode-orl/`) — forwarded to `ORI_*` env vars when spawning `ori-lsp`. |

## Compiler Pipeline

```
Source (.orl)
  → Lexer (ori-lexer): tokens
  → Parser (ori-parser): AST
  → Resolver (ori-hir): name resolution, binding
  → Type Checker (ori-types): type inference + diagnostics
  → Codegen (ori-codegen):
      ├── Native: Cranelift → object file → link with ori-runtime
      └── C debug: transpile to C (partial parity)
  → Binary
```

## Current Status (2026-07-01)

- **Rust:** 1.95.0 (via `rust-toolchain.toml`)
- **Version:** `0.2.0` (Etapas 0–9 do `PLANO-MATURIDADE-COMPLETO.md` concluídas). **Congelado em `0.2.x`** — ver "Versioning policy" abaixo. Marcos de desenvolvimento ativos (Rust removal, Stdlib Phase 0, rede v2) permanecem em `[Unreleased]` sem atribuir versão.
- **cargo check --workspace:** PASSES cleanly
- **cargo test --workspace:** PASSES cleanly (~690+ tests, including stdlib Layer 2/3, net v2 E2E, io streams, JIT default)
- **Release smoke:** `tools/smoke_native_release.ps1 -SkipBuild` passes — `ori compile` + `ori test` validados em package isolado com runtime empacotada (Windows MSVC).
- **Rust removal Phase 1 — Windows MSVC (unreleased):** `ORI_USE_BUNDLED_RUST_LLD=1` engaja estratégia `BundledRustLld` que invoca `rust-lld` diretamente (sem `rustc` driver). CRT discovery via `vswhere.exe` + Windows SDK layout. Validado end-to-end com `examples/hello_world.orl` em Windows MSVC. `tools/stage_native_runtime.ps1` agora copia `rust-lld.exe` para `runtime/bin/`.
- **Rust removal Phase 1 — Linux GNU (unreleased):** Estratégia `BundledRustLld` estendida para `x86_64-unknown-linux-gnu`. CRT discovery via `cc -print-file-name` (crt1.o/crti.o/crtn.o) + `cc -print-search-dirs` (lib dirs) + fallback de paths comuns para dynamic linker. `tools/stage_native_runtime.sh` agora copia `rust-lld` para `runtime/bin/`.
- **Rust removal Phase 1 — macOS (unreleased):** Estratégia `BundledRustLld` estendida para `x86_64-apple-darwin` e `aarch64-apple-darwin`. CRT/SDK discovery via `xcrun --show-sdk-path` + `xcrun --show-sdk-version` (requer Xcode Command Line Tools). Link line `rust-lld -flavor darwin` com `-arch`, `-platform_version macos <min> <sdk>`, `-syslibroot`. Deployment target default `10.12` (x86_64) / `11.0` (arm64), override via `MACOSX_DEPLOYMENT_TARGET`. **Phase 1 completa para todos os 3 desktop OSes** (Windows MSVC, Linux GNU, macOS).
- **Rust removal Phase 2 — SystemLinker (unreleased):** Nova estratégia `SystemLinker` que invoca o linker nativo do sistema (`link.exe`/`ld`) diretamente, sem `rust-lld` nem `rustc`. Opt-in via `ORI_USE_SYSTEM_LINKER=1`, override via `ORI_SYSTEM_LINKER`. Reutiliza CRT discovery da Phase 1. Discovery: Windows — `link.exe` derivado do MSVC tools dir; Linux — `cc -print-prog-name=ld`; macOS — `xcrun --find ld`. Prioridade: `ORI_NATIVE_LINKER` (raw) → `ORI_USE_BUNDLED_RUST_LLD` → `ORI_USE_SYSTEM_LINKER` → `RustcDriver`. **Phase 2 completa para todos os 3 desktop OSes**. 4 testes de regressão em `native_backend/tests.rs`.
- **Rust removal Phase 3 — JIT Cranelift (unreleased):** `ori run` usa JIT por default quando cdylib disponível; `ORI_USE_JIT=1` força JIT; `ORI_USE_AOT=1` força AOT. Código Cranelift executado in-process via `JITModule` com símbolos `ori_*` resolvidos on-demand da cdylib do runtime através de `libloading`. Sem `.o` temporário, sem linker, sem subprocesso. `ori-runtime` builda 3 artefatos (`staticlib` + `rlib` + `cdylib`); stage scripts copiam cdylib para `runtime/<triple>/`; smoke release valida cdylib staged + `ori run` JIT no package isolado. `ori compile` e `ori test` permanecem AOT. **Híbrido A→B→D completo** para `ori run`.
- **Stdlib Phase 0 + Gap parity (unreleased):** Prelude loading + **Layer 2/3 `.orl` fechados** para paridade `std.*` v1 (`docs/planning/stdlib-gap-parity.md`): 28 utils + 8 algorithms + `validate`/`path`; Layer 1 hot path Rust (FS metadados, `os.current_dir`, `process.*`, `net.*`, `lazy.is_consumed`, …). Lowering `ori.net.Connection`/`Listener`/`UdpSocket` e `ori.io.Input`/`Output` para módulos `.orl`. ~36 testes stdlib E2E em `multifile_imports.rs` (incl. rede v2).
- **Stdlib/Rede v2 (unreleased):** `connect_tls`, servidor TCP (`listen`/`accept`), UDP síncrono, `task.run_blocking`; design `docs/planning/net-v2-design.md`; exemplo `examples/http_get.orl`.
- **LSP/VS Code (unreleased):** Catálogo stdlib Layer 1+2, hover/goto stdlib, sync incremental, dot-complete via aliases, `ori doctor`, extensão `extensions/vscode-orl/`.
- **Docs website (unreleased):** Site Starlight em [github.com/raillen/ori-website](https://github.com/raillen/ori-website) — i18n en/pt/es/ja, Pagefind + busca ⌘K, referência gerada via `ori doc export`. Deploy Vercel-ready (`vercel.json`).
- **Master plan:** `docs/planning/PLANO-MATURIDADE-COMPLETO.md` — Etapas 0–9 concluídas; backlog v2 em Apêndice C (stdlib em `.orl`, paridade C debug para async, mais triples, registry/installer, `ori doc` HTML). Roadmap fechado: híbrido A→B→D para Rust removal (Phase 3 completa), 3 camadas explícitas para stdlib (detalhes em CHANGELOG `[Unreleased]`).

## Versioning policy (2026-06-29)

**Congelado em `0.2.x`.** A escalada `0.1.0 → 0.2.0 → 0.3.0` (planejada) em dias foi precipitada. Comparação com pares:

| Linguagem | Tempo em 0.x | Versão atual | Status |
|-----------|-------------|--------------|--------|
| Zig | ~10 anos | 0.14 | Consolidada, ainda sem 1.0 |
| Rust | ~6 anos (pre-1.0) | 1.0 em 2015 | Estável após 0.12 |
| Ori | dias | 0.2.0 | Pre-1.0, desenvolvimento ativo |

**Regras até 1.0:**
- Marcos de desenvolvimento ficam em `[Unreleased]` no CHANGELOG **sem atribuir versão**.
- `0.3.0` só quando houver **breaking change real** que usuários precisem saber (não por ter terminado um marco).
- Patch versions (`0.2.1`, `0.2.2`) para correções e small additive features.
- `1.0` é critério de maturidade (anos, não dias):
  1. Rust dependency totalmente removida (Rust removal Phase 1+2+3).
  2. Stdlib portada em `.orl` (Layer 2+3 substantivas).
  3. Compiler self-hosting (ou pelo menos provando que consegue).
  4. ABI estável documentada.
  5. Usuários reais (mesmo que poucos).
  6. Sem breaking changes por ≥6 meses.

**Motivo:** o `ori compile` ainda precisa de Rust toolchain (Phase 1 mitiga, não resolve), a stdlib é 95% manifesto Rust (Phase 0 mal começou), não há bootstrapping, não há usuários além de testes. Chamar isso de "release" 0.3/0.4/0.5 infla a percepção de maturidade.

## Rust Independence Strategy (2026-07-02)

> Definição de "independência do Rust": **um usuário final pode instalar Ori, compilar, rodar e testar programas Ori sem ter a toolchain Rust instalada.**  
> Isso não significa que o *compilador em si* deixa de ser escrito em Rust — isso só ocorre com self-hosting (futuro longo prazo).

### O que já foi feito (Rust removal híbrido A→B→D)

| Phase | O que faz | Status | Gap residual |
|-------|-----------|--------|--------------|
| **Phase 1 — BundledRustLld** | Invoca `rust-lld` direto, sem `rustc` driver | ✅ Completo (3 OSes) | Ainda precisa do binário `rust-lld` (vem do Rust toolchain) |
| **Phase 2 — SystemLinker** | Invoca linker nativo do sistema (`link.exe`/`ld`/`ld64`) direto | ✅ Completo (3 OSes) | Requer toolchain do OS (VS Build Tools, build-essential, Xcode CLT) |
| **Phase 3 — JIT Cranelift** | `ori run` sem `.o`, sem linker, sem subprocesso | ✅ Completo | `ori compile`/`ori test` ainda usam AOT (precisam de linker) |
| **Phase 4 — Self-hosting** | Compilador escrito em Ori | ❌ Não iniciado | Anos de trabalho; adiado até haver usuários reais |

### Pré-requisitos do sistema por OS (para AOT)

O `ori compile` e `ori test` precisam de um linker. O default é o **SystemLinker** (desde 2026-07-02). O usuário final precisa ter instalado:

| OS | Pré-requisito | Como instalar | Ori precisa de Rust? |
|----|---------------|---------------|----------------------|
| **Windows** | Visual Studio Build Tools (ou VS Community) | `winget install Microsoft.VisualStudio.2022.BuildTools` ou via installer com workload "Desktop development with C++" | **Não** |
| **Linux** | `build-essential` (`gcc` + `ld`) | `sudo apt install build-essential` (Debian/Ubuntu) ou equivalente | **Não** |
| **macOS** | Xcode Command Line Tools | `xcode-select --install` | **Não** |

Para `ori run` (JIT): **nenhum linker é necessário** — apenas o cdylib do runtime (`ori_runtime.dll` / `.so` / `.dylib`) empacotado no release.

### Decisões arquiteturais fechadas

1. **Self-hosting adiado indefinidamente.** Não é pré-requisito para utilidade. Python, Ruby, Lua nunca foram self-hosted. Zig está em 0.14 após ~10 anos. Self-hosting será reconsiderado quando houver usuários reais estáveis.
2. **Runtime Layer 1 permanece Rust.** ARC, async executor, FFI, I/O e rede são hot paths que beneficiam da safety do Rust. A ABI C é o contrato público; a implementação interna pode mudar no futuro.
3. **SystemLinker é o default para AOT.** A partir de 2026-07-02, `NativeLinker::discover()` tenta `SystemLinker` antes de `BundledRustLld` no caminho default. Isso elimina a dependência de `rust-lld` (binário do Rust toolchain) para usuários finais que já têm o linker do sistema.
4. **Rust continua necessário apenas para *desenvolver* o compilador.** Quem clona o repo e trabalha no código do compilador precisa de `cargo` + `rustc`. Quem instala via release package não precisa.

### Critérios para 1.0 (mantidos, sem mudança)

1. Rust dependency totalmente removida para usuários finais
2. Stdlib portada em `.orl` (Layer 2+3 substantivas; Layer 1 permanece Rust)
3. Compiler self-hosting **ou** bootstrapping documentado
4. ABI estável documentada
5. Usuários reais
6. Sem breaking changes por ≥6 meses

### Próximos passos táticos

- [ ] Smoke em máquina Windows sem Rust instalado (apenas VS Build Tools)
- [ ] Smoke em máquina Linux sem Rust instalado (apenas build-essential)
- [ ] Smoke em máquina macOS sem Rust instalado (apenas Xcode CLT)
- [ ] CI job que valida release package em runner sem Rust
- [ ] Documentar instalação de prereqs do sistema (`docs/install.md`)

## Known Pitfalls

1. **Native runtime staging:** `compile_runs` tests fail with `native.link_failed` → runtime needs re-staging. Fix: `cargo build -p ori-runtime --lib && cp target/debug/libori_runtime.a runtime/x86_64-unknown-linux-gnu/`

2. **OnceLock cache in tests:** `find_native_runtime_link()` caches the FIRST result across all tests. If first test finds broken runtime, all subsequent tests fail. Run single test first to verify fix.

3. **Runtime config:** `.cargo/config.toml` requires `relocation-model=pic`. `runtime-link.json` requires `-lpthread -ldl -lm -no-pie`.

4. **Diagnostic code prefixes:** MUST match catalog in `docs/spec/13-error-catalog.md` (enforced by `diagnostic_catalog_matches_emitted_codes`). Convention: `name.*` for name resolution (`name.undefined`, `name.private`, `name.duplicate` for top-level duplicates); `bind.*` for binding/import/field/param errors (`bind.duplicate_field`, `bind.duplicate_param`, `bind.import_not_found`, `bind.stdlib_module_unknown`, `bind.stdlib_module_unavailable`). `bind.undefined` is a reserved alias only — the emitted code is `name.undefined`.

5. **Ori syntax:** `end`-delimited blocks (not braces). Struct fields and enum variants are newline-separated. Enum variants with named fields use commas inside parens.

6. **Lock file:** Regenerate with Rust 1.95 if build fails: `cargo update` or delete `Cargo.lock` and rebuild.

7. **Windows LSP process lock:** `ori-lsp.exe` may remain locked in memory on Windows, preventing `cargo test --workspace` from rebuilding the `ori-lsp` crate. Workaround: `taskkill /F /IM ori-lsp.exe` before running tests, or use `cargo test -p <crate>` crate-by-crate.

8. **CDYLIB desynchronization after runtime changes:** When new FFI functions are added to `ori-runtime`, the `cdylib` (`.dll`/`.so`/`.dylib`) must be re-staged alongside the static library. An outdated `cdylib` causes `ori run` (JIT default) to produce corrupted results or panic with undefined behavior (`ptr::copy_nonoverlapping` violation). Fix: `cargo build -p ori-runtime --lib` and copy both the staticlib and the cdylib into `runtime/<triple>/`.
