# Decision 052 - File Layout And Namespace Path Mapping

- Status: accepted
- Date: 2026-04-17
- Type: project / source layout / namespace resolution
- Scope: source tree layout, namespace-to-path validation, import granularity, test source layout, ZDoc pairing

## Summary

Zenith projects use explicit `namespace` declarations for human readability.

The filesystem validates namespace organization, but it does not replace the explicit `namespace` declaration.

A namespace maps to a directory under the configured source root.

A source filename is an implementation partition inside that namespace and does not become part of the namespace name.

## Decision

Canonical project layout:

```text
my_app/
  zenith.ztproj
  src/
    app/
      main/
        main.zt
      users/
        service.zt
        validation.zt
        types/
          administrator.zt
          customer.zt
  tests/
    app/
      users/
        users_test.zt
  zdoc/
    app/
      users/
        service.zdoc
        validation.zdoc
        types/
          administrator.zdoc
          customer.zdoc
  build/
```

Normative rules for this cut:

- `source.root` points to the root of executable/library source files
- `test.root` points to test-only source files
- `zdoc.root` points to external documentation files
- a namespace maps to a directory relative to a source root
- a source filename does not add a namespace segment
- every `.zt` file still declares exactly one explicit `namespace`
- the declared namespace must match the file's directory path relative to the active source root
- source files directly under `source.root` are not canonical in the MVP
- multiple files in the same directory may declare the same namespace
- imports resolve namespaces, not files
- generated output directories such as `build` and `dist` are not source roots
- `.zdoc` files are not executable source files

## Namespace Mapping

The directory path defines the expected namespace.

Canonical:

```text
src/app/users/types.zt
```

```zt
namespace app.users
```

Also canonical:

```text
src/app/users/service.zt
src/app/users/validation.zt
```

Both files declare:

```zt
namespace app.users
```

The namespace is `app.users`.

The files are internal physical partitions of that namespace.

## File Name Is Not Namespace

The filename `types.zt` does not create namespace `app.users.types`.

Invalid under this decision:

```text
src/app/users/types.zt
```

```zt
namespace app.users.types
```

Reason:

- `types` is a file name here, not a directory segment
- import granularity comes from namespace directories, not from individual source files
- this avoids making file names part of the public API by accident

## Importing A Smaller Part

To import `app.users.types`, make `types` a real namespace directory.

Canonical:

```text
src/app/users/types/administrator.zt
src/app/users/types/customer.zt
```

Both files declare:

```zt
namespace app.users.types
```

A consumer may import that namespace:

```zt
namespace app.main

import app.users.types as types

func main() -> int
    const admin: types.Administrator = types.Administrator(name: "Julia")
    return 0
end
```

This keeps the import readable and explicit:

- `app.users` can expose broader user behavior
- `app.users.types` can expose only user data types
- the folder structure shows the same boundary that the import shows

## Multiple Files Per Namespace

A namespace may be split across several files in the same directory.

Example:

```text
src/app/users/types.zt
src/app/users/service.zt
src/app/users/validation.zt
```

All declare:

```zt
namespace app.users
```

Private declarations are shared by files in the same effective namespace according to the visibility decision.

Duplicate top-level symbol names in the same namespace remain errors.

## Tests

Tests use `test.root` as a separate source root that is active only in test builds.

Canonical:

```text
tests/app/users/users_test.zt
```

```zt
namespace app.users

attr test
func creates_user()
    check(true)
end
```

A test file may declare the same namespace as the code it tests.

That allows tests to access namespace-private helpers without making those helpers public.

Canonical test filenames end with `_test.zt`.

Test discovery still relies on `attr test`, not only on filename.

## ZDoc

ZDoc files mirror source files by relative path under `zdoc.root`.

Canonical:

```text
src/app/users/types.zt
zdoc/app/users/types.zdoc
```

This pairing documents the concrete source file.

It does not mean that `types.zdoc` defines namespace `app.users.types`.

A namespace split across several source files may have several paired ZDoc files.

## Build And Generated Output

Generated directories are not source roots.

Canonical generated directories:

```text
build/
dist/
```

The compiler must not scan generated output unless a user explicitly misconfigures a source root to point there.

Recommended diagnostic direction:

```text
error[project.source_output_overlap]
source.root must not point inside build.output.

help
  Keep source files under src and generated files under build or dist.
```

## Rationale

This model is the most aligned with Zenith's readability philosophy for the MVP.

The explicit `namespace` answers "where am I?" inside the file.

The path validation answers "is this project organized consistently?".

Keeping filenames out of namespace identity avoids accidental public API changes when a file is renamed or split.

Using directories for import granularity makes structure visible in the project tree and keeps imports predictable.

This favors reading and navigation over shorter paths.

## Non-Canonical Forms

File name as namespace segment:

```text
src/app/users/types.zt
```

```zt
namespace app.users.types
```

Importing a file:

```zt
import app.users.types.zt
```

Implicit namespace from path only:

```zt
import std.io as io

func main() -> int
    return 0
end
```

Source file directly under `source.root`:

```text
src/main.zt
```

Using generated output as source:

```toml
[source]
root = "build"
```

## Out of Scope

This decision does not yet define:

- package dependency directory layout
- ZPM cache layout
- workspace/monorepo layout
- generated C file layout
- final `dist` artifact layout
- platform-specific output layout
- source glob include/exclude syntax
- automatic namespace rewriting during refactors
