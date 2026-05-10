# Decision 002 - Import Declarations

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: imports, name visibility, frontend

## Summary

Zenith Next imports namespaces, not individual symbols.

Imported names remain qualified in user code.

Selective imports are not part of the MVP syntax.

Granularity comes from namespace structure, not from pulling isolated symbols into local scope.

## Decision

The canonical forms are:

```zt
namespace app.user.profile

import std.text
import app.user.types as user_types
```

Normative rules for this cut:

- `import` declarations appear after `namespace`
- each `import` imports exactly one namespace
- the canonical syntax is `import path.to.namespace`
- `import path.to.namespace as alias` is allowed when a local alias improves clarity
- if `as` is omitted, the local name is the last path segment
- imported names are used in qualified form
- `from ... import ...` is not part of the language
- `import *` is not part of the language
- multiple imports on the same line are not part of the canonical style
- only absolute imports are defined in this cut

## Rationale

This decision keeps symbol origin visible at the call site.

That improves reading flow, reduces collisions and avoids hidden context.

The language goal here is not to minimize characters. It is to make code easier to scan, revisit and reason about.

Rejecting selective import in the MVP also keeps the binder and name-resolution model simpler and more predictable.

## Canonical Examples

Default local name from the last path segment:

```zt
namespace app.user.profile

import std.text
import app.user.types

func render_name(user: types.User) -> text
    return text.trim(user.name)
end
```

Explicit alias when the default local name is too broad or unclear:

```zt
namespace app.user.profile

import app.user.types as user_types
import app.shared.errors as shared_errors

func promote(user: user_types.Administrator) -> shared_errors.Result
    ...
end
```

More specific namespace instead of selective symbol import:

```zt
namespace app.user.profile

import app.users.types.administrator as administrator

func promote(user: administrator.Administrator) -> outcome.Success
    ...
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

selective import:

```zt
import app.users.types select Administrator
```

symbol import:

```zt
from app.users.types import Administrator
```

wildcard import:

```zt
import app.users.types.*
```

multiple imports on one line:

```zt
import std.text, app.user.types
```

## Out of Scope

This decision does not yet define:

- ordering rules between standard-library and project imports
- relative imports
- re-export syntax
- cyclic import policy
- formatter behavior for very long import lines
