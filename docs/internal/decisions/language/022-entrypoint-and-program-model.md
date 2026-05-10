# Decision 022 - Entrypoint And Program Model

- Status: accepted
- Date: 2026-04-16
- Type: language / tooling
- Scope: entry module, `main` signature, exit codes, host integration

## Summary

Zenith Next uses an explicit function entrypoint named `main`.

The project manifest points to the entry module namespace, and the compiler looks for `func main` inside that module.

The MVP entrypoint takes no parameters and may return nothing, `int`, `result<void, text>` or `result<int, text>`.

`main` does not need to be `public` in the MVP.

## Decision

The canonical project entry configuration remains:

```toml
[build]
entry = "app.main"
```

And the canonical source form is one of:

```zt
namespace app.main

func main()
    stdio.write_stdout("hello")
end
```

```zt
namespace app.main

func main() -> int
    return 0
end
```

```zt
namespace app.main

func main() -> result<void, text>
    const content: text = host.read_file("input.txt")?
    host.write_stdout(content)
    return success()
end
```

Normative rules for this cut:

- `[build].entry` identifies the namespace of the entry module
- the entry module must contain exactly one function named `main` in canonical MVP programs
- the entry function is `main`
- `main` does not take parameters in the MVP
- `main` may omit the return clause
- `main` may return `int`
- `main` may return `result<void, text>`
- `main` may return `result<int, text>`
- `main` does not need to be marked `public`
- `main` may not be generic in the MVP
- `main` is ordinary Zenith code and is not replaced by top-level executable file code
- the file name has no magical entrypoint meaning by itself

## Exit Semantics

Entrypoint return behavior is:

- `func main()` -> exit code `0` on normal completion
- `func main() -> int` -> the returned integer is the process exit code
- `func main() -> result<void, text>` -> `success()` exits with `0`; `error(message)` writes `message` to stderr and exits with `1`
- `func main() -> result<int, text>` -> `success(code)` exits with `code`; `error(message)` writes `message` to stderr and exits with `1`

## Host Integration

The entrypoint model is intentionally compatible with `extern host`.

Normative rules for this cut:

- using `extern host` requires a configured host boundary in project configuration
- in the MVP, that host boundary is declared through `[target].host_api`
- if host APIs are used and no host boundary is configured, semantic analysis must reject the program
- error reporting for failed `main -> result<..., text>` programs goes through the stderr host/output boundary of the active target

## Rationale

Keeping `main` explicit preserves a clear executable symbol without introducing magic based on file names or top-level statements.

Pointing the manifest at the entry module keeps project configuration simple while still allowing the language to require a stable symbol inside that module.

Allowing `main()` with no return avoids boilerplate for ordinary programs.

Allowing `int` and `result<..., text>` returns covers practical CLI and host-facing programs without forcing every program into one error-handling shape.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

relying on a file name alone as the entrypoint contract:

```text
src/main.zt
```

with no explicit `func main` symbol.

using top-level executable statements as the program body:

```zt
namespace app.main

host.write_stdout("hello")
```

using a parameterized main function:

```zt
func main(args: list<text>) -> int
    return 0
end
```

using a generic main function:

```zt
func main<Item>() -> int
    return 0
end
```

expecting `result<int, ErrorType>` with a non-`text` error type to be part of the MVP entry contract:

```zt
func main() -> result<int, ErrorInfo>
    ...
end
```

## Out of Scope

This decision does not yet define:

- command-line argument injection into `main`
- async entrypoints
- multiple binaries or multiple named entry modules in one manifest
- library-only projects with no executable entrypoint
- custom exit-code mapping policies for non-`text` errors
- test-specific entry models
