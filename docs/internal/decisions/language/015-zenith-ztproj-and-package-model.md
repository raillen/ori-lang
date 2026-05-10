# Decision 015 - Zenith Ztproj And Package Model

- Status: accepted
- Date: 2026-04-16
- Type: tooling / project system
- Scope: `zenith.ztproj`, build metadata, dependency declaration, future `zpm` alignment

## Summary

Zenith Next uses a declarative, TOML-like `zenith.ztproj` file at the project root.

The manifest is configuration, not executable language code.

The MVP manifest defines project identity, source layout, external documentation layout, build entry metadata, target metadata and dependency declarations.

The manifest must already reserve a stable dependency model for the future `zpm`, but it does not yet define the full package resolution algorithm.

## Decision

The canonical Zenith Next project manifest is a root file named `zenith.ztproj`.

Its syntax is TOML-like and declarative.

Normative rules for this cut:

- `zenith.ztproj` is required at the root of a standalone Zenith Next project
- the manifest uses a TOML-like syntax rather than Zenith source syntax
- the manifest contains configuration only and does not allow arbitrary control flow or executable logic
- the manifest is the single canonical source for project identity and build metadata in the MVP
- the manifest must be readable without compiling user Zenith code first
- the manifest must remain simple enough for tooling, package managers and editors to parse independently

## Required MVP Sections

The MVP manifest supports the following top-level tables:

- `[project]`
- `[layout]`
- `[build]`
- `[target]`
- `[dependencies]`

### `[project]`

The `[project]` table defines the identity of the package or application.

Required fields in this cut:

- `name`
- `version`
- `edition`

Canonical example:

```toml
[project]
name = "app.game"
version = "0.1.0"
edition = "next"
```

### `[layout]`

The `[layout]` table defines where Zenith source, symbol-linked documentation and related project material live.

Canonical fields for this cut:

- `source_root`
- `test_root`
- `docs_root`
- `zdoc_root`

Canonical example:

```toml
[layout]
source_root = "src"
test_root = "tests"
docs_root = "docs"
zdoc_root = "zdoc"
```

### `[build]`

The `[build]` table defines the primary build entry metadata for the project. In the MVP, `entry` names the namespace of the entry module that contains `func main`.

Canonical fields for this cut:

- `backend`
- `entry`
- `output_name`
- `output_dir`

In this cut, `entry` is the namespace path of the module that defines `func main`.

Canonical example:

```toml
[build]
backend = "c"
entry = "app.main"
output_name = "app-game"
output_dir = "build"
```

### `[target]`

The `[target]` table captures target-level metadata needed by the build system and host boundary.

Canonical fields for this cut:

- `platform`
- `host_api`

Canonical example:

```toml
[target]
platform = "windows-x64"
host_api = "desktop"
```

### `[dependencies]`

The `[dependencies]` table declares package requirements in a form that future `zpm` tooling can consume.

This decision defines the declaration surface, not the full dependency resolution behavior.

Normative rules for this cut:

- dependency declarations belong in `[dependencies]`
- dependency keys are package names
- dependency values must be declarative metadata, not scripts
- the manifest must distinguish source kind from version requirement
- the manifest must support at least builtin, local path and registry-oriented dependency declarations in the model
- a future lockfile is expected to exist separately from `zenith.ztproj`

Canonical examples:

```toml
[dependencies]
std = { source = "builtin" }
ui.widgets = { source = "registry", version = "^0.3.0" }
local.tools = { source = "path", path = "../local.tools" }
```

## Versioning Model

Zenith Next should adopt explicit package versions from the first project-system cut, even before the full `zpm` implementation exists.

Normative rules for this cut:

- `project.version` is mandatory
- dependency declarations may carry a version requirement when the source kind supports versioned resolution
- the version requirement syntax should be stable and semver-oriented
- local path dependencies do not require a version in the manifest
- builtin dependencies do not require a version in the manifest
- exact resolution results belong in a lockfile, not in `zenith.ztproj`

This keeps the manifest forward-compatible with a future package manager without forcing the package manager itself to be implemented now.

## Canonical Example

```toml
[project]
name = "app.game"
version = "0.1.0"
edition = "next"

[layout]
source_root = "src"
test_root = "tests"
docs_root = "docs"
zdoc_root = "zdoc"

[build]
backend = "c"
entry = "app.main"
output_name = "app-game"
output_dir = "build"

[target]
platform = "windows-x64"
host_api = "desktop"

[dependencies]
std = { source = "builtin" }
ui.widgets = { source = "registry", version = "^0.3.0" }
local.tools = { source = "path", path = "../local.tools" }
```

## Rationale

Using a TOML-like file keeps the project manifest simple, familiar and independent from the Zenith parser.

This reduces bootstrap cost for the compiler, editor tooling and future package manager.

It also avoids turning the manifest into a second programming language with its own execution model.

Defining dependency identity and version metadata now prevents churn later when `zpm` arrives, while still keeping this cut small and implementable.

Separating `docs_root` from `zdoc_root` also keeps narrative project documentation apart from symbol-linked API documentation, which avoids cluttering source directories while still preserving deterministic tooling discovery.

## Non-Canonical Forms

These forms are not accepted as the canonical Zenith Next project manifest style:

using Zenith-like blocks as the primary manifest syntax:

```zt
project
    name: "app.game"
    version: "0.1.0"
end
```

embedding scripts or hooks in the manifest:

```toml
[scripts]
test = "zt test"
run = "zt run"
```

using arbitrary conditional logic inside the manifest:

```toml
if platform == "windows-x64"
backend = "c"
```

mixing resolved lock information directly into dependency declarations:

```toml
[dependencies]
ui.widgets = { source = "registry", version = "^0.3.0", resolved = "0.3.4", hash = "..." }
```

## Out of Scope

This decision does not yet define:

- the full `zpm` dependency solver
- the exact registry protocol or registry URL layout
- lockfile filename or on-disk schema
- workspace or monorepo manifests
- build hooks, scripts or task runners
- environment-specific overrides
- multi-backend matrix builds in one manifest
- formatter, linter or editor configuration inside `zenith.ztproj`
