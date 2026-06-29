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
| `ORI_USE_BUNDLED_RUST_LLD=1` | Bypass `rustc` entirely — invoke `rust-lld` directly with compiler-side CRT discovery (Rust removal Phase 1: Windows MSVC via `vswhere.exe` + Linux GNU via `cc -print-file-name` + macOS via `xcrun --show-sdk-path`) |
| `ORI_RUST_LLD` | Explicit path to `rust-lld[.exe]` for the bundled strategy (else discovered from `<ori.exe dir>` or `rustc` sysroot) |
| `ORI_USE_SYSTEM_LINKER=1` | Bypass `rustc` and `rust-lld` — invoke the platform system linker directly (`link.exe`/`ld`) with compiler-side CRT discovery (Rust removal Phase 2: Windows MSVC via `vswhere.exe` + `link.exe` discovery, Linux GNU via `cc -print-prog-name=ld`, macOS via `xcrun --find ld`) |
| `ORI_SYSTEM_LINKER` | Explicit path to the system linker (`link.exe`, `ld`, etc.) for the `SystemLinker` strategy |
| `ORI_USE_JIT=1` | Bypass the AOT compile+link path for `ori run` — execute Cranelift code in-process via `JITModule` with runtime symbols resolved from the staged cdylib through `libloading` (Rust removal Phase 3: no `.o` file, no linker, no subprocess). `ori compile` and `ori test` remain AOT. |
| `ORI_RUNTIME_CDYLIB` | Explicit path to the runtime cdylib (`ori_runtime.dll`/`libori_runtime.so`/`libori_runtime.dylib`) for the JIT path. When unset, resolves via packaged runtime → cargo fallback (same search order as `ORI_RUNTIME_LIB` but for the cdylib artifact). |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Validate release package uses only packaged runtime |
| `UPDATE_EXPECT=1` | Update expected diagnostic outputs in tests |
| `ORI_TEST_LEAK_CHECK=1` | When set, `ori.test.assert_no_leaks(label)` aborts with a stderr diagnostic if live ARC allocations remain after running the cycle collector. Use in E2E tests to fail fast on memory leaks. |
| `ORI_COOPERATIVE_COLLECT_THRESHOLD=N` | Number of managed allocations between cooperative cycle collections in the async executor (default 256). Set to a small value in tests to force frequent collection. |
| `ORI_STDLIB_ROOT` | Override path to the `stdlib/` directory containing `.orl` source modules (Stdlib Phase 0). When unset, resolves to `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode) or `<ori.exe dir>/stdlib` (release package). |

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

## Current Status (2026-06-29)

- **Rust:** 1.95.0 (via `rust-toolchain.toml`)
- **Version:** `0.2.0` (Etapas 0–9 do `PLANO-MATURIDADE-COMPLETO.md` concluídas). **Congelado em `0.2.x`** — ver "Versioning policy" abaixo. Marcos de desenvolvimento ativos (Rust removal Phase 1+2, Stdlib Phase 0) permanecem em `[Unreleased]` sem atribuir versão.
- **cargo check --workspace:** PASSES cleanly
- **cargo test --workspace:** PASSES cleanly (~594 tests, including advanced structural equality, file handles, cooperative task cancellation, async branching, Etapa 5 leak-check plumbing, Etapa 6 LSP cross-file semantics + `project.*` diagnostics, Etapa 7 diagnostic catalog audit, Etapa 8 monolith extractions, Etapa 9 release smoke with `ORI_REQUIRE_PACKAGED_RUNTIME=1`, BundledRustLld strategy + Windows MSVC CRT discovery + Linux GNU CRT discovery + macOS SDK discovery regression tests, SystemLinker strategy + system linker discovery regression tests, JIT Cranelift path + cdylib loading regression tests, Stdlib Phase 0 prelude loading + Layer 2 expansion regression tests)
- **Release smoke:** `tools/smoke_native_release.ps1 -SkipBuild` passes — `ori compile` + `ori test` validados em package isolado com runtime empacotada (Windows MSVC).
- **Rust removal Phase 1 — Windows MSVC (unreleased):** `ORI_USE_BUNDLED_RUST_LLD=1` engaja estratégia `BundledRustLld` que invoca `rust-lld` diretamente (sem `rustc` driver). CRT discovery via `vswhere.exe` + Windows SDK layout. Validado end-to-end com `examples/hello_world.orl` em Windows MSVC. `tools/stage_native_runtime.ps1` agora copia `rust-lld.exe` para `runtime/bin/`.
- **Rust removal Phase 1 — Linux GNU (unreleased):** Estratégia `BundledRustLld` estendida para `x86_64-unknown-linux-gnu`. CRT discovery via `cc -print-file-name` (crt1.o/crti.o/crtn.o) + `cc -print-search-dirs` (lib dirs) + fallback de paths comuns para dynamic linker. `tools/stage_native_runtime.sh` agora copia `rust-lld` para `runtime/bin/`.
- **Rust removal Phase 1 — macOS (unreleased):** Estratégia `BundledRustLld` estendida para `x86_64-apple-darwin` e `aarch64-apple-darwin`. CRT/SDK discovery via `xcrun --show-sdk-path` + `xcrun --show-sdk-version` (requer Xcode Command Line Tools). Link line `rust-lld -flavor darwin` com `-arch`, `-platform_version macos <min> <sdk>`, `-syslibroot`. Deployment target default `10.12` (x86_64) / `11.0` (arm64), override via `MACOSX_DEPLOYMENT_TARGET`. **Phase 1 completa para todos os 3 desktop OSes** (Windows MSVC, Linux GNU, macOS).
- **Rust removal Phase 2 — SystemLinker (unreleased):** Nova estratégia `SystemLinker` que invoca o linker nativo do sistema (`link.exe`/`ld`) diretamente, sem `rust-lld` nem `rustc`. Opt-in via `ORI_USE_SYSTEM_LINKER=1`, override via `ORI_SYSTEM_LINKER`. Reutiliza CRT discovery da Phase 1. Discovery: Windows — `link.exe` derivado do MSVC tools dir; Linux — `cc -print-prog-name=ld`; macOS — `xcrun --find ld`. Prioridade: `ORI_NATIVE_LINKER` (raw) → `ORI_USE_BUNDLED_RUST_LLD` → `ORI_USE_SYSTEM_LINKER` → `RustcDriver`. **Phase 2 completa para todos os 3 desktop OSes**. 4 testes de regressão em `native_backend/tests.rs`.
- **Rust removal Phase 3 — JIT Cranelift (unreleased):** `ORI_USE_JIT=1` despacha `ori run` para o path JIT — código Cranelift executado in-process via `JITModule` com símbolos `ori_*` resolvidos on-demand da cdylib do runtime através de `libloading`. Sem `.o` temporário, sem linker, sem subprocesso. `ori-runtime` agora builda 3 artefatos (`staticlib` + `rlib` + `cdylib`); stage scripts copiam cdylib para `runtime/<triple>/` e registram `runtime_cdylib` em `runtime-link.json`. `NativeBackend` refatorado para genérico sobre `M: Module` com `prepare()`/`into_module()`/`main_func_id()`. `ori compile` e `ori test` permanecem AOT (distribuição + isolamento de processo para `ori_test_assert`). 1 teste unitário em `native_backend/jit.rs` + 2 testes de integração em `ori-driver/tests/jit_run.rs` (subprocesso `ori run` com `ORI_USE_JIT=1`). **Híbrido A→B→D completo** para `ori run`.
- **Stdlib Phase 0 — prelude loading + Layer 2 expansion (unreleased):** Infraestrutura de prelude loading para `stdlib/*.orl` entregue. `import ori.string.utils` carrega `stdlib/string/utils.orl` (Layer 2 em `.orl`) que importa `ori.string` (Layer 1 manifesto) e expõe 7 funções `public`: `is_empty`/`blank`/`replicate` (bootstrap inicial) + `default`/`equals_ignore_case`/`center`/`count` (expansão Layer 2 — composição sobre `str.len`/`str.concat`/`str.trim`/`str.to_lower`/`str.pad_left`/`str.pad_right`/`str.slice`). Convenção de path: `ori.X.Y` → `stdlib/X/Y.orl`. Stdlib root resolvia via `ORI_STDLIB_ROOT` → `CARGO_MANIFEST_DIR/../../../stdlib` → `<ori.exe dir>/stdlib`. Naming collision documentada: variável local `len` colide com símbolo interno `ori_len` do runtime nativo — usar `s_len`/`sub_len`/etc. Validado end-to-end (check → compile → run) com 3 testes de regressão em `multifile_imports.rs`.
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

## Known Pitfalls

1. **Native runtime staging:** `compile_runs` tests fail with `native.link_failed` → runtime needs re-staging. Fix: `cargo build -p ori-runtime --lib && cp target/debug/libori_runtime.a runtime/x86_64-unknown-linux-gnu/`

2. **OnceLock cache in tests:** `find_native_runtime_link()` caches the FIRST result across all tests. If first test finds broken runtime, all subsequent tests fail. Run single test first to verify fix.

3. **Runtime config:** `.cargo/config.toml` requires `relocation-model=pic`. `runtime-link.json` requires `-lpthread -ldl -lm -no-pie`.

4. **Diagnostic code prefixes:** MUST match catalog in `docs/spec/13-error-catalog.md` (enforced by `diagnostic_catalog_matches_emitted_codes`). Convention: `name.*` for name resolution (`name.undefined`, `name.private`, `name.duplicate` for top-level duplicates); `bind.*` for binding/import/field/param errors (`bind.duplicate_field`, `bind.duplicate_param`, `bind.import_not_found`, `bind.stdlib_module_unknown`, `bind.stdlib_module_unavailable`). `bind.undefined` is a reserved alias only — the emitted code is `name.undefined`.

5. **Ori syntax:** `end`-delimited blocks (not braces). Struct fields and enum variants are newline-separated. Enum variants with named fields use commas inside parens.

6. **Lock file:** Regenerate with Rust 1.95 if build fails: `cargo update` or delete `Cargo.lock` and rebuild.
