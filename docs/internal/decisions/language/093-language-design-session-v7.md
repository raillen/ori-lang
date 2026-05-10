# Decision 093 — Language Design Session v7

- Status: accepted
- Date: 2026-04-27
- Scope: language surface, stdlib, builtins, tooling, roadmap

## Context

A block-by-block design session reviewed every proposed and deferred feature
for the path to Zenith 1.0. Each topic was analyzed for pros, cons,
implementation cost, and alignment with the language philosophy
(explicit, reading-first, neurodivergent-friendly).

This decision consolidates all outcomes into a single record.

## Decisions

### Bloco 1 — Resource Cleanup

- **`using`** is the public keyword for resource cleanup.
- **`defer`** is the internal lowering mechanism (not exposed to users).
- Execution order: **LIFO** (last declared, first cleaned).
- Cleanup runs on: `end`, `return`, `?` propagation, `panic`.
- Two forms:
  - Block: `using x = expr ... end` (scoped to block).
  - Flat: `using x = expr` (scoped to enclosing function).
  - Custom: `using x = expr then cleanup_expr`.
- `Disposable` trait for automatic cleanup is deferred until `using` is validated.

### Bloco 2 — Destructuring and Collections

- **Destructuring in `match`**: accepted. `case variant(binding)` syntax.
- **Destructuring in `const`** (`unpack`): rejected. `pos.x` is already explicit.
- **Rest operator** (`...`): rejected as syntax. Solved by `list.rest()` / `list.skip()`.
- **Tuples**: confirmed out. Structs are the Zenith answer.
- **`set<T>`**: accepted as new built-in type (hash-based, O(1) lookup).
- **List helpers**: `list.first()`, `list.last()`, `list.rest()`, `list.skip()` accepted.

### Bloco 3 — Types

- **`char` type**: rejected. `text` with helpers suffices.
- **`text` keyword**: confirmed as canonical. `string` remains absent.
- **`uint` standalone**: rejected. `u64` exists for unsigned needs.
- **Primitive specializations**: accepted. Emitter rule: if T is primitive,
  emit contiguous array without per-element ARC. Applies to list and optional.

### Bloco 4 — Ergonomics

- **`?.` safe navigation**: rejected. Dense symbol, hides control flow.
- **`??` null coalescing**: rejected. `.or(default)` already solves this.
- **Implicit return**: rejected. Anti-explicit, minimal gain.
- **`.or_return(value)`**: accepted as helper for early return pattern.

### Bloco 5 — Manual Memory

- **`using`** (MM.M1): accepted (see Bloco 1).
- **`std.mem.Allocator`** (MM.M2): deferred. No real case validated.
- **Debug allocator** (MM.M3): deferred. Depends on MM.M2.
- **Containers with allocator** (MM.M4): deferred. Depends on MM.M2.
- **`std.unsafe` / raw pointers** (MM.M5): deferred. FFI 1.0 works without it.
- **`owned<T>` / `borrow<T>` / lifetimes** (MM.M6): rejected. Changes language identity.

### Bloco 6 — Stdlib, Builtins, Backends

- **New builtins**: `print`, `read`, `len`, `debug`, `type_name`, `size_of`, `range`.
- **`f"..."`** replaces `fmt "..."` for string interpolation. `fmt` kept as
  deprecated alias with diagnostic.
- **`std.fs`**: expanded (exists, is_file, is_dir, create_dir, remove, rename, copy, list_dir).
- **`std.math`**: native surface wrapping libm (abs, pow, sqrt, min, max, clamp, trig, pi, e).
- **`std.time`**: minimal (now, elapsed, sleep).
- **`std.regex`**: wrap PCRE2/RE2 (compile, is_match, find_all).
- **`std.net`**: deferred (needs async/blocking decision).
- **Compiler optimizations**: deferred (when benchmarks show need).
- **JS transpilation, LLVM, WASM**: deferred.

### Bloco 7 — Product and Readiness

- Stabilization (clean clone, CI, surface freeze): priority.
- VSCode Marketplace: accepted.
- C-Binding Pack (sqlite3/libcurl): accepted.
- REPL (`zt repl`): accepted, compile-and-run approach first.
- Self-hosting SH1 (3 tools in Zenith): accepted.
- SH2-SH6: deferred.
- ZPM: lock file + semver accepted for 1.0.
- Docs: English canonical, other languages best-effort.

### Bloco 8 — Additional Topics

- **`range(start, end, step?)`**: accepted as builtin, inclusive only.
- **Mutable closure capture**: accepted with explicit `[mut var]` syntax.
- **Concurrency**: confirmed out of 1.0. Host runtimes handle this.
- **Error model**: no changes. `.or_wrap(context)` helper added for error chaining.
- **Trait default implementations**: accepted.
- **Testing DX**: filter, output, `test.throws()` accepted.
- **ZPM lock file + semver**: accepted.
- **i18n**: English-first, translations best-effort.

## Consequences

- Old roadmaps (v1-v6) and specialized checklists are superseded by
  `roadmap-v7.md` and `checklist-v7.md`.
- The language surface gains: `using`, destructuring in match, `f"..."`,
  `range()`, mutable closures, trait defaults, `set<T>`, 7 new builtins.
- The language surface does NOT gain: `char`, `uint`, tuples, `?.`, `??`,
  implicit return, rest operator, `try/catch`, `async/await`, ownership/lifetimes.
- Manual memory features are documented as future possibilities with zero commitment.

## Related

- `docs/internal/planning/roadmap-v7.md`
- `docs/internal/planning/checklist-v7.md`
- `docs/internal/decisions/language/092-single-file-execution.md`
