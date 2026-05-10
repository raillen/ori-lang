# Zenith Standard Library Reference By Topic

> Audience: Zenith users, maintainers, docs authors, tooling authors
> Status: topical stdlib reference
> Source of truth: current `stdlib/std/**/*.zt`, `stdlib/zdoc/std/*.zdoc`, `stdlib-model.md`, and compiler-known stdlib rules
> Last updated: 2026-05-04

This document lists Zenith standard library modules, public types, functions, helpers, constants, and observable namespace state.

It complements `syntax-semantics-by-topic.md`. That file documents language syntax and semantics; this file documents the library surface built on top of the language.

When this file conflicts with the final language contract or source files, precedence is:

1. `final-language-contract.md`
2. current `stdlib/std/**/*.zt` and compiler-known stdlib rules
3. `stdlib-model.md`
4. this topical reference
5. older generated or historical docs

---

## 1. Stdlib Principles

### Import Style

Stdlib modules are imported explicitly:

```zt
import std.io as io
import std.text as text
import std.fs as fs
```

Use qualified names from the import alias:

```zt
const line: result<optional<text>, io.Error> = io.read_line()
```

### Error And Absence Policy

- Expected absence returns `optional<T>`.
- Expected failure returns `result<T, E>`.
- Side-effecting modules should own a module-specific error type when useful.
- Direct invalid operations may panic when the language contract says the operation is strict.
- Safe lookup helpers should avoid panic and return `optional<T>` or `result<T, E>`.

### Public State Policy

- Prefer `public const`.
- Use `public var` only when observable module state is part of the contract.
- External users may read public namespace state through a qualified import.
- External users must mutate state through explicit public functions, not direct writes.

### Layer Ownership

- `core` is implicit language/runtime support, not ordinary public stdlib.
- `std.*` is public library surface.
- `platform` is internal host/target adaptation and is not public app API.

---

## 2. Module Index

| Area | Modules |
|---|---|
| Primitives and conversion | `std.bool`, `std.int`, `std.float`, `std.text`, `std.bytes` |
| Collections | `std.list`, `std.map`, `std.set`, `std.collections` |
| Formatting and data | `std.format`, `std.json`, `std.encoding`, `std.hash`, `std.regex` |
| System IO | `std.io`, `std.console`, `std.fs`, `std.fs.path`, `std.os`, `std.os.process` |
| Time, math, random | `std.time`, `std.math`, `std.random` |
| Validation and tests | `std.validate`, `std.test` |
| Lazy and concurrency | `std.lazy`, `std.concurrent`, `std.jobs`, `std.channels` |
| Advanced runtime and memory | `std.shared`, `std.atomic`, `std.orc`, `std.mem`, `std.unsafe` |
| Network | `std.net`, `std.http` |

---

## 3. `std.bool`

Purpose: boolean conversion helpers.

### Functions

| Function | Semantics |
|---|---|
| `to_text(value: bool) -> text` | Converts a boolean to text. |

---

## 4. `std.int`

Purpose: integer conversion and parsing helpers.

### Functions

| Function | Semantics |
|---|---|
| `to_float(value: int) -> float` | Converts an integer to float. |
| `to_text(value: int) -> text` | Converts an integer to text. |
| `parse(value: text) -> optional<int>` | Parses text as an integer; returns `none` on invalid input. |

---

## 5. `std.float`

Purpose: floating-point conversion, rounding, and parsing helpers.

### Functions

| Function | Semantics |
|---|---|
| `to_int(value: float) -> int` | Converts a float to integer according to runtime conversion rules. |
| `round(value: float) -> int` | Rounds a float to an integer. |
| `to_text(value: float) -> text` | Converts a float to text. |
| `parse(value: text) -> optional<float>` | Parses text as a float; returns `none` on invalid input. |

---

## 6. `std.text`

Purpose: text conversion, slicing, searching, casing, joining, replacement, padding, and predicates.

### Conversion Functions

| Function | Semantics |
|---|---|
| `to_utf8(value: text) -> bytes` | Encodes text as UTF-8 bytes. |
| `from_utf8(value: bytes) -> result<text, text>` | Decodes UTF-8 bytes; returns text error payload on invalid bytes. |

### Trimming And Slicing

| Function | Semantics |
|---|---|
| `trim(value: text) -> text` | Removes leading and trailing whitespace. |
| `trim_start(value: text) -> text` | Removes leading whitespace. |
| `trim_end(value: text) -> text` | Removes trailing whitespace. |
| `get(value: text, index: int) -> optional<text>` | Safe character lookup by index. |
| `slice(value: text, start: int, finish: int) -> text` | Returns a clamped inclusive slice. |
| `limit(value: text, max_len: int) -> text` | Truncates text to at most `max_len` characters. |

### Search And Predicate Functions

| Function | Semantics |
|---|---|
| `contains(value: text, needle: text) -> bool` | Checks whether `needle` appears in `value`. |
| `starts_with(value: text, prefix: text) -> bool` | Checks prefix. |
| `ends_with(value: text, suffix: text) -> bool` | Checks suffix. |
| `has_prefix(value: text, prefix: text) -> bool` | Alias for `starts_with`. |
| `has_suffix(value: text, suffix: text) -> bool` | Alias for `ends_with`. |
| `has_whitespace(value: text) -> bool` | Checks whether text contains whitespace. |
| `index_of(value: text, needle: text) -> optional<int>` | Returns first index, or `none` when absent. |
| `last_index_of(value: text, needle: text) -> optional<int>` | Returns last index, or `none` when absent. |
| `index_of_or_minus_one(value: text, needle: text) -> int` | Compatibility helper for sentinel-based code. |
| `last_index_of_or_minus_one(value: text, needle: text) -> int` | Compatibility helper for sentinel-based code. |
| `is_empty(value: text) -> bool` | Checks `len(value) == 0`. |
| `is_blank(value: text) -> bool` | Checks whether trimmed text is empty. |
| `is_digits(value: text) -> bool` | Checks whether all characters are ASCII digits and text is non-empty. |

### Transform And Composition Functions

| Function | Semantics |
|---|---|
| `concat(left: text, right: text) -> text` | Concatenates two text values. |
| `split(value: text, separator: text) -> list<text>` | Splits text by separator. |
| `chars(value: text) -> list<text>` | Returns a list of text characters. |
| `to_lower(value: text) -> text` | ASCII lowercase conversion. |
| `to_upper(value: text) -> text` | ASCII uppercase conversion. |
| `repeat_text(value: text, count: int) -> text` | Repeats text `count` times. |
| `pad_left(value: text, width: int, fill: text) -> text` | Left-pads text. |
| `pad_right(value: text, width: int, fill: text) -> text` | Right-pads text. |
| `capitalize(value: text) -> text` | ASCII capitalization. |
| `join(parts: list<text>, separator: text = "") -> text` | Joins text parts with a separator. |
| `replace_all(value: text, needle: text, replacement: text) -> text` | Replaces every occurrence. |
| `replace(value: text, needle: text, replacement: text) -> text` | Alias for `replace_all`. |

### Public Implementation Helpers

The current module also exposes underscore-prefixed helpers used by stdlib code:

| Helper | Semantics |
|---|---|
| `_eq(a: text, b: text) -> bool` | Runtime text equality wrapper. |
| `_slice_from(value: text, start: int) -> text` | Internal clamped suffix helper. |
| `_slice_to(value: text, finish: int) -> text` | Internal clamped prefix helper. |
| `_is_whitespace_char(ch: text) -> bool` | Internal whitespace predicate. |
| `_is_ascii_digit_char(ch: text) -> bool` | Internal digit predicate. |
| `_starts_at(value: text, needle: text, start: int) -> bool` | Internal substring-at predicate. |

These helpers are public in the current files but are not the preferred teaching surface for application code.

---

## 7. `std.bytes`

Purpose: byte-buffer construction, conversion, concatenation, search, slicing, and predicates.

### Functions

