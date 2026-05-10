# Decision 046 - Modules, Public API And Reexport

- Status: accepted
- Date: 2026-04-17
- Type: language / modules / visibility
- Scope: namespace visibility, public symbols, private symbols, imports, reexport, API facade

## Summary

Zenith MVP uses `public` as the only public API marker.

Symbols without `public` are private to the effective namespace.

Reexport is useful for API facades, but it is not part of the MVP.

Post-MVP may introduce reexport when package APIs, ZPM and ZDoc need stable public facades.

## Decision

Canonical public symbol:

```zt
namespace app.users.types

public struct User
    name: text
end
```

Canonical private symbol:

```zt
namespace app.users.types

func normalize_name(name: text) -> text
    return name
end
```

External use:

```zt
namespace app.main

import app.users.types as user_types

func main() -> int
    const user: user_types.User = user_types.User(name: "Julia")
    return 0
end
```

Normative rules for this cut:

- `public` exposes a symbol outside its namespace
- symbols without `public` are private to the effective namespace
- files with the same namespace share private namespace symbols
- imports remain explicit
- external use remains qualified through imported namespace aliases
- reexport is not part of the MVP
- `export` is not part of the MVP
- `public import` is not part of the MVP
- API facades are deferred to post-MVP package/API design
- reexport, if added later, requires a separate decision

## Namespace Privacy

Private means private to the effective namespace, not only to the file.

If two files declare:

```zt
namespace app.users.types
```

they may share non-public helper symbols inside that namespace.

This allows implementation to be split across files without exposing all helpers as public API.

## Public API

A public declaration can be imported and used from another namespace.

```zt
public func load_user(id: int) -> result<User, text>
    ...
end
```

A private declaration cannot be used outside the namespace:

```zt
func validate_user(user: User) -> bool
    ...
end
```

Attempting to access it externally should produce a visibility diagnostic.

## No Reexport In MVP

Reexport would allow a namespace to expose symbols that are declared elsewhere.

Potential post-MVP example:

```zt
namespace app.users

import app.users.types as types
import app.users.service as service

export types.User
export service.load_user
```

External user:

```zt
import app.users as users

const user: users.User = users.load_user(id)?
```

This creates a facade API.

It is useful, but not essential for the MVP.

## Why Defer Reexport

Reexport affects:

- package API design
- ZDoc generated public docs
- namespace facade rules
- cyclic export diagnostics
- symbol identity
- semver and breaking changes
- import resolution
- tooling navigation

Deferring it keeps the MVP module system simpler.

Users can still import the namespace where the public symbol is actually declared.

## Import Model

Imports continue to point to real namespaces:

```zt
import app.users.types as user_types
import app.users.service as user_service
```

Use remains qualified:

```zt
const user: user_types.User = user_service.load_user(id)?
```

This is more verbose than a facade, but explicit and sufficient for the MVP.

## Non-Canonical Forms

Reexport is not supported in the MVP:

```zt
export types.User
```

Public import is not supported in the MVP:

```zt
public import app.users.types as types
```

Unqualified external use is not canonical:

```zt
const user: User = load_user(id)?
```

unless `User` and `load_user` are declared in the current namespace or otherwise brought into scope by a future explicit mechanism.

## Diagnostics

Expected diagnostic directions:

```text
error[visibility.private_symbol]
Symbol validate_user is private to namespace app.users.types.

help
  Mark it public or call it from the same namespace.
```

```text
error[feature.unsupported]
Reexport is not supported in the MVP.

help
  Import the namespace where the symbol is declared.
```

```text
error[feature.unsupported]
public import is not supported in the MVP.
```

## Rationale

A single `public` visibility marker keeps the MVP module model clear.

Namespace-private helpers allow files to be split without exposing implementation details.

Qualified imports preserve explicitness and avoid hidden symbol injection.

Reexport is valuable for package-facing APIs, but it should be designed together with package management, ZDoc and semver rather than rushed into the MVP.

## Out of Scope

This decision does not yet define:

- post-MVP `export` syntax
- package-level public API facade files
- reexport cycles
- export aliases
- wildcard exports
- public import
- semver interaction with reexport
- ZDoc API facade generation
- internal/package-only visibility
