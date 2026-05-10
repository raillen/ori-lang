# Decision 053 - Build Pipeline And Artifact Modes

- Status: accepted
- Date: 2026-04-17
- Type: compiler / build / packaging
- Scope: compiler pipeline, C backend, native runtime, intermediate output, standalone artifacts, bundle artifacts

## Summary

Zenith compiles user source code to native artifacts through the compiler pipeline and the C backend.

The C backend is an implementation detail.

The normal Zenith build model does not include a VM.

`Standalone` and `Bundle` are artifact/distribution modes, not different source languages.

## Decision

The canonical compilation path is:

```text
.zt source
  -> lexer
  -> parser
  -> AST
  -> semantic analysis
  -> HIR
  -> ZIR
  -> C backend
  -> native C compiler/linker
  -> native artifact
```

Normative rules for this cut:

- Zenith source files are `.zt`
- generated C is not user-facing Zenith source
- C is the first backend target
- the runtime is native support code linked with generated code
- the runtime is not a VM in the default build model
- `standalone` means one primary native executable artifact where the platform permits
- `bundle` means an organized distribution directory with the executable and required runtime/native files
- `build.output` remains the compiler output root
- final distribution layout is conceptually separate from compiler intermediates
- exact CLI spelling is decided separately
- exact generated C/object directory layout is decided separately

## Runtime Versus VM

Zenith has a runtime.

That runtime provides support for language features such as:

- managed `text`
- lists
- maps
- reference counting
- panic reporting
- bounds checks
- contract checks
- platform abstraction used by the stdlib

This is not the same as a VM.

A VM would usually mean:

```text
.zt -> bytecode -> virtual machine executes bytecode
```

That is not the selected primary architecture.

The selected primary architecture is:

```text
.zt -> C -> native executable
```

Future VM or bytecode backends are out of scope and require a separate decision.

## Build Pipeline

The compiler pipeline has these conceptual stages:

1. Load `zenith.ztproj`.
2. Validate project paths and manifest keys.
3. Scan `source.root` for `.zt` files.
4. Parse each file into AST.
5. Resolve namespaces and imports.
6. Run semantic validation.
7. Lower AST to HIR.
8. Lower HIR to ZIR.
9. Generate C from ZIR.
10. Compile generated C and runtime C with the selected native toolchain.
11. Link the final native artifact.
12. Assemble the requested distribution artifact.

Diagnostics may stop the pipeline before code generation.

Warnings do not stop compilation unless a future strict mode is enabled.

## Compiler Output

`build.output` is the compiler output root.

Default:

```toml
[build]
output = "build"
```

The compiler may place generated C, object files, dependency metadata and debug artifacts under this root.

The precise internal layout is not part of the language surface.

Recommended conceptual layout:

```text
build/
  debug/
    native/
      generated/
      objects/
      artifact/
  release/
    native/
      generated/
      objects/
      artifact/
```

Tools may clean or recreate `build/`.

Users should not edit generated files under `build/`.

## Standalone Artifact

`standalone` means a single primary native executable artifact.

Conceptual output:

```text
dist/
  hello
```

or on Windows:

```text
dist/
  hello.exe
```

The executable includes:

- generated Zenith program code
- Zenith runtime code needed by the program
- statically linked dependencies when available and appropriate

Platform system libraries or toolchain runtime libraries may still be dynamically linked when the platform requires it.

So `standalone` means "one primary executable for the user", not "mathematically no external operating system dependency".

Non-canonical wording:

```text
Standalone contains the VM.
```

Canonical wording:

```text
Standalone contains the native runtime and compiled program code.
```

## Bundle Artifact

`bundle` means an organized distribution directory.

Conceptual output:

```text
dist/
  hello.exe
  lib/
    native_dependency.dll
  assets/
    ...
  manifest/
    artifact.ztmeta
```

The bundle may contain:

- native executable
- native dynamic libraries
- assets
- generated metadata
- license files
- package metadata
- future dependency lock metadata

Bundle mode is useful when an application cannot reasonably be represented as one executable.

Examples:

- GUI applications with assets
- games
- tools that ship templates
- programs with native dynamic dependencies
- applications with plugin directories

## Artifact Mode Status

The design recognizes both artifact modes.

Recommended implementation order:

1. Implement `standalone` first.
2. Implement `bundle` after the native runtime, stdlib file/path APIs and dependency model are more mature.

The compiler may initially reject bundle mode with a clear unsupported-feature diagnostic.

Expected diagnostic direction:

```text
error[feature.unsupported]
Bundle artifact mode is not implemented yet.

help
  Use standalone mode for now.
```

## Manifest Boundary

This decision does not add a new required key to `zenith.ztproj`.

The MVP manifest stays small.

Artifact mode can be selected by CLI command or CLI flag until the packaging model is finalized.

Future project decisions may add optional manifest defaults such as:

```toml
[package_artifact]
mode = "standalone"
dist = "dist"
```

That syntax is not accepted in the MVP.

## Rationale

Separating compilation from packaging keeps the model easier to understand.

Compilation answers:

```text
How does Zenith source become executable machine code?
```

Packaging answers:

```text
How is the executable delivered to the user?
```

Using C as the backend keeps the implementation practical while preserving Zenith as the user-facing language.

Avoiding a VM in the primary path keeps performance, deployment and mental model simpler for the current project goal.

Recognizing bundle mode early prevents the design from assuming every real program is only one file forever.

## Non-Canonical Forms

Treating C as user-facing source:

```text
User writes C to complete normal Zenith programs.
```

Treating standalone as VM packaging:

```text
.zt -> bytecode -> VM embedded in exe
```

Treating generated files as source:

```text
src/
  generated.c
```

Adding packaging dependency management before ZPM:

```toml
[bundle.dependencies]
...
```

## Out of Scope

This decision does not yet define:

- exact CLI command names
- exact `dist/` layout
- exact generated C layout
- linker flag configuration
- cross-compilation
- static versus dynamic linking policy
- asset declaration syntax
- package dependency resolution
- ZPM lockfile format
- installer generation
- VM or bytecode backend
