# Zenith Tooling Model Spec

- Status: canonical consolidated spec
- Date: 2026-04-17
- Scope: CLI, diagnostics, formatter, tests, docs and package tooling

## CLI Command

The canonical compiler/tooling executable is `zt`.

Commands use:

```text
zt <command>
```

Canonical command family:

```text
zt check
zt build
zt run
zt test
zt fmt
zt clean
zt package
zt doc
```

## Project Discovery

`zt` searches upward from the current directory for `zenith.ztproj`.

A future `--project <path>` option may select a project explicitly.

If no manifest is found, report:

```text
error[project.not_found]
Could not find zenith.ztproj.

help
  Run this command inside a Zenith project or pass --project <path>.
```

## zt check

`zt check` validates the project without linking a final executable.

It runs:

- manifest validation
- source discovery
- lexing and parsing
- namespace/import validation
- semantic/type validation
- HIR/ZIR lowering when useful for validation

## zt build

`zt build` compiles the project.

For app projects, it uses `app.entry`.

It does not run the program.

Common options:

```text
zt build --profile release
zt build --target native
zt build --output build
```

## zt run

`zt run` builds and runs an app project.

Program arguments come after `--`.

```text
zt run --profile release -- input.txt --verbose
```

Arguments after `--` are passed to the user program and are not interpreted by `zt`.

## zt test

`zt test` compiles and runs tests.

It activates:

- `source.root`
- `test.root`
- functions marked with `attr test`
- test helpers such as `check`

Ordinary `zt build` does not run tests.

## zt fmt

`zt fmt` runs the official deterministic formatter.

Canonical use:

```text
zt fmt
zt fmt --check
```

Formatter rules:

- 4 spaces
- no tabs
- target width 100
- `end` aligns with block opener
- `case` aligns with `match`
- one `attr` line per attribute
- comments use `--` and `--- ... ---`
- long public documentation belongs in ZDoc

Naming conventions:

- types and enum cases use PascalCase
- functions, variables and fields use snake_case
- namespaces use lower snake_case segments
- generic parameters use descriptive PascalCase

## zt clean

`zt clean` removes generated compiler output.

Default scope:

- remove `build.output`

It must not remove source files, tests, ZDoc or package metadata.

## zt package

`zt package` creates distribution artifacts.

Future forms:

```text
zt package --mode standalone
zt package --mode bundle
```

Rules:

- standalone creates one primary native executable where possible
- bundle creates an organized `dist/` directory
- unsupported modes produce explicit diagnostics

## zt doc

`zt doc` validates and builds documentation.

Canonical future forms:

```text
zt doc check
zt doc build
```

`zt doc check` validates:

- malformed ZDoc blocks
- missing `@target` or `@page`
- unresolved targets
- duplicate targets
- invalid `@param` and `@return`
- unresolved `@link`
- public symbols missing docs

Severity:

- malformed block: error
- broken `@target`: error
- invalid `@param` or `@return`: error
- unresolved `@link`: warning
- missing public docs: warning

## zpm

`zpm` manages package dependencies and publication.

Canonical future commands:

```text
zpm add json
zpm update
zpm publish
zpm remove json
```

`zpm build` is not canonical.

Build remains a `zt` responsibility.

## Diagnostics

Diagnostics are part of the language experience.

Required qualities:

- explicit
- actionable
- stable
- low-noise
- human-first
- IDE/LSP-ready

Canonical shape:

```text
error[type.mismatch]
Cannot assign text to int.

where
  src/app/main/main.zt:8:5

help
  Convert the value explicitly or change the variable type.
```

Warnings should be sparse and actionable.

Ignoring `result` is an error.

Ignoring standalone `optional` is an error in the MVP.

For CI pipelines, `zt` supports `--ci` to emit short deterministic diagnostics.

## Process Arguments

User program arguments are accessed through future `std.process`, not `main` parameters.

Canonical future source direction:

```zt
import std.process as process

func main() -> int
    const args: list<text> = process.arguments()
    return 0
end
```

The exact stdlib API is discussed in the stdlib phase.
