# Decision 065 - Stdlib Fs Path Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / filesystem / path
- Scope: pure lexical path operations, path decomposition, normalization and relative/absolute transformation boundaries

## Summary

`std.fs.path` is the pure path-manipulation module.

It does not access the filesystem and does not resolve symlinks, permissions or current-directory state implicitly.

All operations are lexical and path-text oriented for the current platform.

## Decision

Canonical import:

```zt
import std.fs.path as path
```

Accepted first-wave API direction:

- `path.join(parts: list<text>) -> text`
- `path.base_name(path: text) -> text`
- `path.name_without_extension(path: text) -> text`
- `path.extension(path: text) -> optional<text>`
- `path.parent(path: text) -> optional<text>`
- `path.normalize(path: text) -> text`
- `path.absolute(path: text, base: text) -> text`
- `path.relative(path: text, from: text) -> text`
- `path.is_absolute(path: text) -> bool`
- `path.is_relative(path: text) -> bool`

Accepted soon-after direction, but not required in the first MVP surface:

- `path.change_extension(path: text, extension: text) -> text`
- `path.has_extension(path: text, extension: text) -> bool`
- `path.split(path: text) -> list<text>`

Examples:

```zt
import std.fs.path as path

const file_path: text = path.join(["config", "app.json"])
const base: text = path.base_name(file_path)
const name: text = path.name_without_extension(file_path)
const ext: optional<text> = path.extension(file_path)

const parent: optional<text> = path.parent(file_path)
const clean: text = path.normalize("./config/../config/app.json")

const full: text = path.absolute("app.json", base: "/workspace/config")
const rel: text = path.relative("/workspace/config/app.json", from: "/workspace")
```

Normative rules for this cut:

- `std.fs.path` is pure and has no filesystem side effects
- operations are lexical and do not check path existence
- operations do not resolve symlinks
- `absolute(...)` must not read the current directory implicitly
- `absolute(...)` requires an explicit `base:` argument in this cut
- `relative(path, from: ...)` is the accepted relative-path shape
- `parent(...)` returns `optional<text>` because some paths have no parent
- `extension(...)` returns `optional<text>` because some paths have no extension
- the module does not use empty text as a sentinel for missing path components
- decomposition and normalization semantics are platform-aware but filesystem-independent

## Naming Direction

Accepted canonical names in this cut:

- `base_name`
- `name_without_extension`
- `extension`
- `parent`

Rejected as canonical MVP names:

- `base`
- `dir`
- `ext`
- `stem`

Rationale:

- `base_name` is clearer than `base`
- `name_without_extension` is more legible than `stem`
- `extension` is clearer than `ext`
- `parent` is clearer and less ambiguous than `dir` for the accepted MVP surface

## Join Shape

`join(...)` is accepted conceptually as joining multiple path parts.

Because variadic function support is not yet fixed as a language feature, the current accepted MVP shape is:

```zt
path.join(parts: list<text>) -> text
```

If Zenith later standardizes variadic parameters, a more direct surface can be revisited.

## Boundaries

The following capabilities are intentionally outside `std.fs.path` in this cut:

- checking whether a path exists
- resolving a path through the real filesystem
- reading current directory implicitly
- file metadata queries
- directory listing

Those capabilities belong in `std.fs`, not in `std.fs.path`.

## Rationale

Separating lexical path logic from side-effecting filesystem logic keeps both modules easier to understand.

Requiring an explicit `base:` for `absolute(...)` avoids hidden dependence on process state.

Using `optional<text>` for missing parent and missing extension is more honest than overloading the empty string.

The `base_name` / `name_without_extension` / `extension` naming family is longer, but substantially clearer for Zenith readability goals.

## Non-Canonical Forms

Implicitly using the process current directory inside `path.absolute(...)`.

Using `std.fs.path` to check the filesystem:

```zt
path.exists(value)
```

Returning empty text when a component does not exist instead of using `optional<text>`.

Using short technical names when clearer names exist:

```zt
path.base(value)
path.ext(value)
path.stem(value)
```

## Out of Scope

This decision does not yet define:

- the exact lexical behavior for every platform edge case
- UNC-specific surface examples
- drive-letter edge cases
- trailing-separator preservation policy
- whether `join` later gains a variadic surface
- the exact semantics of `change_extension` and `has_extension`
