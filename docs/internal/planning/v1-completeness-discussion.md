# Zenith v1.0 Completeness Discussion Guide

> Audience: maintainer
> Status: historical discussion input

> Surface: internal
> Source of truth: no; superseded by `docs/spec/language/final-language-contract.md`

> Created: 2026-04-29
> Rule: discuss each module incrementally with Raillen before implementation.

This document was the central discussion guide for making Zenith v1.0 complete.
It is retained as historical planning context. Current final/current/future status
lives in `docs/spec/language/final-language-contract.md` and topic-specific specs.

## How This Document Works

Each module has:

- **Current State**: what exists today in code and tests.
- **Gaps**: what is missing for v1.0 completeness.
- **Proposals**: concrete options with pros, cons, and rationale.
- **Decision**: filled after discussion with Raillen.
- **Status**: `pending` → `discussing` → `decided` → `implemented`.

---

## Module 1: String Interpolation Migration (`fmt` → `f`)

### Current State

- Parser accepts both `fmt "..."` and `f"..."`.
- `language-reference.md` uses `f"..."` as canonical.
- Checklist v7 marks L.17-L.20 as done.
- Most existing tests/examples still use `fmt`.

### Gap

- Formatter should output `f"..."`.
- All tests, examples, docs, and cookbook entries need migration.
- `fmt` should emit a deprecation diagnostic.

### Proposal

1. Verify formatter outputs `f"..."` (L.19).
2. Verify `fmt` emits deprecation warning (L.18).
3. Migrate all `.zt` files in `tests/`, `examples/`, `stdlib/`.
4. Update all docs to use `f"..."`.
5. Keep `fmt` accepted for one release cycle, then remove.

### Decision

Status: `decided` (2026-04-29) — essentially complete.

- Deprecation diagnostic for `fmt` already implemented (parser.c:1121).
- All `.zt` files (tests, stdlib, examples) already migrated to `f"..."`.
- Remaining: verify formatter outputs `f"..."` (minor), documentation updates (Module 18).
- `fmt` stays accepted for one more release cycle, then removed.

---

## Module 2: Collections — `list<T>`

### Current State

- `list<T>` is a built-in generic type with literal `[]` syntax.
- Compiler-known operations: `append`, `prepend`, `len`, `get`, `first`, `last`, `rest`, `skip`.
- Tier 4.1 adds the value-style explicit API in the C backend for `list<int>` and `list<text>`:
  `contains`, `reverse`, `set`, `remove_first`, `remove_last`, `remove_at`, `slice`,
  `concat`, and `index_of`.
- Tier 4.2 adds the executable HOF subset in the C backend for `list<int>`:
  `map`, `filter`, `reduce`, `find`, `any`, `all`, and `count`.
- Read indexing `values[i]` and slicing `values[start..end]` work.
- `std.list` is a 5-line marker file.
- Int-specialized HOFs exist in `std.collections`: `map_int`, `filter_int`, `reduce_int`.

### Gaps for v1.0

