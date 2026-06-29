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
| `ORI_USE_BUNDLED_RUST_LLD=1` | Bypass `rustc` entirely — invoke `rust-lld` directly with compiler-side CRT discovery (v0.3 Phase 1: Windows MSVC via `vswhere.exe` + Linux GNU via `cc -print-file-name`; macOS deferred) |
| `ORI_RUST_LLD` | Explicit path to `rust-lld[.exe]` for the bundled strategy (else discovered from `<ori.exe dir>` or `rustc` sysroot) |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Validate release package uses only packaged runtime |
| `UPDATE_EXPECT=1` | Update expected diagnostic outputs in tests |
| `ORI_TEST_LEAK_CHECK=1` | When set, `ori.test.assert_no_leaks(label)` aborts with a stderr diagnostic if live ARC allocations remain after running the cycle collector. Use in E2E tests to fail fast on memory leaks. |
| `ORI_COOPERATIVE_COLLECT_THRESHOLD=N` | Number of managed allocations between cooperative cycle collections in the async executor (default 256). Set to a small value in tests to force frequent collection. |
| `ORI_STDLIB_ROOT` | Override path to the `stdlib/` directory containing `.orl` source modules (v0.3 Chunk 3). When unset, resolves to `CARGO_MANIFEST_DIR/../../../stdlib` (dev mode) or `<ori.exe dir>/stdlib` (release package). |

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
- **Version:** `0.2.0` (release consolidada — Etapas 0–9 do `PLANO-MATURIDADE-COMPLETO.md` concluídas); v0.3 em desenvolvimento (Rust removal Phase 1 + stdlib Phase 0)
- **cargo check --workspace:** PASSES cleanly
- **cargo test --workspace:** PASSES cleanly (~580 tests, including advanced structural equality, file handles, cooperative task cancellation, async branching, Etapa 5 leak-check plumbing, Etapa 6 LSP cross-file semantics + `project.*` diagnostics, Etapa 7 diagnostic catalog audit, Etapa 8 monolith extractions, and Etapa 9 release smoke with `ORI_REQUIRE_PACKAGED_RUNTIME=1`, v0.3 BundledRustLld strategy + Windows MSVC CRT discovery regression tests)
- **Release smoke:** `tools/smoke_native_release.ps1 -SkipBuild` passes — `ori compile` + `ori test` validados em package isolado com runtime empacotada (Windows MSVC).
- **v0.3 Chunk 1 (Rust removal Phase 1 — Windows MSVC):** `ORI_USE_BUNDLED_RUST_LLD=1` engaja estratégia `BundledRustLld` que invoca `rust-lld` diretamente (sem `rustc` driver). CRT discovery via `vswhere.exe` + Windows SDK layout. Validado end-to-end com `examples/hello_world.orl` em Windows MSVC. `tools/stage_native_runtime.ps1` agora copia `rust-lld.exe` para `runtime/bin/`.
- **v0.3 Chunk 2 (Rust removal Phase 1 — Linux GNU):** Estratégia `BundledRustLld` estendida para `x86_64-unknown-linux-gnu`. CRT discovery via `cc -print-file-name` (crt1.o/crti.o/crtn.o) + `cc -print-search-dirs` (lib dirs) + fallback de paths comuns para dynamic linker. `tools/stage_native_runtime.sh` agora copia `rust-lld` para `runtime/bin/`. macOS deferido (requer `-flavor darwin` + `xcrun`).
- **v0.3 Chunk 3 (Stdlib Phase 0 — prelude loading):** Infraestrutura de prelude loading para `stdlib/*.orl` entregue. `import ori.string.utils` carrega `stdlib/string/utils.orl` (Layer 2 em `.orl`) que importa `ori.string` (Layer 1 manifesto) e expõe `is_empty`/`blank`/`replicate`. Convenção de path: `ori.X.Y` → `stdlib/X/Y.orl`. Stdlib root resolvia via `ORI_STDLIB_ROOT` → `CARGO_MANIFEST_DIR/../../../stdlib` → `<ori.exe dir>/stdlib`. Validado end-to-end (check → compile → run) com 2 testes de regressão em `multifile_imports.rs`.
- **Master plan:** `docs/planning/PLANO-MATURIDADE-COMPLETO.md` — Etapas 0–9 concluídas; backlog v2 em Apêndice C (stdlib em `.orl`, paridade C debug para async, mais triples, registry/installer, `ori doc` HTML). Roadmap v0.3+ fechado: híbrido A→B→D para Rust removal, 3 camadas explícitas para stdlib (detalhes em CHANGELOG `[Unreleased]`).

## Known Pitfalls

1. **Native runtime staging:** `compile_runs` tests fail with `native.link_failed` → runtime needs re-staging. Fix: `cargo build -p ori-runtime --lib && cp target/debug/libori_runtime.a runtime/x86_64-unknown-linux-gnu/`

2. **OnceLock cache in tests:** `find_native_runtime_link()` caches the FIRST result across all tests. If first test finds broken runtime, all subsequent tests fail. Run single test first to verify fix.

3. **Runtime config:** `.cargo/config.toml` requires `relocation-model=pic`. `runtime-link.json` requires `-lpthread -ldl -lm -no-pie`.

4. **Diagnostic code prefixes:** MUST match catalog in `docs/spec/13-error-catalog.md` (enforced by `diagnostic_catalog_matches_emitted_codes`). Convention: `name.*` for name resolution (`name.undefined`, `name.private`, `name.duplicate` for top-level duplicates); `bind.*` for binding/import/field/param errors (`bind.duplicate_field`, `bind.duplicate_param`, `bind.import_not_found`, `bind.stdlib_module_unknown`, `bind.stdlib_module_unavailable`). `bind.undefined` is a reserved alias only — the emitted code is `name.undefined`.

5. **Ori syntax:** `end`-delimited blocks (not braces). Struct fields and enum variants are newline-separated. Enum variants with named fields use commas inside parens.

6. **Lock file:** Regenerate with Rust 1.95 if build fails: `cargo update` or delete `Cargo.lock` and rebuild.
