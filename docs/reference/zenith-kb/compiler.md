# Zenith Compiler Pipeline

## Compilation Flow

```
.zt source
  -> lexer
  -> parser
  -> AST
  -> semantic analysis (binder + type checker)
  -> HIR (High-level IR)
  -> ZIR (Zenith IR)
  -> C backend
  -> native C compiler (gcc)
  -> native executable
```

Rules:
- `.zt` = user source language
- Generated C is NOT user-facing source
- C is the first (and currently only) backend target
- ZIR structured = internal IR contract
- Textual ZIR exists only for debug, fixtures, golden tests
- Backend must NOT parse textual expressions as main contract

## Source File Locations

### Stage 1: Lexer
```
compiler/frontend/lexer/lexer.c    -- tokenizer
compiler/frontend/lexer/lexer.h
compiler/frontend/lexer/token.c    -- token types
compiler/frontend/lexer/token.h
```

### Stage 2: Parser
```
compiler/frontend/parser/parser.c  -- recursive descent parser
compiler/frontend/parser/parser.h
```

### Stage 3: AST
```
compiler/frontend/ast/model.c      -- AST node definitions
compiler/frontend/ast/model.h
```

### Stage 4: Semantic Analysis

**Binder** (name resolution, scope, imports):
```
compiler/semantic/binder/binder.c
compiler/semantic/binder/binder.h
```

**Type System** (type checking, constraints):
```
compiler/semantic/types/types.c
compiler/semantic/types/types.h
compiler/semantic/types/checker.c
compiler/semantic/types/checker.h
```

**Symbols**:
```
compiler/semantic/symbols/
```

**Diagnostics**:
```
compiler/semantic/diagnostics/
```

**Parameter validation**:
```
compiler/semantic/parameter_validation.c
compiler/semantic/parameter_validation.h
```

### Stage 5: HIR (High-level IR)
```
compiler/hir/lowering/from_ast.c   -- AST-to-HIR lowering
compiler/hir/lowering/from_ast.h
compiler/hir/nodes/                -- HIR node definitions
```

HIR responsibilities:
- Remove surface ambiguity
- Normalize named arguments
- Apply default parameters
- Normalize method calls and receiver behavior
- Represent resolved symbols and types
- Preserve source spans

### Stage 6: ZIR (Zenith Intermediate Representation)
```
compiler/zir/model.c               -- structured IR model
compiler/zir/model.h
compiler/zir/parser.c              -- textual ZIR parser (debug/tests)
compiler/zir/parser.h
compiler/zir/printer.c             -- textual ZIR output
compiler/zir/printer.h
compiler/zir/verifier.c            -- invariant checking
compiler/zir/verifier.h
compiler/zir/lowering/from_hir.c   -- HIR-to-ZIR lowering
compiler/zir/lowering/from_hir.h
```

ZIR responsibilities:
- Structured operands
- Structured terminators
- Canonical IR types
- Explicit control flow
- Explicit runtime operations
- Verifier-friendly invariants

### Stage 7: C Backend
```
compiler/targets/c/emitter.c       -- ZIR-to-C emission
compiler/targets/c/emitter.h
compiler/targets/c/legalization.c  -- C sequence legalization
compiler/targets/c/legalization.h
```

Rules:
- Generated C is implementation artifact, not readable target
- Must emit `#line` directives mapping C to original `.zt`
- Native debuggers (GDB/LLDB) display Zenith code
- C nullable values must NOT introduce `null` into Zenith

## Driver / CLI

```
compiler/driver/driver_internal.h  -- shared types, extern globals
compiler/driver/main.c             -- CLI dispatch, pipeline, handlers
compiler/driver/pipeline.c         -- full compilation pipeline
compiler/driver/paths.c            -- file I/O, path manipulation
compiler/driver/lsp.c              -- LSP server
compiler/driver/doc_show.c         -- documentation display
compiler/driver/project.c          -- project handling
```

Rules:
- `driver_internal.h` is internal; do not include from outside `compiler/driver/`
- Globals defined in `main.c`, declared `extern` in header
- `paths.c` = pure utilities, no cross-deps
- Build auto-discovers all `.c` files via `os.walk('compiler')`

## Project / ZDoc

```
compiler/project/ztproj.c          -- manifest parsing
compiler/project/ztproj.h
compiler/project/zdoc.c            -- documentation processing
compiler/project/zdoc.h
```

## Tooling

```
compiler/tooling/formatter.c       -- deterministic formatter
compiler/tooling/formatter.h
```

## Utilities

```
compiler/utils/arena.c             -- memory arena allocator
compiler/utils/arena.h
compiler/utils/string_pool.c       -- string interning
compiler/utils/string_pool.h
compiler/utils/diagnostics.h       -- diagnostic types
compiler/utils/l10n.c              -- localization
compiler/utils/l10n.h
```

## Runtime (C)

```
runtime/c/zenith_rt.c              -- core runtime (ARC, text, list, map, panic, contracts)
runtime/c/zenith_rt.h
runtime/c/zenith_rt_templates.h    -- code generation templates
```

## Generics and Monomorphization

Strategy: **Monomorphization** (zero-cost abstraction)

Rules:
- Compiler creates canonical type keys for generic instantiations
- Each canonical type argument set generates at most one concrete instance
- Equivalent instantiations are deduplicated
- Recursive/excessive instantiation produces diagnostic before unbounded code gen
- Nested generic containers covered by conformance tests
- Uncontrolled code bloat is NOT accepted

## Payload Enums and Match

- Payload enums lower to tag + payload representation
- Match without `case else` must be exhaustive
- Missing enum cases = compile-time diagnostic
- `case else` is allowed but opts out of strict future-proof exhaustiveness
- Generated C must NOT rely on invalid fallthrough for unhandled cases

## Value Semantics and Ownership

- No user-facing borrow model in MVP
- No `ref`, `&`, `&mut`, or explicit `move` in MVP
- Observable `const` collection values are immutable
- Cleanup on: normal return, early return, `?` propagation
- `optional<T>` and `result<T,E>` should use stack/in-place representation
- Heap-first wrappers = implementation debt unless required by payload escape/size

## Evaluation Order

- Left-to-right always
- Function arguments evaluate in written order
- Receiver evaluates before method arguments
- Defaults evaluate at call time after provided earlier arguments
- Temporaries live until end of statement
- Locals live until scope end
- Generated C must NOT rely on unspecified C evaluation order

## Artifact Modes

| Mode | Description |
|------|-------------|
| `standalone` | One primary native executable |
| `bundle` | Organized distribution directory |
| `hot-reload` | DLL injection for dev mode (`zt run --dev`) |

Build defaults:
- `target = "native"`
- `output = "build"`
- `profile = "debug"`
- `monomorphization_limit = 1024`

Accepted profiles: `debug`, `release`

## Out of Scope (MVP)

- VM/bytecode backend
- Public C interop
- LLVM backend
- WASM backend
- JS backend
- Cross-compilation policy
- Native package installer generation
