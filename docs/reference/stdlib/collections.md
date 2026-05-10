# Collections Reference

> Surface: reference
> Status: current

Modules: `std.collections`, `std.list` and `std.map`.

The current alpha exposes specialized helpers for `int` and `text` collection
modules. Basic `std.list` value helpers also execute for primitive lists and
`list<text>`. `std.list` higher-order helpers execute for same-type
primitive/text lists, including `list.reduce<T,T>`. Basic `std.map` value
helpers execute for generated maps with `int` or `text` keys and primitive or
`text` values. `std.map.set`,
`std.map.has_key`/`std.map.contains`, and `std.map.remove` also execute for safe
`map<Struct,V>` keys when the key fields are `bool`, integral types, or `text`,
and `V` is currently materialized by the generic runtime. `std.collections`
exposes iterable snapshots as `list<T>` where the current runtime storage
supports it.

`std.collections` does not currently provide arbitrary advanced generic storage.
The v1 executable subset is:

| Structure | Supported shapes |
| --- | --- |
| Queue and stack | `list<int>`, `list<text>`, plus `queue_values<T>` and `stack_values<T>` snapshots for list-backed values |
| Grid2D and Grid3D | `grid2d<int>`, `grid2d<text>`, `grid3d<int>`, `grid3d<text>` |
| Priority queue | `pqueue<int>`, `pqueue<text>` |
| Circular buffer | `circbuf<int>`, `circbuf<text>` |
| BTree map | `btreemap<text,text>` |
| BTree set | `btreeset<text>` |
| Higher-order helpers | `std.collections.map_int/filter_int/reduce_int`; `std.list` same-type HOFs for primitive/text lists |

General `grid2d<T>`, `circbuf<T>`, `pqueue<T>`, `btreemap<K,V>`, and
`btreeset<T>` support is post-RC technical debt. `pqueue`, `btreemap`, and
`btreeset` also need an ordering or comparator contract before they can be
generic over arbitrary user types.
Nested managed payloads in advanced materialized containers, such as
`grid2d<list<text>>` or `circbuf<map<text, text>>`, are also post-RC work.

## Iteration And Ordering

Use the `*_values` and `*_keys` helpers when a specialized collection is not directly iterable.

| Collection | Snapshot helper | Order |
| --- | --- | --- |
| Queue | `collections.queue_values<T>(queue)` | Existing list order: front to back. Compiler-known helper for list-backed queues. |
| Stack | `collections.stack_values<T>(stack)` | Existing list storage order: bottom to top. Use `pop`/`peek` for LIFO top behavior. Compiler-known helper for list-backed stacks. |
| Grid2D | `grid2d_*_values(grid)` | Row-major: row, then column. |
| Grid3D | `grid3d_*_values(grid)` | Layer, then row, then column. |
| Priority queue | `pqueue_*_values(heap)` | Stable pop order, lowest value first in this min-heap runtime. The source heap is not mutated. |
| Circular buffer | `circbuf_*_values(buf)` | Oldest to newest. |
| BTree map | `btreemap_text_keys(map)`, `btreemap_text_values(map)` | Sorted key order. Values follow the same key order. |
| BTree set | `btreeset_text_values(set)` | Sorted value order. |

Ordering constraints:

- Priority queues, BTree maps, and BTree sets require a runtime ordering relation for the element or key type.
- The current specialized runtime has ordering for `int` priority queues, `text` priority queues, `btreemap<text,text>`, and `btreeset<text>`.
- Fully generic managed collection runtime support remains deferred; unsupported shapes should stay documented instead of being implied.

## Built-in List and Map Helpers

These helpers work with built-in collection types.

| API | Description |
| --- | --- |
| `list.is_empty(items: list<T>) -> bool` | Checks whether a list has no items. |
| `map.is_empty(values: map<K,V>) -> bool` | Checks whether a map has no entries. |
| `map.has_key(values: map<K,V>, key: K) -> bool` | Checks whether a map contains the key. |
| `map.set(values: map<K,V>, key: K, value: V) -> map<K,V>` | Returns a map with the key set in the generated `int`/`text` key subset and safe structural-key subset. |
| `map.remove(values: map<K,V>, key: K) -> map<K,V>` | Returns a map without the key in the generated `int`/`text` key subset and safe structural-key subset. |
| `map.keys(values: map<K,V>) -> list<K>` | Returns keys for generated maps with `int` or `text` keys. |
| `map.values(values: map<K,V>) -> list<V>` | Returns values for generated maps with primitive or `text` values. |
| `map.merge(left: map<K,V>, right: map<K,V>) -> map<K,V>` | Returns a merged map; right-side values win. |

## Queue

Types:

