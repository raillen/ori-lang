# Zenith - Not Implemented (Deferred Post-MVP)

## Language Features

| Feature | Status | Notes |
|---------|--------|-------|
| `unless` keyword | Deferred | Use `if not condition` |
| Ternary expressions | Deferred | Use `if/else` |
| Pipeline expansion beyond current `|>` | Deferred | Current surface keeps the simple pipeline form only |
| `?.` safe navigation | Deferred | Use `?` propagation + match |
| `??` null-coalescing | Deferred | No `null` in Zenith |
| Selective imports (`from ... import ...`) | Deferred | Import whole namespaces only |
| Wildcard imports (`import *`) | Deferred | |
| Relative imports | Deferred | |
| Implicit `self.` omission | Deferred | Always use `self.field` |
| Implicit `return` | Deferred | `return` is always explicit |
| `uint` standalone type | Deferred | Use `u8`/`u16`/`u32`/`u64` |
| `char` type | Deferred | |
| `**` exponentiation operator | Deferred | Use `math.pow` |
| `//` floor division | Deferred | |
| C-style for loops | Deferred | Use collection-oriented `for` |
| Range literal loop syntax | Deferred | Use `range(start, end, step)` with collection-oriented `for` |
| async/await | Deferred | |
| Exception handling beyond `result<T,E>` | Deferred | |
| Complex nested pattern matching beyond current `match`/`case else` | Deferred | Current surface supports literal, binding, enum, tuple, and simple struct patterns |
| Broad destructuring patterns | Deferred | Keep destructuring small and explicit |
| Method overloading | Deferred | |
| Operator overloading beyond `Addable`, `Subtractable`, and `Comparable` | Deferred | No broad custom operator set |
| Variadic parameters | Deferred | |
| Broader callable ABI | Deferred | Current surface supports callable types and the validated callback subset |
| Closures capturing mutable state | Deferred | |
| Richer nested-function capture forms | Deferred | Current nested-function subset is validated |
| Universal dynamic value or reflection type | Deferred | Use `any<Trait>` for object-safe trait dispatch |
| `null` | Deferred | Never; use `optional<T>` |
| Macros | Deferred | |
| User-facing C interop | Deferred | |
| Multi-statement lambda expressions | Deferred | Current surface keeps explicit callable syntax |
| Conditional compilation in `.zt` | Deferred | |
| Doc tests | Deferred | |
| VM/bytecode execution model | Deferred | Native C backend only |

## Standard Library

| Feature | Status | Notes |
|---------|--------|-------|
| Advanced filesystem APIs (watch, permissions, symlinks) | Deferred | `std.fs` covers the Phase 4A sync API surface |
| Networking and pipes | Deferred | `std.net` is next wave |
| Calendar/date formatting APIs | Deferred | `std.time` currently covers instant, duration, Unix timestamps, elapsed time and sleep |
| Advanced regex APIs (groups, captures, flags, Unicode classes) | Deferred | `std.regex` currently covers validation, match, full match, first/count/find all, split, replace all and escape for simple patterns |
| Public generic stream abstraction | Deferred | |
| Async IO | Deferred | |
| TLS | Deferred | |
| Websocket | Deferred | |
| Package registry integration | Deferred | |
| Optional dependencies / feature flags | Deferred | |

## Compiler

| Feature | Status | Notes |
|---------|--------|-------|
| Incremental compilation | Deferred | |
| Optimization passes beyond C compiler | Deferred | |
| Source maps / debug info generation | Deferred | `#line` directives exist |
| Plugin or extension system | Deferred | |

## Runtime

| Feature | Status | Notes |
|---------|--------|-------|
| Cycle collection in RC | Deferred | RC cycles = leak risk |
| Thread-safe reference counting | Deferred | Use isolates |
| Custom allocator hooks | Deferred | |
| Separate compilation units in C runtime | Deferred | |
| `map` specializations beyond `map<text,text>` | Deferred | |
| Struct runtime support (generic field access) | Deferred | |
| Full `std.text` helper surface | Deferred | Subset shipped in alpha |

## Backend Targets

| Target | Status |
|--------|--------|
| Zig | Deferred |
| LLVM | Deferred |
| WASM | Deferred |
| JavaScript | Deferred |
| Cross-compilation policy | Deferred |
| Native package installer generation | Deferred |

## ZIR Notes

- ZIR textual format (`.zir` files) = debug, fixtures, golden tests ONLY
- ZIR structured in-memory representation = canonical contract between stages
- Frontend never produces or consumes `.zir` files directly