| Function | Semantics |
|---|---|
| `empty() -> bytes` | Returns empty bytes. |
| `from_list(values: list<int>) -> result<bytes, core.Error>` | Converts integer byte values into bytes; invalid values fail. |
| `to_list(value: bytes) -> list<int>` | Converts bytes to a list of integer byte values. |
| `join(left: bytes, right: bytes) -> bytes` | Concatenates byte buffers. |
| `concat(left: bytes, right: bytes) -> bytes` | Alias for `join`. |
| `starts_with(value: bytes, prefix: bytes) -> bool` | Checks prefix. |
| `ends_with(value: bytes, suffix: bytes) -> bool` | Checks suffix. |
| `contains(value: bytes, part: bytes) -> bool` | Checks whether byte sequence appears. |
| `get(value: bytes, index: int) -> optional<int>` | Safe byte lookup by index. |
| `slice(value: bytes, start: int, finish: int) -> bytes` | Returns a clamped byte slice. |
| `index_of(value: bytes, part: bytes) -> optional<int>` | Finds a byte sequence. |
| `len(value: bytes) -> int` | Returns byte length. |
| `is_empty(value: bytes) -> bool` | Checks byte length zero. |

---

## 8. `std.list`

Purpose: compiler-known helpers for built-in `list<T>` values.

The module file is intentionally minimal because these helpers are compiler-known for the built-in generic collection.

Backend support note: the C backend currently executes the basic value helpers for primitive lists and `list<text>`. Selected `list<any<Trait>>` operations are covered separately by the trait-object runtime. Managed struct, enum, and fully generic managed-list helper coverage remain future work.

### Compiler-Known Helpers

| Helper | Semantics |
|---|---|
| `list.is_empty(items)` | Returns `true` when the list has no items. |
| `list.first(items)` | Returns `optional<T>` for the first item. |
| `list.last(items)` | Returns `optional<T>` for the last item. |
| `list.rest(items)` | Returns a new list without the first item. |
| `list.skip(items, count)` | Returns a new list after skipping `count` items. |
| `list.append(items, value)` | Returns a new list with `value` at the end. |
| `list.prepend(items, value)` | Returns a new list with `value` at the start. |
| `list.contains(items, value)` | Returns whether the list contains `value`. |
| `list.reverse(items)` | Returns a new list in reverse order. |
| `list.concat(left, right)` | Returns a new list with both inputs. |
| `list.index_of(items, value)` | Returns `optional<int>` with the first index. |
| `list.set(items, index, value)` | Returns `result<list<T>, core.Error>`. |
| `list.remove_first(items)` | Returns `result<list<T>, core.Error>`. |
| `list.remove_last(items)` | Returns `result<list<T>, core.Error>`. |
| `list.remove_at(items, index)` | Returns `result<list<T>, core.Error>`. |
| `list.slice(items, start, end)` | Returns `result<list<T>, core.Error>`. |
| `list.map(items, mapper)` | Maps list items using a callable mapper. |
| `list.filter(items, predicate)` | Filters list items using a predicate. |
| `list.reduce(items, initial, reducer)` | Reduces primitive/text list items into a same-type accumulator. |
| `list.find(items, predicate)` | Returns the first matching item as `optional<T>`. |
| `list.any(items, predicate)` | Returns whether any item matches. |
| `list.all(items, predicate)` | Returns whether all items match. |
| `list.count(items, predicate)` | Counts matching items. |

### Current Backend Notes

- Basic helpers execute for `list<int>` and `list<text>` in the current C backend subset.
- Higher-order helpers currently execute for important primitive/text subsets.
- `list.map<T,T>` and `list.reduce<T,T>` execute for primitive/text lists.
  Cross-type `map<T,U>` and `reduce<T,U>` remain future implementation work
  and are rejected with explicit post-RC diagnostics.

---

## 9. `std.map`

Purpose: compiler-known helpers for built-in `map<K, V>` values.

### Compiler-Known Helpers

| Helper | Semantics |
|---|---|
| `map.is_empty(values)` | Returns `true` when the map has no entries. |
| `map.get(values, key)` | Safe lookup; returns `optional<V>` without panicking. |
| `map.set(values, key, value)` | Returns a new map with key set. |
| `map.remove(values, key)` | Returns a new map without key. |
| `map.contains(values, key)` | Returns whether key exists. |
| `map.has_key(values, key)` | Alias for key existence. |
| `map.keys(values)` | Returns a list of keys. |
| `map.values(values)` | Returns a list of values. |
| `map.merge(left, right)` | Returns merged map; right-side values win. |

### Current Backend Notes

- `map.get` and `map.contains` follow generic map lowering.
- `map.set`, `map.remove`, `map.keys`, `map.values`, and `map.merge` are implemented for generated maps where the key is `int` or `text` and the value is a primitive value or `text`.
- Other key shapes remain outside the executable C backend subset until equality/hash support is widened.

---

## 10. `std.set`

Purpose: compiler-known helpers for built-in `set<T>` values.

### Compiler-Known Helpers

| Helper | Semantics |
|---|---|
| `set.is_empty(values)` | Returns `true` when the set has no elements. |
| `set.len(values)` | Returns element count. |
| `set.add(values, value)` | Returns a set with value included. |
| `set.remove(values, value)` | Returns a set without value. |
| `set.has(values, value)` | Returns whether value exists. |
| `set.union(left, right)` | Returns set union. |
| `set.intersect(left, right)` | Returns set intersection. |
| `set.difference(left, right)` | Returns left-minus-right difference. |

### Current Backend Notes

- Runtime lowering currently targets generated set helpers for supported element types.
- Fallback runtime names exist for integer sets in the current backend.

---

## 11. `std.collections`

Purpose: advanced collection helpers and specialized structures. This module is broader than the ordinary `list`, `map`, and `set` helpers.

Current v1 support matrix:

| Structure | Supported shapes |
|---|---|
| Queue and stack | `list<int>`, `list<text>`, plus compiler-known `queue_values<T>` and `stack_values<T>` for list-backed snapshots |
| Grid2D and Grid3D | `grid2d<int>`, `grid2d<text>`, `grid3d<int>`, `grid3d<text>` |
| Priority queue | `pqueue<int>`, `pqueue<text>` |
| Circular buffer | `circbuf<int>`, `circbuf<text>` |
| BTree map | `btreemap<text,text>` |
| BTree set | `btreeset<text>` |
| Higher-order helpers | `map_int`, `filter_int`, `reduce_int`, plus same-type `std.list` HOFs for primitive/text lists |

Unsupported shapes such as `grid2d<bool>`, `circbuf<float>`, `pqueue<User>`,
`btreemap<text,int>`, and `btreeset<int>` are outside the v1 runtime subset.
General `grid2d<T>`, `circbuf<T>`, `pqueue<T>`, `btreemap<K,V>`, and
`btreeset<T>` support is post-RC technical debt.

Iterable snapshot policy:

- Queue and stack are list-backed. `collections.queue_values<T>` and `collections.stack_values<T>` are compiler-known helpers that return their `list<T>` snapshot.
- Grid snapshots use stable dimensional order: Grid2D is row-major; Grid3D is layer, then row, then column.
- Priority queue snapshots use pop order and do not mutate the source heap.
- Circular buffer snapshots are oldest to newest.
- BTree map and set snapshots are sorted by text key/value order in the current specialized runtime.
- Priority queues, BTree maps, and BTree sets require an ordering relation. The current runtime exposes that for `int`/`text` priority queues and text B-tree structures.

### Public Result Types

| Type | Fields | Semantics |
|---|---|---|
| `QueueNumberDequeueResult` | `queue: list<int>`, `value: optional<int>` | Queue dequeue result for integer queue helpers. |
| `QueueTextDequeueResult` | `queue: list<text>`, `value: optional<text>` | Queue dequeue result for text queue helpers. |
| `StackNumberPopResult` | `stack: list<int>`, `value: optional<int>` | Stack pop result for integer stack helpers. |
| `StackTextPopResult` | `stack: list<text>`, `value: optional<text>` | Stack pop result for text stack helpers. |

### Queue Helpers

| Function | Semantics |
|---|---|
| `queue_int_new() -> list<int>` | Creates integer queue. |
| `queue_int_enqueue(queue: list<int>, value: int) -> list<int>` | Enqueues integer. |
| `queue_int_dequeue(queue: list<int>) -> collections.QueueNumberDequeueResult` | Dequeues integer with remaining queue. |
| `queue_int_peek(queue: list<int>) -> optional<int>` | Peeks next integer. |
| `queue_text_new() -> list<text>` | Creates text queue. |
| `queue_text_enqueue(queue: list<text>, value: text) -> list<text>` | Enqueues text. |
| `queue_text_dequeue(queue: list<text>) -> collections.QueueTextDequeueResult` | Dequeues text with remaining queue. |
| `queue_text_peek(queue: list<text>) -> optional<text>` | Peeks next text. |
| `queue_values<T>(queue: list<T>) -> list<T>` | Compiler-known snapshot in front-to-back order. |

