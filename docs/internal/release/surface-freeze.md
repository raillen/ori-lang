# Surface Freeze

> Audience: maintainer
> Surface: release engineering
> Status: Phase 0 stabilization artifact

This file classifies public surface before the 1.0 push.

Labels:

- `stable`: user-facing contract for the current release line.
- `experimental`: available, but may still change with release notes.
- `internal`: implementation detail; not promised to users.
- `deferred`: documented direction, not shipped as a supported surface.

## Stable

- Core syntax listed as `Conformant` in `docs/spec/language/surface-implementation-status.md`.
- `zt` commands used by gates: `check`, `build`, `run`, `emit-c`, `fmt`, `doc check`, `test`, `repl`.
- `zenith.ztproj` project model documented in `docs/spec/language/project-model.md`.
- `zenith.lock` schema documented in `docs/spec/language/lockfile-schema.md`.
- Runtime diagnostics model and stable diagnostic codes under `docs/spec/language/diagnostics-model.md` and `docs/spec/language/diagnostic-code-catalog.md`.
- Standard library modules documented under `docs/spec/language/stdlib-model.md`, `docs/reference/stdlib/`, or the final language contract.
- Public examples under `examples/`.

## Experimental

- `packages/borealis` public package surface.
- Borealis editor metadata and Studio integration files.
- `std.console` terminal controls beyond basic line output and prompt helpers.
- Performance benchmark comparisons. They are regression signals, not marketing claims.
- Future translated docs. Translation trees are post-RC unless rebuilt from the
  current public docs and marked current.

## Internal

- Compiler C internals under `compiler/`.
- HIR, ZIR and generated C representation details, except documented debug fixtures.
- Runtime C implementation details under `runtime/c/`.
- Test harness internals under `tests/`.
- Reports, roadmaps and planning material under `docs/internal/`.

## Deferred

- Manual memory APIs.
- LLVM backend.
- Full implicit type inference.
- Full local type inference.
- Broad cycle-forming public ownership APIs.
- Non-`int` jobs, channels, shared state, and atomic runtime payloads.

## Promotion Rule

To promote a surface to `stable`:

- the final language contract or topic-specific spec must describe it;
- positive and negative tests must exist when behavior can fail;
- specs or decisions must match the implementation;
- `pr_gate --no-perf` must pass;
- release notes must mention user-visible behavior.