| API | Status | Priority |
|-----|--------|----------|
| `list.append(values, item) -> list<T>` | Compiler-known | Document |
| `list.prepend(values, item) -> list<T>` | Compiler-known | Document |
| `list.get(values, index) -> optional<T>` | Compiler-known | Document |
| `list.first(values) -> optional<T>` | Compiler-known | Document |
| `list.last(values) -> optional<T>` | Compiler-known | Document |
| `list.rest(values) -> list<T>` | Compiler-known | Document |
| `list.skip(values, count) -> list<T>` | Compiler-known | Document |
| `list.len(values) -> int` | Via builtin `len()` | Decide: also `list.len`? |
| `list.is_empty(values) -> bool` | Compiler-known | Done |
| `list.contains(values, value) -> bool` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.reverse(values) -> list<T>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.set(values, index, value) -> result<list<T>, core.Error>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.remove_first(values) -> result<list<T>, core.Error>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.remove_last(values) -> result<list<T>, core.Error>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.remove_at(values, index) -> result<list<T>, core.Error>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.slice(values, start, end) -> result<list<T>, core.Error>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.concat(left, right) -> list<T>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.index_of(values, value) -> optional<int>` | C backend for `list<int>`/`list<text>` | Done for v1 executable subset |
| `list.map<T,U>(values, fn) -> list<U>` | `list<int> -> list<int>` executable subset | Generic pending |
| `list.filter<T>(values, predicate) -> list<T>` | `list<int>` executable subset | Generic pending |
| `list.find<T>(values, predicate) -> optional<T>` | `list<int>` executable subset | Generic pending |
| `list.any<T>(values, predicate) -> bool` | `list<int>` executable subset | Generic pending |
| `list.all<T>(values, predicate) -> bool` | `list<int>` executable subset | Generic pending |
| `list.count<T>(values, predicate) -> int` | `list<int>` executable subset | Generic pending |
| `list.reduce<T,U>(values, initial, reducer) -> U` | `list<int> + int` executable subset | Generic pending |
| `list.sort_by<T>(values, compare) -> result<list<T>, core.Error>` | Missing | Implement |

### Proposals

**P2.A — Value-style free functions (v8 direction)**

All list operations are `list.fn(values, ...)` — consistent with Zenith's
explicit, reading-first philosophy. No method syntax on list values.

- Pro: explicit, no hidden `self`, consistent across all collections.
- Pro: works naturally with the current compiler-known model.
- Con: verbose for chaining: `list.filter(list.map(values, fn1), fn2)`.

**P2.B — Method syntax on list values**

Allow `values.map(fn)`, `values.filter(pred)` as sugar.

- Pro: more ergonomic for chaining.
- Con: requires method resolution on built-in generic types — complex compiler work.
- Con: inconsistent with current explicit API style.

**P2.C — Value-style functions + future pipe operator**

Use P2.A now. Explore `|>` pipe operator post-v1 for chaining.

- Pro: keeps v1 simple.
- Pro: pipe is a general language feature, not collection-specific.
- Con: pipe is deferred, so v1 chaining stays verbose.

**Sorting discussion:**

- `list.sort(values) -> list<T>` requires a natural ordering trait (`Comparable`).
  - v8 decided to defer natural sort and accept only `sort_by` with explicit comparator.
  - Question: should we accept `list.sort` for types that already implement `Comparable`?
  - Pro: convenient for `list<int>`, `list<text>`.
  - Con: ordering semantics for text are locale-dependent.
  - Recommendation: keep `sort_by` only for v1, add `sort` when `Comparable` trait is mature.

### Decision

Status: `implemented` (2026-04-30)

**API style**: P2.A — value-style free functions (`list.fn(values, ...)`).
Pipe operator `|>` deferred to post-v1.

**Full v1 API surface:**
- Existing (compiler-known): `append`, `prepend`, `get`, `first`, `last`, `rest`, `skip`.
- New: `len`, `is_empty`, `contains`, `reverse`, `set`, `remove_at`,
  `slice` (clamping), `concat`, `index_of` → `optional<int>`,
  `sort_by` → `result<list<T>, core.Error>` with explicit comparator.
- HOFs: `map`, `filter`, `find`, `any`, `all`, `count`, `reduce`, `flat_map`.
- `list.sort` deferred until `Comparable` trait.

**Returns**: `list.set`, `list.remove_at` return new list (value semantics, COW).
**Sorting**: `sort_by` returns `result` (comparator errors handled).

---

## Module 3: Collections — `map<K,V>`

### Current State

- `map<K,V>` is a built-in generic type with literal `{}` syntax.
- Compiler-known: `map[key]` (panics on miss), receiver `map.get(key) -> optional<V>`, and `std.map` free helpers.
- Specialized implementations exist for `map<text,text>`, `map<text,int>`, etc.
- `std.map` is a marker file; helpers are compiler-known because `map<K,V>` is built in.
- Tier 4.3 adds the executable C backend value-style subset for `map<text,text>`:
  `map.get`, `map.contains`/`map.has_key`, `map.set`, `map.remove`,
  `map.keys`, `map.values`, and `map.merge` (right wins).

### Gaps for v1.0

| API | Status |
|-----|--------|
| `map.get(values, key) -> optional<V>` | Compiler-known; generic lowering follows existing `map.get` path |
| `map.set(values, key, value) -> map<K,V>` | Done for C backend `map<text,text>` |
| `map.remove(values, key) -> map<K,V>` | Done for C backend `map<text,text>` |
| `map.contains(values, key) -> bool` | Compiler-known; `map.has_key` retained as alias |
| `map.keys(values) -> list<K>` | Done for C backend `map<text,text>` |
| `map.values(values) -> list<V>` | Done for C backend `map<text,text>` |
| `map.len(values) -> int` | Via builtin `len()` |
| `map.is_empty(values) -> bool` | Compiler-known |
| `map.merge(left, right) -> map<K,V>` | Done for C backend `map<text,text>`; right wins |

### Proposals

**P3.A — Explicit value-style API (v8 direction)**

- `map.set` returns a new map (value semantics with COW).
- `map.remove` returns a new map (silently returns equivalent map if key absent).
- `map.contains` checks key presence.
- Iteration via `for key, value in my_map`.
- Iteration order: **not guaranteed** (hash-based).

**Discussion: `map[key] = value` sugar for var maps?**

- Currently `map[key]` read works. Should `map[key] = value` write work for `var` maps?
- Pro: natural syntax for mutation.
- Con: hides COW cost; inconsistent with value-style API direction.
- Recommendation: support it for `var` bindings as syntactic sugar for `map.set`.

### Decision

Status: `decided` (2026-04-29)

**API surface:**
- `map.get(m, key) -> optional<V>`, `map.set(m, key, value) -> map<K,V>`,
  `map.remove(m, key) -> map<K,V>`, `map.contains(m, key) -> bool`,
  `map.keys(m) -> list<K>`, `map.values(m) -> list<V>`,
  `map.len(m) -> int`, `map.is_empty(m) -> bool`,
  `map.merge(left, right) -> map<K,V>` (right wins on conflict).
- HOFs: `map.map_values(m, fn) -> map<K,U>`, `map.filter(m, pred) -> map<K,V>`,
  `map.any(m, pred) -> bool`, `map.all(m, pred) -> bool`.
  Predicates receive `(key: K, value: V)`.

**Sugar**: `map[key] = value` for `var` maps = syntactic sugar for `map.set`.
**Iteration**: `for key, value in my_map` — must work. O(n²) scan accepted for v1.
**Tuple dependency**: `map.entries(m) -> list<tuple<K, V>>` is still a follow-up:
`tuple` exists, but `std.map.entries` needs generated list-of-tuple helpers.
Merge with resolver and ordered iteration remain separate work.
**Order**: iteration order is **not guaranteed** (hash-based).

---

## Module 4: Collections — `set<T>`

### Current State

- `set<T>` keyword exists in lexer.
- Literal syntax: `set {"a", "b"}`.
- Compiler-known hash-based implementation for `set<int>`, `set<text>`.
- `set.add`, `set.remove`, `set.has/contains`, `set.len` exist as compiler-known.
- `set.union`, `set.intersect`, `set.difference` exist as compiler-known.

### Gaps for v1.0

| API | Status |
|-----|--------|
| `set.empty<T>() -> set<T>` | Missing (explicit constructor) |
| `set.of<T>(values...) -> set<T>` | Missing (variadic constructor) |
| `set.add(values, value) -> set<T>` | Compiler-known |
| `set.remove(values, value) -> set<T>` | Compiler-known |
| `set.contains(values, value) -> bool` | Compiler-known |
| `set.len(values) -> int` | Via builtin `len()` |
| `set.is_empty(values) -> bool` | Missing |
| `set.union(left, right) -> set<T>` | Compiler-known |
| `set.intersection(left, right) -> set<T>` | Compiler-known |
| `set.difference(left, right) -> set<T>` | Compiler-known |
| `set.is_subset(sub, super) -> bool` | Missing |
| `set.to_list(values) -> list<T>` | Missing |

### Decision

Status: `decided` (2026-04-29)

**API surface:**
- Existing: `add`, `remove`, `has`, `contains`, `union`, `intersect`, `intersection`, `difference`.
- New: `len`, `is_empty`, `is_subset`, `to_list` (unordered), `from_list` (dedup),
  `symmetric_difference(a, b) -> set<T>`.
- HOFs: `filter`, `any`, `all`, `count`.
- `set.map` deferred (requires `Hashable` trait). Offer `set.map_to_list(s, fn) -> list<U>` instead.

**Implementation note (2026-04-30):** C backend now accepts `contains` and `intersection` aliases on top of the existing set runtime. `to_list` and HOF expansion stay coupled to broader generic collection work.

**Constructors**: use literals `set {}` and `set {1, 2, 3}`. No `set.empty<T>()` or `set.of()`.
**Iteration**: `for value in my_set` must work. Unordered. O(n) scan accepted for v1.

---

## Module 5: Text Primitives — `std.text`

### Current State (266 lines)

Implemented: `to_utf8`, `from_utf8`, `trim`, `trim_start`, `trim_end`, `contains`,
`join`, `replace_all`, `starts_with`, `ends_with`, `has_prefix`, `has_suffix`,
`has_whitespace`, `index_of`, `last_index_of`, `is_empty`, `is_blank`,
`is_digits`, `limit`.

Internal helpers: `_eq`, `_slice_from`, `_slice_to`, `_is_whitespace_char`,
`_is_ascii_digit_char`, `_starts_at`.

### Gaps for v1.0

| API | Status | Notes |
|-----|--------|-------|
| `text.len(value) -> int` | Via builtin `len()` | Decide: also in module? |
| `text.get(value, index) -> optional<text>` | Missing | Safe single-codepoint access |
| `text.slice(value, start, end) -> result<text, core.Error>` | Missing (has `_slice_from/to`) | Safe bounded slice |
| `text.concat(left, right) -> text` | Missing (has `+`) | Explicit form |
| `text.split(value, separator) -> list<text>` | Missing | Critical for parsing |
| `text.to_lower(value) -> text` | Missing | ASCII-first |
| `text.to_upper(value) -> text` | Missing | ASCII-first |
| `text.capitalize(value) -> text` | Missing | First char of each word |
| `text.replace(value, from, to) -> text` | Has `replace_all` | Rename or alias? |
| `text.repeat(value, count) -> text` | Missing | Useful for formatting |
| `text.pad_left(value, width, fill) -> text` | Missing | Table/CLI alignment |
| `text.pad_right(value, width, fill) -> text` | Missing | Table/CLI alignment |
| `text.chars(value) -> list<text>` | Missing | Codepoint iteration |

### Proposals

**P5.A — `split` implementation**

```zt
public func split(value: text, separator: text) -> list<text>
```

- Empty separator: split every codepoint.
- Empty value: return `[""]`.
- Separator not found: return `[value]`.
- This is the most requested missing text API.

**P5.B — Case conversion scope**

- `to_lower` / `to_upper`: ASCII-only for v1.0 (locale-neutral).
- Full Unicode case mapping: deferred (complex, locale-dependent).
- `capitalize`: uppercase first codepoint of each word, keep rest unchanged.

**P5.C — `replace` vs `replace_all` naming**

- Current: `replace_all(value, needle, replacement)`.
- v8 decision: `text.replace(value, from, to)` (replaces all occurrences).
- Recommendation: rename `replace_all` → `replace` (replace ALL is the default behavior, like Python/Go).
- `replace_first` can be added later if needed.

### Decision

Status: `decided` (2026-04-29)

**New APIs for v1.0:**
- `text.split(value, separator) -> list<text>` — empty sep = per-codepoint.
- `text.to_lower(value) -> text` — ASCII-only via C extern.
- `text.to_upper(value) -> text` — ASCII-only via C extern.
- `text.capitalize(value) -> text` — first char of each word uppercased.
- `text.repeat(value, count) -> text` — Zenith pure.
- `text.pad_left(value, width, fill) -> text` — Zenith pure.
- `text.pad_right(value, width, fill) -> text` — Zenith pure.
- `text.chars(value) -> list<text>` — equivalent to split(value, "").
- `text.get(value, index) -> optional<text>` — safe, returns none if OOB.
- `text.slice(value, start, end) -> text` — clamps bounds, never fails.
- `text.concat(left, right) -> text` — expose existing C extern publicly.

**Renames:**
- `replace_all` → `replace` (replace-all is default, like Python/Go).
  `replace_all` kept as alias for one release cycle.
- `join(parts)` → `join(parts, separator)` (standard signature).

**Kept as-is:** `has_prefix`/`has_suffix` aliases for `starts_with`/`ends_with`.
**Deferred:** Full Unicode case mapping (post-v1).

---

## Module 6: Bytes Primitives — `std.bytes`

### Current State (41 lines)

Implemented: `empty`, `from_list`, `to_list`, `join`, `concat`, `starts_with`,
`ends_with`, `contains`, `get`, `slice`, `index_of`, `len`, `is_empty`.

### Gaps for v1.0

| API | Status |
|-----|--------|
| `bytes.get(value, index) -> optional<int>` | Missing |
| `bytes.slice(value, start, end) -> result<bytes, core.Error>` | Missing |
| `bytes.concat(left, right) -> bytes` | Missing (has `join`) |
| `bytes.index_of(value, part) -> optional<int>` | Missing |
| `bytes.len(value) -> int` | Via builtin `len()` |
| `bytes.is_empty(value) -> bool` | Missing |
| `bytes.of(values...) -> result<bytes, core.Error>` | Missing |

### Proposals

- Rename `join` → `concat` (v8 decision; `join` kept as compatibility alias).
- `bytes.from_list` should return `result<bytes, core.Error>` for values outside 0..255.
- `bytes.get` returns `optional<int>` in 0..255 range.

### Decision

Status: `decided` (2026-04-29)

**API surface:**
- Existing: `empty`, `from_list`, `to_list`, `starts_with`, `ends_with`, `contains`.
- New: `get(value, index) -> optional<int>`, `slice(value, start, end) -> bytes` (clamping),
  `len(value) -> int`, `is_empty(value) -> bool`, `index_of(value, part) -> optional<int>`.
- Rename: `join` → `concat` (`join` kept as alias for one cycle).
- `from_list` returns `result<bytes, core.Error>` for values outside 0..255.

**Implementation note (2026-04-30):** M06 is shipped with safe APIs and compatibility alias `join`.

**No HOFs**: bytes is a transport type. Convert to `list<int>` for processing.

---

## Module 7: I/O — `std.io`

### Current State (52 lines)

- Has `io.Error` enum (ReadFailed, WriteFailed, Unknown).
- Public functions now return `result<..., io.Error>`: `read_line`, `read_all`, `write`, `print`.
- The C host bridge still returns `core.Error` internally and is mapped at the `std.io` boundary.
- `io.to_core_error(err)` exists for callers that still return `core.Error`.

### Gap

Closed. The public return types now follow `stdlib-model.md` and v8 IO.06.

### Proposal

1. Keep extern C stubs returning `core.Error` as the runtime ABI bridge.
2. Map host errors to `io.Error` inside `std.io`.
3. Keep public functions returning `result<..., io.Error>`.
4. Add `io.to_core_error(err: io.Error) -> core.Error` conversion helper.

### Decision

Status: `decided` (2026-04-29)

- Migrate all public returns from `core.Error` → `io.Error`.
- Add `io.to_core_error(err: io.Error) -> core.Error` conversion.
- Keep C externs as `core.Error` bridge; update tests and docs around the public surface.

---

## Module 8: Filesystem — `std.fs`

### Current State (267 lines)

Fully implemented: `read_text`, `write_text`, `append_text`, `exists`, `is_file`,
`is_dir`, `create_dir`, `create_dir_all`, `list_dir`, `remove_file`, `remove_dir`,
`remove_dir_all`, `copy_file`, `move`, `metadata`, `size`, `modified_at`, `created_at`.

### Gaps for v1.0

| API | Status | Notes |
|-----|--------|-------|
| `fs.read_bytes(path) -> result<bytes, fs.Error>` | Missing | Binary file reading |
| `fs.write_bytes(path, content) -> result<void, fs.Error>` | Missing | Binary file writing |
| `fs.walk_dir(path) -> result<list<text>, fs.Error>` | Missing | Recursive directory walk |

### Proposal

- `read_bytes`/`write_bytes`: straightforward runtime C helpers, low complexity.
- `walk_dir`: returns relative paths in deterministic text order, does not follow symlinks, fails entirely on any subdirectory error (no partial results).

### Decision

Status: `decided` (2026-04-29)

- Add `read_bytes`, `write_bytes`, `walk_dir` as proposed.
- `walk_dir`: relative paths, deterministic order, no symlinks, fails on any error.

---

## Module 9: Paths — `std.fs.path`

### Current State (189 lines)

Fully implemented: `join`, `normalize`, `is_absolute`, `is_relative`, `absolute`,
`relative`, `base_name`, `name_without_extension`, `extension`, `parent`,
`has_extension`, `change_extension`.

### Gaps for v1.0

| Issue | Current | Required |
|-------|---------|----------|
| `path.extension(path)` returns `text` | Returns `""` for no extension | Should return `optional<text>` |
| `path.parent(path)` returns `text` | Returns `""` for root/no parent | Should return `optional<text>` |
| `/` as canonical separator | Partially done | Windows backslash normalization needed |

### Decision

Status: `decided` (2026-04-29)

- `extension()` → `optional<text>`, `parent()` → `optional<text>`.
- Windows backslash normalization to `/` as canonical separator.

---

## Module 10: Process — `std.os.process`

### Current State (68 lines)

Implemented: `run`, `run_capture` with `ExitStatus`, `CapturedRun`, `Error`.

### Gaps for v1.0

- Non-zero exit as successful `CapturedRun` needs explicit tests.
- Missing program as `process.Error` needs explicit tests.
- v8 defers env, timeout, spawn/wait/kill, pipes, shell mode, binary capture.

### Proposal

Keep current MVP. Add missing tests. No API expansion for v1.0.

### Decision

Status: `decided` (2026-04-29)

- Keep current MVP. Add tests for non-zero exit + missing program error.
- No API expansion for v1.0.

---

## Module 11: Regex — `std.regex`

### Current State

Implemented: `compile`, `is_match`, `find_all` with `Regex` struct and `Error` enum.

### Gaps for v1.0

| API | Status |
|-----|--------|
| `regex.try_first(pattern, input) -> result<optional<text>, regex.Error>` | Missing |
| `regex.try_find_all(pattern, input) -> result<list<text>, regex.Error>` | Missing |
| `regex.try_split(pattern, input) -> result<list<text>, regex.Error>` | Missing |
| `regex.try_replace_all(pattern, input, replacement) -> result<text, regex.Error>` | Missing |

### Proposals

**P11.A — Naming: `try_*` vs direct**

- v8 decided `try_*` prefix for result-returning variants.
- Question: should the existing `find_all` that panics on bad pattern be kept, or replaced?
- Option 1: keep both — `find_all` (convenience, panics on bad pattern) + `try_find_all` (safe).
- Option 2: only `try_*` variants — force explicit error handling.
- Recommendation: Option 1 — matches Zenith's two-layer model (assertive + safe).

**P11.B — Captures/groups for v1.0?**

- v8 deferred captures. Should v1.0 ship without captures?
- Pro of deferring: simpler API surface, avoid underspecifying.
- Con of deferring: regex without captures is limited for real text processing.
- Recommendation: defer captures for v1.0. Basic matching + find + split + replace covers most use cases.

### Decision

Status: `decided` (2026-04-29)

- Option 1: keep `find_all` (panics) + add `try_first`, `try_find_all`, `try_split`, `try_replace_all`.
- Captures deferred to post-v1.

---

## Module 12: Networking — `std.net`

### Current State (63 lines)

Blocking TCP client exists: `connect`, `read_some`, `write_all`, `close`, `is_closed`, `kind`, `to_core_error`.
Public operations return `net.Error` and accept `optional<int>` timeouts. The runtime bridge still returns `core.Error` internally.

### Gaps for v1.0

| Issue | Detail |
|-------|--------|
| Error type | Closed: public API uses `net.Error` |
| `kind()` function | Closed: maps runtime `core.Error` codes to `net.Error` |
| Timeouts | Closed: public API uses `optional<int>`; runtime sentinel stays internal |
| TLS | Not supported |
| DNS resolution | Implicit in `connect` |
| UDP | Not implemented |
| Server/listener | Not implemented |

### Proposals

**P12.A — Blocking-first TCP client for v1.0**

- Keep current blocking model.
- Fix error types to use `net.Error`.
- Fix `kind()` to properly map host errors.
- Change timeout from `-1` magic to `optional<int>`.
- Scope: TCP client only. No server, no UDP, no TLS for v1.0.
- Pro: small, testable, useful for basic tools and ZPM.
- Con: no TLS means no HTTPS — limits real-world HTTP usage.

**P12.B — Include TLS via system libraries**

- Use OS-provided TLS (SChannel on Windows, OpenSSL/LibreSSL on Linux).
- Add `net.connect_tls(host, port, timeout) -> result<Connection, net.Error>`.
- Pro: enables HTTPS for ZPM and real tools.
- Con: complex, platform-specific, certificate validation is hard.
- Con: increases binary size and build complexity.

**P12.C — Defer networking entirely**

- Mark `std.net` as experimental, not v1.0 stable.
- Pro: avoids shipping underspecified networking.
- Con: ZPM registry, examples, and real tools need HTTP.

### Decision

Status: `implemented` (2026-04-30) — **v1 as blocking TCP client**.

- P12.A: blocking TCP client for v1.0.
- Public error types use `net.Error`, timeouts use `optional<int>`, and `kind()` maps runtime bridge errors.
- Scope: TCP client only. No server, no UDP, no TLS for v1.0.
- TLS via `connect_tls` added post-v1.
- async layer added post-v1 on top of blocking APIs.

---

## Module 13: HTTP — `std.http` (NEW)

### Current State

Minimal blocking `std.http` module exists.

- Public API: `http.get`, `http.post`, `http.Response`, `http.Error`, `http.ErrorKind`.
- Transport is HTTP-only. TLS/HTTPS remains post-v1.
- Runtime bridge returns raw HTTP text; `std.http` parses status/body and exposes typed errors.

### Proposal

**P13.A — Minimal blocking HTTP client**

```zt
import std.http as http