### Same-Type Higher-Order List Helpers

| Function | Semantics |
|---|---|
| `map_int(values: list<int>, mapper: func(int) -> int) -> list<int>` | Maps integer list. |
| `filter_int(values: list<int>, predicate: func(int) -> bool) -> list<int>` | Filters integer list. |
| `reduce_int(values: list<int>, initial: int, reducer: func(int, int) -> int) -> int` | Reduces integer list. |

### Stack Helpers

| Function | Semantics |
|---|---|
| `stack_int_new() -> list<int>` | Creates integer stack. |
| `stack_int_push(stack: list<int>, value: int) -> list<int>` | Pushes integer. |
| `stack_int_pop(stack: list<int>) -> collections.StackNumberPopResult` | Pops integer with remaining stack. |
| `stack_int_peek(stack: list<int>) -> optional<int>` | Peeks integer. |
| `stack_text_new() -> list<text>` | Creates text stack. |
| `stack_text_push(stack: list<text>, value: text) -> list<text>` | Pushes text. |
| `stack_text_pop(stack: list<text>) -> collections.StackTextPopResult` | Pops text with remaining stack. |
| `stack_text_peek(stack: list<text>) -> optional<text>` | Peeks text. |
| `stack_values<T>(stack: list<T>) -> list<T>` | Compiler-known snapshot in bottom-to-top storage order. |

### Grid Helpers

Unsupported advanced collection shapes fail during `zt check`, not during C
compilation. Nested managed payloads in advanced materialized containers, such
as `grid2d<list<text>>` or `circbuf<map<text, text>>`, are also post-RC work.

| Function | Semantics |
|---|---|
| `grid2d_int_new(rows: int, cols: int) -> grid2d<int>` | Creates integer 2D grid. |
| `grid2d_int_get(grid: grid2d<int>, row: int, col: int) -> int` | Reads integer cell. |
| `grid2d_int_set(grid: grid2d<int>, row: int, col: int, value: int) -> grid2d<int>` | Returns grid with integer cell set. |
| `grid2d_int_fill(grid: grid2d<int>, value: int) -> grid2d<int>` | Fills integer grid. |
| `grid2d_int_rows(grid: grid2d<int>) -> int` | Returns row count. |
| `grid2d_int_cols(grid: grid2d<int>) -> int` | Returns column count. |
| `grid2d_int_size(grid: grid2d<int>) -> int` | Returns rows times columns. |
| `grid2d_int_values(grid: grid2d<int>) -> list<int>` | Returns row-major cell values. |
| `grid2d_text_new(rows: int, cols: int) -> grid2d<text>` | Creates text 2D grid. |
| `grid2d_text_get(grid: grid2d<text>, row: int, col: int) -> text` | Reads text cell. |
| `grid2d_text_set(grid: grid2d<text>, row: int, col: int, value: text) -> grid2d<text>` | Returns grid with text cell set. |
| `grid2d_text_fill(grid: grid2d<text>, value: text) -> grid2d<text>` | Fills text grid. |
| `grid2d_text_rows(grid: grid2d<text>) -> int` | Returns row count. |
| `grid2d_text_cols(grid: grid2d<text>) -> int` | Returns column count. |
| `grid2d_text_size(grid: grid2d<text>) -> int` | Returns rows times columns. |
| `grid2d_text_values(grid: grid2d<text>) -> list<text>` | Returns row-major cell values. |
| `grid3d_int_new(depth: int, rows: int, cols: int) -> grid3d<int>` | Creates integer 3D grid. |
| `grid3d_int_get(grid: grid3d<int>, layer: int, row: int, col: int) -> int` | Reads integer 3D cell. |
| `grid3d_int_set(grid: grid3d<int>, layer: int, row: int, col: int, value: int) -> grid3d<int>` | Returns grid with integer 3D cell set. |
| `grid3d_int_fill(grid: grid3d<int>, value: int) -> grid3d<int>` | Fills integer 3D grid. |
| `grid3d_int_depth(grid: grid3d<int>) -> int` | Returns depth. |
| `grid3d_int_rows(grid: grid3d<int>) -> int` | Returns rows. |
| `grid3d_int_cols(grid: grid3d<int>) -> int` | Returns columns. |
| `grid3d_int_size(grid: grid3d<int>) -> int` | Returns depth times rows times columns. |
| `grid3d_int_values(grid: grid3d<int>) -> list<int>` | Returns values in layer-row-column order. |
| `grid3d_text_new(depth: int, rows: int, cols: int) -> grid3d<text>` | Creates text 3D grid. |
| `grid3d_text_get(grid: grid3d<text>, layer: int, row: int, col: int) -> text` | Reads text 3D cell. |
| `grid3d_text_set(grid: grid3d<text>, layer: int, row: int, col: int, value: text) -> grid3d<text>` | Returns grid with text 3D cell set. |
| `grid3d_text_fill(grid: grid3d<text>, value: text) -> grid3d<text>` | Fills text 3D grid. |
| `grid3d_text_depth(grid: grid3d<text>) -> int` | Returns depth. |
| `grid3d_text_rows(grid: grid3d<text>) -> int` | Returns rows. |
| `grid3d_text_cols(grid: grid3d<text>) -> int` | Returns columns. |
| `grid3d_text_size(grid: grid3d<text>) -> int` | Returns depth times rows times columns. |
| `grid3d_text_values(grid: grid3d<text>) -> list<text>` | Returns values in layer-row-column order. |

### Priority Queue Helpers

| Function | Semantics |
|---|---|
| `pqueue_int_new() -> pqueue<int>` | Creates integer priority queue. |
| `pqueue_int_push(heap: pqueue<int>, value: int) -> pqueue<int>` | Pushes integer. |
| `pqueue_int_pop(heap: pqueue<int>) -> optional<int>` | Pops integer. |
| `pqueue_int_peek(heap: pqueue<int>) -> optional<int>` | Peeks integer. |
| `pqueue_int_len(heap: pqueue<int>) -> int` | Returns count. |
| `pqueue_int_is_empty(heap: pqueue<int>) -> bool` | Checks empty. |
| `pqueue_int_values(heap: pqueue<int>) -> list<int>` | Returns values in pop order without mutating `heap`. |
| `pqueue_text_new() -> pqueue<text>` | Creates text priority queue. |
| `pqueue_text_push(heap: pqueue<text>, value: text) -> pqueue<text>` | Pushes text. |
| `pqueue_text_pop(heap: pqueue<text>) -> optional<text>` | Pops text. |
| `pqueue_text_peek(heap: pqueue<text>) -> optional<text>` | Peeks text. |
| `pqueue_text_len(heap: pqueue<text>) -> int` | Returns count. |
| `pqueue_text_is_empty(heap: pqueue<text>) -> bool` | Checks empty. |
| `pqueue_text_values(heap: pqueue<text>) -> list<text>` | Returns values in pop order without mutating `heap`. |

### Circular Buffer Helpers

| Function | Semantics |
|---|---|
| `circbuf_int_new(capacity: int) -> circbuf<int>` | Creates integer circular buffer. |
| `circbuf_int_push(buf: circbuf<int>, value: int) -> circbuf<int>` | Pushes integer. |
| `circbuf_int_pop(buf: circbuf<int>) -> optional<int>` | Pops integer. |
| `circbuf_int_peek(buf: circbuf<int>) -> optional<int>` | Peeks integer. |
| `circbuf_int_len(buf: circbuf<int>) -> int` | Returns count. |
| `circbuf_int_capacity(buf: circbuf<int>) -> int` | Returns capacity. |
| `circbuf_int_is_full(buf: circbuf<int>) -> bool` | Checks full. |
| `circbuf_int_is_empty(buf: circbuf<int>) -> bool` | Checks empty. |
| `circbuf_int_values(buf: circbuf<int>) -> list<int>` | Returns values from oldest to newest. |
| `circbuf_text_new(capacity: int) -> circbuf<text>` | Creates text circular buffer. |
| `circbuf_text_push(buf: circbuf<text>, value: text) -> circbuf<text>` | Pushes text. |
| `circbuf_text_pop(buf: circbuf<text>) -> optional<text>` | Pops text. |
| `circbuf_text_peek(buf: circbuf<text>) -> optional<text>` | Peeks text. |
| `circbuf_text_len(buf: circbuf<text>) -> int` | Returns count. |
| `circbuf_text_capacity(buf: circbuf<text>) -> int` | Returns capacity. |
| `circbuf_text_is_full(buf: circbuf<text>) -> bool` | Checks full. |
| `circbuf_text_is_empty(buf: circbuf<text>) -> bool` | Checks empty. |
| `circbuf_text_values(buf: circbuf<text>) -> list<text>` | Returns values from oldest to newest. |

