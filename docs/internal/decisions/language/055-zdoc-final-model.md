# Decision 055 - ZDoc Final Model

- Status: accepted
- Date: 2026-04-17
- Type: tooling / documentation / docs generation
- Scope: public docs, private docs, Markdown prose, symbol links, guide pages, doc diagnostics

## Summary

ZDoc is Zenith's external documentation system.

Source files remain focused on implementation.

ZDoc files provide Markdown-oriented documentation, symbol metadata and future generated documentation output.

The model supports both paired API documentation and free guide/tutorial pages.

## Decision

Normative rules for this cut:

- paired API documentation remains the primary ZDoc model
- paired `.zdoc` files mirror `.zt` files under `zdoc.root`
- a ZDoc entry documents one target through `@target`
- ZDoc bodies are Markdown-oriented prose
- code examples inside ZDoc are Markdown code fences, not executable tests in the MVP
- private symbols may be documented in paired ZDoc files
- public undocumented symbols produce warnings, not errors
- private undocumented symbols do not produce warnings by default
- symbol links use `@link SymbolName`
- guide/tutorial pages are allowed under a separate guide area inside `zdoc.root`
- guide pages do not define executable Zenith code
- `zt doc check` validates ZDoc structure and target/link resolution
- `zt doc build` is future documentation generation

## Paired API Documentation

Canonical pairing:

```text
src/app/users/types.zt
zdoc/app/users/types.zdoc
```

A paired `.zdoc` file documents symbols declared in the matching `.zt` file.

Example source:

```zt
namespace app.users

public struct User
    name: text
end

func normalize_name(name: text) -> text
    return name
end
```

Example ZDoc:

```zdoc
--- @target: namespace
User domain types and helpers.
---

--- @target: User
Represents an application user.
---

--- @target: User.name
Display name for the user.
---

--- @target: normalize_name
Normalizes a user name before storage.
---
```

## Private Documentation

Private symbols may be documented by targeting them normally in the paired `.zdoc` file.

Private documentation does not need a separate syntax.

Example:

```zt
namespace app.users

func normalize_name(name: text) -> text
    return name
end

public func create_user(name: text) -> User
    const normalized: text = normalize_name(name)
    return User(name: normalized)
end
```

```zdoc
--- @target: normalize_name
Internal normalization helper used before creating users.

This is not public API, but documenting it helps maintainers understand the validation path.
---
```

Implementation model:

- the compiler/doc checker already knows whether a target symbol is public or private
- ZDoc stores documentation metadata attached to that symbol
- doc generation can choose whether to include private docs
- editor tooling can show private docs while editing the same package/namespace
- public site generation should exclude private docs by default

Recommended future generation options:

```text
zt doc build
zt doc build --include-private
```

Private documentation is useful for maintainers, but should not accidentally become published API.

## Missing Documentation Severity

Missing public documentation is a warning.

Expected direction:

```text
warning[doc.missing_public]
Public symbol User has no ZDoc entry.

help
  Add a ZDoc block with @target: User.
```

Missing private documentation is not reported by default.

Rationale:

- public symbols shape the user-facing API
- private symbols may be intentionally small or obvious
- warnings preserve progress without blocking compilation

A future strict documentation mode may promote warnings to errors.

## Markdown Body

ZDoc bodies are Markdown-oriented prose.

Example:

````zdoc
--- @target: create_user
Creates a validated user.

Example:

```zt
const user: User = create_user(name: "Julia")
```

@param name Name used to create the user.
@return The created user.
---
````

Code fences are documentation examples only in the MVP.

They are not compiled, run or type-checked by `zt doc check`.

Future doc tests require a separate decision.

## Symbol Links

The canonical inline symbol link form is:

```zdoc
@link User
```

Example:

```zdoc
--- @target: create_user
Creates and returns a @link User.

Use @link validate_user_name when the caller needs to validate first.
---
```

Rules:

- `@link SymbolName` references a symbol
- links in paired API docs resolve relative to the paired `.zt` file first
- links may resolve to imported/public symbols when fully qualified or unambiguous in a future expanded resolver
- unresolved links are warnings in the MVP
- broken `@target` remains an error because the documentation block cannot attach to anything

