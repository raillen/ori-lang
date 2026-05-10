# Decision 016 - Zdoc And Doc Linking

- Status: accepted
- Date: 2026-04-16
- Type: tooling / documentation
- Scope: external symbol documentation, `.zdoc` files, target linking, documentation validation

## Summary

Zenith Next keeps source files focused on implementation and supports optional external documentation files with the `.zdoc` extension.

A `.zdoc` file documents symbols declared in the matching `.zt` file.

The canonical linking mechanism is the `@target` tag inside block-delimited documentation entries.

ZDoc is documentation metadata, not executable code.

## Decision

The canonical external documentation form for Zenith Next is a `.zdoc` file placed under the project's `zdoc_root`, mirroring the relative path of its paired `.zt` source file under `source_root`.

Canonical file pairing:

- `src/app/user/profile.zt`
- `zdoc/app/user/profile.zdoc`

Normative rules for this cut:

- a `.zdoc` file is optional
- `zdoc_root` is discovered from the project manifest layout
- if present, a `.zdoc` file documents the matching `.zt` file at the mirrored relative path under `zdoc_root`
- the canonical pairing rule is `source_root/relative/path/file.zt` -> `zdoc_root/relative/path/file.zdoc`
- ZDoc content is documentation only and contains no executable Zenith code
- ZDoc entries are written as block-delimited documentation records
- each record must declare exactly one `@target`
- the body of a ZDoc record is Markdown-oriented prose
- ZDoc is merged into symbol metadata for tooling, documentation generation and editor help
- no per-file registration inside `zenith.ztproj` is required beyond declaring `zdoc_root`

## Canonical Block Format

The canonical ZDoc block format is:

```zdoc
--- @target: SymbolName
Documentation prose goes here.
---
```

Multiple blocks may appear in the same `.zdoc` file.

Canonical example:

```zdoc
--- @target: namespace
Utilities for reading and validating user profiles.
---

--- @target: Player
Represents a player with tracked hit points.
---

--- @target: Player.hp
Current hit points for the player. This value should never be negative.
---

--- @target: Healable.heal
Restores hit points to the receiver.

@param amount Amount of hp to restore.
---
```

## Target Model

The `@target` tag links prose to the symbol surface declared in the paired `.zt` file.

Canonical target forms in this cut:

- `namespace` for the file-level namespace declaration
- `FunctionName` for a top-level function
- `StructName` for a struct
- `TraitName` for a trait
- `EnumName` for an enum
- `StructName.field_name` for a struct field
- `StructName.method_name` for an inherent method introduced by `apply Type`
- `TraitName.method_name` for a trait method contract
- `EnumName.VariantName` for an enum variant
- `extern_block_function_name` as an ordinary top-level function target when documenting an extern declaration

Normative rules for this cut:

- targets are resolved relative to the paired `.zt` file, not by global string search across the project
- a target must refer to a symbol actually declared in the paired file
- one ZDoc record documents one target
- duplicate records for the same target are not canonical in the MVP
- implementation blocks such as `apply Trait to Type` are not separate documentation targets in this cut
- documentation for trait behavior belongs on the trait or its methods, not on the apply implementation block
- documentation for inherent behavior belongs on the resulting method symbol, for example `Player.is_alive`

## Structured Tags

The only required structural tag in the MVP is `@target`.

The following tags are recognized as optional structured documentation tags in this cut:

- `@param name`
- `@return`

Normative rules for this cut:

- `@param name` is valid only for function-like targets
- when `@param name` is present, `name` must match a declared parameter name
- `@return` is valid only for value-returning function-like targets
- additional lines that do not begin with a recognized structured tag are treated as prose
- future tags may be added later without changing the basic ZDoc pairing model

## Semantic Validation

ZDoc participates in semantic validation at the documentation layer.

The validator must detect at least the following problems:

- a malformed documentation block
- a missing `@target`
- a target that does not resolve in the paired `.zt` file
- duplicate records for the same target
- an invalid `@param` name
- an `@return` tag attached to a target that does not return a value

Severity policy such as warning versus error is outside this decision, but the diagnostics themselves must exist.

## Rationale

A mirrored external documentation tree keeps Zenith source focused on implementation while still allowing high-quality API documentation.

Using one simple pairing rule, mirrored relative paths between `source_root` and `zdoc_root`, avoids clutter in source directories while keeping editor tooling straightforward.

Using `@target` as the only required structural tag keeps the format small, easy to read and easy to parse.

Resolving targets only against the paired source file reduces ambiguity and avoids turning documentation linking into a project-wide name search problem.

Keeping the body as Markdown-oriented prose makes the files useful both for documentation generation and for human editing without introducing another markup language.

## Non-Canonical Forms

These forms are not accepted as the canonical ZDoc style for Zenith Next:

embedding executable Zenith code in a `.zdoc` file:

```zdoc
func helper() -> int
    return 10
end
```

using a free-form file without target tags:

```zdoc
This file documents whatever the reader can infer.
```

documenting symbols from another file in the current sidecar:

```zdoc
--- @target: app.other.file.SomeType
Cross-file documentation block.
---
```

placing the `.zdoc` file next to the `.zt` source as the primary layout:

```text
src/app/user/profile.zt
src/app/user/profile.zdoc
```

using implementation blocks as primary documentation targets:

```zdoc
--- @target: apply Healable to Player
Implementation notes.
---
```

## Out of Scope

This decision does not yet define:

- inline documentation comments inside `.zt` source files
- doc test execution
- documentation inheritance or symbol overload disambiguation
- localized or multi-language ZDoc bundles
- rich schema tags beyond `@target`, `@param` and `@return`
- project-wide cross-file doc linking
- generated documentation site themes or output formats
- whether private symbols should be published in generated external docs