### B-Tree Helpers

| Function | Semantics |
|---|---|
| `btreemap_text_new() -> btreemap<text, text>` | Creates text-to-text B-tree map. |
| `btreemap_text_set(self_map: btreemap<text, text>, key: text, value: text) -> btreemap<text, text>` | Sets key. |
| `btreemap_text_get(self_map: btreemap<text, text>, key: text) -> text` | Direct lookup. |
| `btreemap_text_get_optional(self_map: btreemap<text, text>, key: text) -> optional<text>` | Safe lookup. |
| `btreemap_text_contains(self_map: btreemap<text, text>, key: text) -> bool` | Checks key. |
| `btreemap_text_remove(self_map: btreemap<text, text>, key: text) -> btreemap<text, text>` | Removes key. |
| `btreemap_text_len(self_map: btreemap<text, text>) -> int` | Returns count. |
| `btreemap_text_is_empty(self_map: btreemap<text, text>) -> bool` | Checks empty. |
| `btreemap_text_keys(self_map: btreemap<text, text>) -> list<text>` | Returns keys in sorted text order. |
| `btreemap_text_values(self_map: btreemap<text, text>) -> list<text>` | Returns values following sorted key order. |
| `btreeset_text_new() -> btreeset<text>` | Creates text B-tree set. |
| `btreeset_text_insert(self_set: btreeset<text>, value: text) -> btreeset<text>` | Inserts value. |
| `btreeset_text_contains(self_set: btreeset<text>, value: text) -> bool` | Checks value. |
| `btreeset_text_remove(self_set: btreeset<text>, value: text) -> btreeset<text>` | Removes value. |
| `btreeset_text_len(self_set: btreeset<text>) -> int` | Returns count. |
| `btreeset_text_is_empty(self_set: btreeset<text>) -> bool` | Checks empty. |
| `btreeset_text_values(self_set: btreeset<text>) -> list<text>` | Returns values in sorted text order. |

---

## 12. `std.math`

Purpose: numeric constants, arithmetic helpers, rounding, trigonometry, logarithms, exponentials, and float inspection.

### Constants

| Constant | Value |
|---|---|
| `pi: float` | `3.141592653589793` |
| `e: float` | `2.718281828459045` |
| `tau: float` | `6.283185307179586` |

### Functions

| Function | Semantics |
|---|---|
| `infinity() -> float` | Produces positive infinity. Function, not constant, because current constants only expose finite float literals safely. |
| `nan() -> float` | Produces NaN. Function, not constant, because current constants only expose finite float literals safely. |
| `abs(value: float) -> float` | Absolute value for float. |
| `abs_int(value: int) -> int` | Absolute value for integer. |
| `min(a: float, b: float) -> float` | Smaller float. |
| `max(a: float, b: float) -> float` | Larger float. |
| `clamp(value: float, min: float, max: float) -> float` | Clamps value to range. |
| `pow(base: float, exponent: float) -> float` | Power. |
| `sqrt(value: float) -> float` | Square root. |
| `floor(value: float) -> float` | Floor. |
| `ceil(value: float) -> float` | Ceiling. |
| `round(value: float) -> float` | Round half away from zero. |
| `trunc(value: float) -> float` | Truncation. |
| `deg_to_rad(x: float) -> float` | Degrees to radians. |
| `rad_to_deg(x: float) -> float` | Radians to degrees. |
| `approx_equal(a: float, b: float, epsilon: float) -> bool` | Approximate equality. |
| `sin(value: float) -> float` | Sine. |
| `cos(value: float) -> float` | Cosine. |
| `tan(value: float) -> float` | Tangent. |
| `asin(value: float) -> float` | Arc sine. |
| `acos(value: float) -> float` | Arc cosine. |
| `atan(value: float) -> float` | Arc tangent. |
| `atan2(y: float, x: float) -> float` | Two-argument arc tangent. |
| `ln(value: float) -> float` | Natural logarithm. |
| `log_ten(value: float) -> float` | Base-10 logarithm. |
| `log2(value: float) -> float` | Base-2 logarithm. |
| `log(value: float, base: float) -> float` | Logarithm with explicit base. |
| `exp(value: float) -> float` | Exponential. |
| `is_nan(value: float) -> bool` | NaN predicate. |
| `is_infinite(value: float) -> bool` | Infinity predicate. |
| `is_finite(value: float) -> bool` | Finite predicate. |

---

## 13. `std.random`

Purpose: pseudo-random number generation, seeding, range helpers, and observable RNG state.

### Types

| Type | Fields |
|---|---|
| `Stats` | `seeded: bool`, `last_seed: int`, `draw_count: int` |

### Observable Namespace State

| Variable | Initial Value | Semantics |
|---|---|---|
| `seeded: bool` | `false` | Whether `seed(...)` has been called. |
| `last_seed: int` | `0` | Last explicit seed. |
| `draw_count: int` | `0` | Number of calls to `next()` since last seed. |

### Functions

| Function | Semantics |
|---|---|
| `seed(seed: int) -> void` | Seeds the RNG, updates `seeded`, `last_seed`, and resets `draw_count`. |
| `next() -> int` | Returns next pseudo-random integer and increments `draw_count`. |
| `between(min: int, max: int) -> result<int, core.Error>` | Inclusive integer range; errors when `max < min`. |
| `float_between(min: float, max: float) -> result<float, core.Error>` | Float range helper. |
| `stats() -> random.Stats` | Returns observable RNG state snapshot. |

---

## 14. `std.time`

Purpose: instants, durations, system clock, sleeping, arithmetic, and Unix timestamp conversion.

### Types

| Type | Fields |
|---|---|
| `Instant` | `millis: int` |
| `Duration` | `millis: int` |

### Functions

| Function | Semantics |
|---|---|
| `now() -> time.Instant` | Current instant. |
| `now_ms() -> int` | Current Unix time in milliseconds. |
| `sleep(duration: time.Duration) -> result<void, core.Error>` | Sleeps for duration. |
| `sleep_ms(ms: int) -> result<void, core.Error>` | Sleeps for milliseconds. |
| `since(start: time.Instant) -> time.Duration` | Duration from start until now. |
| `until(target: time.Instant) -> time.Duration` | Duration from now until target. |
| `elapsed(start: time.Instant, finish: time.Instant) -> int` | Milliseconds between two instants. |
| `diff(a: time.Instant, b: time.Instant) -> time.Duration` | Duration from `a` to `b`. |
| `add(at: time.Instant, duration: time.Duration) -> time.Instant` | Adds duration. |
| `sub(at: time.Instant, duration: time.Duration) -> time.Instant` | Subtracts duration. |
| `from_unix(ts: int) -> time.Instant` | Seconds timestamp to instant. |
| `from_unix_ms(ts: int) -> time.Instant` | Milliseconds timestamp to instant. |
| `to_unix(at: time.Instant) -> int` | Instant to seconds timestamp. |
| `to_unix_ms(at: time.Instant) -> int` | Instant to milliseconds timestamp. |
| `milliseconds(n: int) -> time.Duration` | Duration constructor. |
| `seconds(n: int) -> time.Duration` | Duration constructor. |
| `minutes(n: int) -> time.Duration` | Duration constructor. |
| `hours(n: int) -> time.Duration` | Duration constructor. |

---

## 15. `std.format`

Purpose: presentation formatting for numbers, dates, byte counts, integer bases, and `TextRepresentable` values.

### Types

| Type | Variants |
|---|---|
| `BytesStyle` | `Binary`, `Decimal` |

### Functions

