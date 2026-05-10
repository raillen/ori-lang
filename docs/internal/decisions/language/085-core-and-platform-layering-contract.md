# Decision 085 - Core And Platform Layering Contract

- Status: accepted
- Date: 2026-04-21
- Type: language / stdlib / architecture
- Scope: `core` boundary, `platform` boundary, ownership with `std.os` and `std.os.process`

## Summary

Zenith keeps `core` implicit and keeps user-facing OS/process APIs in `std.os` and `std.os.process`.

`platform` is reserved for internal adaptation logic and must not become a second public OS/process API surface.

## Decision

Normative rules for this cut:

- `core.*` remains implicit and language-owned.
- users do not import `core`.
- `stdlib/core/` is reserved for internal documentation, contracts and internal test helpers.
- user-facing APIs for current-process state stay in `std.os`.
- user-facing APIs for child-process execution stay in `std.os.process`.
- `stdlib/platform/` is internal and may be used by stdlib/runtime implementation code.
- `platform` is not a public module family for ordinary app code in this cut.
- `platform` must not duplicate APIs already owned by `std.os` or `std.os.process`.

## Ownership Boundary

Public ownership:

- `std.os`: environment, platform/arch identity, pid, current directory.
- `std.os.process`: child-process execution and child-process result handling.

Internal ownership:

- `platform`: target and host adaptation details needed to implement `std.*` modules safely across systems.

## Activation Gate For `platform`

`platform` implementation beyond placeholder docs is justified only when at least one of these conditions is true:

- repeated cross-platform adaptation code appears in two or more `std.*` modules
- repeated platform error mapping causes drift or inconsistent behavior
- recurring cross-platform regressions show that a shared internal layer is needed
- the same capability-detection logic is duplicated and cannot stay clean inside one module

If these conditions are not met, keep `platform` as a reserved placeholder.

## Non-Canonical Forms

Treating `platform` as a public substitute for `std.os`:

```zt
import platform.os as os
```

Duplicating process APIs under `platform` while `std.os.process` already owns them.

Moving language-level core semantics into an explicit stdlib import.

## Out of Scope

This decision does not define:

- detailed internal file layout for `platform`
- full list of future cross-platform adapters
- any public compatibility layer that re-exports `platform` to app code
