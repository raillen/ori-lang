# Decision 092 — Single-File Execution

- Status: accepted
- Date: 2026-04-26
- Scope: driver, pipeline, CLI

## Context

Zenith previously required a `zenith.ztproj` manifest and a project directory
structure for any compilation. This added friction for quick scripts,
experimentation, learning, and CI one-liners.

## Decision

The compiler now supports single-file execution: `zt run file.zt`,
`zt check file.zt`, `zt build file.zt`, and `zt emit-c file.zt`.

When the CLI receives a `.zt` file as input, it routes to a dedicated handler
(`zt_handle_single_file_command`) which calls `zt_compile_single_file` instead
of `zt_compile_project`.

### Synthetic manifest

`zt_compile_single_file` creates a `zt_project_manifest` in memory with:

- `project_name`: stem of the filename (e.g. `hello` for `hello.zt`).
- `project_kind`: `app`.
- `source_root`: `.` (parent directory of the file).
- `app_entry` / `entry`: updated after parsing to the declared namespace.
- `build_output` / `output_dir`: `build`.
- `output_name`: stem of the filename.
- Defaults for `build_target` (`native`), `build_profile` (`debug`),
  `accessibility_profile` (`balanced`), `monomorphization_limit` (default).

### Namespace validation

`zt_validate_source_namespaces` is intentionally skipped. A single file can
declare any `namespace` regardless of its location on disk.

### Stdlib loading

The stdlib import scanner runs on the single file. Any `import std.*`
declarations are resolved and loaded from the standard library path, identical
to project mode.

### Pipeline

After loading, the file goes through the full pipeline:
parse → bind → check → HIR → ZIR → verify → emit C → compile → run.

## Consequences

- Users can execute standalone `.zt` files without project setup.
- `zt test` still requires a project (needs test directory discovery).
- `zt fmt` still requires a project (needs source root).
- Multifile programs still require a `zenith.ztproj` project.
- The `.zt` intercept in CLI dispatch occurs after `.zir` but before project
  handler, so existing workflows are unaffected.

## Related

- `docs/internal/decisions/language/022-entrypoint-and-program-model.md`
- `docs/internal/decisions/language/052-file-layout-and-namespace-path-mapping.md`
- `docs/internal/planning/roadmap-v5.md` (R5.M7)