| Function | Semantics |
|---|---|
| `number(value: float, decimals: int = 0) -> text` | Formats a number. |
| `percent(value: float, decimals: int = 0) -> text` | Formats a percent. |
| `date(millis: int, style: text = "iso") -> text` | Formats date. |
| `datetime(millis: int, style: text = "short", locale: text = "") -> text` | Formats date/time. |
| `date_pattern(millis: int, pattern: text) -> text` | Formats date with pattern. |
| `datetime_pattern(millis: int, pattern: text) -> text` | Formats date/time with pattern. |
| `bytes(value: int, style: format.BytesStyle = format.BytesStyle.Binary, decimals: int = 1) -> text` | Formats byte count. |
| `hex(value: int) -> text` | Formats integer as hexadecimal text. |
| `bin(value: int) -> text` | Formats integer as binary text. |
| `pretty(value: any<TextRepresentable>) -> text` | Renders via `to_text()`. |
| `compact(value: any<TextRepresentable>) -> text` | Renders via `to_text()`. |
| `as_json(value: any<TextRepresentable>) -> text` | Wraps rendered text as a JSON string value. |
| `yaml(value: any<TextRepresentable>) -> text` | Renders simple YAML-style value text. |
| `table(rows: list<any<TextRepresentable>>) -> text` | Joins rendered rows with newlines. |
| `csv(rows: list<any<TextRepresentable>>) -> text` | Joins rendered rows with commas. |

---

## 16. `std.encoding`

Purpose: hex and base64 encoding/decoding for bytes.

### Functions

| Function | Semantics |
|---|---|
| `hex_encode(data: bytes) -> text` | Encodes bytes as hex text. |
| `hex_decode(text_value: text) -> result<bytes, core.Error>` | Decodes hex text. |
| `base64_encode(data: bytes) -> text` | Encodes bytes as base64 text. |
| `base64_decode(text_value: text) -> result<bytes, core.Error>` | Decodes base64 text. |

---

## 17. `std.hash`

Purpose: hash/digest helpers for text and bytes.

### Functions

| Function | Semantics |
|---|---|
| `sha256(value: text) -> text` | SHA-256 digest for text. |
| `sha256_bytes(value: bytes) -> text` | SHA-256 digest for bytes. |
| `md5(value: text) -> text` | MD5 digest for text. |
| `md5_bytes(value: bytes) -> text` | MD5 digest for bytes. |

---

## 18. `std.json`

Purpose: JSON map subset plus raw JSON value helpers.

### Types

| Type | Variants/Fields |
|---|---|
| `Kind` | `Null`, `Bool`, `Number`, `Text`, `Array`, `Object` |
| `Value` | `raw: text` |

### Map-Based Functions

| Function | Semantics |
|---|---|
| `parse(input: text) -> result<map<text, text>, core.Error>` | Parses object JSON subset into `map<text, text>`. |
| `stringify(value: map<text, text>) -> text` | Serializes text map. |
| `pretty(value: map<text, text>, indent: int = 2) -> text` | Pretty-serializes text map. |
| `read(file_path: text) -> result<map<text, text>, core.Error>` | Reads and parses JSON file. |
| `write(file_path: text, value: map<text, text>) -> result<void, core.Error>` | Writes JSON file. |

### Value-Based Functions

| Function | Semantics |
|---|---|
| `parse_value(input: text) -> result<json.Value, core.Error>` | Validates full JSON and wraps raw value. |
| `stringify_value(value: json.Value) -> text` | Returns raw JSON text. |
| `pretty_value(value: json.Value, indent: int = 2) -> text` | Pretty-prints raw JSON value. |
| `read_value(file_path: text) -> result<json.Value, core.Error>` | Reads JSON file as raw value. |
| `write_value(file_path: text, value: json.Value) -> result<void, core.Error>` | Writes raw JSON value. |
| `kind(value: json.Value) -> json.Kind` | Returns JSON kind. |
| `as_text(value: json.Value) -> optional<text>` | Extracts text if kind matches. |
| `as_int(value: json.Value) -> optional<int>` | Extracts integer if representable. |
| `as_float(value: json.Value) -> optional<float>` | Extracts float if representable. |
| `as_bool(value: json.Value) -> optional<bool>` | Extracts bool if kind matches. |
| `get(value: json.Value, key: text) -> optional<json.Value>` | Safe object member lookup. |
| `at(value: json.Value, index: int) -> optional<json.Value>` | Safe array index lookup. |
| `len(value: json.Value) -> int` | Returns object/array length where applicable. |

---

## 19. `std.regex`

Purpose: portable regex validation, matching, searching, splitting, replacement, and escaping.

### Types

| Type | Variants/Fields |
|---|---|
| `Error` | `InvalidPattern` |
| `Regex` | `pattern: text` |

### Functions

| Function | Semantics |
|---|---|
| `compile(pattern: text) -> result<regex.Regex, regex.Error>` | Validates and wraps pattern. |
| `is_valid(pattern: text) -> bool` | Checks pattern validity. |
| `is_match(pattern: text, input: text) -> bool` | Direct match predicate. |
| `contains(pattern: text, input: text) -> bool` | Alias for `is_match`. |
| `matches(pattern: text, input: text) -> result<bool, regex.Error>` | Validating match predicate. |
| `full_match(pattern: text, input: text) -> result<bool, regex.Error>` | Validating full-string match. |
| `first(pattern: text, input: text) -> optional<text>` | First match without returning pattern errors. |
| `try_first(pattern: text, input: text) -> result<optional<text>, regex.Error>` | Validating first match. |
| `count(pattern: text, input: text) -> int` | Match count. |
| `find_all(pattern: text, input: text) -> list<text>` | All matches without returning pattern errors. |
| `try_find_all(pattern: text, input: text) -> result<list<text>, regex.Error>` | Validating all matches. |
| `split(pattern: text, input: text) -> list<text>` | Splits input by pattern. |
| `try_split(pattern: text, input: text) -> result<list<text>, regex.Error>` | Validating split. |
| `replace_all(pattern: text, input: text, replacement: text) -> text` | Replaces all matches. |
| `try_replace_all(pattern: text, input: text, replacement: text) -> result<text, regex.Error>` | Validating replace-all. |
| `escape(input: text) -> text` | Escapes regex metacharacters. |

---

## 20. `std.io`

Purpose: typed textual input, output, and error streams.

### Types

| Type | Variants/Fields |
|---|---|
| `Input` | `handle: int` |
| `Output` | `handle: int` |
| `Error` | `ReadFailed`, `WriteFailed`, `Unknown` |

### Constants

| Constant | Semantics |
|---|---|
| `input: io.Input` | Standard input handle. |
| `output: io.Output` | Standard output handle. |
| `stderr: io.Output` | Standard error handle. |

### Functions

| Function | Semantics |
|---|---|
| `to_core_error(err: io.Error) -> core.Error` | Converts `io.Error` to `core.Error`. |
| `read_line(from: io.Input = io.input) -> result<optional<text>, io.Error>` | Reads one line; EOF is `none`. |
| `read_all(from: io.Input = io.input) -> result<text, io.Error>` | Reads all input text. |
| `write(value: text, to: io.Output = io.output) -> result<void, io.Error>` | Writes text to output stream. |
| `print(value: text, to: io.Output = io.output) -> result<void, io.Error>` | Writes text; current implementation delegates to `write`. |

---

## 21. `std.console`

Purpose: interactive terminal helpers built on top of host IO.

### Types

| Type | Fields |
|---|---|
| `Size` | `columns: int`, `rows: int` |

### Functions

| Function | Semantics |
|---|---|
| `write_line(value: text = "") -> result<void, core.Error>` | Writes line to stdout. |
| `error_line(value: text) -> result<void, core.Error>` | Writes line to stderr. |
| `pause(message: text = "Press Enter to continue...") -> result<void, core.Error>` | Prompts and waits for Enter. |
| `prompt(message: text) -> result<text, core.Error>` | Writes prompt and reads line; EOF returns empty text. |
| `confirm(message: text, default_value: bool = false) -> result<bool, core.Error>` | Yes/no prompt with default. |
| `is_terminal(stream: text = "stdout") -> bool` | Checks whether stream is a terminal. |
| `size() -> console.Size` | Returns terminal size. |
| `columns() -> int` | Returns terminal columns. |
| `rows() -> int` | Returns terminal rows. |
| `clear() -> result<void, core.Error>` | Clears terminal. |
| `color(name: text) -> result<void, core.Error>` | Sets terminal color by name. |
| `style(name: text) -> result<void, core.Error>` | Sets terminal style by name. |
| `reset_style() -> result<void, core.Error>` | Resets terminal style. |
| `read_key() -> result<optional<text>, core.Error>` | Reads one key where supported. |

