# Zenith Language Knowledge Base

Quick-reference for generating correct Zenith (`.zt`) code.

## Files

| File | Content |
|------|---------|
| [syntax.md](syntax.md) | Complete syntax: keywords, operators, all constructs with examples |
| [types.md](types.md) | Type system: primitives, generics, traits, enums, structs |
| [semantics.md](semantics.md) | Scope, mutability, value semantics, ARC, visibility, public var |
| [stdlib.md](stdlib.md) | Standard library modules, APIs, naming conventions |
| [diagnostics.md](diagnostics.md) | Error codes, severity levels, diagnostic format |
| [compiler.md](compiler.md) | Pipeline stages, IR boundaries, source file locations |
| [formatting.md](formatting.md) | Mandatory formatting rules, naming conventions |
| [project-model.md](project-model.md) | Manifest, layout, imports, dependencies, lockfile |
| [runtime.md](runtime.md) | ARC, panic, contracts, managed values, checks |
| [not-implemented.md](not-implemented.md) | Features deferred post-MVP |

## Quick Facts

- **Version:** `0.4.1-alpha.1` (2026-05-09)
- **File extension:** `.zt`
- **Executable:** `zt`
- **Backend:** C (monomorphization for generics)
- **Memory:** Non-atomic ARC + COW
- **Philosophy:** reading-first, explicit, no hidden magic
- **Indentation:** 4 spaces, all blocks close with `end`
