# Zenith Project Model Spec

- Status: canonical consolidated spec
- Date: 2026-04-17
- Scope: `zenith.ztproj`, file layout, app/lib split, ZDoc layout and ZPM package model

## Project Manifest

A Zenith project is described by `zenith.ztproj` at the project root.

The format is TOML-like and strict.

Unknown sections and keys are errors.

## App Manifest

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
monomorphization_limit = 1024

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Rules:

- `project.name`, `project.kind` and `project.version` are required
- accepted MVP kinds are `app` and `lib`
- app projects require `[app] entry`
- `app.entry` points to a namespace containing `func main()`
- `source.root` is required
- build defaults are `target = "native"`, `output = "build"`, `profile = "debug"` and `monomorphization_limit = 1024`
- `build.monomorphization_limit` is a positive integer and counts concrete
  generic runtime type shapes plus concrete generic function specializations
- accepted MVP profiles are `debug` and `release`
- `test.root` defaults to `tests`
- `zdoc.root` defaults to `zdoc`

## Lib Manifest

```toml
[project]
name = "text_utils"
kind = "lib"
version = "1.0.0"

[source]
root = "src"

[lib]
root_namespace = "text_utils"

[build]
target = "native"
output = "build"
profile = "debug"
monomorphization_limit = 1024

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Rules:

- lib projects do not require `func main()`
- lib projects require `lib.root_namespace`
- `lib.root_namespace` defines the public namespace boundary of the library
- a publishable package is a library project prepared for ZPM publication
- apps are not importable dependencies by default

## Source Layout

A namespace rigorously and identically maps to a directory physical layout under `source.root`. 
The "Folder is the Namespace" paradigm is absolute.

Canonical layout:

```text
my_app/
  zenith.ztproj
  src/
    app/
      main/
        main.zt
      users/
        service.zt
        validation.zt
        types/
          administrator.zt
  tests/
    app/
      users/
        users_test.zt
  zdoc/
    app/
      users/
        service.zdoc
        types/
          administrator.zdoc
  build/
```

Rules:

- `src/app/users/service.zt` MUST declare `namespace app.users`
- `src/app/users/types/administrator.zt` MUST declare `namespace app.users.types`
- Any mismatch between the declared namespace and the physical directory path results in a fatal Compilation Error. This forbids "floating" or unorganized files.
- multiple files in the same directory declare the identical namespace. The file name itself does not become part of the namespace string.
- imports resolve namespaces, not files
- generated directories such as `build/` and `dist/` are not source roots

## Imports And Package Boundaries

Packages expose namespaces through their root namespace.

Example library:

```toml
[lib]
root_namespace = "text_utils"
```

```text
src/text_utils/formatting/functions.zt
src/text_utils/parsing/parser.zt
```

Consumer:

```zt
import text_utils.formatting as formatting
```

Rules:

- namespace aliases are written in source imports
- dependency aliases in the manifest are not MVP
- exported API is controlled by `public`, not by a manifest export list

## Dependencies

Zenith uses a decentralized package architecture. `[dependencies]` uses Git/URL sources or paths, completely eschewing a global central registry web-server (like npm or Crates.io).

Normal dependencies use `[dependencies]`.

Development-only dependencies use `[dev_dependencies]`.

```toml
[dependencies]
json = { git = "https://github.com/zenith-lang/json.git", tag = "v1.2.0" }
net = { git = "https://github.com/zenith-lang/net.git", branch = "main" }

[dev_dependencies]
test_helpers = { path = "../local_libs/test_helpers" }
```

Rules:

- origin resolution strictly adheres to the Git or local file protocol.
- `git` URLs with strict `tag` or `rev` hashes are required for reproducible releases.
- dependency version constraints use SemVer-shaped strings: `1.2.3`, `^1.2.3`, or `~1.2.3`.
- dependency version constraints are matched directly against git tags in the MVP.
- local `path` dependencies are allowed for monorepos or local development tooling.
- advanced ranges (`>=`, `<`, `1.2.*`) are post-MVP
- optional dependencies are post-MVP
- feature flags are post-MVP

## Versioning

Versions use SemVer-shaped strings.

```text
major.minor.patch
```

Example:

```toml
version = "1.4.2"
```

## Lockfile

The canonical lockfile name is `zenith.lock`.

Responsibilities:

- `zenith.ztproj` declares requested dependencies
- `zenith.lock` records exact resolved dependencies
- apps should commit the lockfile
- libs may commit the lockfile for local development reproducibility
- initial lockfile schema is defined in `lockfile-schema.md`

## ZDoc Layout

ZDoc root defaults to `zdoc`.

Paired API docs mirror source files by relative path.

```text
src/app/users/service.zt
zdoc/app/users/service.zdoc
```

Guides live under `zdoc/guides/`.

```text
zdoc/guides/getting-started.zdoc
```

Rules:

- paired docs use `@target`
- guide pages use `@page`
- private symbols may be documented
- public generated docs exclude private docs by default
- missing public documentation is a warning

## ZPM Boundary

`zpm` manages packages and dependencies.

`zt` compiles, checks, runs, tests, formats and generates docs.

Canonical future package commands:

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

Build remains a `zt` responsibility.
