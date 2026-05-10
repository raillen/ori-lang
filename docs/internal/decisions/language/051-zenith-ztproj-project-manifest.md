# Decision 051 - Zenith Ztproj Project Manifest

- Status: accepted
- Date: 2026-04-17
- Type: project / manifest / build
- Scope: `zenith.ztproj`, project metadata, app/lib kind, source root, entrypoint, build defaults, tests, ZDoc

## Summary

A Zenith project is described by a `zenith.ztproj` file at the project root.

The manifest uses a TOML-like format.

The manifest describes a Zenith project, not necessarily a publishable package.

A project may be an executable app or an importable library.

Package publication and dependency management are handled by future ZPM decisions.

## Decision

Canonical app manifest:

```toml
[project]
name = "hello"
kind = "app"
version = "0.1.0"

[source]
root = "src"

[app]
entry = "app.main"

[build]
target = "native"
output = "build"
profile = "debug"

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Canonical lib manifest:

```toml
[project]
name = "text_utils"
kind = "lib"
version = "0.1.0"

[source]
root = "src"

[lib]
root_namespace = "text_utils"

[build]
target = "native"
output = "build"
profile = "debug"

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Normative rules for this cut:

- manifest filename is `zenith.ztproj`
- manifest lives at the project root
- manifest format is TOML-like
- paths are relative to the manifest directory
- `[project]` is required
- `project.name` is required
- `project.kind` is required
- `project.version` is required
- accepted MVP project kinds are `app` and `lib`
- `source.root` is required
- app projects require `[app] entry`
- lib projects require `[lib] root_namespace`
- `[app] entry` points to a namespace
- the app entry namespace must contain `func main()`
- app entry namespace may be any namespace, not necessarily `app.main`
- `build.target` is optional and defaults to `native`
- `build.output` is optional and defaults to `build`
- `build.profile` is optional and defaults to `debug`
- MVP accepted profiles are `debug` and `release`
- `test.root` is optional and defaults to `tests`
- `zdoc.root` is optional and defaults to `zdoc`
- unknown sections and keys are errors
- CLI may override `target`, `output` and `profile`

## Project

The project section describes the current Zenith project.

```toml
[project]
name = "hello"
kind = "app"
version = "0.1.0"
```

`project.kind` determines the project shape.

Accepted MVP values:

- `app` for executable applications
- `lib` for importable libraries

`zenith.ztproj` does not declare multiple projects.

A publishable ZPM package is a library project prepared for publication, not every Zenith project by default.

## App

App projects compile to executable programs.

```toml
[project]
kind = "app"

[app]
entry = "app.main"
```

`app.entry` points to a namespace.

The entry namespace must contain `func main()`.

Example:

```zt
namespace app.main

func main() -> int
    return 0
end
```

The namespace can be any valid namespace.

The function name remains `main`.

## Library

Library projects are importable by other projects and may become publishable packages through ZPM.

```toml
[project]
kind = "lib"

[lib]
root_namespace = "text_utils"
```

`lib.root_namespace` is required for lib projects.

It defines the namespace boundary exposed by the library.

Canonical structure:

```text
src/
  text_utils/
    formatting/
      functions.zt
    parsing/
      parser.zt
```

Example source:

```zt
namespace text_utils.formatting

public func title_case(value: text) -> text
    return value
end
```

A lib project does not require `func main()`.

## Source

```toml
[source]
root = "src"
```

`source.root` is required.

The compiler scans this root for `.zt` source files according to the namespace/path rules.

## Build

```toml
[build]
target = "native"
output = "build"
profile = "debug"
```

`build.target` defaults to:

```toml
target = "native"
```

`native` means the compiler targets the current native platform.

`build.output` defaults to:

```toml
output = "build"
```

`build.profile` defaults to:

```toml
profile = "debug"
```

Accepted MVP profiles:

```toml
profile = "debug"
profile = "release"
```

The CLI may override the profile, target and output.

Example:

```text
zt build --profile release
```

## Dependencies

Dependencies are allowed conceptually but resolved by ZPM.

Simple future form:

```toml
[dependencies]
json = "1.2.0"

[dev_dependencies]
test_helpers = "0.3.0"
```

Dependency resolution is outside the core compiler MVP.

## Tests

```toml
[test]
root = "tests"
```

`test.root` is optional and defaults to `tests`.

The test runner may scan this root for test sources and functions marked with `attr test`.

## ZDoc

```toml
[zdoc]
root = "zdoc"
```

`zdoc.root` is optional and defaults to `zdoc`.

The ZDoc root stores external `.zdoc` documentation files.

ZDoc files mirror source files by relative path:

```text
src/app/users/types.zt
zdoc/app/users/types.zdoc
```

The ZDoc root is documentation-only.

The compiler must not treat `.zdoc` files as executable Zenith source.

## Unknown Keys

Unknown sections and keys are errors.

Invalid:

```toml
[app]
entyr = "app.main"
```

Expected diagnostic direction:

```text
error[project.unknown_key]
Unknown key app.entyr.

help
  Did you mean app.entry?
```

Strict manifests align with Zenith's explicitness philosophy.

## CLI Overrides

The CLI may override:

- `build.target`
- `build.output`
- `build.profile`

Manifest remains the project default.

CLI overrides are command-time choices.

## Non-Canonical Forms

Using `[package]` as the main project section is not canonical:

```toml
[package]
name = "hello"
version = "0.1.0"
```

Putting app entry under `[build]` is not canonical:

```toml
[build]
entry = "app.main"
```

Full function path as entry is not canonical:

```toml
[app]
entry = "app.main.main"
```

Use namespace entry:

```toml
[app]
entry = "app.main"
```

Lib project without root namespace is invalid:

```toml
[project]
kind = "lib"
```

Implicit source root is not canonical because `source.root` is required.

## Diagnostics

Expected diagnostic directions:

```text
error[project.missing_key]
Missing required key project.name.
```

```text
error[project.missing_key]
Missing required key project.kind.
```

```text
error[project.missing_key]
App project requires app.entry.
```

```text
error[project.missing_key]
Lib project requires lib.root_namespace.
```

```text
error[project.invalid_profile]
Unknown build profile fast.

help
  Use debug or release.
```

```text
error[project.entry_missing_main]
Entry namespace my_game.startup does not contain func main().
```

## Rationale

A small manifest keeps the project model understandable.

Using `[project]` avoids confusing apps with publishable packages.

Separating `[app]` from `[lib]` makes executable and importable projects explicit.

Keeping dependencies conceptually present but resolved by ZPM prevents the compiler MVP from becoming a package manager too early.

Strict unknown-key handling prevents silent configuration mistakes.

## Out of Scope

This decision does not yet define:

- feature syntax
- optional dependencies
- version ranges
- custom profiles
- multiple binaries
- multiple libraries per project
- workspace/monorepo manifests
- lockfile format details
- ZPM registry metadata
- exact CLI override flags