const response: http.Response = http.get("https://api.example.com/data")?
print(response.body)
```

API surface:

- `http.get(url) -> result<http.Response, http.Error>`
- `http.post(url, body, content_type) -> result<http.Response, http.Error>`
- `http.Response { status: int, body: text, headers: map<text,text> }`
- `http.Error { kind: http.ErrorKind, message: text }`

- Pro: extremely useful for tools, ZPM, examples.
- Con: depends on `std.net` + TLS being stable.
- Con: HTTP is complex (redirects, chunked encoding, auth).

**P13.B — Defer to post-v1**

- Pro: avoids shipping half-baked HTTP.
- Con: limits practical tooling.

### Decision

Status: `implemented` (2026-04-30) — **v1 as minimal blocking HTTP client**.

- P13.A: minimal blocking HTTP client for v1.0.
- API: `http.get`, `http.post`, `http.Response`, `http.Error`, `http.ErrorKind`.
- No TLS for v1 (HTTP only). TLS + HTTPS added post-v1 with `std.net` TLS.
- No redirects, no chunked encoding, no auth for v1.
- async HTTP added post-v1 when async/await exists.

---

## Module 14: Diagnostics — `std.diagnostic`

### Current State

Runtime diagnostic codes exist. CLI renders formatted strings.
No structured diagnostic data model exposed to Zenith code.

### Gap (v8 DIA.01-06)

All six diagnostic decisions are accepted but not implemented.

### Proposal

Implement the full v8 diagnostic model:
- `diagnostic.Diagnostic` struct with severity/code/title/message/action/why/next.
- `diagnostic.DiagnosticReport` for multi-error collection.
- Value-style helpers: `diagnostic.error(...)`, `diagnostic.with_label(...)`, etc.
- CLI, LSP, and test renderers consuming structured data.

This is high complexity but critical for compiler quality and self-hosting.

### Decision

Status: `decided` (2026-04-29)

- Implement full v8 diagnostic model (DIA.01-06).
- Priority: after stdlib collection work is complete.

---

## Module 15: Compiler & Backend Optimization

### Current State

- C emitter: 457KB, streaming/spill for large modules.
- Symbol sanitization with collision detection.
- Overflow-checked arithmetic (GCC/Clang/MSVC portable).
- Monomorphization for generics.

### Gaps for v1.0

| Area | Issue | Impact |
|------|-------|--------|
| Generic map/set monomorphization | Only specialized types work (map<text,text>, set<int>) | Blocks generic collection APIs |
| HOF monomorphization | Only int-specialized HOFs exist | Blocks `list.map<T,U>`, `list.filter<T>` |
| ARC elision | No retain/release optimization | Performance debt |
| Incremental compilation | Full recompile every time | Compile time for large projects |
| Dead code elimination | Minimal | Binary size |

### Proposals

**P15.A — Generic collection monomorphization (BLOCKER)**

- Must be implemented before generic `list.map`, `map<K,V>`, `set<T>` APIs.
- This is the single most critical compiler gap for v1.0.
- Without it, collection APIs remain type-specialized stubs.

**P15.B — ARC elision (optimization)**

- Defer to post-v1. Current performance is acceptable for alpha.
- Track as performance debt.

**P15.C — Incremental compilation (optimization)**

- Defer to post-v1. Build times are acceptable for current project sizes.

### Decision

Status: `decided` (2026-04-29)

**Bloco A — Runtime Generic Types**: Approved.
- Add `zt_list_generic`, `zt_map_generic`, `zt_set_generic` structs to runtime.
- Each uses `void**` storage + function pointer callbacks for hash/eq/retain/release.
- New heap tags: `ZT_HEAP_LIST_GENERIC`, `ZT_HEAP_MAP_GENERIC`, `ZT_HEAP_SET_GENERIC`.
- Add corresponding cases in `zt_release()` and `zt_deep_copy()`.

**Bloco B — Emitter Wiring**: Approved.
- Emitter generates callback `static` functions **once at the top** of the .c output.
- Per-type callbacks: `__zt_hash_<type>`, `__zt_eq_<type>`, `__zt_retain_<type>`, `__zt_release_<type>`.
- Scalars (int, float, bool) use tagged pointer trick (cast `void* ↔ intptr_t`, zero alloc).
- Managed types (text, bytes, list, map) use `zt_retain`/`zt_release` callbacks.
- `c_resolve_type_mapping` fallback for `map<K,V>`/`set<T>` wires to generic types.

**Bloco C — list<UserStruct>**: Approved.
- Each element is `malloc(sizeof(UserStruct))` + field copy.
- Emitter generates retain/release callbacks that walk struct fields.
- Boxing overhead accepted for v1.0.

**Deferred to post-v1**: ARC elision (P15.B), incremental compilation (P15.C),
dead code elimination.

---

## Module 16: Performance Baseline

### Summary from v8

- Lexer/parser throughput baselines accepted (PERF.01-02).
- Memory/allocation baselines accepted (PERF.03).
- Tiered corpus accepted (PERF.04).
- Regression threshold policy accepted (PERF.05).

### Proposal

Implement the v8 performance decisions as-is. Create `perf/corpus/` with
smoke/standard/stress tiers and manifest.

### Decision

Status: `decided` (2026-04-29)

- Implement v8 perf decisions as-is. Create `perf/corpus/` with tiers.

---

## Module 17: Test Runner Improvements

### Summary from v8

- `attr test` stable (TST.01).
- Golden/snapshot tests accepted (TST.02).
- Snapshot update policy accepted (TST.03).
- Negative test pattern accepted (TST.04).
- Cross-platform temp dirs accepted (TST.05).
- Test output format accepted (TST.06).

### Proposal

Implement the v8 test decisions as-is.

### Decision

Status: `decided` (2026-04-29)

- Implement v8 test decisions as-is (golden/snapshot, negative, cross-platform temp).

---

## Module 18: Documentation Refactor

### Current State

- `docs/public/` has guides, tutorials, cookbook.
- `docs/reference/` has syntax, types, stdlib reference.
- `docs/spec/language/` has normative specs.
- Much content refers to pre-v8 syntax (e.g., `dyn`, `case default`).

### Gaps

1. All docs must use `f"..."` not `fmt`.
2. All docs must use `any<Trait>` not `dyn<Trait>`.
3. All docs must use `case else:` not `case default:`.
4. All docs must use `case some(name):` not `case value name:`.
5. All docs must use `<T: Trait>` not `where T is Trait` for inline constraints.
6. Collection API docs must reflect new explicit APIs.
7. Stdlib reference must be regenerated from updated zdocs.
8. Tutorial ("Learn Zenith in 30 min") must be updated.
9. Cookbook must cover new APIs.

### Proposal

Execute documentation refactor AFTER all API decisions are made and
implemented. Otherwise docs will need multiple rewrites.

### Decision

Status: `decided` (2026-04-29)

- Execute documentation refactor AFTER all APIs are implemented and stable.

---

## Module 19: Runtime Risks

### RC Cycles

- No cycle collector. Documented as accepted limitation.
- Proposal: document which APIs can create cycles. Add `weak<T>` post-v1.

### Thread Safety

- Single-isolate by default. Non-atomic ARC.
- `std.concurrent.copy_*` for boundary transfer.
- Proposal: keep current model for v1.0. Workers/channels are post-v1.

### Decision

Status: `decided` (2026-04-29)

- Document RC cycle limitation. `weak<T>` post-v1.
- **ORC (Ownership-based RC) as future alternative**: Nim's approach adds a
  lightweight cycle collector on top of ARC. Would eliminate the need for
  `weak<T>` entirely. Evaluate post-v1 as potential migration path:
  - Phase 1 (post-v1): `weak<T>` for manual cycle breaking
  - Phase 2 (future): ORC cycle collector if `weak<T>` proves insufficient
  - ORC trade-off: slight runtime overhead for cycle scans vs zero programmer burden
- Keep single-isolate model. Workers/channels post-v1.

---

## Module 20: Future Features (Decide Now)

These must be explicitly decided as `v1` or `post-v1`:

| Feature | Recommendation | Rationale |
|---------|---------------|-----------|
| `async/await` | Post-v1 | Changes language identity; needs design RFC |
| Custom allocators | Post-v1 | No proven use case yet |
| Wildcard imports | Rejected | Against explicit philosophy |
| Relative imports | Post-v1 | Useful but not critical |
| Selective imports | Post-v1 | Useful but current model works |
| Macros | Rejected | Against language philosophy |
| LLVM backend | Post-v1 | C backend is sufficient for v1 |
| WASM backend | Post-v1 | Explore via Emscripten later |
| JS backend | Post-v1 | ZIR->JS is possible but premature |
| Debugger | Post-v1 | GDB/LLDB work on emitted C |
| Overloading | Rejected | Explicit naming is Zenith's answer |
| Type inference `const x = 42` | Post-v1 | Requires inference engine |
| Generic inference `foo(42)` | Post-v1 | Requires unification |

### Decision

Status: `decided` (2026-04-29)

- **Rejected**: wildcard imports, macros, overloading.
- **Post-v1**: async/await, LLVM/WASM/JS backends, debugger, type/generic inference,
  custom allocators, relative imports, selective imports (`import X { a, b }`).
- async/await is HIGH priority post-v1, after std.net + std.http stabilize.

---

## Module 21: Runtime & Compiler Architecture Audit

> This module documents every hardcoded pattern, misallocation, and
> optimization gap found in `zenith_rt.h` (1654 lines), `zenith_rt.c`
> (11871 lines), and `emitter.c` (457KB). These are structural issues
> that affect language scalability and must be resolved before or during v1.0.

### Finding 1: Hardcoded Heap Kind Enum (CRITICAL)

`zt_heap_kind` is a manually maintained enum with **45 tags**. Every new
`list<T>`, `map<K,V>`, or `set<T>` combination requires:

1. A new enum entry in `zenith_rt.h`.
2. A new struct typedef in `zenith_rt.h`.
3. A new `case` in `zt_release()` (40-case switch).
4. A new `case` in `zt_deep_copy()` (similar switch).
5. Full API surface (new, push, get, set, len, slice, free) per type.
6. Emitter knowledge of the new type name.

**Currently hardcoded combinations:**

| Collection | Element types with tags |
|------------|----------------------|
| `list<T>` | i64, text, f64, bool, i8, i16, i32, u8, u16, u32, u64, dyn, dyn_text_repr (13 variants) |
| `map<K,V>` | text→text only (1 variant) |
| `set<T>` | i64, text only (2 variants) |
| `grid2d<T>` | i64, text (2 variants) |
| `grid3d<T>` | i64, text (2 variants) |
| `pqueue<T>` | i64, text (2 variants) |
| `circbuf<T>` | i64, text (2 variants) |
| `btreemap<K,V>` | text→text only (1 variant) |
| `btreeset<T>` | text only (1 variant) |

**What's missing that blocks v1:**

- `map<text, int>`, `map<int, text>`, `map<text, float>`, etc.
- `set<float>`, `set<bool>`, `set<UserStruct>`
- `list<UserStruct>` (no generic list for user-defined types)

**Impact**: Every new collection × element-type combination requires ~200
lines of hand-written C. This model does not scale.

**Proposals:**

- **P21.1A — Generic runtime (type-erased `void*` storage)**
  - Add `ZT_HEAP_LIST_GENERIC`, `ZT_HEAP_MAP_GENERIC`, `ZT_HEAP_SET_GENERIC`.
  - Store elements as `void*` with compiler-emitted cast/retain/release callbacks.
  - Keep existing specialized variants for performance (int, text, float).
  - Pro: one implementation serves all types. Scalable.
  - Con: boxing overhead for non-pointer types (int stored as heap-allocated int).

- **P21.1B — Emitter-generated specializations**
  - Emitter generates `zt_list_<mangled>` structs and functions per instantiation.
  - Pro: zero-overhead, native C performance.
  - Con: massive code bloat, emitter complexity, long compile times.

- **P21.1C — Hybrid (RECOMMENDED)**
  - Primitives (int, float, bool, text, bytes) keep specialized tags.
  - User structs and uncommon combinations use generic `void*` path.
  - Emitter chooses the path based on element type category.
  - Pro: best of both — fast hot paths, scalable cold paths.

### Finding 2: Optional/Result Type Explosion (HIGH)

The runtime manually defines a separate C struct for EVERY combination:

| Pattern | Count | Examples |
|---------|-------|---------|
| `zt_optional_<T>` | 12 structs | `zt_optional_i64`, `zt_optional_text`, `zt_optional_bytes`, `zt_optional_list_i64`, `zt_optional_list_text`, `zt_optional_map_text_text`, plus 6 primitive variants |
| `zt_outcome_<V>_<E>` | 18 structs | `zt_outcome_i64_text`, `zt_outcome_void_text`, `zt_outcome_text_core_error`, `zt_outcome_list_text_core_error`, etc. |

Each `zt_outcome_*` type requires **7-8 functions**: `_success`, `_failure`,
`_failure_message`, `_is_success`, `_value`, `_propagate`, `_dispose`.

**Total: ~30 manually written type combinations × ~7 functions each = ~210 boilerplate functions.**

**Impact**: Adding `result<bytes, fs.Error>` or `result<set<int>, core.Error>`
requires writing yet another full struct + function set in C.

**Proposals:**

- **P21.2A — Generic result/optional via void* + tag**
  - `zt_generic_result { bool is_success; void *value; void *error; uint32_t value_kind; uint32_t error_kind; }`
  - Pro: one struct, one set of functions.
  - Con: always heap-allocates value/error; loses struct-return optimization.

- **P21.2B — C macro templates (RECOMMENDED)**
  - Use C preprocessor macros to generate `zt_outcome_*` structs and functions.
  - Already partially done: `ZT_DECLARE_PRIMITIVE_LIST_API` and
    `ZT_DECLARE_PRIMITIVE_OPTIONAL_API` macros exist in the header.
  - Extend this pattern to outcome types.
  - Pro: eliminates manual copy-paste, keeps value semantics.
  - Con: still generates code per type, but automated.

### Finding 3: Stack vs Heap Misallocation (MEDIUM)

**Currently on heap (via RC) that could be stack-allocated:**

| Type | Size | RC overhead | Should be stack? |
|------|------|-------------|-----------------|
| `zt_optional_i64` | 16 bytes | No header | ✅ Already stack (no RC header) |
| `zt_optional_text` | 16 bytes | No header | ✅ Already stack |
| `zt_outcome_*` | 16-32 bytes | No header | ✅ Already stack |
| `zt_core_error` | 24 bytes | No header | ✅ Already stack |
| `zt_text` (short strings) | 24+ bytes | RC header | ⚠️ Could use SSO |
| `zt_dyn_text_repr` | 48 bytes | RC header | ⚠️ Wastes space (union would be 24) |

**Good news**: optional and result types are already stack-allocated value types
(no `zt_header`). The ARC design is correct for collections and text.

**Issues found:**

1. **`zt_dyn_text_repr`** (L110-117): stores ALL fields simultaneously
   (int_value + float_value + bool_value + text_value) regardless of tag.
   Should use a C union to save 24 bytes per instance.

2. **Short string optimization (SSO)** not implemented: every string,
   even `""` or `"a"`, allocates `zt_text` + separate `data` buffer on heap.
   SSO could inline strings ≤ ~22 bytes into the `zt_text` struct itself.

3. **User structs** are always stack-allocated (value types), which is correct.
   But when stored in a `list<UserStruct>`, there's no mechanism to manage them
   without the generic heap path.

### Finding 4: ARC Inefficiencies (MEDIUM)

1. **No elision**: the emitter inserts `zt_retain(x)` before every use and
   `zt_release(x)` at scope exit. There is no analysis to skip retain/release
   for values that don't escape their scope.

2. **Non-atomic RC**: `header->rc += 1` is a plain increment (L1936).
   This is correct for single-isolate but means the `zt_shared_*` wrappers
   (L677-689) are the ONLY thread-safe path. Documented and intentional.

3. **RC overflow check**: `if (header->rc == UINT32_MAX)` (L1932) —
   good, prevents silent overflow.

4. **`zt_release` switch**: the 40-case switch in `zt_release` (L1959-2066)
   has `O(1)` dispatch (compiler generates jump table) but the function itself
   is called for EVERY scope exit of EVERY managed value. This is the hottest
   path in the runtime.

**Proposals:**

- **P21.4A — Scope-local elision (v1 optimization)**
  - If a managed value is created, used, and destroyed within the same scope,
    skip retain/release entirely. The emitter can detect this pattern.
  - Pro: significant perf improvement for temporary strings and lists.
  - Con: requires escape analysis in the emitter.

- **P21.4B — Defer to post-v1**
  - Current perf is acceptable. Track as known debt.

### Finding 5: Emitter Scalability (MEDIUM)

1. **457KB single-file emitter**: `emitter.c` is one of the largest files.
   Adding generic collection support will grow it further.

2. **Type name string matching**: the emitter uses string comparison on ZIR
   type names (`"list<int>"`, `"map<text,text>"`) to decide which C type
   to emit. Adding new types requires new string match branches.

3. **Struct field retain/release**: `c_emit_retain_for_struct_fields` and
   `c_emit_release_for_struct_fields` (L7738-7878) walk struct fields at
   emit time to generate correct ARC calls. This works but is fragile for
   deeply nested types.

**Proposals:**

- **P21.5A — Type category table**
  - Instead of string matching, build a type-category table at ZIR level
    (scalar/managed/composite/generic) and dispatch on category.
  - Pro: cleaner, extensible.
  - Con: requires refactoring ~100 string-match sites in emitter.

- **P21.5B — Accept current model for v1**
  - The string-matching works. Focus effort on generic collection support.

- **P21.5C - Modularize the C emitter without changing semantics**
  - Split `compiler/targets/c/emitter.c` into focused backend modules while
    keeping `emitter.h` and `c_emitter_emit_module` as the public facade.
  - Proposed first split: buffer/output, C names/mangling, type mapping,
    generated helpers, structured ZIR expressions, cleanup/ARC, and vtables.
  - Pro: lowers merge conflicts, makes regressions easier to isolate, and
    matches how mature compiler backends split codegen responsibilities.
  - Pro: does not require changing Zenith syntax, ZIR, or generated C output.
  - Con: needs careful mechanical moves plus focused golden-output tests.

### Finding 6: Map/Set Coverage Gaps (HIGH)

**`map<K,V>` runtime**: only `zt_map_text_text` exists. The hash map
implementation uses `zt_text_hash()` for keys and `zt_text_eq()` for
comparison. Supporting `map<int, text>` or `map<text, int>` requires:

1. New struct: `zt_map_text_int`, `zt_map_int_text`, etc.
2. New hash function: `zt_i64_hash()` already exists but isn't wired for maps.
3. New comparison: `zt_i64_eq()` exists.
4. Full API: new, set, get, get_optional, contains, key_at, value_at, len.

**`set<T>` runtime**: only `zt_set_i64` and `zt_set_text`. Supporting
`set<float>` requires new hash + comparison for floats (NaN handling!).

**For-in iteration**: map iteration uses `key_at`/`value_at` with index.
This works but is O(n) per access on a hash map (scans for the n-th occupied
slot). Should use a proper iterator or ordered storage.

**Proposals:**

- **P21.6A — Add common map/set variants manually**
  - `map<text,int>`, `map<int,text>`, `map<int,int>`, `set<float>`.
  - Pro: immediate unblock for common use cases.
  - Con: doesn't scale; each new variant is ~300 lines of C.

- **P21.6B — Generic map/set via void* (RECOMMENDED)**
  - Use function pointers for hash and equality: `zt_generic_map { ...; size_t (*hash_fn)(const void*); bool (*eq_fn)(const void*, const void*); }`.
  - Compiler emits the correct hash/eq callbacks per instantiation.
  - Pro: unlimited K,V combinations with one implementation.
  - Con: indirect function call overhead for hash/eq.

### Summary of All Findings

| # | Finding | Severity | Blocks v1? | Recommendation |
|---|---------|----------|-----------|---------------|
| 1 | Hardcoded heap enum (45 tags) | CRITICAL | Yes (generic collections) | Hybrid: specialized + generic |
| 2 | Optional/result type explosion (30+ structs) | HIGH | Partially (new error types) | Extend C macro templates |
| 3 | Stack vs heap (dyn_text_repr, no SSO) | MEDIUM | No | Union for dyn_text_repr; defer SSO |
| 4 | ARC no elision | MEDIUM | No | Defer to post-v1 |
| 5 | Emitter string matching | MEDIUM | No | Accept string matching; modularize file in Tier 2 |
| 6 | Map/set missing variants | HIGH | Yes (map<text,int> etc.) | Generic map/set via void* |

### Decision

Status: `decided` (2026-04-29)

1. **Finding 1 — Opção C (Híbrida)**: manter especializações existentes para
   escalares (int, float, bool, text, bytes); adicionar generic `void*` path
   para user structs e combinações raras. Escalares 64-bit usam tagged pointer
   trick (cast direto `void* ↔ intptr_t`, zero alloc).
2. **Finding 2 — Opção A (Macro Templates)**: estender macros `ZT_DECLARE_*`
   para outcome/result types. Mantém stack-allocated struct returns.
3. **Finding 3 — Aprovado**: refatorar `zt_dyn_text_repr` para usar C union.
   Economia de 24 bytes por instância.
4. **Finding 4 — Post-v1**: ARC elision não bloqueia v1. Rastrear como debt.
5. **Finding 5 - Aceito para v1 com modularizacao**: string matching no emitter
   permanece para v1, mas `emitter.c` deve ser modularizado durante Tier 2.
   Post-v1: implementar type category system
   (SCALAR/MANAGED/COMPOSITE/GENERIC) no ZIR.
6. **Finding 6 — Confirmado**: generic path desbloqueia map<text,int>,
   list<UserStruct>, map<text,list<text>>, etc. `set<UserStruct>` e
   `map<UserStruct, V>` (struct como key) requerem trait `Hashable` —
   bloqueados para v1 com diagnostic claro, preparar `Hashable` post-v1.

---

## Module 22: First-Class Functions Audit

> Audit of the closure/lambda/HOF pipeline to ensure it supports
> generic collection operations (list.map, list.filter, etc.).

### Current State (SOLID)

The full pipeline is implemented end-to-end:

- **Parser**: `func(x: int) => expr` (lambda), `func(params) do...end` (multi-line),
  `(x: int) => expr` (arrow lambda), `func(T) -> R` (callable type syntax).
- **AST**: `ZT_AST_CLOSURE_EXPR` (params, return_type, body, is_lambda),
  `ZT_AST_TYPE_CALLABLE` (params, return_type).
- **HIR→ZIR**: Closure lowering with automatic capture detection.
- **ZIR**: `ZIR_EXPR_MAKE_CLOSURE`, `ZIR_EXPR_FUNC_REF`, `ZIR_EXPR_CALL_INDIRECT`.
- **Runtime**: `zt_closure { header, fn, ctx, drop_ctx }` — ARC-managed,
  `zt_closure_create()` (no captures), `zt_closure_create_with_drop()` (with captures).
- **Emitter**: Closure creation with ARC retain on captures, context struct generation,
  indirect call through `((fn_ptr_type)(closure->fn))(closure->ctx, args)`.
- **Tests**: `test_lambdas.zt`, `lambda_hof_basic/`, `callable_basic.zt`,
  `syntax_coherence_core/`, `lazy_explicit_order_basic/` — all passing.

### Gaps for v1.0

| Gap | Severity | Blocks |
|-----|----------|--------|
| HOFs are int-only (`map_int`, `filter_int`, `reduce_int`) | 🔴 HIGH | Generic HOFs (Module 15 resolves) |
| No closure param type inference | 🟡 MEDIUM | Verbosity (post-v1) |
| No short lambda syntax (`\|x\| x*2` or `_ * 2`) | 🟡 MEDIUM | Style preference (post-v1) |
| `call_indirect` evaluates callable twice (emitter L4477+L4482) | 🟠 HIGH | Correctness bug |
| No currying/partial application | 🟢 LOW | Post-v1 |

### Proposals

**P22.A — Fix call_indirect double evaluation (v1 REQUIRED)**

Current emitter generates:
```c
((fn_ptr_type)(callable->fn))(callable->ctx, args)
//            ^^^^^^^^          ^^^^^^^^ — evaluated twice!
```
If `callable` is a complex expression (e.g., `get_fn()`), this calls `get_fn()` twice.
Fix: extract to temp variable before destructuring.

**P22.B — Closure param type inference (post-v1)**

Allow:
```zt
const doubled = list.map(values, func(x) => x * 2)  -- infer x: int from list<int>
```
Instead of:
```zt
const doubled = list.map(values, func(x: int) => x * 2)
```
Requires type unification engine — significant compiler work.

**P22.C — Short lambda syntax (post-v1 RFC)**

Possible syntaxes:
- `|x| x * 2` (Rust-style)
- `{ x => x * 2 }` (Kotlin-style)
- `_ * 2` (Scala-style placeholder)
- Decision: requires RFC, impacts language identity.

### Decision

Status: `decided` (2026-04-29)

1. **P22.A — Fix call_indirect double eval**: v1 required. Extract to temp var.
2. **P22.B — Param type inference**: post-v1. Requires unification engine.
3. **P22.C — Short lambda syntax**: no change for v1. RFC post-v1 with community.
4. **P22.D — Return type inference**: keep (already works). Optionally explicit with `-> T`.
5. **P22.E — Named function references**: migrate to **static immortal closures**.
   - Emitter generates `static zt_closure` with `rc = UINT32_MAX` per referenced function.
   - Zero allocation, zero ARC overhead for `list.map(values, my_func)` pattern.
   - Closures with captures continue using heap-allocated `zt_closure_create_with_drop`.

---

## Module 23: Dead Code & Redundancy Audit

> Systematic scan of runtime (`zenith_rt.h`, `zenith_rt.c`) and compiler
> (`emitter.c`) for dead code, duplicate entries, and redundant patterns
> that can be safely removed to reduce maintenance burden and binary size.

### Finding D1: Duplicate Type Table Entries (emitter.c)

`C_TYPE_TABLE` has **duplicate entries** that map to the same C type:

```
L772: "outcome<connection,core.error>" → "zt_outcome_net_connection_core_error"
L782: "outcome<net.connection,core.error>" → "zt_outcome_net_connection_core_error"  ← DUPLICATE