| Type | Description |
| --- | --- |
| `collections.QueueNumberDequeueResult` | Result object returned when dequeuing an integer queue. |
| `collections.QueueTextDequeueResult` | Result object returned when dequeuing a text queue. |

Functions:

| API | Description |
| --- | --- |
| `collections.queue_int_new() -> list<int>` | Creates an empty integer queue. |
| `collections.queue_int_enqueue(queue: list<int>, value: int) -> list<int>` | Returns a queue with `value` added at the end. |
| `collections.queue_int_dequeue(queue: list<int>) -> collections.QueueNumberDequeueResult` | Removes the first integer and returns the new queue plus value information. |
| `collections.queue_int_peek(queue: list<int>) -> optional<int>` | Reads the first integer without removing it. |
| `collections.queue_text_new() -> list<text>` | Creates an empty text queue. |
| `collections.queue_text_enqueue(queue: list<text>, value: text) -> list<text>` | Returns a queue with `value` added at the end. |
| `collections.queue_text_dequeue(queue: list<text>) -> collections.QueueTextDequeueResult` | Removes the first text value and returns the new queue plus value information. |
| `collections.queue_text_peek(queue: list<text>) -> optional<text>` | Reads the first text value without removing it. |
| `collections.queue_values<T>(queue: list<T>) -> list<T>` | Compiler-known helper that returns the list-backed queue snapshot in front-to-back order. |

## Stack

Types:

| Type | Description |
| --- | --- |
| `collections.StackNumberPopResult` | Result object returned when popping an integer stack. |
| `collections.StackTextPopResult` | Result object returned when popping a text stack. |

Functions:

| API | Description |
| --- | --- |
| `collections.stack_int_new() -> list<int>` | Creates an empty integer stack. |
| `collections.stack_int_push(stack: list<int>, value: int) -> list<int>` | Returns a stack with `value` pushed on top. |
| `collections.stack_int_pop(stack: list<int>) -> collections.StackNumberPopResult` | Pops the top integer and returns the new stack plus value information. |
| `collections.stack_int_peek(stack: list<int>) -> optional<int>` | Reads the top integer without removing it. |
| `collections.stack_text_new() -> list<text>` | Creates an empty text stack. |
| `collections.stack_text_push(stack: list<text>, value: text) -> list<text>` | Returns a stack with `value` pushed on top. |
| `collections.stack_text_pop(stack: list<text>) -> collections.StackTextPopResult` | Pops the top text value and returns the new stack plus value information. |
| `collections.stack_text_peek(stack: list<text>) -> optional<text>` | Reads the top text value without removing it. |
| `collections.stack_values<T>(stack: list<T>) -> list<T>` | Compiler-known helper that returns the list-backed stack snapshot in bottom-to-top storage order. |

## Higher-Order Helpers

| API | Description |
| --- | --- |
| `collections.map_int(values: list<int>, mapper: func(int) -> int) -> list<int>` | Applies a function to each integer and returns a new list. |
| `collections.filter_int(values: list<int>, predicate: func(int) -> bool) -> list<int>` | Keeps only integers accepted by the predicate. |
| `collections.reduce_int(values: list<int>, initial: int, reducer: func(int, int) -> int) -> int` | Folds a list into one integer accumulator. |
| `list.reduce(values: list<T>, initial: T, reducer: func(T, T) -> T) -> T` | Folds primitive/text lists into a same-type accumulator. Cross-type `reduce<T,U>` remains deferred. |

## Grid 2D

| API | Description |
| --- | --- |
| `collections.grid2d_int_new(rows: int, cols: int) -> grid2d<int>` | Creates an integer 2D grid. |
| `collections.grid2d_int_get(grid: grid2d<int>, row: int, col: int) -> int` | Reads an integer cell. |
| `collections.grid2d_int_set(grid: grid2d<int>, row: int, col: int, value: int) -> grid2d<int>` | Returns a grid with one integer cell changed. |
| `collections.grid2d_int_fill(grid: grid2d<int>, value: int) -> grid2d<int>` | Returns a grid with all integer cells set to `value`. |
| `collections.grid2d_int_rows(grid: grid2d<int>) -> int` | Returns the number of rows. |
| `collections.grid2d_int_cols(grid: grid2d<int>) -> int` | Returns the number of columns. |
| `collections.grid2d_int_size(grid: grid2d<int>) -> int` | Returns total cell count. |
| `collections.grid2d_int_values(grid: grid2d<int>) -> list<int>` | Returns row-major cell values. |
| `collections.grid2d_text_new(rows: int, cols: int) -> grid2d<text>` | Creates a text 2D grid. |
| `collections.grid2d_text_get(grid: grid2d<text>, row: int, col: int) -> text` | Reads a text cell. |
| `collections.grid2d_text_set(grid: grid2d<text>, row: int, col: int, value: text) -> grid2d<text>` | Returns a grid with one text cell changed. |
| `collections.grid2d_text_fill(grid: grid2d<text>, value: text) -> grid2d<text>` | Returns a grid with all text cells set to `value`. |
| `collections.grid2d_text_rows(grid: grid2d<text>) -> int` | Returns the number of rows. |
| `collections.grid2d_text_cols(grid: grid2d<text>) -> int` | Returns the number of columns. |
| `collections.grid2d_text_size(grid: grid2d<text>) -> int` | Returns total cell count. |
| `collections.grid2d_text_values(grid: grid2d<text>) -> list<text>` | Returns row-major cell values. |

