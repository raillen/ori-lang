# Decision 056 - ZPM Package And Library Model

- Status: accepted
- Date: 2026-04-17
- Type: package management / project model / publication
- Scope: apps, libs, packages, ZPM, dependencies, dev dependencies, versions, lockfile, namespace exposure

## Summary

Zenith separates executable applications from importable libraries/packages.

A Zenith project may be an app or a lib.

A package is a versioned, publishable library unit managed by ZPM.

`zt` remains the compiler/tooling command.

`zpm` manages dependencies and publication.

## Decision

Normative rules for this cut:

- `project.kind` accepts only `app` and `lib` in the MVP
- app projects are executable and require `[app] entry`
- lib projects are importable and require `[lib] root_namespace`
- apps are not importable dependencies by default
- libs may become publishable packages through ZPM
- dependency versions are exact in the MVP
- normal dependencies use `[dependencies]`
- development-only dependencies use `[dev_dependencies]`
- optional dependencies are post-MVP
- version ranges are post-MVP
- lockfile name is `zenith.lock`
- `zpm` is separate from `zt`
- `zt` builds, runs, tests, formats, packages artifacts and generates docs
- `zpm` adds, resolves, updates and publishes packages
- package APIs are exposed through public symbols under the library root namespace
- ZDoc participates in package publication as public API documentation

## App Versus Lib

An app is a project meant to produce an executable artifact.

Example:

```toml
[project]
name = "my_app"
kind = "app"
version = "0.1.0"

[source]
root = "src"

[app]
entry = "app.main"
```

A lib is a project meant to be imported by other projects.

Example:

```toml
[project]
name = "text_utils"
kind = "lib"
version = "1.0.0"

[source]
root = "src"

[lib]
root_namespace = "text_utils"
```

This distinction keeps beginner-facing app projects separate from package publication concerns.

## Package

A package is a versioned library intended to be consumed by other projects.

In the MVP concept:

- every publishable package is a lib project
- not every app is a package
- app artifacts may be distributed, but they are not imported as dependencies
- package publication is a ZPM concern

This avoids using the word package for every local Zenith project.

## Dependencies

Normal dependencies are declared in `[dependencies]`.

Example:

```toml
[dependencies]
json = "1.2.0"
text_utils = "1.0.0"
```

Development-only dependencies are declared in `[dev_dependencies]`.

Example:

```toml
[dev_dependencies]
test_helpers = "0.1.0"
```

Meaning:

- `[dependencies]` are available to normal builds
- `[dev_dependencies]` are available to tests, examples and local development tooling
- dev dependencies are not part of the public dependency surface of a lib package

## Versions

Project and package versions use SemVer-shaped strings.

Canonical form:

```text
major.minor.patch
```

Example:

```toml
version = "1.4.2"
```

MVP dependency constraints use exact versions.

Canonical:

```toml
[dependencies]
json = "1.4.2"
```

Version ranges are post-MVP.

Non-MVP future direction:

```toml
[dependencies]
json = "^1.4.0"
```

Exact versions are less flexible, but simpler and more predictable for the early implementation.

## Lockfile

The canonical lockfile name is:

```text
zenith.lock
```

Responsibilities:

- `zenith.ztproj` declares requested dependencies
- `zenith.lock` records exact resolved packages
- apps should commit the lockfile
- libs may commit the lockfile for local development reproducibility
- consumers resolve a library's dependency graph through ZPM

The exact lockfile format is out of scope.

## ZT Versus ZPM

`zt` handles language tooling.

Canonical responsibilities:

```text
zt check
zt build
zt run
zt test
zt fmt
zt package
zt doc
```

`zpm` handles packages.

Canonical future responsibilities:

```text
zpm add json
zpm update
zpm publish
zpm remove json
```

Non-canonical:

```text
zpm build
```

Reason:

- building is a compiler/tooling concern
- package resolution and publication are package-manager concerns
- separating them keeps the mental model clear

## Namespace Exposure

A lib exposes APIs through public symbols under its root namespace.

Manifest:

```toml
[lib]
root_namespace = "text_utils"
```

Structure:

```text
src/
  text_utils/
    formatting/
      functions.zt
    parsing/
      parser.zt
```

Source:

```zt
namespace text_utils.formatting

public func title_case(value: text) -> text
    return value
end
```

The package exposes namespaces under `text_utils`.

Symbols without `public` remain private according to the visibility rules.

The MVP does not define a separate export list in the manifest.

Reason:

- `public` is already the language-level API marker
- duplicating exports in the manifest creates divergence risk
- namespace/path structure remains visible and explicit

## ZDoc And Publication

A package may include ZDoc.

During future publication, ZPM should validate public documentation.

Expected publication steps:

```text
zpm publish
  -> validate manifest
  -> resolve dependencies
  -> run zt check
  -> run zt test when configured or requested
  -> run zt doc check
  -> package source and public docs
```

Public generated docs include:

- public symbols
- public API ZDoc
- guide pages

Public generated docs exclude by default:

- private symbols
- private implementation notes
- generated C details

Missing public docs remain warnings unless a future strict publication mode changes that.

## MVP Cut

Included in the conceptual MVP:

- `project.kind = "app"`
- `project.kind = "lib"`
- `[app] entry`
- `[lib] root_namespace`
- `[dependencies]`
- `[dev_dependencies]`
- exact dependency versions
- `zenith.lock`
- separation between `zt` and `zpm`

Excluded from the MVP:

- optional dependencies
- feature flags
- version ranges
- workspaces
- multiple apps in one project
- multiple libs in one project
- dependency aliases in the manifest
- scripts in the manifest
- real remote registry protocol
- registry authentication

## Rationale

Separating apps from libs makes project intent explicit.

Using `[project]` instead of `[package]` keeps local apps from feeling like publishable packages.

Keeping exact versions and a lockfile supports reproducible builds.

Separating `zt` from `zpm` avoids turning one command into a large ambiguous tool.

Using `public` plus `root_namespace` avoids extra export syntax and keeps APIs visible in code.

## Non-Canonical Forms

Treating every app as an importable package:

```toml
[project]
kind = "app"

[package]
publish = true
```

Using package as the primary manifest section:

```toml
[package]
name = "hello"
version = "0.1.0"
```

Using dependency aliases in the manifest as the primary import model:

```toml
[dependencies]
json_lib = { package = "json", version = "1.2.0" }
```

Use import aliases in source instead:

```zt
import json.parser as parser
```

## Out of Scope

This decision does not yet define:

- lockfile schema
- package registry API
- package cache location
- dependency solver algorithm
- publishing authentication
- package signing
- checksums
- license metadata
- README metadata
- repository metadata
- package yanking/deprecation
- semantic version compatibility policy beyond exact MVP versions
