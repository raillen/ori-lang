# Decision 025 - Process Model And CLI Arguments

- Status: draft
- Date: 2026-04-16
- Type: language / tooling
- Scope: process arguments, process environment, current directory, process-capable host boundary

## Summary

This document is provisional and remains pending a dedicated stdlib/process discussion.

Zenith Next keeps `main` parameterless in the MVP.

Command-line arguments and other read-only process data are accessed through `std.process`, not injected into the `main` signature.

The canonical process-capable host boundary for the MVP is `host_api = "cli"`.

## Draft Direction

The canonical CLI project configuration is:

```toml
[target]
host_api = "cli"
```

And the canonical source form is:

```zt
namespace app.main

import std.process

func main() -> int
    const args: list<text> = process.arguments()
    return 0
end
```

Normative rules for this cut:

- `main` remains parameterless in the MVP
- command-line arguments are not injected into the `main` signature
- the canonical API for user-supplied process arguments is `process.arguments() -> list<text>`
- the returned argument list excludes the executable path and starts at the first user-supplied argument
- the canonical API for environment lookup is `process.environment(name: text) -> optional<text>`
- the canonical API for reading the current working directory is `process.current_directory() -> result<text, text>`
- these process APIs are read-only in the MVP
- the canonical path for these APIs is `std.process`
- using `std.process` in canonical CLI programs requires a process-capable host boundary
- in the MVP, the canonical process-capable host boundary is `[target].host_api = "cli"`
- using `std.process` with an incompatible configured host boundary is a semantic error in the MVP
- the primary way to choose the process exit code remains the return value of `main`
- direct process termination APIs such as `process.exit(...)` are not part of the MVP surface
- process arguments are ordinary `text` values and therefore follow the UTF-8 text model already defined for the language

## Canonical Examples

Reading CLI arguments:

```zt
namespace app.main

import std.process

func main() -> int
    const args: list<text> = process.arguments()
    return 0
end
```

Reading one environment variable:

```zt
import std.process

func main()
    const home_dir: optional<text> = process.environment("HOME")
end
```

Reading the current working directory:

```zt
import std.process

func main() -> result<void, text>
    const cwd: text = process.current_directory()?
    return success()
end
```

## Rationale

Keeping `main` parameterless preserves one stable program-entry shape and avoids turning the function signature into a host-specific contract.

Placing process APIs in `std.process` makes CLI capabilities explicit at the use site and prevents the language core from pretending that every target is a process-oriented platform.

Treating process access as a host-boundary capability also aligns with the earlier `extern host` and `host_api` decisions.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

injecting CLI arguments into `main`:

```zt
func main(args: list<text>) -> int
    return 0
end
```

assuming the executable path appears as the first user argument:

```zt
const argv0: text = process.arguments()[0]
```

if the intention is to read the executable name rather than the first user argument.

using direct exit APIs as the canonical control path:

```zt
process.exit(1)
```

using mutable process APIs in the MVP:

```zt
process.set_environment("MODE", "debug")
process.change_directory("tmp")
```

## Out of Scope

This decision does not yet define:

- executable-path lookup
- process identifiers
- spawning child processes
- signal handling
- mutable environment APIs
- mutable working-directory APIs
- non-CLI host boundaries that still expose compatible process capabilities
