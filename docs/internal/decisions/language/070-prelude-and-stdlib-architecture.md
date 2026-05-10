# Decision 070 - Prelude And Stdlib Architecture

- Status: accepted
- Date: 2026-04-17
- Type: language / stdlib / architecture
- Scope: implicit prelude boundary, explicit stdlib policy, canonical import aliases and next-wave stdlib planning

## Summary

Zenith uses a minimal implicit prelude and an explicit standard library.

The standard library is intentionally not auto-imported as a large convenience layer.

`std.net` is recognized as an important next-wave stdlib module. Its first-wave model is now defined by Decisions 073, 074 and 075, but it is still not part of the current consolidated core stdlib implementation wave.

## Decision

Accepted architectural direction:

- the implicit prelude is minimal and structural
- the standard library is explicit by import
- `std.*` modules are not auto-imported into every file
- the "main stdlib" exists as a documentation and onboarding category, not as a magical import layer

## Minimal Implicit Prelude

The implicit prelude should include only elements that are fundamental to the language model itself.

Accepted examples of prelude-level concepts include:

- core structural types and language-level forms
- `optional`
- `result`
- `none`
- `success`
- `error`
- core traits such as `Equatable<T>`, `Hashable<T>`, `Comparable<T>` and `TextRepresentable<T>`
- `len(...)`
- `check(...)` in the testing model already defined by the language

These are treated as part of the language/core experience, not as auto-imported `std.*` modules.

## Explicit Stdlib Policy

All ordinary library modules remain explicit imports.

This includes the currently consolidated core stdlib modules:

- `std.text`
- `std.bytes`
- `std.math`
- `std.validate`
- `std.format`
- `std.time`
- `std.io`
- `std.fs`
- `std.fs.path`
- `std.json`
- `std.os`
- `std.os.process`
- `std.test`

None of these are implicitly imported by default.

## Canonical Alias Direction

The documentation should use stable, readable aliases for the main stdlib modules.

Accepted canonical alias direction:

```zt
import std.text as text
import std.bytes as bytes
import std.math as math
import std.validate as validate
import std.format as format
import std.time as time
import std.io as io
import std.fs as fs
import std.fs.path as path
import std.json as json
import std.os as os
import std.os.process as process
import std.test as test
```

These aliases improve consistency across examples and documentation.

## Why Stdlib Stays Explicit

The standard library stays explicit because:

- symbol origin remains visible in code
- name collisions are reduced
- tooling has a clearer module boundary
- readability improves when calls appear as `text.trim(...)`, `math.sqrt(...)`, `json.parse(...)` and `fs.read_text(...)`

A richer implicit prelude would make the language shorter in trivial cases, but less explicit and less predictable overall.

## Stdlib Learning Order

The "main stdlib" is still useful as a learning and documentation concept.

Accepted documentation order for the current consolidated wave:

1. core/prelude
2. `std.text`
3. `std.bytes`
4. `std.math`
5. `std.validate`
6. `std.format`
7. `std.time`
8. `std.io`
9. `std.fs`
10. `std.fs.path`
11. `std.json`
12. `std.os`
13. `std.os.process`
14. `std.test`

This ordering is for teaching and documentation, not for implicit import semantics.

## Next-Wave Modules

`std.net` is accepted as an important next-wave stdlib module.

Its first-wave MVP model is now closed by three decisions:

- Decision 073: no public `std.stream` module in the MVP; use concrete module-owned types first
- Decision 074: `std.net` is synchronous/blocking in the MVP, with explicit connection timeout
- Decision 075: first-wave `std.net` is TCP client only, using `host: text`, `port: int` and concrete `net.Connection`

A likely architectural sequence is:

- `std.net`
- `std.net.tls`
- websocket discussion after the basic network layer is designed

The exact final placement of websocket remains open.

## Rationale

A minimal implicit prelude matches Zenith explicitness and readability goals.

Keeping the stdlib explicit avoids hidden imports and reduces semantic ambiguity.

Documenting a canonical alias set improves consistency without introducing language magic.

With `std.bytes` now defined by Decision 072, binary data has a first-wave stdlib home.

With the stream policy now defined by Decision 073, the timeout policy now defined by Decision 074 and the socket/address model now defined by Decision 075, `std.net` has enough first-wave design to become an implementation planning item after the current core stdlib wave.

## Non-Canonical Forms

Auto-importing broad library modules into every file by default.

Using a hidden "basic" or "standard" umbrella module as an implicit import layer.

Treating documentation grouping as if it implied magical import semantics.

Folding `std.net` into the already-consolidated current wave before implementation planning and the current core stdlib wave are complete.

## Out of Scope

This decision does not yet define:

- the detailed API of `std.net`
- the detailed API of `std.net.tls`
- websocket placement and API
- package-level prelude customization
- any future compatibility layer for alternative import ergonomics
