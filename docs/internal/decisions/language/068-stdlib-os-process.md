# Decision 068 - Stdlib Os Process Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / operating system / process
- Scope: child-process execution MVP, typed exit status, captured output, no-shell execution and deferred lifecycle control

## Summary

`std.os.process` is the child-process module.

The MVP keeps the surface intentionally small: explicit program execution, optional working-directory override, optional text capture and typed process results.

Shell execution, pipes and long-lived child-process control are deferred beyond this cut.

## Decision

Canonical import:

```zt
import std.os.process as process
```

Accepted principal types:

- `process.Error`
- `process.ExitStatus`
- `process.CapturedRun`

Accepted first-wave API direction:

- `process.run(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.ExitStatus, process.Error>`
- `process.run_capture(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.CapturedRun, process.Error>`

Deferred after this first wave:

- `process.spawn(...)`
- `process.wait(...)`
- `process.try_wait(...)`
- `process.kill(...)`
- stdin customization
- stdout/stderr mode configuration
- pipes
- shell mode
- timeouts
- per-command environment overrides

Examples:

```zt
import std.os.process as process

const status: process.ExitStatus = process.run(
    program: "git",
    args: ["status", "--short"],
    cwd: repo_root
)?

const captured: process.CapturedRun = process.run_capture(
    program: "git",
    args: ["status", "--short"],
    cwd: repo_root
)?
```

Normative rules for this cut:

- `std.os.process` is reserved for child-process execution and control
- `std.os` remains the module for current-process state
- command execution is explicit and uses `program` plus `args`, not a shell command string
- shell execution is not part of the MVP surface
- `cwd` is accepted as an optional named parameter in the MVP
- non-zero process exit does not become `process.Error` by itself
- `process.Error` is reserved for failures such as spawn, wait or capture/decoding failure
- `run_capture(...)` captures textual stdout and stderr in this cut
- captured text is expected to decode as UTF-8 in this cut
- binary capture is deferred

## Exit Status Direction

`process.ExitStatus` represents a completed child-process result.

The exact public field surface remains implementation work, but it must be rich enough to represent at least the exit code of a completed process.

The important accepted semantic rule is that a child process exiting with code `1` or another non-zero code is still a valid process result, not a transport-level process error.

## Captured Output Direction

`process.CapturedRun` is accepted as the structured return for captured execution.

The intended public payload includes at least:

- the process exit status
- captured stdout text
- captured stderr text

The exact field names remain implementation work.

## No-Shell Direction

The MVP deliberately rejects shell-string execution such as a single `"git status"` command line.

Accepted style:

```zt
process.run(program: "git", args: ["status"])
```

Rejected style:

```zt
process.run("git status")
```

This keeps behavior more predictable, more portable and less vulnerable to shell-quoting ambiguity.

## Rationale

Keeping the process module small prevents it from becoming an accidental shell, pipeline framework and process supervisor all at once.

Separating child-process control from current-process OS queries preserves a clean boundary with `std.os`.

Treating non-zero exit as ordinary process outcome matches common systems expectations and keeps error handling honest.

Using explicit `program` plus `args` is clearer and safer than shell-string execution.

## Non-Canonical Forms

Shell-style single-string execution as the primary API:

```zt
process.run("git status --short")
```

Treating exit code `1` as `process.Error` automatically.

Starting the MVP with lifecycle-heavy APIs before the basic run surface is stable:

```zt
process.spawn(...)
process.kill(...)
process.wait(...)
```

Assuming binary capture is part of the textual MVP surface.

## Out of Scope

This decision does not yet define:

- long-lived child-process handles
- wait/kill semantics
- shell invocation support
- pipe composition
- binary stdin/stdout/stderr handling
- timeouts and cancellation
- per-command environment overrides
- the exact public fields of `ExitStatus` and `CapturedRun`
