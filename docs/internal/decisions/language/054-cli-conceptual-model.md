# Decision 054 - CLI Conceptual Model

- Status: accepted
- Date: 2026-04-17
- Type: tooling / compiler / CLI
- Scope: `zt` command, project discovery, check, build, run, test, format, package, docs, diagnostics

## Summary

The canonical Zenith command-line tool is `zt`.

The CLI should be explicit, predictable and reading-first.

Commands are organized by user intent:

- `zt check` validates code without producing a final artifact
- `zt build` compiles the project
- `zt run` compiles and runs the project
- `zt test` compiles and runs tests
- `zt fmt` formats source files
- `zt clean` removes generated compiler output
- `zt package` creates distribution artifacts
- `zt doc` validates or generates external documentation

## Decision

Normative rules for this cut:

- the canonical CLI executable name is `zt`
- commands are written as `zt <command>`
- the CLI operates on a project described by `zenith.ztproj`
- the CLI searches upward from the current directory to find `zenith.ztproj`
- a future `--project <path>` option may select a project explicitly
- command output must follow the diagnostic philosophy from Decisions 038 and 039
- command names should be short but semantically obvious
- build-related commands must not require users to understand generated C
- program arguments are separated from compiler arguments with `--`
- exact flag names may evolve until implementation, but command responsibilities are accepted

## Project Discovery

When run without an explicit project path, `zt` searches from the current directory upward until it finds `zenith.ztproj`.

If no manifest is found, the CLI reports a project diagnostic.

```text
error[project.not_found]
Could not find zenith.ztproj.

help
  Run this command inside a Zenith project or pass --project <path>.
```

This improves usability without hiding project identity.

## Global Shape

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

The MVP implementation does not need to implement every command at once.

Recommended implementation order:

1. `zt check`
2. `zt build`
3. `zt run`
4. `zt test`
5. `zt fmt --check`
6. `zt fmt`
7. `zt clean`
8. `zt package`
9. `zt doc`

## zt check

`zt check` validates the project without creating a final executable.

It runs:

- manifest validation
- source discovery
- lexing
- parsing
- namespace validation
- import resolution
- semantic validation
- type checking
- HIR/ZIR lowering when useful for validation

It does not link a final executable.

Canonical use:

```text
zt check
```

This should be the fastest correctness feedback command.

Future options may include:

```text
zt check --tests
zt check --profile release
```

## zt build

`zt build` compiles the project entrypoint into a native artifact.

Canonical use:

```text
zt build
```

Default behavior:

- uses `zenith.ztproj`
- uses `app.entry` for app projects
- uses `build.profile` or `debug` by default
- writes compiler output under `build.output`
- produces a native executable artifact
- does not run the program

Common future options:

```text
zt build --profile release
zt build --target native
zt build --output build
```

`zt build` is compilation, not distribution packaging.

## zt run

`zt run` builds the project and runs the resulting executable.

Canonical use:

```text
zt run
```

Program arguments come after `--`:

```text
zt run -- input.txt --verbose
```

In this example, the Zenith program receives:

```text
input.txt
--verbose
```

Compiler/CLI options must appear before `--`.

```text
zt run --profile release -- input.txt
```

The user program should access arguments through the future process API, not through `main` parameters.

## zt test

`zt test` compiles and runs tests.

Canonical use:

```text
zt test
```

It activates:

- `source.root`
- `test.root`
- functions marked with `attr test`
- test helper APIs such as `check`

It does not run tests during ordinary `zt build`.

A test failure is reported with normal Zenith diagnostics.

Future options may include:

```text
zt test --filter users
zt test --profile release
```

## zt fmt

`zt fmt` formats source files using the official formatter.

Canonical use:

```text
zt fmt
```

CI-friendly validation:

```text
zt fmt --check
```

MVP scope:

- `.zt` files under `source.root`
- `.zt` files under `test.root`

Future scope may include:

- `zenith.ztproj`
- `.zdoc`

## zt clean

`zt clean` removes generated compiler output.

Canonical use:

```text
zt clean
```

Default scope:

- remove `build.output`

It should not remove source files, tests, ZDoc or package metadata.

If future `dist` output is removed by this command, that behavior should be explicit.

```text
zt clean --dist
```

## zt package

`zt package` creates distribution artifacts.

Canonical future use:

```text
zt package --mode standalone
zt package --mode bundle
```

Relationship to Decision 053:

- `standalone` creates one primary native executable where the platform permits
- `bundle` creates an organized `dist/` directory
- `bundle` may be recognized before it is implemented
- unsupported modes must produce explicit diagnostics

Expected diagnostic direction while bundle is not implemented:

```text
error[feature.unsupported]
Bundle artifact mode is not implemented yet.

help
  Use zt package --mode standalone for now.
```

The MVP may implement `zt build` before `zt package`.

## zt doc

`zt doc` validates and later generates documentation from ZDoc.

Canonical future uses:

```text
zt doc check
zt doc build
```

`zt doc check` validates:

- ZDoc file pairing
- `@target` resolution
- malformed blocks
- invalid parameter references
- duplicate records

`zt doc build` may generate human-readable documentation output.

Full documentation generation is not required before the compiler MVP.

## Program Arguments

The separator `--` divides compiler arguments from program arguments.

Compiler arguments:

```text
zt run --profile release -- input.txt
```

Program arguments:

```text
input.txt
```

No arguments after `--` are interpreted by `zt`.

This avoids ambiguity and matches the explicitness philosophy.

## Profiles And Targets

The CLI may override manifest defaults.

Examples:

```text
zt build --profile release
zt build --target native
zt run --profile debug
```

The manifest remains the project default.

CLI flags are command-time choices.

Unknown targets and profiles must produce explicit diagnostics.

## Output Philosophy

CLI output should be calm, explicit and structured.

Success output should be brief.

```text
Build completed.
artifact
  build/debug/native/artifact/hello.exe
```

Error output should use the diagnostic format.

Warnings should be actionable and low-noise.

The CLI should avoid dumping raw C compiler output unless needed for an internal compiler/backend failure.

## Relationship To C

The CLI must not expose C as the ordinary user model.

Generated C may exist for debugging or compiler development, but it is not the primary beginner-facing workflow.

Possible future advanced command:

```text
zt build --keep-generated
```

This is a tooling/debugging option, not the normal language experience.

## Rationale

The CLI should map to user questions:

```text
Is my code valid?        -> zt check
Can I compile it?        -> zt build
Can I execute it now?    -> zt run
Do my tests pass?        -> zt test
Is formatting correct?   -> zt fmt --check
Can I distribute it?     -> zt package
Can I publish docs?      -> zt doc
```

This keeps the interface teachable and reduces cognitive load.

The command set is short enough to remember while still separating different intents.

## Non-Canonical Forms

Single overloaded command for everything:

```text
zt --run --test --format --package
```

Running tests during ordinary build by default:

```text
zt build
```

and unexpectedly executing tests.

Passing program arguments without a separator:

```text
zt run input.txt --profile release
```

Requiring users to manually compile generated C:

```text
zt build
cc generated.c runtime.c
```

## Out of Scope

This decision does not yet define:

- exact final flag set
- shell completion
- colored output
- watch mode
- package registry commands
- dependency install/update commands
- cross-compilation flags
- IDE/LSP command protocol
- exact artifact paths
- exact generated C debug options
- localization of CLI diagnostics
