# Decision 049 - Build Conditionals And Feature Flags

- Status: accepted
- Date: 2026-04-17
- Type: language / project / build
- Scope: conditional compilation, target-specific code, feature flags, `zenith.ztproj`, stdlib target selection

## Summary

Zenith does not include conditional compilation syntax in the MVP language.

Target selection, build profiles and feature flags belong in `zenith.ztproj`, future ZPM/package metadata and stdlib/runtime target selection.

Source code should remain portable and avoid preprocessor-style branches.

## Decision

Normative rules for this cut:

- no conditional compilation syntax in MVP `.zt`
- no `when target is ...` in MVP
- no `#ifdef`-style preprocessor
- no `cfg` syntax in MVP
- no target-specific attributes in MVP
- target selection belongs in `zenith.ztproj`
- feature flags belong in project/package configuration
- stdlib/runtime may select platform implementations internally
- user code should prefer portable imports and APIs
- platform-specific code should be isolated in modules/packages
- any future source-level conditional compilation requires a separate decision

## Non-Canonical Source-Level Conditionals

Not part of the MVP:

```zt
when target is windows
    const separator: text = "\\"
else
    const separator: text = "/"
end
```

Not part of the MVP:

```zt
when feature sqlite
    ...
end
```

Not part of the MVP:

```zt
attr target(os: "windows")
func windows_only()
    ...
end
```

## Project-Level Direction

Build and target information should live in `zenith.ztproj` or future package metadata.

Possible future direction:

```toml
[build]
target = "native"

[features]
sqlite = true
network = false

[target.windows]
...
```

The exact schema belongs to project/ZPM design, not language syntax.

## Stdlib Target Selection

User code should prefer portable stdlib APIs:

```zt
import std.path as path
```

The stdlib/runtime may choose the target implementation internally.

The user should not need to write platform branches for ordinary path handling, filesystem access or process APIs.

## Why Keep This Out Of The Language

Source-level conditional compilation can:

- fragment code paths
- hide behavior
- make diagnostics harder
- complicate parsing and semantic analysis
- create target-specific dialects
- reduce readability
- encourage preprocessor-style design

Keeping target selection in project configuration preserves cleaner source files.

## Diagnostics

Expected diagnostic direction if unsupported syntax appears:

```text
error[feature.unsupported]
Source-level build conditionals are not supported in the MVP.

help
  Put target and feature configuration in zenith.ztproj.
```

## Rationale

Zenith's reading-first philosophy favors clean, portable source code.

Build configuration is a project concern.

Package features are a package manager concern.

Platform-specific runtime details are a stdlib/runtime concern.

The language surface should not grow preprocessor-like constructs before the project and package model need them.

## Out of Scope

This decision does not yet define:

- exact `zenith.ztproj` target schema
- ZPM feature flag schema
- target triples
- cross-compilation configuration
- stdlib target-resolution mechanism
- conditional dependency resolution
- future source-level conditional syntax
