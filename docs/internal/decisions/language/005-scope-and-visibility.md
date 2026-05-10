# Decision 005 - Scope And Visibility

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: lexical scope, top-level declarations, visibility, name resolution

## Summary

Zenith Next uses lexical scope.

Local scope is the default inside functions and nested blocks.

Top-level declarations are namespace-scoped by position, without a `global` keyword.

Visibility is private by default and exported explicitly with `public`.

Name shadowing is not allowed in the MVP.

## Decision

The canonical form is:

```zt
namespace app.config

public const app_name: text = "Zenith"
var startup_count: int = 0

public func read_app_name() -> text
    return app_name
end
```

Normative rules for this cut:

- scope is lexical
- each function introduces a new local scope
- each nested block construct introduces a nested scope
- declarations inside functions and nested blocks are local by default
- declarations at the top level of a file are namespace-scoped by position
- there is no `global` keyword in the canonical syntax
- namespace-scope declarations are private by default
- `public` marks a namespace-scope declaration as visible outside the namespace
- `public` is the canonical keyword; `pub` is not part of the MVP syntax
- name shadowing is not allowed in the MVP

## Rationale

This keeps scope and visibility as separate concepts.

`global` was rejected because it is misleading for declarations that actually live at namespace scope, not at universal program scope.

Using `public` instead of `pub` favors readability and matches the language goal of reducing compressed syntax.

Rejecting shadowing improves code reading because a name keeps one meaning within the visible region instead of silently hiding an outer declaration.

## Canonical Examples

Namespace-scoped declaration with explicit export:

```zt
namespace app.config

public const app_name: text = "Zenith"
var startup_count: int = 0
```

Public function using a private namespace-scoped binding:

```zt
namespace app.config

const default_port: int = 8080

public func read_default_port() -> int
    return default_port
end
```

Nested local scope:

```zt
func normalize_retry_count(input_count: int) -> int
    var retry_count: int = input_count
    if retry_count < 0
        const retry_count_floor: int = 0
        retry_count = retry_count_floor
    end
    return retry_count
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `global` for top-level declarations:

```zt
global const app_name: text = "Zenith"
```

using `pub` instead of `public`:

```zt
pub func read_app_name() -> text
    return app_name
end
```

shadowing an outer name:

```zt
const app_name: text = "Zenith"

func show_name() -> text
    const app_name: text = "Local"
    return app_name
end
```

## Out of Scope

This decision does not yet define:

- member visibility inside `struct` or other composite types
- import re-export rules
- module initialization order for namespace-scope `var`
- whether some block forms should reuse or avoid nested scopes in future sugar