L773: "outcome<connection,text>" → "zt_outcome_net_connection_text"
L783: "outcome<net.connection,text>" → "zt_outcome_net_connection_text"  ← DUPLICATE
```

Both `"connection"` and `"net.connection"` map to the same C type. One should be
canonical, the other resolved by the canonicalization layer — not hardcoded twice.

**Action**: Remove `"connection"` entries (keep `"net.connection"` as canonical).
Lines saved: 2 entries in table.

### Finding D2: `outcome<*,text>` Types — Legacy Debt (~800 lines)

The runtime had **full implementations** of `outcome<T, text>` types:
- `zt_outcome_i64_text` (success/failure/propagate/dispose)
- `zt_outcome_void_text`
- `zt_outcome_text_text`
- `zt_outcome_bytes_text` (removed in Tier 3.5)
- `zt_outcome_optional_text_text` (removed in Tier 3.5)
- `zt_outcome_optional_bytes_text` (removed in Tier 3.5)
- `zt_outcome_net_connection_text` (removed in Tier 3.5)
- `zt_outcome_list_i64_text`
- `zt_outcome_list_text_text`
- `zt_outcome_map_text_text`

These use `text` as the error type. But the **canonical error type** in Zenith is
`core.Error`, and Module 7 decided to migrate all stdlib to typed error enums
(`io.Error`, `fs.Error`, `net.Error`).

**Question**: Are `outcome<*,text>` types still used by any stdlib or user code?
If not, they represent ~800 lines of dead runtime code (structs, functions, dispose).

**Resolution for v1**: The audit found active `result<T,text>` coverage, so a total removal would break current language behavior. Tier 3.5 removed the unused variants listed above and kept the active ones until their public APIs migrate.

### Finding D3: `zt_fs_outcome_*_failure_error` Helpers — Boilerplate (6 functions)

`zenith_rt.c` L538-570 has **6 nearly identical helper functions**:
```c
static zt_outcome_void_core_error zt_fs_outcome_void_failure_error(zt_core_error error) { ... }
static zt_outcome_text_core_error zt_fs_outcome_text_failure_error(zt_core_error error) { ... }
static zt_outcome_bool_core_error zt_fs_outcome_bool_failure_error(zt_core_error error) { ... }
static zt_outcome_i64_core_error zt_fs_outcome_i64_failure_error(zt_core_error error) { ... }
static zt_outcome_list_text_core_error zt_fs_outcome_list_text_failure_error(zt_core_error error) { ... }
static zt_outcome_optional_i64_core_error zt_fs_outcome_optional_i64_failure_error(zt_core_error error) { ... }
```

Each does the same thing: `{ result.is_success = 0; result.error = error; return result; }`.

**Action**: Replace with a C macro: `ZT_FS_FAILURE(TYPE, error)`. Saves 30+ lines.
This is exactly the pattern M21.F2 (macro templates) was designed for.

### Finding D4: Legacy Emitter Path — `c_emit_zir_expr_as_legacy` (~3000 lines)

`emitter.c` has a **massive legacy emission system** spanning:
- `c_emit_zir_expr_as_legacy()` (L4586+): ~1000 lines of string-based expression emission
- `c_legacy_expr_resolve_type()` (L1798+): type resolution via string parsing
- `c_legacy_call_return_type()` (L1765+): return type inference from string expressions
- `c_legacy_call_extern_needs_ffi_shield()` (L5438+): FFI shield detection
- `c_emit_ffi_shielded_legacy_call_statement()` (L5766+): FFI shield emission

This is the old text-based emitter that predates the ZIR-based structured emitter.
Multiple ZIR expression kinds **still delegate to it** (L7140-7146, L7198-7199):
```c
case ZIR_EXPR_MAKE_MAP:
case ZIR_EXPR_CALL_RUNTIME_INTRINSIC:
case ZIR_EXPR_SET_FIELD:
case ZIR_EXPR_LIST_PUSH:
case ZIR_EXPR_LIST_SET:
case ZIR_EXPR_MAP_SET:
    return c_emit_zir_expr_as_legacy(...);  // ← still needed