---

## 22. `std.fs`

Purpose: synchronous filesystem operations.

### Types

| Type | Variants/Fields |
|---|---|
| `Error` | `NotFound`, `PermissionDenied`, `AlreadyExists`, `NotADirectory`, `IsADirectory`, `IOError`, `InvalidPath`, `Unknown` |
| `Metadata` | `size_bytes: int`, `modified_at_ms: int`, `created_at_ms: optional<int>`, `is_file: bool`, `is_dir: bool` |

### Functions

| Function | Semantics |
|---|---|
| `read_text(file_path: text) -> result<text, fs.Error>` | Reads text file. |
| `write_text(file_path: text, content: text) -> result<void, fs.Error>` | Writes text file. |
| `append_text(file_path: text, content: text) -> result<void, fs.Error>` | Appends text file. |
| `read_bytes(file_path: text) -> result<bytes, fs.Error>` | Reads binary file. |
| `write_bytes(file_path: text, content: bytes) -> result<void, fs.Error>` | Writes binary file. |
| `exists(target_path: text) -> result<bool, fs.Error>` | Checks path existence. |
| `is_file(file_path: text) -> result<bool, fs.Error>` | Checks regular file. |
| `is_dir(dir_path: text) -> result<bool, fs.Error>` | Checks directory. |
| `create_dir(dir_path: text) -> result<void, fs.Error>` | Creates one directory. |
| `create_dir_all(dir_path: text) -> result<void, fs.Error>` | Creates directories recursively. |
| `list_dir(dir_path: text) -> result<list<text>, fs.Error>` | Lists directory entries. |
| `walk_dir(dir_path: text) -> result<list<text>, fs.Error>` | Recursively walks directory. |
| `remove_file(file_path: text) -> result<void, fs.Error>` | Removes file. |
| `remove_dir(dir_path: text) -> result<void, fs.Error>` | Removes empty directory. |
| `remove_dir_all(dir_path: text) -> result<void, fs.Error>` | Removes directory tree. |
| `copy_file(from: text, to: text) -> result<void, fs.Error>` | Copies file. |
| `copy(from: text, to: text) -> result<void, fs.Error>` | Alias for `copy_file`. |
| `move(from: text, to: text) -> result<void, fs.Error>` | Moves/renames path. |
| `rename(from: text, to: text) -> result<void, fs.Error>` | Alias for `move`. |
| `metadata(target_path: text) -> result<fs.Metadata, fs.Error>` | Returns combined metadata. |
| `size(target_path: text) -> result<int, fs.Error>` | Returns size in bytes. |
| `file_size(target_path: text) -> result<int, fs.Error>` | Alias for `size`. |
| `modified_at(target_path: text) -> result<int, fs.Error>` | Returns modified time in milliseconds. |
| `created_at(target_path: text) -> result<optional<int>, fs.Error>` | Returns creation time when available. |

---

## 23. `std.fs.path`

Purpose: pure lexical path operations.

### Functions

| Function | Semantics |
|---|---|
| `join(parts: list<text>) -> text` | Joins path parts with `/`. |
| `normalize(value: text) -> text` | Normalizes path. |
| `is_absolute(value: text) -> bool` | Checks absolute path. |
| `is_relative(value: text) -> bool` | Checks relative path. |
| `absolute(value: text, base: text) -> text` | Resolves absolute path against base. |
| `relative(value: text, from: text) -> text` | Computes relative path. |
| `base_name(value: text) -> text` | Returns final path segment. |
| `name_without_extension(value: text) -> text` | Returns base name without final extension. |
| `extension(value: text) -> optional<text>` | Returns extension without dot when present. |
| `parent(value: text) -> optional<text>` | Returns parent path. |
| `has_extension(value: text, expected: text) -> bool` | Checks extension; accepts expected with or without dot. |
| `change_extension(value: text, new_ext: text) -> text` | Replaces final extension. |

### Public Implementation Helpers

| Helper | Semantics |
|---|---|
| `_text_eq(a: text, b: text) -> bool` | Runtime equality wrapper. |
| `_last_index_of(value: text, needle: text) -> int` | Last substring index helper. |
| `_last_separator_index(value: text) -> int` | Last slash or backslash index helper. |

---

## 24. `std.os`

Purpose: process arguments, environment, process id, platform, architecture, and current directory.

### Types

| Type | Variants |
|---|---|
| `Platform` | `Windows`, `Linux`, `MacOS`, `Unknown` |
| `Arch` | `X64`, `X86`, `Arm64`, `Unknown` |
| `Error` | `NotFound`, `PermissionDenied`, `IOError`, `Unknown` |

### Functions

| Function | Semantics |
|---|---|
| `args() -> list<text>` | Returns process arguments. `args()[0]` is the executable name/path received from the host. With `zt run`, values after `--` are forwarded as program args. |
| `env(name: text) -> optional<text>` | Reads environment variable. |
| `pid() -> int` | Returns current process id. |
| `platform() -> os.Platform` | Returns platform enum. |
| `arch() -> os.Arch` | Returns architecture enum. |
| `current_dir() -> result<text, os.Error>` | Returns current directory. |
| `change_dir(dir_path: text) -> result<void, os.Error>` | Changes current directory. |

---

## 25. `std.os.process`

Purpose: child-process execution and captured output.

### Types

| Type | Variants/Fields |
|---|---|
| `ExitStatus` | `code: int` |
| `CapturedRun` | `status: process.ExitStatus`, `stdout_text: text`, `stderr_text: text` |
| `Error` | `NotFound`, `PermissionDenied`, `IOFailure`, `DecodeFailed`, `Unknown` |

### Functions

| Function | Semantics |
|---|---|
| `run(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.ExitStatus, process.Error>` | Runs a child process and returns exit status. |
| `run_capture(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.CapturedRun, process.Error>` | Runs a child process and captures stdout/stderr. |

---

## 26. `std.validate`

Purpose: pure boolean predicates for `where` clauses and conditions.

### Numeric Predicates

| Function | Semantics |
|---|---|
| `between(value: int, min: int, max: int) -> bool` | Inclusive range check. |
| `positive(value: int) -> bool` | Checks greater than zero. |
| `non_negative(value: int) -> bool` | Checks greater than or equal to zero. |
| `negative(value: int) -> bool` | Checks less than zero. |
| `non_zero(value: int) -> bool` | Checks not equal to zero. |
| `one_of(value: int, candidates: list<int>) -> bool` | Checks integer membership in candidate list. |

### Text Predicates

| Function | Semantics |
|---|---|
| `one_of_text(value: text, candidates: list<text>) -> bool` | Checks text membership in candidate list. |
| `not_empty(value: text) -> bool` | Checks non-empty text. |
| `not_empty_text(value: text) -> bool` | Alias for `not_empty`. |
| `min_length(value: text, min: int) -> bool` | Checks minimum length. |
| `min_len(value: text, min: int) -> bool` | Alias for `min_length`. |
| `max_length(value: text, max: int) -> bool` | Checks maximum length. |
| `max_len(value: text, max: int) -> bool` | Alias for `max_length`. |
| `length_between(value: text, min: int, max: int) -> bool` | Inclusive text length range. |

### Broader Predicate Families

| Function family | Semantics |
|---|---|
| `between_float`, `positive_float`, `non_negative_float`, `negative_float`, `non_zero_float`, `one_of_float` | Float range, sign, non-zero, and candidate checks. |
| `is_true`, `is_false`, `one_of_bool` | Bool readability helpers. |
| `is_some_int/is_none_int`, `is_some_float/is_none_float`, `is_some_bool/is_none_bool` | Optional state helpers for executable primitive optional shapes. |
| `is_success_int_text/is_error_int_text` | Result state helpers for the common `result<int, text>` shape. |
| `*_list_int`, `*_list_text`, `*_list_float`, `*_list_bool` | Non-empty, minimum length, maximum length, and inclusive length range checks for supported lists. |
| `*_map_text_int`, `*_map_text_text`, `*_map_int_text`, `*_map_int_int` | Non-empty, minimum size, maximum size, and inclusive size range checks for supported maps. |

