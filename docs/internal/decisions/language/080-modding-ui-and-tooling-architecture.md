# Decision 080 - Tooling and UI Architecture

- Status: accepted
- Date: 2026-04-18
- Type: tooling / project model / ui architecture
- Scope: debugger mapping, namespace-path contract, package source policy, hot reload strategy, UI composition model

## Summary

After the runtime architecture closure, Zenith defines tooling and project ergonomics needed for real daily use.

This decision closes five pillars: source-level debugging, path-driven namespace enforcement, Git-first package sourcing, native hot reload direction, and no-magic UI composition.

## Decision

### 1. Source-Level Debugging in C Backend

Users should debug `.zt` source, not generated C.

The backend should emit source mapping metadata (for example `#line` directives and debug symbols) so breakpoints and stack traces map back to Zenith source locations.

### 2. File Path Defines Namespace Contract

Namespace and directory structure must stay aligned.

If a file is under `src/app/banco_de_dados/remoto.zt`, its namespace must follow that path contract (`app.banco_de_dados` in this example).

This avoids drift between physical structure and semantic module identity.

### 3. Git-First Package Source Policy

The project does not require a central package registry in the core architecture phase.

Dependency sources can be resolved through explicit Git/HTTP package sources managed by tooling.

### 4. Native Hot Reload Direction

`zt run` may support fast reload workflows, but the core direction is native dynamic reload rather than introducing a mandatory VM/bytecode runtime for this purpose.

The host process keeps long-lived state where possible, and reloadable logic is rebuilt as dynamic native artifacts.

### 5. Zero-Magic UI Composition

Zenith does not adopt hidden DSL parsing (for example JSX-style language-level magic) for UI trees.

UI should be represented as explicit typed composition with ordinary language constructs and standard data structures.

## Consequences

- better debugger ergonomics without exposing generated C as user-facing source
- lower project confusion by enforcing namespace/path consistency
- simpler early ecosystem operation with Git-first dependency sourcing
- hot reload path aligned with native runtime strategy
- UI code remains explicit and predictable, consistent with reading-first philosophy
