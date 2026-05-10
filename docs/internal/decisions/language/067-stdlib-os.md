# Decision 067 - Stdlib Os Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / operating system
- Scope: process environment, platform identification, process-state queries and current-directory control

## Summary

`std.os` covers operating-system information and process-state operations for the current process.

It is intentionally separate from `std.os.process`, which is reserved for child-process creation and control.

The MVP keeps `std.os` small and focused on arguments, environment lookup, platform identity, process identity and current-directory state.

## Decision

Canonical import:

```zt
import std.os as os
```

Accepted principal types:

- `os.Error`
- `os.Platform`
- `os.Arch`

Accepted first-wave API direction:

- `os.args() -> list<text>`
- `os.env(name: text) -> optional<text>`
- `os.platform() -> os.Platform`
- `os.arch() -> os.Arch`
- `os.pid() -> int`
- `os.current_dir() -> result<text, os.Error>`
- `os.change_dir(path: text) -> result<void, os.Error>`

Deferred after this first wave:

- `os.exe_path()`
- `os.hostname()`
- `os.home_dir()`
- `os.temp_dir()`
- `os.SpecialDir`
- `os.set_env(...)`
- `os.remove_env(...)`
- `os.exit(...)`

Examples:

```zt
import std.os as os

const args: list<text> = os.args()
const home: text = os.env("HOME") or "/tmp"

const platform: os.Platform = os.platform()
const arch: os.Arch = os.arch()
const pid: int = os.pid()

const cwd: text = os.current_dir()?
os.change_dir("/workspace")?
```

Normative rules for this cut:

- `std.os` is the home of current-process operating-system queries and mutations
- child-process creation and control belong in `std.os.process`, not in `std.os`
- `args()` returns the process argument vector as `list<text>`
- `args()[0]` is the executable name/path received from the host or launcher; it is not a stable program identity contract
- `zt run <project> -- <args>` forwards only values after `--` to the program, so compiler options stay before `--`
- `env(name)` returns `optional<text>` rather than a result type in this cut
- missing environment variables are represented as `none`
- `platform()` and `arch()` return dedicated types rather than free-form text
- `pid()` is part of the MVP
- current-directory queries and mutation belong in `std.os`, not in `std.fs`
- `change_dir(...)` returns `result<void, os.Error>`
- `exit(...)` is not part of the MVP surface because Zenith already uses the `main` return contract as the primary process-exit mechanism

## Platform Direction

`os.Platform` and `os.Arch` are accepted as typed public values instead of raw text identifiers.

The exact enum member spelling remains implementation work, but the intended purpose is to avoid stringly-typed platform checks throughout Zenith programs.

This also makes one-off boolean helpers such as platform-specific `is_windows()` unnecessary in the MVP surface.

## Environment Direction

`os.env(name)` is accepted as the environment lookup primitive.

The absence of a variable is treated as ordinary missing data, not as an error condition.

This keeps ordinary fallback code simple:

```zt
const value: text = os.env("APP_MODE") or "dev"
```

As a consequence, `get_env_or(...)` is not accepted as a canonical MVP helper.

## Current Directory Direction

`os.current_dir()` and `os.change_dir(...)` are accepted together as the canonical current-directory APIs.

Current directory is treated as process state, not as filesystem metadata.

This is why the canonical query and mutation pair live in `std.os`.

## Rationale

Keeping `std.os` small prevents it from turning into an unstructured bag of miscellaneous helpers.

Using typed `Platform` and `Arch` values improves readability and avoids repeated textual comparisons throughout code.

Locating both `current_dir()` and `change_dir(...)` in `std.os` preserves a coherent process-state model instead of splitting query and mutation across different modules.

Leaving `exit(...)` out of the MVP keeps the main return contract as the primary, explicit process-termination path.

## Non-Canonical Forms

Boolean platform helpers as the primary platform API:

```zt
os.is_windows()
os.is_linux()
os.is_macos()
```

Treating environment lookup as a filesystem-style result-returning operation in the MVP.

Leaving `current_dir()` in `std.fs` while moving only `change_dir(...)` to `std.os`.

Using `get_env_or(...)` as a separate canonical helper when `env(...)` plus `or` already covers the fallback case.

## Out of Scope

This decision does not yet define:

- child-process APIs
- typed executable-path queries
- hostname retrieval
- home and temp directory APIs
- a `SpecialDir` model
- environment mutation semantics
- platform-specific privilege or user-account APIs