```

**Status**: NOT dead code yet — actively used. But it's technical debt.
**Action**: Post-v1 — migrate remaining expression kinds to structured emitter,
then remove legacy path. Estimated savings: ~3000 lines from emitter.c.

### Finding D5: Specialized Collection Types — Candidates for Generic Path

With the M21 decision to add `zt_list_generic`, `zt_map_generic`, `zt_set_generic`,
some **specialized types become redundant** once generic path is working:

| Type | Lines (approx) | Keep? | Rationale |
|------|---------------|-------|-----------|
| `zt_list_i64` | ~200 | ✅ Keep | Hot path, most used |
| `zt_list_text` | ~200 | ✅ Keep | Hot path, ARC managed |
| `zt_list_f64` | ~200 | ✅ Keep | Hot path |
| `zt_list_bool` | ~200 | ⚠️ Review | Rarely used, generic could handle |
| `zt_list_i8/i16/i32/u8/u16/u32/u64` | ~1400 | ❌ Remove | Macro-generated, generic handles all |
| `zt_list_dyn_text_repr` | ~100 | ❌ Remove | Replaced by `zt_list_dyn` |
| `zt_circbuf_i64/text` | ~400 | ❌ Remove | Not in stdlib, only in `std.collections` |
| `zt_grid2d_i64/text` | ~400 | ❌ Remove | Not in stdlib, only in `std.collections` |
| `zt_grid3d_i64/text` | ~500 | ❌ Remove | Not in stdlib, only in `std.collections` |
| `zt_pqueue_i64/text` | ~400 | ❌ Remove | Not in stdlib, only in `std.collections` |
| `zt_btreemap_text_text` | ~300 | ❌ Remove | Not in stdlib, only in `std.collections` |
| `zt_btreeset_text` | ~200 | ❌ Remove | Not in stdlib, only in `std.collections` |

**Estimated removable**: ~3700 lines from runtime if `circbuf`, `grid2d/3d`,
`pqueue`, `btreemap`, `btreeset` are moved out of core runtime into an optional
`std.collections` runtime extension.

**Important**: `std.collections` stdlib uses these types, so they can't be fully deleted —
they should be **extracted to a separate C file** (`zenith_collections_rt.c`) that is
only linked when `std.collections` is imported. This keeps the core runtime lean.

### Finding D6: `zt_list_dyn_text_repr` vs `zt_list_dyn` — Redundant

Both exist in the runtime:
- `zt_list_dyn_text_repr` — legacy, only for `list<any<TextRepresentable>>`
- `zt_list_dyn` — new, generic `list<any<Trait>>` for any trait

`zt_list_dyn` supersedes `zt_list_dyn_text_repr`. The latter can be removed once
the emitter maps `list<any<textrepresentable>>` to `zt_list_dyn` instead.

**Action**: Remove `zt_list_dyn_text_repr`, use `zt_list_dyn` for all heterogeneous lists.
Saves: ~100 lines runtime + 1 heap tag.

### Finding D7: Emitter `c_type_suggest_closest` — Low Value

`emitter.c` L1095-1142 implements a **Levenshtein distance** function for suggesting
closest type names on error. While nice for UX, it:
- Uses a 32×32 stack matrix (1KB per call)
- Is only called in error paths
- The Levenshtein implementation itself is 50 lines

**Action**: Keep for v1 (low risk, good UX). Mark as optimization candidate
post-v1 (could use a simpler "prefix match" heuristic instead).

### Finding D8: C Emitter Modularization - Architecture Debt

`compiler/targets/c/emitter.c` is now too large to remain a single maintenance
unit. It contains output buffering, C symbol naming, type mapping, legacy
expression emission, structured ZIR expression emission, FFI shielding,
cleanup/ARC emission, generated collection/outcome helpers, vtables, and module
orchestration.

**Action**: v1 Tier 2 should split it into focused files while preserving the
same public API:

- `emitter.c`: module orchestration and public `c_emitter_emit_module` facade.
- `emitter_buffer.c/.h`: string buffer, spill file, write stream/file.
- `emitter_names.c/.h`: sanitization, C symbol mangling, block labels.
- `emitter_types.c/.h`: type canonicalization, C type mapping, generic specs.
- `emitter_expr.c` / `emitter_zir_expr.c`: legacy and structured expression
  emission during the transition.
- `emitter_cleanup.c/.h`: local collection, ARC retain/release, cleanup paths.
- `emitter_closure.c/.h`: closure context structs, constructors, destructors,
  and context unpack helpers.
- `emitter_helpers.c/.h`: generated optional/map/set/list/outcome helpers.
- `emitter_vtable.c/.h`: trait vtable registry and emission.

This is a v1 implementation-quality step, not a language-surface change.

Implementation status (2026-04-30): the v1 mechanical split is complete.
`emitter_buffer.c`, `emitter_names.c`, `emitter_types.c`, and
`emitter_helpers.c` now carry the buffer/output, naming/sanitization, type
canonicalization/mapping, shared generated-helper tracking, list value specs,
map spec construction, generated optional/map/outcome helper-body emission,
generic list/map/set helper-body emission, and generic `zt_elem_ops`
callback-emission slices. `emitter_closure.c` carries closure context emission.
`emitter_vtable.c` carries trait vtable registry and emission.
`emitter.h` remains the public facade; shared backend-only declarations live in
`emitter_internal.h`. Deeper extraction of ZIR expression emission and cleanup
paths is optional post-v1 maintenance, not a Tier 2 blocker.

### Summary

| Finding | Type | Lines Saveable | Priority |
|---------|------|---------------|----------|
| D1 | Duplicate table entries | ~4 lines | v1 quick fix |
| D2 | `outcome<*,text>` types | ~800 lines | v1 (after M7 migration) |
| D3 | FS outcome helpers | ~30 lines | v1 (with M21.F2 macros) |
| D4 | Legacy emitter path | ~3000 lines | Post-v1 |
| D5 | Collection type extraction | ~3700 lines | v1 (extract to separate .c) |
| D6 | `list_dyn_text_repr` redundant | ~100 lines | v1 |
| D7 | Levenshtein suggest | Keep | N/A |
| D8 | C emitter modularization | No direct line removal | v1 Tier 2 |
| **Total** | | **~7630 lines** | |

### Decision

Status: `decided` (2026-04-29)

- **D1**: Remove duplicate type table entries (v1, quick fix).
- **D2**: Remove `outcome<*,text>` types after M7 migration (~800 lines).
- **D3**: Replace FS outcome helpers with `ZT_FS_FAILURE` macro (~30 lines).
- **D4**: Legacy emitter path — post-v1 (~3000 lines).
- **D5**: Extract `circbuf/grid2d/grid3d/pqueue/btreemap/btreeset` to
  `zenith_collections_rt.c` (~3700 lines out of core runtime).
- **D6**: Remove `zt_list_dyn_text_repr`, use `zt_list_dyn` (~100 lines).
- **D7**: Keep Levenshtein suggest (good UX, low risk).
- **D8**: Modularize `compiler/targets/c/emitter.c` during Tier 2, keeping
  `emitter.h` stable and treating the first pass as a mechanical split.

---

## Module 24: Basic Types & Builtins

> Audit of primitive types, builtin functions, type conversions,
> and sub-integer types for v1.0 completeness.

### Primitive Types

| Type | C Runtime | Status |
|------|-----------|--------|
| `int` | `int64_t` | ✅ Complete |
| `float` | `double` | ✅ Complete |
| `bool` | `bool` | ✅ Complete |
| `text` | `zt_text *` (ARC) | ✅ Complete |
| `bytes` | `zt_bytes *` (ARC) | ✅ Complete |
| `void` | — | ✅ Complete |

### Builtins

| Builtin | Decision |
|---------|----------|
| `print(value)` | Accept `any<TextRepresentable>`, auto-convert. Remains `void`. |
| `debug(value)` | Same as print — `any<TextRepresentable>`, writes to stderr. |
| `read()` | Return `optional<text>` (none = EOF). `io.read_line()` for result. |
| `check(cond, msg)` | Keep as sole assertion builtin. |
| `assert(cond, msg)` | **Remove** — redundant with `check`. |
| `type_name(value)` | Keep. |
| `size_of(value)` | **Removed from builtins**; shipped as `std.debug.size_of(value: text)`. |
| `range(start, end)` | v1: optimize `for i in range` to C for-loop (no allocation). |
| `range(start, end, step)` | Same optimization. Lazy iterator post-v1. |
| `len(value)` | Keep. |

### Type Conversions (NEW)

No type conversion functions exist today. Add:

```
int.to_float(value: int) -> float
int.to_text(value: int) -> text
int.parse(value: text) -> optional<int>
float.to_int(value: float) -> int          -- truncates
float.round(value: float) -> int
float.to_text(value: float) -> text
float.parse(value: text) -> optional<float>
bool.to_text(value: bool) -> text
```

Implementation status: done for v1 as `std.int`, `std.float`, and `std.bool` modules. User code imports them with aliases when using the short documented form:

```zt
import std.int as int
import std.float as float
import std.bool as bool
```

### Text Concatenation

`"hello" + " world"` ✅ works — emitter generates `zt_text_concat()`.

### Sub-Integer Types

- **Keep**: `int8`, `uint8` (needed for bytes/binary/FFI).
- **Remove from language surface**: `int16`, `int32`, `uint16`, `uint32`, `uint64`.
- Removed types remain as internal FFI/C interop types via `extern c`.
- Saves ~1000 lines runtime + 5 heap tags.

### Iterator Protocol

- **v1**: optimize `for i in range(start, end)` to C `for` loop (~3 days).
- **Post-v1 (high priority)**: full `Iterable<T>` trait with `next() -> optional<T>`.
  Enables lazy range, lazy map/filter chains, custom iterators.
  Estimated: ~2 weeks. Requires static trait dispatch (monomorphization).

Implementation status: v1 now lowers `for ... in range(...)` to an allocation-free counter loop before C emission. The C output uses the backend's existing labeled-block form, but the old list allocation path is gone for this pattern.

### Manual Memory Management

- **v1**: NO. ARC covers all use cases.
- **Post-v1**: `std.memory` with safe arena/pool allocators.
  Arena auto-frees when scope exits. Zero manual `free()`. Zero UB.

### Decision

Status: `decided` (2026-04-29)

All items approved as documented above.

---

## Module 25: CLI `zt` Commands

### Decision

Status: `decided` (2026-04-29)

Canonical v1 surface:
```
zt create [path]          -- creates new project (default: ./)
zt build [dir]            -- compiles project, produces binary
zt run [dir|file]         -- compiles + runs
zt test [dir]             -- compiles + runs attr test functions
zt check [dir|file]       -- verifies without compiling (exists)
zt version                -- compiler version
zt help                   -- help (exists)
```

- Deferred post-v1: `zt fmt`, `zt doc`
- CLI visual redesign: clean, minimal, aligned with Zenith philosophy

---

## Module 26: CLI `zpm` Commands

### Decision

Status: `decided` (2026-04-29)

Current surface confirmed as canonical:
`init`, `add`, `remove`, `install`, `update`, `list`, `find`, `run`, `publish`, `help`, `version`.

---

## Module 27: Error Message Format

### Decision

Status: `decided` (2026-04-29)

Zenith-clean format adopted:

```
✗ type mismatch [ZT001]

    main.zt:12:5

    12 │ const x: int = "hello"
       │                ─────── expected int, found text

    → try: const x = int.parse("hello")
