# Zenith Compiler Model Spec

- Status: canonical consolidated spec
- Date: 2026-04-17
- Scope: compiler pipeline, IR boundaries, C backend, runtime and artifacts

## Primary Pipeline

The canonical compilation path is:

```text
.zt source
  -> lexer
  -> parser
  -> AST
  -> semantic analysis
  -> HIR
  -> ZIR
  -> C backend
  -> native C compiler/linker
  -> native artifact
```

Rules:

- `.zt` is the user source language
- generated C is not user-facing source
- C is the first backend target
- ZIR structured is the internal IR contract
- textual ZIR exists only for debug, fixtures and golden tests
- the backend must not parse textual expressions as its main contract

## Frontend Stages

Frontend responsibilities:

- lex source into tokens with spans
- parse tokens into AST
- resolve namespaces and imports
- bind names and scopes
- type-check expressions and declarations
- validate mutability, traits, `where`, optional/result and generic constraints
- produce diagnostics with stable spans where possible

## HIR

HIR is typed and resolved.

HIR responsibilities:

- remove surface ambiguity
- normalize named arguments
- apply default parameters
- normalize method calls and receiver behavior
- represent resolved symbols and types
- preserve source spans for diagnostics

## ZIR

ZIR is the backend-facing compiler IR.

ZIR responsibilities:

Post-v1 ZIR closure requires:

- a verifier for structural and type invariants;
- explicit representation for calls, closures, ownership/runtime operations, cleanup, and control flow;
- a stable canonical type model for all backend-visible types;
- golden fixtures for representative source features;
- a backend conformance suite using the C backend as behavior oracle;
- source-span preservation sufficient for diagnostics and future debug information;
- no backend target may redefine Zenith semantics to fit its own model.

Core ZIR responsibilities:

- structured operands
- structured terminators
- canonical IR types
- explicit control flow
- explicit runtime operations where needed
- verifier-friendly invariants

Textual ZIR is allowed for:

- debug dumps
- fixtures
- golden tests
- human inspection

Textual ZIR is not:

- the user source language
- the primary backend contract
- a permanent public ABI

## C Backend

Backend C means:

```text
.zt -> compiler -> C -> native executable
```

Rules:

- generated C is an implementation artifact, not a readable target.
- **Source Maps:** The backend MUST emit `#line` directives mapping the generated C code directly to the original `.zt` source file. This guarantees that native debuggers (GDB/LLDB) halt and display Zenith code, eliminating the need for the user to ever read `.c`.
- users are not expected to write C to complete normal Zenith programs
- public C interop is not MVP
- runtime/stdlib may use C internally
- C nullable values must not introduce `null` into Zenith
- future C nullable interop maps to `optional<T>`

## Future Backend Direction

The current backend map is:

```text
C      = stable backend, compatibility layer, and behavior oracle
Zig    = modern textual systems backend exploration
C3     = C-like textual systems backend exploration
LLVM   = strategic native release backend
WASM   = sandbox/web backend
```

The future backend infrastructure map is:

```text
C backend = remains the oracle
ZIR       = rigid backend contract
Cranelift = fast native backend spike
LLVM      = official native release backend direction
```

Rules:

- C remains supported while other backends prove conformance.
- ZIR must be the shared contract for all backends, and that contract must be closed before Zig/LLVM/WASM backend work becomes active.
- Cranelift is a spike path for native lowering speed and development workflows.
- LLVM is the strategic native backend for optimized release artifacts.
- WASM is a sandbox/web target, either through LLVM or direct lowering.
- Zig and C3 are textual backend explorations, not replacements for the C oracle.

## Function ABI And Closures

Generated Zenith functions use an internal context-first ABI:

```c
return_type zt_module__function(void *zt_ctx, ...user_args)
```

Rules:

- `zt_ctx` is internal compiler/runtime plumbing
- direct calls pass `NULL` when there is no closure context
- closure calls pass the stored context pointer
- `func(...)` values lower to managed closure fat pointers
- trait/any wrappers must also pass the internal context argument
- user-facing Zenith syntax does not expose `zt_ctx`

## Generics And Monomorphization

The first C target strategy for generics is monomorphization.

Rules:

