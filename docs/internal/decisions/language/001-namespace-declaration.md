# Decision 001 - Explicit Namespace Declaration

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: file identity, imports, frontend

## Summary

Zenith Next uses an explicit `namespace` declaration to define the logical identity of a source file.

The declaration is not required to be the literal first line of the file, but it must be the first real declaration.

Comments, doc comments and blank lines may appear before it.

`namespace` must appear before any `import`.

## Decision

The canonical form is:

```zt
namespace app.user.profile

import std.text as text
```

Normative rules for this cut:

- every `.zt` source file declares exactly one top-level `namespace`
- `namespace` defines the logical identity of the file
- `namespace` is the first real declaration in the file
- only comments, doc comments and blank lines may appear before `namespace`
- `import` declarations come after `namespace`

## Rationale

This decision favors reading over brevity.

The first question during reading is "where am I?". An explicit `namespace` answers that before dependencies and declarations appear.

Using `namespace` instead of `module` also keeps the public syntax closer to the idea of logical naming and organization, while still allowing the compiler to treat the file as an importable compilation unit internally.

## Canonical Example

```zt
/// User profile rendering helpers.
namespace app.user.profile

import std.text as text
import app.user.types as user_types

func render_name(user: user_types.User) -> text
    return text.trim(user.name)
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

`namespace` after imports:

```zt
import std.text as text

namespace app.user.profile
```

missing explicit namespace:

```zt
import std.text as text
```

using `module` instead of `namespace`:

```zt
module app.user.profile
```

## Out of Scope

This decision does not yet define:

- path-to-namespace validation rules
- relative imports
- nested namespaces inside a file
- import alias policy in detail
