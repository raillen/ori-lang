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
└── vendor/                    # Vendored dependencies
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
| **Pre-implementation** | Check `IMPLEMENTATION_CHECKLIST.md` and `IMPLEMENTATION_CHECKLIST_2.md` |
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
| `ORI_USE_RUST_LLD=1` | Use rust-lld instead of system linker |
| `ORI_REQUIRE_PACKAGED_RUNTIME=1` | Validate release package uses only packaged runtime |
| `UPDATE_EXPECT=1` | Update expected diagnostic outputs in tests |

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

## Current Status (2026-06-03)

- **Rust:** 1.95.0 (via rustup)
- **cargo check --workspace:** PASSES cleanly
- **cargo test --workspace:** PASSES cleanly (100% of tests pass, including advanced structural equality, file handles, and cooperative task cancellation)

## Known Pitfalls

1. **Native runtime staging:** `compile_runs` tests fail with `native.link_failed` → runtime needs re-staging. Fix: `cargo build -p ori-runtime --lib && cp target/debug/libori_runtime.a runtime/x86_64-unknown-linux-gnu/`

2. **OnceLock cache in tests:** `find_native_runtime_link()` caches the FIRST result across all tests. If first test finds broken runtime, all subsequent tests fail. Run single test first to verify fix.

3. **Runtime config:** `.cargo/config.toml` requires `relocation-model=pic`. `runtime-link.json` requires `-lpthread -ldl -lm -no-pie`.

4. **Diagnostic code prefixes:** MUST match catalog in `docs/spec/13-error-catalog.md`. Use `bind.duplicate_*` (not `name.duplicate_*`), `bind.stdlib_*` for stdlib availability.

5. **Ori syntax:** `end`-delimited blocks (not braces). Struct fields and enum variants are newline-separated. Enum variants with named fields use commas inside parens.

6. **Lock file:** Regenerate with Rust 1.95 if build fails: `cargo update` or delete `Cargo.lock` and rebuild.
