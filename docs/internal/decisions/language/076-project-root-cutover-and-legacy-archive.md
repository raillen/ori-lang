# Decision 076: Project Root Cutover and Legacy Archive

Status: accepted

Date: 2026-04-18

## Context

The active Zenith implementation moved from the previous Lua/self-hosted compiler
line to the Zenith Next compiler architecture.

The root of the repository must not keep presenting the legacy compiler as the
official entrypoint, because that conflicts with the language direction, the C
backend goal, and the current documentation in `docs/`.

## Decision

The repository root is now an index for the active Zenith Next project.

The active compiler, language specs, implementation roadmap, checklist, runtime,
stdlib work and tests live under:

```text
compiler/
runtime/c/
stdlib/
docs/spec/language/
docs/internal/decisions/language/
tests/
```

The previous implementation is preserved under:

```text
_legacy/
```

## Rules

- New compiler work goes in `compiler/`.
- New runtime work goes in `runtime/`.
- New stdlib work goes in `stdlib/`.
- New language specs go in `docs/spec/language/`.
- New language decisions go in `docs/internal/decisions/language/`.
- New tests for the C compiler path go in `tests/`.
- `_legacy/` is read-only for normal development.
- No new feature should be implemented in `_legacy/`.
- If a legacy behavior is still useful, it must be redesigned against the Zenith
  Next specs and reimplemented in the active compiler/runtime tree.

## Rationale

This keeps the project visually and operationally coherent:

- the root no longer advertises Lua/self-host as the official compiler path;
- the active implementation is easy to find;
- legacy knowledge is not lost;
- future contributors do not have to infer which compiler line is authoritative;
- the repository structure matches the goal of compiling Zenith source to C.

## Consequences

The repository now has two clear areas:

```text
compiler/, runtime/c/, stdlib/, docs/, tests/   active implementation
_legacy/   archived previous implementation
```

Some old root-level docs, scripts and generated artifacts were moved into
`_legacy/`. They are historical material, not implementation guidance for the
current compiler.

The root `README.md` must remain a short entrypoint that points readers to
`docs/`.