```

Severity hierarchy:
- `✗` error — does not compile
- `⚠` warning — compiles but has issues
- `ℹ` note — contextual information
- `→ try:` — concrete fix suggestion

---

## Module 28: Project Structure

### Decision

Status: `decided` (2026-04-29)

Simplified structure:
```
my-project/
├── zenith.ztproj          -- project manifest (1:1 convertible with .toml)
├── src/
│   ├── main.zt            -- entry point (configurable)
│   └── utils.zt
└── tests/
    └── test_utils.zt
```

- Keep `.ztproj` extension (language identity) with TOML-compatible format.
- Entry point configurable in ztproj: `entry = "main.zt"` (default).
- `.ztproj` ↔ `.toml` 1:1 conversion supported.

---

## Module 29: Pattern Match Exhaustiveness

### Decision

Status: `decided` (2026-04-29)

- Compiler emits **warning** (not error) if match cases are not exhaustive.
- `case else:` silences the warning.

---

## Module 30: TextRepresentable & `std.format`

### Decision

Status: `decided` (2026-04-29)

**TextRepresentable for all built-in types** (Ruby-style):
- All primitives (`int`, `float`, `bool`), collections (`list`, `map`, `set`),
  `optional`, `result`, `bytes` have auto `to_text()`.
- Structs auto-derive `to_text()` showing all fields: `Player(name: "A", score: 100)`.
- Users can override via `apply TextRepresentable for MyType`.

**`std.format` module** (NEW):
- `format.table(collection)` — renders as ASCII table
- `format.pretty(value)` — indented, multi-line representation
- `format.compact(value)` — single-line compact
- `format.json(value)` — JSON-like representation
- `format.csv(collection)` — CSV representation
- `format.yaml(value)` — YAML-like indented representation

---

## Module 31: Enum Associated Values

### Decision

Status: `decided` (2026-04-29)

Associated values required for v1:
```zt
enum Shape
    Circle(radius: float)
    Rectangle(width: float, height: float)
    Point
end
```

Destructuring in match:
```zt
match shape
    case .Circle(radius): ...
    case .Rectangle(width, height): ...
    case .Point: ...
end
```

Verify end-to-end implementation (parser → emitter).

---

## Module 31b: Tuple Product Types

### Decision

Status: `decided` (2026-04-30)

Tuples are accepted for v1 as fixed-size positional product types.

Canonical syntax:

```zt
tuple<int, text>
("id", 10)
```

Required v1 scope:

- `tuple<T1, T2, ...>` type syntax.
- Tuple literal syntax with at least two values: `(a, b)`.
- Fixed arity and positional typing.
- Tuple values may contain any valid non-`void` type, including structs, enums,
  lists, maps, sets, `optional`, `result`, `any<Trait>`, and function/callable
  values when callable type syntax is stable.
- Field access by positional index, using a syntax still to be finalized before
  implementation (`value.0` is the likely candidate).
- `map.entries(m) -> list<tuple<K, V>>` after tuple support is available.

Out of v1 scope:

- Named tuple fields.
- Variadic tuples.
- Tuple splat/spread.
- Tuple pattern destructuring outside already accepted enum associated-value
  destructuring.
- `group` as a language alias. `group` remains a possible future alias, but
  `tuple` is the contract name.

Rationale:

- `struct` remains the right answer for named domain data and public API models.
- `tuple` fills a different gap: small temporary groupings, multiple returns,
  and key/value entries without forcing a throwaway struct.
- Keeping v1 tuples positional avoids the larger complexity of named structural
  records while preserving the main ergonomic win.

Implementation notes:

- Parser must distinguish tuple literals from parenthesized expressions.
- AST/HIR/ZIR need tuple type and tuple literal nodes.
- Type checker validates arity, per-position type compatibility, and rejects
  `void` positions.
- C backend can lower each canonical tuple instantiation to a generated struct
  with ARC/copy/destroy callbacks when managed fields are present.
- Generic collection fallback must accept tuple element types through generated
  `zt_elem_ops`, like struct elements.

Implementation note:

- Tier 2.14 landed the executable slice for tuple instantiations: tuple
  literals lower to generated C structs with positional fields `item0`,
  `item1`, etc., and `list<tuple<...>>` uses the same generic callback path as
  generated/user structs. Positional tuple field access syntax remains separate
  surface work.

---

## Module 32: Error Handling

### Decision

Status: `decided` (2026-04-29)

v1 error handling model:
- `?` operator for result/optional propagation
- `match` for explicit handling
- `using` for resource cleanup (already specified in language)
- `.or_default()`, `.or_else()` convenience methods on result/optional

No `try/catch/finally`. No `defer` (use `using`).
Future `attempt/rescue` may be explored post-v1 per spec notes.

---

## Module 33: Struct Features

### Decision

Status: `decided` (2026-04-29)

**Default field values**:
```zt
struct Player
    name: text
    health: int = 100
    score: int = 0
end
```

**`with` expression** (creates new copy, no mutation):
```zt
const p2 = p1 with health: 50
-- p1 is unchanged (value semantics, no mut needed)
```

**Methods via `apply`**:
```zt
apply Player
    func is_alive(self) -> bool
        return self.health > 0
    end
end
```

**`where` contracts**: continue working as-is.

---

## Module 34: Naming Conventions

### Decision

Status: `decided` (2026-04-29)

- `snake_case` for variables, functions, parameters — **warning** for violations
- `PascalCase` for types, structs, enums, traits — **warning** for violations
- `UPPER_SNAKE_CASE` for module-level constants — convention, no warning

---

## Module 35: Documentation (ZDocs)

### Decision

Status: `decided` (2026-04-29)

- Code files (`.zt`) stay **clean** — only simple `--` comments.
- Documentation in separate `.zdoc` files with `@func`, `@param`, `@returns`, `@example`.
- `---` triple-dash doc comments allowed in `.zt` for brief inline docs.

---

## Module 36: Attributes

### Decision

Status: `decided` (2026-04-29)

v1 attributes:
- `attr test` — marks test function ✅ (exists)
- `attr deprecated("message")` — deprecation warning (NEW)
- `attr todo("message")` — pending work warning (NEW)
- `attr skip` — skips test (NEW)

Post-v1: `attr inline`, `attr pure`, `attr platform("windows")`.

---

## Module 37: Zenith v1 Philosophy Manifesto

### Decision

Status: `decided` (2026-04-29)

### Identity

**Zenith is a compiled, statically-typed programming language focused on clarity,
productivity, and accessibility.** Designed for games, UI, desktop apps,
automation, and CLI tools.

### Supported Paradigms

- **Procedural** — functions, modules, sequential control flow
- **Functional (lite)** — closures, HOFs, immutable-by-default, value semantics
- **Generic** — parametric polymorphism via `<T>`, trait bounds
- **Trait-based composition** — traits + `apply` instead of inheritance

### Explicitly NOT Supported

- **Object-Oriented Programming** — no classes, no inheritance, no `this`,
  no constructors, no method overloading, no virtual dispatch chains.
  Zenith uses **traits + composition** instead. OOP adds complexity layers
  that conflict with reading-first philosophy.

### Core Principles

**1. Reading-First** — Code is read 10x more than written.
Every syntax choice prioritizes the reader, not the writer.

**2. Explicit over Implicit** — No magic, no hidden behavior. Types are
declared, imports are qualified, conversions are explicit.

**3. One Way to Do It** — One clear path for each problem. One loop style,
one error model, one assertion builtin, one way to define types.

**4. Safe by Default** — No null (`optional<T>`), no exceptions
(`result<T,E>`), no manual free (ARC), no data races (single-isolate),
no silent overflow (checked arithmetic).

**5. Progressive Complexity** — Simple to start (`print("hello")`), deep
when needed (generics, traits, closures, FFI). No cliff of complexity.

**6. Clean Code is the Default** — Blocks with `end`, no braces, no semicolons,
no parentheses in `if`/`while`, readable keywords.

**7. Batteries Included, Modular** — Rich stdlib, but every import is explicit
and qualified. No globals beyond builtins.

**8. Accessible by Design** — Zenith is designed with cognitive accessibility:
- Minimal syntax noise (no `{}`, `()`, `;`)
- Consistent, predictable patterns (one way to do things)
- Clear error messages with concrete suggestions
- Natural language keywords (`func`, `const`, `var`, `struct`, `check`)
- Low cognitive load: what you read is what happens
- Friendly to neurodivergent developers (ADHD, autism, dyslexia):
  reduced visual clutter, consistent structure, explicit flow

### What Zenith is NOT

| Not... | Because... |
|--------|-----------|
| Systems language (Rust/C) | No borrow checker, no manual memory |
| Scripting language (Python/Lua) | Compiled, typed, no heavy runtime |
| Enterprise (Java/C#) | No class hierarchies, no heavyweight frameworks |
| Pure functional (Haskell) | Allows controlled mutation, explicit side effects |
| Minimalist (Go) | Has generics, traits, enums, closures, pattern matching |
| Object-Oriented (C++/Java) | No classes, no inheritance, no OOP patterns |

### Target Audience

1. **Game devs** — want something more robust than Lua/GDScript
2. **Tool builders** — want something clearer than Python/Bash
3. **Desktop app devs** — want something simpler than C++/Rust
4. **Students** — want a clean first compiled language
5. **Neurodivergent developers** — benefit from consistent, low-noise,
   predictable syntax with clear visual structure

### Tagline

*"Zenith — Code that reads like intent."*

---

## Module 38: Language Comparison Guide

### Decision

Status: `decided` (2026-04-29)

Create a comprehensive educational comparison document that shows how Zenith
approaches common programming concepts vs other languages. This is NOT a
"better vs worse" comparison — it demonstrates **how Zenith works** by
contrasting familiar patterns.

> "Not which is better, but how Zenith thinks."

### Comparison Areas

**1. Type Systems**

| Concept | Python | Ruby | JS/TS | Go | Rust | Zig | Java | Swift | Nim | Zenith |
|---------|--------|------|-------|-----|------|-----|------|-------|-----|--------|
| Typing | Dynamic | Dynamic | Dynamic/Static | Static | Static | Static | Static | Static | Static | Static |
| Null | `None` | `nil` | `null`/`undefined` | `nil` | `Option<T>` | `null` | `null` | `Optional` | `Option[T]` | `optional<T>` |
| Errors | Exceptions | Exceptions | Exceptions | `error` return | `Result<T,E>` | `error` return | Exceptions | `throws` | `Result[T]` | `result<T,E>` |
| Generics | Duck typing | Duck typing | TS generics | Yes (1.18+) | Yes | `comptime` | Yes (erasure) | Yes | Yes | Yes (monomorphized) |
| Inference | Full | Full | TS partial | Partial | Full | Full | Partial | Full | Full | Explicit (v1) |

**2. Standard Library Philosophy**

| Aspect | Python | Ruby | JS/TS | Go | Rust | Zig | Swift | Nim | Zenith |
|--------|--------|------|-------|-----|------|-----|-------|-----|--------|
| Import style | `from X import Y` | `require` | `import { X }` | `import "pkg"` | `use crate::mod` | `@import("std")` | `import Module` | `import module` | `import std.X as X` |
| Namespacing | Mix global/qualified | Global | ES modules | Qualified | Qualified | Qualified | Qualified | Qualified | Always qualified |
| Collections | `list`, `dict`, `set` | `Array`, `Hash`, `Set` | `Array`, `Map`, `Set` | slice, map | `Vec`, `HashMap` | ArrayList, HashMap | `Array`, `Dictionary`, `Set` | `seq`, `Table`, `HashSet` | `list<T>`, `map<K,V>`, `set<T>` |
| String type | `str` (unicode) | `String` (mutable) | `string` (utf16) | `string` (utf8) | `String`/`&str` | `[]const u8` | `String` (utf8) | `string` (utf8) | `text` (utf8, ARC) |

**3. The 4 OOP Pillars — Zenith Equivalents**

Zenith does not have OOP, but every concept OOP solves has a Zenith answer:

**Encapsulation** (hiding internal state):
```zt
-- OOP (Java):
-- class Player { private int health; public int getHealth() { return health; } }

