# MVP Out of Scope

Historical MVP note plus current alpha deferred list.

This file is not the canonical source for "what Zenith supports today".

Read these first for the current alpha cut:

- `docs/spec/language/surface-implementation-status.md`
- `docs/spec/language/diagnostic-code-catalog.md`
- `README.md`

This file now has a narrower job:

- keep the original MVP boundary visible;
- list what is still explicitly deferred after the current alpha cut.

## Language features deferred post-MVP

- `unless` keyword
- ternary expressions
- pipe operators
- `?.` safe navigation
- `??` null-coalescing
- selective imports (`from ... import ...`)
- wildcard imports (`import *`)
- implicit `self.` omission
- implicit `return`
- `uint` as standalone type
- `char` type
- math operators as syntax (`**`, `//`)
- C-style for loops
- `for` with numeric ranges (only collection-oriented `for`)
- async/await
- exception handling beyond `result<T,E>`
- pattern matching beyond `match case`/`default`
- destructuring in patterns
- method overloading
- operator overloading beyond `==`/`!=` via `Equatable`
- variadic parameters
- first-class functions / function types as generic arguments (MVP limits to named references)
- closures capturing mutable state
- nested functions (inner `func`)

## Standard library deferred post-MVP

- public API expansion for `std.fs`
- public API expansion for networking and pipes
- `std.math` expansion - `math.pow`, `math.sin`, etc (use `extern c` for now)
- `std.time` expansion - dates, timestamps
- `std.regex` - pattern matching on text

## Compiler features deferred post-MVP

- Incremental compilation
- Optimization passes beyond what the C compiler provides
- Source maps / debug information generation
- Plugin or extension system

## Runtime features deferred post-MVP

Current MVP runtime contract before the deferred items below:

- ordinary managed values use non-atomic ARC on single-isolate paths
- cross-thread work uses isolate/message-passing boundaries, not shared ordinary managed values
- RC cycles are a known leak risk today; no cycle collector ships in the MVP

Still deferred:

- Cycle collection in RC
- Thread-safe reference counting
- User-facing `workers/jobs/channels` concurrency surface
- Custom allocator hooks
- Separate compilation units in the C runtime
- `map` specializations beyond `map<text,text>`
- `list<float>` specialization
- `optional<float>` and `optional<bool>` specializations
- Struct runtime support (generic field access)
- Full `std.text` helper surface beyond the subset already shipped in the alpha runtime

## Backend targets deferred post-MVP

- Zig target
- LLVM target
- WASM target
- JavaScript target

## ZIR contractual status

- ZIR textual format (`.zir` files) is for debug, fixtures, and golden tests only
- ZIR structured in-memory representation is the canonical contract between compiler stages
- The frontend never produces or consumes `.zir` files directly