- the compiler must create canonical type keys for generic instantiations
- each canonical type argument set should generate at most one concrete instance
- equivalent instantiations must be deduplicated when possible
- generated backend helper names must use canonical type identity, not only a
  readable simple type name
- `build.monomorphization_limit` counts both generic runtime type shapes and
  concrete generic function specializations
- recursive or excessive instantiation must produce a diagnostic or build report before it becomes unbounded code generation
- nested generic containers must be covered by conformance tests

Monomorphization is accepted for the C backend, but uncontrolled code bloat is not accepted.

## Payload Enums And Exhaustive Match

Payload enums lower to a tag plus payload representation.

For enum values, `match` without `case else` must be exhaustive when the compiler knows the enum definition.

Rules:

- missing enum cases are compile-time diagnostics
- `case else` is allowed but opts out of strict future-proof exhaustiveness
- payload binding must be validated semantically before lowering
- generated C must not rely on invalid fallthrough for unhandled enum cases

## Runtime

Zenith has a native runtime, not a VM in the primary architecture.

Runtime responsibilities include:

- managed `text`
- lists
- maps
- reference counting
- panic reporting
- bounds checks
- contract checks
- platform abstraction used by future stdlib

The selected architecture is:

```text
.zt -> C -> native executable
```

Not selected as the primary architecture:

```text
.zt -> bytecode -> VM
```

## Value Semantics And Ownership

Zenith source uses value semantics by default.

Implementation may use RC, COW and internal moves to make this practical.

Rules:

- no user-facing borrow model in the MVP
- no `ref`, `&`, `&mut` or explicit `move` in the MVP
- observable `const` collection values are immutable
- cleanup must happen correctly on normal return, early return and `?` propagation
- `optional<T>` and `result<T,E>` should use stack/in-place representation where practical
- heap-first wrappers are implementation debt unless required by payload escape or size

## Evaluation Order

Evaluation order is left-to-right.

Rules:

- function arguments evaluate in written order
- receiver evaluates before method arguments
- defaults evaluate at call time after provided earlier arguments
- temporaries live until the end of the statement
- locals live until scope end
- generated C must not rely on unspecified C evaluation order

## Artifact Modes

Compilation and packaging are separate concerns.

`standalone` means one primary native executable where the platform permits.

`bundle` means an organized distribution directory.

`hot-reload` (Development Mode) means emitting dynamic shared objects that conform to the backend structure to allow engine reinjection.

Standalone conceptual output:

```text
dist/
  hello.exe
```

Hot Reload (`zt run --dev`) conceptual architecture:

```text
build/
  hello_host.exe  (Immortal state bounds/window)
  app_logic.dll   (Recompiled frame-by-frame on CTRL+S)
```

Rules:

- implement standalone first
- bundle may be recognized before implemented
- bundle should produce a clear unsupported-feature diagnostic until implemented
- hot-reload logic is isolated into DLLs injected natively. Zenith does not use custom bytecode VMs for dev-mode.
- artifact mode does not imply a VM

## Build Output

`build.output` defaults to `build`.

Generated files may be placed under profile/target-specific subdirectories.

Conceptual layout:

```text
build/
  debug/
    native/
      generated/
      objects/
      artifact/
```

Users should not edit generated files under `build/`.

## Driver Architecture

The compiler driver is modular, with shared state managed through an internal header.

```text
compiler/driver/
  driver_internal.h   (shared types, extern globals, cross-module prototypes)
  paths.c             (file I/O, path manipulation - zero cross-deps)
  main.c              (CLI dispatch, pipeline, command handlers)
  lsp.c               (independent LSP server)
  doc_show.c           (documentation display)
```

Rules:

- `driver_internal.h` is an internal header; do not include from outside `compiler/driver/`
- globals are defined in `main.c` and declared `extern` in the header
- `paths.c` contains pure utility functions with no dependencies on other driver modules
- the build system auto-discovers all `.c` files via `os.walk('compiler')`; no build changes needed when adding files
- new modules should follow the pattern: define functions in `.c`, declare prototypes in `driver_internal.h`

## Out Of Scope For MVP

Out of scope:

- VM/bytecode backend
- public C interop
- LLVM backend
- WASM backend
- JS backend
- cross-compilation policy
- native package installer generation