-- Zenith: module-level visibility + value semantics
struct Player
    name: text
    health: int = 100
end
-- Fields are public but immutable by default (const).
-- Mutation requires var binding. No getters/setters needed.
```

**Inheritance** (code reuse via parent classes):
```zt
-- OOP (Java):
-- class Animal { void speak() {} }
-- class Dog extends Animal { void speak() { print("woof"); } }

-- Zenith: composition + traits
trait Speaker
    func speak(self) -> text
end

struct Dog
    name: text
end

apply Speaker for Dog
    func speak(self) -> text
        return "woof"
    end
end

-- No diamond problem. No fragile base class. Clear contracts.
```

**Polymorphism** (same interface, different behavior):
```zt
-- OOP (Java):
-- void makeNoise(Animal a) { a.speak(); }

-- Zenith: trait dispatch
func make_noise(speaker: any<Speaker>) -> text
    return speaker.speak()
end

-- Works with Dog, Cat, any type that applies Speaker.
```

**Abstraction** (hiding complexity behind interfaces):
```zt
-- OOP (Java):
-- interface Serializable { byte[] serialize(); }

-- Zenith: traits ARE the abstraction
trait Serializable
    func serialize(self) -> bytes
    func deserialize(data: bytes) -> result<Self, text>
end

-- Same purpose, no class hierarchy, no abstract classes.
```

**4. Memory Management**

| Language | Model | Pros | Cons |
|----------|-------|------|------|
| C | Manual malloc/free | Fast, zero overhead | Unsafe, leaks, UB |
| C++ | RAII + manual | Deterministic | Complex, footguns |
| Zig | Manual + comptime safety | Zero overhead, explicit | Manual, no RAII |
| Rust | Ownership + borrowing | Zero-cost safe | Complex, learning curve |
| Go | Garbage collector | Simple | GC pauses, memory overhead |
| Java | Garbage collector | Simple | GC pauses, no determinism |
| JS/TS | Garbage collector | Simple, automatic | GC pauses, no control |
| Ruby | GC + reference counting | Simple | Slow, GC pauses |
| Python | GC + reference counting | Simple | Slow, GIL |
| **Swift** | **ARC** | **Deterministic, simple** | **Cycles need `weak`/`unowned`** |
| **Nim** | **ORC (ARC + cycle collector)** | **Deterministic, no manual cycle mgmt** | **Slight overhead from cycle scan** |
| **Zenith** | **ARC** | **Deterministic, simple** | **RC cycles (documented, `weak<T>` post-v1)** |

**5. Error Handling**

| Language | Model | Zenith equivalent |
|----------|-------|-------------------|
| Java | `try/catch/finally` | `match result` + `using` |
| Python | `try/except/finally` | `match result` + `using` |
| Ruby | `begin/rescue/ensure` | `match result` + `using` |
| JS/TS | `try/catch/finally` | `match result` + `using` |
| Swift | `do/catch` + `throws` | `result<T,E>` + `?` |
| Nim | `try/except/finally` | `match result` + `using` |
| Go | `if err != nil` | `?` operator |
| Rust | `Result<T,E>` + `?` | `result<T,E>` + `?` |
| Zig | `error` return + `try`/`catch` | `result<T,E>` + `?` |
| C | errno / return codes | `result<T,E>` |

**6. Concurrency** (v1 vs future)

| Language | Model | Zenith v1 | Zenith future |
|----------|-------|-----------|---------------|
| Go | Goroutines + channels | Blocking I/O | Workers + channels |
| Rust | async/await + tokio | Blocking I/O | async/await |
| Zig | Manual threads + async frames | Blocking I/O | async/await |
| Swift | GCD + async/await (5.5+) | Blocking I/O | async/await |
| Nim | Async dispatcher + threads | Blocking I/O | async/await |
| JS/TS | Event loop + async/await | Blocking I/O | async/await |
| Ruby | Threads + Fibers + Ractor | Blocking I/O | Workers + channels |
| Python | asyncio / threads | Blocking I/O | async/await |
| Java | Threads / virtual threads | Blocking I/O | Structured concurrency |

**7. Composition Patterns**

```zt
-- Instead of class inheritance chains:
-- Animal -> Pet -> Dog -> GuideDog

-- Zenith uses flat composition:
struct GuideDog
    name: text
    breed: text
    handler: text
end

apply Speaker for GuideDog
    func speak(self) -> text
        return "woof"
    end
end

apply Trainable for GuideDog
    func train(self, command: text) -> bool
        return true
    end
end

-- Each trait is independent. No fragile hierarchies.
-- Add capabilities by applying more traits, not extending classes.
```

### Output Format

This comparison should be published as:
1. A section in `docs/public/guides/coming-from-other-languages.md`
2. Individual pages: `coming-from-python.md`, `coming-from-ruby.md`,
   `coming-from-javascript.md`, `coming-from-rust.md`, `coming-from-go.md`,
   `coming-from-zig.md`, `coming-from-java.md`
3. Tone: educational, welcoming, never dismissive of other languages.

---

## Module 39: `std.math` Audit

### Decision

Status: `decided` (2026-04-29)

Accept current API (27 functions). Add `abs_int(value: int) -> int`.

---

## Module 40: `std.time` Audit

### Decision

Status: `decided` (2026-04-29)

Accept current API (16 functions). Date formatting stays in `std.format`.

---

## Module 41: `std.json` Expansion

### Decision

Status: `decided` (2026-04-29)

Current `map<text,text>` is insufficient. Implement full JSON support for v1:

```zt
enum JsonValue
    Null
    Bool(value: bool)
    Number(value: float)
    Text(value: text)
    Array(items: list<JsonValue>)
    Object(fields: map<text, JsonValue>)
end

func parse(input: text) -> result<JsonValue, core.Error>
func stringify(value: JsonValue) -> text
func pretty(value: JsonValue, indent: int = 2) -> text
func read(file_path: text) -> result<JsonValue, core.Error>
func write(file_path: text, value: JsonValue) -> result<void, core.Error>

-- Convenience accessors:
func get(value: JsonValue, key: text) -> optional<JsonValue>
func get_text(value: JsonValue, key: text) -> optional<text>
func get_number(value: JsonValue, key: text) -> optional<float>
func get_bool(value: JsonValue, key: text) -> optional<bool>
func get_array(value: JsonValue, key: text) -> optional<list<JsonValue>>
```

Keep backward-compatible `parse_flat` for `map<text,text>` use case.

---

## Module 42: `std.random` Expansion

### Decision

Status: `decided` (2026-04-29)

- Migrate `between` error type from `text` to `core.Error`.
- Add new functions:

```zt
func float_between(min: float, max: float) -> float
func choice(items: list<T>) -> optional<T>
func shuffle(items: list<T>) -> list<T>
```

Implementation note for v1: `choice` and `shuffle` are compiler-lowered stdlib intrinsics for the C backend. The shipped v1 surface supports `list<int>` and `list<text>`, matching the current specialized list runtime. Full user-defined generic function monomorphization remains post-v1.

---

## Module 43: `std.format` Expansion

### Decision

Status: `decided` (2026-04-29)

Existing format functions (number, percent, date, hex, bin) kept.
Add M30 decided functions:

```zt
func table(collection: any<TextRepresentable>) -> text
func pretty(value: any<TextRepresentable>) -> text
func compact(value: any<TextRepresentable>) -> text
func as_json(value: any<TextRepresentable>) -> text
func csv(collection: any<TextRepresentable>) -> text
func yaml(value: any<TextRepresentable>) -> text
```

---

## Module 44: `std.console` Fix

### Decision

Status: `decided` (2026-04-29)

- `pause()` already accepts custom message parameter. Change default
  from Portuguese to English: `"Press Enter to continue..."`.
- Accept current API as canonical (19 functions).

---

## Module 45: Circular Imports

### Decision

Status: `decided` (2026-04-29)

Already implemented. Compiler detects cycles via DFS and emits
`ZT_DIAG_PROJECT_IMPORT_CYCLE` error with refactoring suggestion.
No changes needed.

---

---

## Module 46: New Standard Libraries (v1)

### Decision

Status: `decided` (2026-04-29)

Add the following libraries to v1:
- `std.encoding` — base64 and hex encoding/decoding.
- `std.hash` — sha256, md5 (useful for package managers).
- `std.toml` — internal use for parsing `.ztproj`.
- `std.env` — environment variables access.

Deferred to post-v1: `std.uuid`, `std.csv` (formatting is in `std.format`), `std.log`.

---

## Module 47: Number Literals

### Decision

Status: `decided` (2026-04-29)

Implement missing number literal formats in the lexer for v1:
- `0xFF` (hexadecimal)
- `0b1010` (binary)
- `1_000_000` (underscore separators for readability)

---

## Module 48: Multiline Strings

### Decision

Status: `decided` (2026-04-29)

Adopt **Option A**: Triple-quote `"""` for multiline strings.
```zt
const html = """
    <html>
        <body>Hello</body>
    </html>