Fully generic public helpers remain deferred until public stdlib generics are stable through import and C emission.

---

## 27. `std.test`

Purpose: test helper module used with `attr test` functions.

### Outcome Helpers

| Function | Semantics |
|---|---|
| `fail(message: text = "test failed") -> void` | Marks current test as failed. |
| `skip(reason: text = "") -> void` | Marks current test as skipped. |
| `throws(body: func() -> void) -> void` | Expects callable body to throw/fail where supported. |

### Boolean And Comparison Helpers

| Function | Semantics |
|---|---|
| `is_true(value: bool) -> void` | Fails if value is false. |
| `is_false(value: bool) -> void` | Fails if value is true. |
| `equal_int(actual: int, expected: int) -> void` | Fails if integers differ. |
| `equal_text(actual: text, expected: text) -> void` | Fails if text differs. |
| `not_equal_int(actual: int, expected: int) -> void` | Fails if integers are equal. |
| `not_equal_text(actual: text, expected: text) -> void` | Fails if text values are equal. |

### Runner Bridge Functions

| Function | Semantics |
|---|---|
| `zt_test_fail(message: text) -> void` | Low-level runner bridge. |
| `zt_test_skip(reason: text) -> void` | Low-level runner bridge. |
| `zt_test_throws_closure(body: func() -> void) -> bool` | Low-level runner bridge. |

Application tests should normally use `fail`, `skip`, `is_true`, `is_false`, and comparison helpers.

---

## 28. `std.lazy`

Purpose: explicit one-shot lazy values for selected payload types.

`0.4.2-beta.rc1` supports only `lazy<int>`, `lazy<float>`, `lazy<bool>`,
and `lazy<text>` through public helpers. Other `lazy<T>` payloads are
diagnosed during `zt check`; fully generic lazy values and lazy iterators are
post-RC work.

### Integer Lazy Helpers

| Function | Semantics |
|---|---|
| `once_int(thunk: func() -> int) -> lazy<int>` | Creates one-shot lazy integer. |
| `force_int(value: lazy<int>) -> int` | Forces lazy integer. |
| `is_consumed_int(value: lazy<int>) -> bool` | Checks whether lazy integer was consumed. |

### Float Lazy Helpers

| Function | Semantics |
|---|---|
| `once_float(thunk: func() -> float) -> lazy<float>` | Creates one-shot lazy float. |
| `force_float(value: lazy<float>) -> float` | Forces lazy float. |
| `is_consumed_float(value: lazy<float>) -> bool` | Checks whether lazy float was consumed. |

### Bool Lazy Helpers

| Function | Semantics |
|---|---|
| `once_bool(thunk: func() -> bool) -> lazy<bool>` | Creates one-shot lazy bool. |
| `force_bool(value: lazy<bool>) -> bool` | Forces lazy bool. |
| `is_consumed_bool(value: lazy<bool>) -> bool` | Checks whether lazy bool was consumed. |

### Text Lazy Helpers

| Function | Semantics |
|---|---|
| `once_text(thunk: func() -> text) -> lazy<text>` | Creates one-shot lazy text. |
| `force_text(value: lazy<text>) -> text` | Forces lazy text. |
| `is_consumed_text(value: lazy<text>) -> bool` | Checks whether lazy text was consumed. |

---

## 29. `std.concurrent`

Purpose: explicit transfer/copy helpers for isolate and worker boundaries.

### Functions

| Function | Semantics |
|---|---|
| `copy_int(value: int) -> int` | Transfer-copy helper for integers. |
| `copy_bool(value: bool) -> bool` | Transfer-copy helper for bool. |
| `copy_float(value: float) -> float` | Transfer-copy helper for float. |
| `copy_text(value: text) -> text` | Transfer-copy helper for text. |
| `copy_bytes(value: bytes) -> bytes` | Transfer-copy helper for bytes. |
| `copy_list_int(value: list<int>) -> list<int>` | Transfer-copy helper for integer lists. |
| `copy_list_text(value: list<text>) -> list<text>` | Transfer-copy helper for text lists. |
| `copy_map_text_text(value: map<text, text>) -> map<text, text>` | Transfer-copy helper for text maps. |

Ordinary user-facing concurrency should prefer `std.jobs` and `std.channels`.

---

## 30. `std.jobs`

Purpose: typed job handles and explicit worker execution.

### Types

| Type | Fields |
|---|---|
| `Job<T>` | `handle: int` |

### Compiler-Known Public Functions

These functions are recognized by the checker/lowering as typed facades. The runtime currently uses type-specialized ABI anchors internally.

| Function | Semantics |
|---|---|
| `spawn(worker: func() -> T) -> jobs.Job<T>` | Spawns a top-level non-generic worker with no argument. |
| `spawn(worker: func(A) -> T, value: A) -> jobs.Job<T>` | Spawns a top-level non-generic worker with one transferable argument. |
| `join(job: jobs.Job<T>) -> T` | Joins job and returns payload. |

### Current Backend Notes

- Current executable backend supports `T = int` and `T = text`.
- The one-argument form currently supports `int` and `text` payload arguments, and the argument/result family must match in this cut.
- Worker must be a top-level non-generic function reference.
- Closures and captured callables are rejected for `spawn`.
- Values crossing the boundary must satisfy `Transferable`.

---

## 31. `std.channels`

Purpose: typed channel handles for explicit message passing.

### Types

| Type | Fields |
|---|---|
| `Channel<T>` | `handle: int` |

### Compiler-Known Public Functions

| Function | Semantics |
|---|---|
| `create() -> channels.Channel<T>` | Creates a channel; requires expected `Channel<T>` type. |
| `send(channel: channels.Channel<T>, value: T) -> int` | Sends a value; return value is backend status code. |
| `receive(channel: channels.Channel<T>) -> optional<T>` | Receives a value or `none` when closed/empty according to runtime behavior. |
| `close(channel: channels.Channel<T>) -> int` | Closes a channel; return value is backend status code. |

### Current Backend Notes

- Current executable backend supports `Channel<int>` and `Channel<text>`.
- Broader payload storage, backpressure, cancellation, and panic capture are deferred.

---

## 32. `std.net`

Purpose: blocking TCP client foundation.

### Types

| Type | Variants/Fields |
|---|---|
| `Error` | `ConnectionRefused`, `HostUnreachable`, `Timeout`, `AddressInUse`, `AlreadyConnected`, `NotConnected`, `NetworkDown`, `Overflow`, `PeerReset`, `SystemLimit`, `Unknown` |
| `Connection` | Opaque connection value. |

### Functions

| Function | Semantics |
|---|---|
| `timeout_to_ms(timeout: optional<int>) -> int` | Converts optional timeout to milliseconds, using `-1` for none. |
| `map_core_error(err: core.Error) -> net.Error` | Maps core network error to `net.Error`. |
| `to_core_error(err: net.Error) -> core.Error` | Converts `net.Error` to `core.Error`. |
| `connect(host: text, port: int where it >= 1 and it <= 65535, timeout: optional<int> = none) -> result<net.Connection, net.Error>` | Opens TCP connection. |
| `read_some(connection: net.Connection, max: int where it > 0, timeout: optional<int> = none) -> result<optional<bytes>, net.Error>` | Reads up to `max` bytes. |
| `write_all(connection: net.Connection, data: bytes, timeout: optional<int> = none) -> result<void, net.Error>` | Writes all bytes. |
| `close(connection: net.Connection) -> result<void, net.Error>` | Closes connection. |
| `is_closed(connection: net.Connection) -> bool` | Checks closed state. |
| `kind(err: core.Error) -> net.Error` | Current placeholder mapping helper. |

### Deferred Network Work

- TLS
- UDP
- server APIs
- WebSocket
- async IO
- stream integration

---

## 33. `std.http`

Purpose: blocking HTTP client helpers built on runtime HTTP core.

### Types

| Type | Variants/Fields |
|---|---|
| `ErrorKind` | `UnsupportedScheme`, `InvalidUrl`, `Network`, `InvalidResponse` |
| `Error` | `kind: http.ErrorKind`, `message: text` |
| `Response` | `status: int`, `body: text`, `headers: map<text, text>` |