## Grid 3D

| API | Description |
| --- | --- |
| `collections.grid3d_int_new(depth: int, rows: int, cols: int) -> grid3d<int>` | Creates an integer 3D grid. |
| `collections.grid3d_int_get(grid: grid3d<int>, layer: int, row: int, col: int) -> int` | Reads an integer cell in a layer. |
| `collections.grid3d_int_set(grid: grid3d<int>, layer: int, row: int, col: int, value: int) -> grid3d<int>` | Returns a grid with one integer cell changed. |
| `collections.grid3d_int_fill(grid: grid3d<int>, value: int) -> grid3d<int>` | Returns a grid with all integer cells set to `value`. |
| `collections.grid3d_int_depth(grid: grid3d<int>) -> int` | Returns the number of layers. |
| `collections.grid3d_int_rows(grid: grid3d<int>) -> int` | Returns the number of rows per layer. |
| `collections.grid3d_int_cols(grid: grid3d<int>) -> int` | Returns the number of columns per layer. |
| `collections.grid3d_int_size(grid: grid3d<int>) -> int` | Returns total cell count. |
| `collections.grid3d_int_values(grid: grid3d<int>) -> list<int>` | Returns values in layer-row-column order. |
| `collections.grid3d_text_new(depth: int, rows: int, cols: int) -> grid3d<text>` | Creates a text 3D grid. |
| `collections.grid3d_text_get(grid: grid3d<text>, layer: int, row: int, col: int) -> text` | Reads a text cell in a layer. |
| `collections.grid3d_text_set(grid: grid3d<text>, layer: int, row: int, col: int, value: text) -> grid3d<text>` | Returns a grid with one text cell changed. |
| `collections.grid3d_text_fill(grid: grid3d<text>, value: text) -> grid3d<text>` | Returns a grid with all text cells set to `value`. |
| `collections.grid3d_text_depth(grid: grid3d<text>) -> int` | Returns the number of layers. |
| `collections.grid3d_text_rows(grid: grid3d<text>) -> int` | Returns the number of rows per layer. |
| `collections.grid3d_text_cols(grid: grid3d<text>) -> int` | Returns the number of columns per layer. |
| `collections.grid3d_text_size(grid: grid3d<text>) -> int` | Returns total cell count. |
| `collections.grid3d_text_values(grid: grid3d<text>) -> list<text>` | Returns values in layer-row-column order. |

## Priority Queue

| API | Description |
| --- | --- |
| `collections.pqueue_int_new() -> pqueue<int>` | Creates an empty integer priority queue. |
| `collections.pqueue_int_push(heap: pqueue<int>, value: int) -> pqueue<int>` | Returns a priority queue with `value` inserted. |
| `collections.pqueue_int_pop(heap: pqueue<int>) -> optional<int>` | Removes and returns the highest-priority integer when present. |
| `collections.pqueue_int_peek(heap: pqueue<int>) -> optional<int>` | Reads the highest-priority integer without removing it. |
| `collections.pqueue_int_len(heap: pqueue<int>) -> int` | Returns the number of integers in the queue. |
| `collections.pqueue_int_is_empty(heap: pqueue<int>) -> bool` | Checks whether the integer priority queue is empty. |
| `collections.pqueue_int_values(heap: pqueue<int>) -> list<int>` | Returns values in pop order without mutating `heap`. |
| `collections.pqueue_text_new() -> pqueue<text>` | Creates an empty text priority queue. |
| `collections.pqueue_text_push(heap: pqueue<text>, value: text) -> pqueue<text>` | Returns a priority queue with `value` inserted. |
| `collections.pqueue_text_pop(heap: pqueue<text>) -> optional<text>` | Removes and returns the highest-priority text value when present. |
| `collections.pqueue_text_peek(heap: pqueue<text>) -> optional<text>` | Reads the highest-priority text value without removing it. |
| `collections.pqueue_text_len(heap: pqueue<text>) -> int` | Returns the number of text values in the queue. |
| `collections.pqueue_text_is_empty(heap: pqueue<text>) -> bool` | Checks whether the text priority queue is empty. |
| `collections.pqueue_text_values(heap: pqueue<text>) -> list<text>` | Returns values in pop order without mutating `heap`. |