"""
```

---

## Module 49: Operator Precedence

### Decision

Status: `decided` (2026-04-29)

Accept current implementation. Precedence (lowest to highest):
1. `or`
2. `and`
3. `==`, `!=`
4. `<`, `<=`, `>`, `>=`
5. `+`, `-`
6. `*`, `/`, `%`
7. Unary (`not`, `-`)
8. Primary

---

## Module 50: Module Visibility

### Decision

Status: `decided` (2026-04-29)

Accept current model. `public` keyword exposes items outside the module. By default, items are private to the module.

---

## Module 51: Type Aliases

### Decision

Status: `decided` (2026-04-29)

Implement `type` aliases for v1 to improve expressiveness:
```zt
type PlayerId = int
type Config = map<text, text>
```

---

## Module 52: Inline If Expression

### Decision

Status: `decided` (2026-04-29)

Ensure support for inline `if` expressions for reading-first variable assignment:
```zt
const status = if age >= 18 then "adult" else "minor"
```

---

## Module 53: For Loop Variants

### Decision

Status: `decided` (2026-04-29)

Accept current parser implementation which supports:
- `for item in list`
- `for item, index in list`
- `for key, value in map`
- `repeat N times`

---

## Module 54: String Escapes

### Decision

Status: `decided` (2026-04-29)

Confirmed existing support for `\n`, `\t`, `\r`, `\\`.
Unicode escapes `\u{XXXX}` and null byte `\0` are deferred to post-v1.

---

## Module 55: Runtime Componentization (Unity Build)

### Current State

- `zenith_rt.c` is a monolithic file of nearly 12,000 lines (412 KB).
- Contains at least 10 completely distinct domains (ARC, Strings, Collections, JSON, Networking, Borealis engine, etc.).
- Even CLI tools that never open a window link the ~2.500 lines of the Borealis raylib engine.

### Gap

- Maintenance is extremely difficult due to file size.
- A change in the JSON parser forces a recompilation of the entire runtime.
- High coupling risk in the future.

### Proposal

- Componentize `zenith_rt.c` into multiple files using the **Unity Build** pattern:
  ```c
  // zenith_rt.c (aggregator)
  #include "zenith_rt_core.c"
  #include "zenith_rt_json.c"
  #include "zenith_rt_net.c"
  #include "zenith_rt_borealis.c"
  // etc.
  ```
- **Pros:** Zero changes to the compiler driver (`pipeline.c`); preserves `static` scoping within the translation unit; enables incremental migration.
- **Priority:**
  1. `zenith_rt_borealis.c` (P0: completely optional domain, largest chunk).
  2. `zenith_rt_net.c` & `zenith_rt_json.c` (P1: self-contained).
  3. `zenith_rt_outcome.c` (P2: pure boilerplate, ~3.200 lines).
  4. Core, Math, Host, etc. (P3).

### Decision

Status: `decided` (2026-04-30)

- Adopt Unity Build componentization.
- Prioritize extracting Borealis, Net, and Json first to quickly reduce monolith size and separate domains.

---

## Suggested Discussion Order

1. **Module 21** — Runtime/compiler architecture audit (FOUNDATIONAL) ✅
2. **Module 15** — Compiler: generic monomorphization (BLOCKER) ✅
3. **Module 1** — `fmt` → `f"..."` migration ✅
4. **Module 5** — `std.text` ✅
5. **Module 2** — `list<T>` complete API + HOFs ✅
6. **Module 22** — First-class functions audit ✅
7. **Module 3** — `map<K,V>` complete API ✅
8. **Module 4** — `set<T>` complete API ✅
9. **Module 6** — `std.bytes` safe APIs ✅
10. **Module 7** — `std.io` error type migration ✅
11. **Module 8** — `std.fs` byte I/O + walk_dir ✅
12. **Module 9** — `std.fs.path` optional returns ✅
13. **Module 10** — `std.os.process` test coverage ✅
14. **Module 11** — `std.regex` try_* APIs ✅
15. **Module 12** — `std.net` blocking TCP client ✅ (v1)
16. **Module 13** — `std.http` blocking client ✅ (v1)
17. **Module 14** — Structured diagnostics ✅
18. **Module 16** — Performance baseline ✅
19. **Module 17** — Test runner improvements ✅
20. **Module 19** — Runtime risk documentation ✅
21. **Module 20** — Future feature decisions ✅
22. **Module 18** — Documentation refactor ✅ (LAST)
23. **Module 23** — Dead code & redundancy audit ✅
24. **Module 24** — Basic types & builtins ✅
25. **Module 25** — CLI `zt` commands ✅
26. **Module 26** — CLI `zpm` commands ✅
27. **Module 27** — Error message format ✅
28. **Module 28** — Project structure ✅
29. **Module 29** — Pattern match exhaustiveness ✅
30. **Module 30** — TextRepresentable & std.format ✅
31. **Module 31** — Enum associated values ✅
32. **Module 32** — Error handling model ✅
33. **Module 33** — Struct features (with, defaults, apply) ✅
34. **Module 34** — Naming conventions ✅
35. **Module 35** — Documentation (ZDocs) ✅
36. **Module 36** — Attributes ✅
37. **Module 37** — Philosophy manifesto ✅
38. **Module 38** — Language comparison guide ✅
39. **Module 39** — std.math audit ✅
40. **Module 40** — std.time audit ✅
41. **Module 41** — std.json expansion ✅
42. **Module 42** — std.random expansion ✅
43. **Module 43** — std.format expansion ✅
44. **Module 44** — std.console fix ✅
45. **Module 45** — Circular imports ✅
46. **Module 46** — New stdlibs ✅
47. **Module 47** — Number literals ✅
48. **Module 48** — Multiline strings ✅
49. **Module 49** — Operator precedence ✅
50. **Module 50** — Module visibility ✅
51. **Module 51** — Type aliases ✅
52. **Module 52** — Inline if expression ✅
53. **Module 53** — For loop variants ✅
54. **Module 54** — String escapes ✅
55. **Module 55** — Runtime Componentization (Unity Build) ✅

---

## Decision Log

| Date | Module | Decision | Notes |
|------|--------|----------|-------|
| 2026-04-29 | M21.F1 | Hybrid collections (Option C) | Specialized scalars + generic void* path |
| 2026-04-29 | M21.F2 | Macro templates (Option A) | Extend ZT_DECLARE_* for outcome types |
| 2026-04-29 | M21.F3 | Union fix approved | dyn_text_repr saves 24 bytes/instance |
| 2026-04-29 | M21.F4 | ARC elision deferred | Post-v1 optimization |
| 2026-04-29 | M21.F5 | String matching accepted | Post-v1: type category system + emitter split |
| 2026-04-29 | M21.F6 | Generic map/set confirmed | set<Struct> blocked until Hashable trait |
| 2026-04-29 | M15.A | Generic runtime types approved | zt_list/map/set_generic with void** + callbacks |
| 2026-04-29 | M15.B | Emitter callbacks at top of .c | Static fns for hash/eq/retain/release per type |
| 2026-04-29 | M15.C | list<UserStruct> boxes elements | malloc per element, accepted overhead |
| 2026-04-29 | M15.D | ARC elision + incremental deferred | Post-v1 optimization |
| 2026-04-29 | M01 | fmt→f migration complete | Deprecation exists, all .zt migrated |
| 2026-04-29 | M05 | std.text API decided | split, to_lower/upper, replace rename, join(sep), pad, chars, get, slice |
| 2026-04-29 | M02 | list<T> API decided | Value-style, HOFs, optional<int> index_of, result sort_by, reduce, flat_map |
| 2026-04-29 | M22.A | Fix call_indirect double eval | v1 required: extract to temp var |
| 2026-04-29 | M22.E | Static immortal closures | Named fn refs: zero-alloc static zt_closure with rc=UINT32_MAX |
| 2026-04-29 | M22.BCD | Inference + syntax deferred | Param inference, short lambda, currying all post-v1 |
| 2026-04-29 | M03 | map<K,V> API decided | set/remove/merge(right-wins), HOFs, map[k]=v sugar, for k,v iteration |
| 2026-04-29 | M04 | set<T> API decided | symmetric_diff, to/from_list, HOFs, set.map deferred, literal constructors |
| 2026-04-29 | M06 | std.bytes API decided | get/slice/index_of, join→concat rename, from_list with result, no HOFs |
| 2026-04-29 | M07 | std.io error migration | core.Error → io.Error, add to_core_error() |
| 2026-04-29 | M08 | std.fs additions | read_bytes, write_bytes, walk_dir |
| 2026-04-29 | M09 | std.fs.path optionals | extension/parent → optional<text>, backslash normalization |
| 2026-04-29 | M10 | std.os.process MVP kept | Add tests only, no API expansion |
| 2026-04-29 | M11 | std.regex try_* APIs | Keep find_all + add try_* variants, captures post-v1 |
| 2026-04-29 | M12 | std.net v1 blocking | TCP client blocking, net.Error, optional timeout |
| 2026-04-29 | M13 | std.http v1 blocking | Minimal HTTP client, get/post, no TLS/redirects |
| 2026-04-29 | M14 | Diagnostics approved | Full v8 DIA.01-06 model, after stdlib |
| 2026-04-29 | M16 | Perf baseline approved | perf/corpus with tiers, threshold policy |
| 2026-04-29 | M17 | Test runner approved | Golden/snapshot, negative, cross-platform |
| 2026-04-29 | M18 | Docs refactor last | After all APIs stable |
| 2026-04-29 | M19 | Runtime risks documented | RC cycles accepted, weak<T> post-v1, ORC as future alternative |
| 2026-04-29 | M20 | Future features decided | Wildcards/macros/overload rejected; async/inference post-v1 |
| 2026-04-29 | M23 | Dead code audit decided | ~7630 lines removable: D1-D6 v1, D4 post-v1, D7 keep |
| 2026-04-29 | M24 | Basic types & builtins | print/debug accept any<TR>, read→optional, remove assert/size_of, type conversions, remove sub-int surface types, range C-loop opt, iterator post-v1 |
| 2026-04-29 | M25 | CLI zt commands | create, build, run, test, check, version, help; fmt/doc post-v1 |
| 2026-04-29 | M26 | CLI zpm confirmed | Current surface canonical: init, add, remove, install, update, list, find, run, publish |
| 2026-04-29 | M27 | Error format Zenith-clean | ✗/⚠/ℹ severity, → try: suggestions, note support |
| 2026-04-29 | M28 | Project structure simplified | .ztproj kept (1:1 toml), src/ flat, configurable entry |
| 2026-04-29 | M29 | Match exhaustiveness warning | Warning for missing cases, case else silences |
| 2026-04-29 | M30 | TextRepresentable + std.format | Auto-derive for all types, table/pretty/compact/json/csv/yaml formatters |
| 2026-04-29 | M31 | Enum associated values | Required for v1, verify end-to-end |
| 2026-04-29 | M32 | Error handling: ? + match + using | No try/catch, using for cleanup, .or_default()/.or_else() |
| 2026-04-29 | M33 | Struct: with, defaults, apply | with creates copy (no mut conflict), default field values, apply for methods |
| 2026-04-29 | M34 | Naming conventions | snake_case vars/fns, PascalCase types, warnings for violations |
| 2026-04-29 | M35 | ZDocs philosophy | .zdoc for detailed docs, --- for inline, clean .zt code |
| 2026-04-29 | M36 | Attributes v1 | test, deprecated(msg), todo(msg), skip; inline/pure/platform post-v1 |
| 2026-04-29 | M37 | Philosophy manifesto | Reading-first, accessible, no OOP, trait composition, neurodivergent-friendly |
| 2026-04-29 | M38 | Language comparison guide | Educational, OOP pillars→traits, memory/errors/concurrency comparisons |
| 2026-04-29 | M39 | std.math accepted | Add abs_int, rest complete |
| 2026-04-29 | M40 | std.time accepted | Complete as-is |
| 2026-04-29 | M41 | std.json v1 full | JsonValue enum, parse/stringify/get accessors, nested support |
| 2026-04-29 | M42 | std.random expanded | float_between, choice, shuffle, error migration |
| 2026-04-29 | M43 | std.format expanded | table, pretty, compact, as_json, csv, yaml |
| 2026-04-29 | M44 | std.console fixed | Default message to English |
| 2026-04-29 | M45 | Circular imports confirmed | Already detected with ZT_DIAG_PROJECT_IMPORT_CYCLE |
| 2026-04-29 | M46 | New stdlibs | Implement std.encoding, std.hash, std.toml, std.env |
| 2026-04-29 | M47 | Number literals | Implement 0xFF, 0b1010, 1_000_000 for v1 |
| 2026-04-29 | M48 | Multiline strings | Option A: triple-quotes `"""` |
| 2026-04-29 | M49 | Operator precedence | Math-standard precedence confirmed |
| 2026-04-29 | M50 | Module visibility | Default private, `public func` exposed confirmed |
| 2026-04-29 | M51 | Type aliases | Implement `type Alias = T` |
| 2026-04-29 | M52 | Inline if | Implement `const x = if cond then a else b` |
| 2026-04-29 | M53 | For loop variants | `for k, v in map` and `repeat N times` confirmed |
| 2026-04-29 | M54 | String escapes | \n \t \r \\ confirmed; \u and \0 post-v1 |
| 2026-04-30 | M55 | Runtime Componentization | Unity Build pattern: extract Borealis, Net, Json, Outcome |