### Functions

| Function | Semantics |
|---|---|
| `get(url: text) -> result<http.Response, http.Error>` | Performs HTTP GET. |
| `post(url: text, body: text, content_type: text = "text/plain") -> result<http.Response, http.Error>` | Performs HTTP POST. |

### Current Backend Notes

- Response parsing extracts status and body from raw HTTP text.
- Headers currently materialize as an empty map in `Response`.
- Runtime/network availability can depend on host environment.
- v1 supports blocking `GET` and `POST` over `http://`.
- TLS/HTTPS, redirects, timeout options, streaming bodies, chunked transfer decoding, custom request headers, non-GET/POST methods, and bytes bodies are deferred.

---

## 34. `std.shared`

Purpose: advanced shared handle facade.

### Types

| Type | Fields |
|---|---|
| `Shared<T>` | `handle: int` |

### Compiler-Known Public Functions

| Function | Semantics |
|---|---|
| `create(value: T) -> shared.Shared<T>` | Creates shared handle from value. |
| `get(handle: shared.Shared<T>) -> T` | Reads shared value. |
| `set(handle: shared.Shared<T>, value: T) -> int` | Writes shared value; return value is backend status code. |

### Current Backend Notes

- Current executable backend supports `Shared<int>`.
- Non-`int` payloads are rejected early by checker diagnostics, for example `Shared<text>`.
- This is advanced/low-level stdlib surface, not ordinary core language teaching surface.

---

## 35. `std.atomic`

Purpose: advanced atomic integer handle facade.

### Types

| Type | Fields |
|---|---|
| `Atomic<T>` | `handle: int` |

### Compiler-Known Public Functions

| Function | Semantics |
|---|---|
| `create(value: int) -> atomic.Atomic<int>` | Creates atomic integer handle. |
| `load(handle: atomic.Atomic<int>) -> int` | Loads current integer value. |
| `store(handle: atomic.Atomic<int>, value: int) -> int` | Stores integer value; return value is backend status code. |
| `add(handle: atomic.Atomic<int>, delta: int) -> int` | Adds delta and returns backend/runtime integer result. |

### Current Backend Notes

- Current executable backend supports `Atomic<int>` only.
- `Atomic<bool>` and arbitrary `Atomic<T>` are not exposed until the runtime has true atomic representation for each supported payload.
- This is advanced/low-level stdlib surface, not ordinary core language teaching surface.

---

## 36. `std.orc`

Purpose: advanced runtime ownership/reference-counting inspection and cycle collection hooks.

### Functions

| Function | Semantics |
|---|---|
| `collect_cycles() -> int` | Runs cycle collection hook; currently returns collection count from runtime. |
| `ref_count_text(value: text) -> int` | Returns runtime reference count for text. |
| `ref_count_list_text(value: list<text>) -> int` | Returns runtime reference count for text list. |
| `is_unique_text(value: text) -> bool` | Checks whether text runtime value is unique. |
| `is_unique_list_text(value: list<text>) -> bool` | Checks whether text list runtime value is unique. |

This module is advanced runtime surface and should not be required for ordinary safe code.
Generic `ref_count<T>` and `is_unique<T>` are deferred until managed generic runtime hooks are widened beyond text and `list<text>`.

---

## 37. `std.mem`

Purpose: advanced ownership/view/edit hooks for managed values.

### Functions

| Function | Semantics |
|---|---|
| `own_text(value: text) -> text` | Materializes owned text. |
| `view_text(value: text) -> text` | Materializes retained/view text. |
| `edit_text(value: text) -> text` | Materializes editable text copy. |
| `own_list_text(value: list<text>) -> list<text>` | Materializes owned text list. |
| `view_list_text(value: list<text>) -> list<text>` | Materializes retained/view text list. |
| `edit_list_text(value: list<text>) -> list<text>` | Materializes editable text-list copy. |
| `own(value: T) -> T` | Compiler-known facade for the Appendix B safe memory subset. |
| `view(value: T) -> T` | Compiler-known facade for the Appendix B safe memory subset. |
| `edit(value: T) -> T` | Compiler-known editable-copy facade for the Appendix B safe memory subset. |

This module is advanced memory surface and should not be required for ordinary safe code.
Generic `own<T>`, `view<T>`, and `edit<T>` are implemented for the stabilized executable subset: primitive scalars, `text`, safe tuples/structs made from scalar/text fields, `list<int>`, `list<float>`, `list<bool>`, `list<int8>`, `list<u8>`, `list<text>`, `list<safe tuple>`, `list<safe struct>`, `set<int>`, `set<text>`, and maps with `int`/`text` keys plus scalar/text values. Enums, optional/result payloads, tuples containing mutable managed values, nested lists such as `list<list<int>>`, tuple/struct set keys, managed map values, and allocator-backed resources remain tracked in Appendix B of `implementation-plan.md`.

`mem.Temp` and `mem.Pool<T>` are reserved future library-level resource names.
They are not exposed in 0.4.2-beta.rc1 and must not be documented as usable
until they have deterministic `using` cleanup fixtures.

---

## 38. `std.unsafe`

Purpose: explicit unsafe/runtime inspection and retain helpers.

### Functions

| Function | Semantics |
|---|---|
| `heap_kind_text(value: text) -> int` | Returns runtime heap-kind code for text. |
| `heap_kind_list_text(value: list<text>) -> int` | Returns runtime heap-kind code for text list. |
| `retain_text(value: text) -> text` | Explicitly retains text through runtime helper. |
| `retain_list_text(value: list<text>) -> list<text>` | Explicitly retains text list through runtime helper. |

This module is intentionally advanced and outside the ordinary safe v1 teaching surface.
Generic unsafe retain/introspection helpers are deferred; the current public API stays type-specific for text and `list<text>`.

---

## 39. `std.debug`

Purpose: debug-oriented static/runtime information helpers.

### Functions

| Function | Semantics |
|---|---|
| `size_of(value) -> int` | Compiler-known helper that returns the current backend representation size for any typed value the checker accepts. |
| `type_name(value) -> text` | Compiler-known helper that returns the static type name for any typed value the checker accepts. |

`std.debug` stays focused on type/debug facts. Heap-kind, retain, and ownership details belong in `std.unsafe` and `std.orc`.

---

## 40. Current Gaps And Teaching Notes

### Stable Ordinary Surface

Teach these first for application users:

- `std.io`
- `std.console`
- `std.text`
- `std.bytes`
- `std.list`
- `std.map`
- `std.set`
- `std.fs`
- `std.fs.path`
- `std.json`
- `std.math`
- `std.random`
- `std.time`
- `std.format`
- `std.validate`
- `std.test`
- `std.jobs`
- `std.channels`

### Advanced Or Low-Level Surface

Teach these only when discussing runtime, backend, concurrency internals, or systems-level diagnostics:

- `std.concurrent`
- `std.shared`
- `std.atomic`
- `std.orc`
- `std.mem`
- `std.unsafe`
- `std.debug`

### Network Surface

`std.net` and `std.http` exist as blocking foundations. Availability and behavior may depend on the host runtime environment. Higher-level networking remains future work.

### Runtime Anchor Names

Names such as `zt_*`, `_i64`, and backend-specific handles are runtime/compiler anchors. They are not the public teaching surface unless explicitly listed here as public bridge helpers.

---

## 41. Compact Examples

### Reading Text And Printing

```zt
namespace app.main

import std.io as io

func main() -> result<void, io.Error>
    const line: optional<text> = io.read_line()?
    match line
        case some(value):
            io.print(value)?
        case none:
            io.print("no input")?
    end
    return success()
end
```

### Safe File Read And Path Helpers

```zt
namespace app.files

import std.fs as fs
import std.fs.path as path

func load_config(root: text) -> result<text, fs.Error>
    const file_path: text = path.join([root, "config.json"])
    return fs.read_text(file_path)
end
```

### Jobs And Channels

```zt
namespace app.concurrent

import std.jobs as jobs
import std.channels as channels

func compute() -> int
    return 42
end

func main()
    const job: jobs.Job<int> = jobs.spawn(compute)
    const answer: int = jobs.join(job)

    const channel: channels.Channel<int> = channels.create()
    channels.send(channel, answer)
    const received: optional<int> = channels.receive(channel)
end
```