## Circular Buffer

| API | Description |
| --- | --- |
| `collections.circbuf_int_new(capacity: int) -> circbuf<int>` | Creates an integer circular buffer with fixed capacity. |
| `collections.circbuf_int_push(buf: circbuf<int>, value: int) -> circbuf<int>` | Pushes an integer, wrapping when needed. |
| `collections.circbuf_int_pop(buf: circbuf<int>) -> optional<int>` | Removes and returns the next integer when present. |
| `collections.circbuf_int_peek(buf: circbuf<int>) -> optional<int>` | Reads the next integer without removing it. |
| `collections.circbuf_int_len(buf: circbuf<int>) -> int` | Returns current integer count. |
| `collections.circbuf_int_capacity(buf: circbuf<int>) -> int` | Returns integer buffer capacity. |
| `collections.circbuf_int_is_full(buf: circbuf<int>) -> bool` | Checks whether the integer buffer is full. |
| `collections.circbuf_int_is_empty(buf: circbuf<int>) -> bool` | Checks whether the integer buffer is empty. |
| `collections.circbuf_int_values(buf: circbuf<int>) -> list<int>` | Returns buffered values from oldest to newest. |
| `collections.circbuf_text_new(capacity: int) -> circbuf<text>` | Creates a text circular buffer with fixed capacity. |
| `collections.circbuf_text_push(buf: circbuf<text>, value: text) -> circbuf<text>` | Pushes text, wrapping when needed. |
| `collections.circbuf_text_pop(buf: circbuf<text>) -> optional<text>` | Removes and returns the next text value when present. |
| `collections.circbuf_text_peek(buf: circbuf<text>) -> optional<text>` | Reads the next text value without removing it. |
| `collections.circbuf_text_len(buf: circbuf<text>) -> int` | Returns current text count. |
| `collections.circbuf_text_capacity(buf: circbuf<text>) -> int` | Returns text buffer capacity. |
| `collections.circbuf_text_is_full(buf: circbuf<text>) -> bool` | Checks whether the text buffer is full. |
| `collections.circbuf_text_is_empty(buf: circbuf<text>) -> bool` | Checks whether the text buffer is empty. |
| `collections.circbuf_text_values(buf: circbuf<text>) -> list<text>` | Returns buffered values from oldest to newest. |

## BTree Map and Set

| API | Description |
| --- | --- |
| `collections.btreemap_text_new() -> btreemap<text,text>` | Creates an empty text-to-text map. |
| `collections.btreemap_text_set(self_map: btreemap<text,text>, key: text, value: text) -> btreemap<text,text>` | Returns a map with `key` set to `value`. |
| `collections.btreemap_text_get(self_map: btreemap<text,text>, key: text) -> text` | Reads a value, using the current fallback behavior when absent. |
| `collections.btreemap_text_get_optional(self_map: btreemap<text,text>, key: text) -> optional<text>` | Reads a value as `optional<text>`. |
| `collections.btreemap_text_contains(self_map: btreemap<text,text>, key: text) -> bool` | Checks whether a key exists. |
| `collections.btreemap_text_remove(self_map: btreemap<text,text>, key: text) -> btreemap<text,text>` | Returns a map without `key`. |
| `collections.btreemap_text_len(self_map: btreemap<text,text>) -> int` | Returns the number of entries. |
| `collections.btreemap_text_is_empty(self_map: btreemap<text,text>) -> bool` | Checks whether the map is empty. |
| `collections.btreemap_text_keys(self_map: btreemap<text,text>) -> list<text>` | Returns keys sorted by text key order. |
| `collections.btreemap_text_values(self_map: btreemap<text,text>) -> list<text>` | Returns values following sorted key order. |
| `collections.btreeset_text_new() -> btreeset<text>` | Creates an empty text set. |
| `collections.btreeset_text_insert(set: btreeset<text>, value: text) -> btreeset<text>` | Returns a set with `value` inserted. |
| `collections.btreeset_text_contains(set: btreeset<text>, value: text) -> bool` | Checks whether a value exists in the set. |
| `collections.btreeset_text_remove(set: btreeset<text>, value: text) -> btreeset<text>` | Returns a set without `value`. |
| `collections.btreeset_text_len(set: btreeset<text>) -> int` | Returns the number of values. |
| `collections.btreeset_text_is_empty(set: btreeset<text>) -> bool` | Checks whether the set is empty. |
| `collections.btreeset_text_values(set: btreeset<text>) -> list<text>` | Returns values in sorted text order. |