Expected unresolved link diagnostic:

```text
warning[doc.unresolved_link]
Could not resolve ZDoc link UserProfile.

help
  Check the symbol name or use a qualified target when supported.
```

Why `@link`:

- textual and readable
- consistent with existing `@target`, `@param`, `@return`
- avoids introducing bracket-heavy syntax into prose
- easy to parse without conflicting with Markdown links

## Guide Pages

Recommendation: allow guide/tutorial pages, but keep them separate from paired API docs.

Canonical layout:

```text
zdoc/
  app/
    users/
      types.zdoc
  guides/
    getting-started.zdoc
    packages.zdoc
    testing.zdoc
```

Guide pages are free documentation pages.

They are not paired to `.zt` files.

They must declare page metadata at the top.

Canonical guide page:

```zdoc
--- @page: getting-started
# Getting Started

This guide explains how to create a basic Zenith project.

Use @link app.main when referring to the application entry namespace.
---
```

Rules for guide pages:

- guide pages live under `zdoc/guides/`
- guide files use `.zdoc`
- guide files use `@page`, not `@target`
- a guide file may contain one page block in the MVP
- guide body is Markdown-oriented prose
- guide pages may use `@link`
- guide pages do not document private symbols by default
- guide pages do not define executable code

Why include guides:

- API reference alone is not enough for a learning-focused language
- Zenith's philosophy values accessibility and teaching
- guides let us explain concepts without polluting `.zt` source files
- separating `guides/` prevents confusion with mirrored API docs

## ZDoc Check

`zt doc check` validates documentation without generating a site.

It should detect:

- malformed ZDoc blocks
- missing `@target` in paired API blocks
- missing `@page` in guide pages
- target that does not exist
- duplicate target documentation
- invalid `@param`
- invalid `@return`
- unresolved `@link`
- public symbols missing documentation
- `.zdoc` files that do not match either paired API layout or guide layout

Recommended severity:

- malformed block: error
- missing `@target` in paired API doc: error
- missing `@page` in guide doc: error
- unresolved `@target`: error
- duplicate `@target`: error
- invalid `@param`: error
- invalid `@return`: error
- unresolved `@link`: warning
- missing public documentation: warning
- missing private documentation: no diagnostic by default

## ZDoc Build

`zt doc build` is a future command for generating human-readable documentation.

Default generated docs should include:

- public API reference
- guide pages
- symbol index
- namespace index
- links between docs and symbols

Default generated docs should not include:

- private symbols
- internal implementation notes
- generated C details

Private docs may be included only through an explicit option.

## Relationship To Source Comments

ZDoc is the primary public documentation system.

Source comments remain local implementation notes.

Example:

```zt
-- Local note about a tricky algorithm.
func normalize_name(name: text) -> text
    ...
end
```

Long public explanations belong in `.zdoc`.

This keeps `.zt` files visually stable and reduces cognitive noise.

## Rationale

The final ZDoc model supports both reference documentation and learning material without turning source code into a documentation container.

Allowing private documentation helps maintainers understand non-public behavior while keeping generated public docs clean.

Keeping examples as Markdown first avoids prematurely adding doc-test complexity.

Using `@link` keeps symbol references explicit and easy to read.

Treating missing public docs as warnings encourages documentation without blocking early development.

## Non-Canonical Forms

Inline public documentation as the main model:

```zt
/// Creates a user.
public func create_user(name: text) -> User
    ...
end
```

Executable code inside ZDoc:

```zdoc
--- @target: User
func helper() -> int
    return 10
end
---
```

Guide pages mixed into mirrored API paths:

```text
zdoc/app/users/tutorial.zdoc
```

Private docs published by default:

```text
zt doc build
```

and unexpectedly including private helpers.

Bracket-heavy symbol links as canonical syntax:

```zdoc
See [[User]] for details.
```

## Out of Scope

This decision does not yet define:

- generated documentation theme
- static site output layout
- search index format
- localization
- doc tests
- versioned documentation publishing
- cross-package symbol linking through ZPM
- private documentation access control
- Markdown extension set
- rich semantic sections beyond current tags
