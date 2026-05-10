# Decision 044 - Comments And ZDoc Boundary

- Status: accepted
- Date: 2026-04-17
- Type: language / syntax / documentation
- Scope: line comments, block comments, doc comments, ZDoc boundary, source cleanliness

## Summary

Zenith source files use textual comment markers:

- `--` for line comments
- `--- ... ---` for block comments

Comments in `.zt` are for local implementation notes and are ignored by the compiler.

Public documentation should live in ZDoc files, not inside long source comments.

The MVP does not include doc comments such as `///` or inline `doc "..."` declarations.

## Decision

Line comment:

```zt
-- This is a local implementation note.
const total: int = price * quantity
```

Block comment:

```zt
---
This is a block comment.
It is ignored by the compiler.
---
```

Normative rules for this cut:

- `--` starts a line comment
- a line comment continues until the end of the line
- `---` starts a block comment
- the next `---` closes the block comment
- block comments do not nest in the MVP
- comments are ignored by parser/semantic stages
- comments do not generate public documentation
- comments should be used for short local implementation notes
- long public documentation belongs in ZDoc
- `///` doc comments are not part of the MVP
- `doc "..."` source documentation declarations are not part of the MVP
- `.zt` files should remain code-focused and visually clean
- ZDoc is the future documentation system for modules, APIs, guides and links

## Line Comments

Line comments are intended for short notes.

```zt
-- Apply discount after validating the amount.
const final_price: int = price - discount
```

They should not be used to write long public API documentation.

## Block Comments

Block comments are intended for temporary or rare local explanations.

```zt
---
This algorithm mirrors the external file format.
Keep the field order stable until the format version changes.
---
func parse_header(source: text) -> Header
    ...
end
```

Block comments do not nest in the MVP.

This is invalid or implementation-defined and should be rejected or diagnosed:

```zt
---
outer
---
inner
---
---
```

## No Inline Public Doc Comments In MVP

Zenith does not use this MVP direction:

```zt
/// Loads a user by id.
func load_user(id: int) -> result<User, text>
    ...
end
```

Zenith also does not use this MVP direction:

```zt
doc "Loads a user by id."
func load_user(id: int) -> result<User, text>
    ...
end
```

Reason:

- long comments make source visually noisy
- public documentation has different structure than implementation notes
- ZDoc should own rich docs, examples, links and module-level explanation
- keeping docs separate helps code remain readable and focused

## ZDoc Boundary

Public documentation belongs in `.zdoc` files.

ZDoc will later define:

- module documentation
- symbol documentation
- examples
- links between symbols
- links to decisions
- guide/tutorial structure
- generated API docs
- package documentation

Possible future organization examples:

```text
docs/reference/api/app.users.zdoc
docs/reference/api/app.users.User.zdoc
docs/reference/api/app.users.load_user.zdoc
```

or another structure defined by the future ZDoc decision.

This decision only establishes that rich documentation is not embedded as long comments in `.zt` source.

## Attribute Interaction

`attr doc_hidden` remains a possible future attribute, but it is not required by this decision.

If used later, it should affect ZDoc generation rather than creating inline documentation semantics.

```zt
attr doc_hidden
func internal_helper()
    ...
end
```

## Non-Canonical Forms

C-style line comments are not canonical:

```zt
// comment
```

C-style block comments are not canonical:

```zt
/*
comment
*/
```

Hash comments are not canonical:

```zt
# comment
```

Doc comments in source are not canonical for the MVP:

```zt
/// Public documentation.
func run()
    ...
end
```

Long documentation blocks inside source are discouraged:

```zt
---
# User API

Long public docs with examples, links and guides.
These belong in ZDoc.
---
```

## Diagnostics

Expected diagnostic directions:

```text
error[syntax.unclosed_comment]
Block comment was not closed.

where
  src/app/main.zt:5:1

help
  Add --- to close the comment block.
```

```text
error[syntax.unsupported_comment]
C-style block comments are not supported.

help
  Use --- ... --- for block comments.
```

## Rationale

Using `--` and `---` gives Zenith comments a distinct visual identity.

Keeping public documentation out of source reduces visual noise and supports the language's reading-first goal.

Separating comments from ZDoc avoids turning code files into mixed documentation/code documents.

ZDoc can provide richer structure than comments without burdening the parser or everyday source reading.

## Out of Scope

This decision does not yet define:

- concrete ZDoc syntax
- ZDoc file organization
- ZDoc link syntax
- documentation generation CLI
- doc coverage warnings
- private/public doc generation rules
- localization of documentation
- migration of existing docs into ZDoc
