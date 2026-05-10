# Zenith Runtime Model

## Architecture

```
.zt -> C -> native executable
```

NOT selected: `.zt -> bytecode -> VM`

## Memory Management

**Automatic Reference Counting (ARC)** without tracing GC.

- Default: **Non-Atomic ARC** (single-isolate, latency-predictable)
- Concurrency: **Isolates** (message-passing / deep copy between boundaries)
- Atomic RC: restricted to explicit wrappers (e.g. `Shared<T>`)

### RC Cycles

- RC cycles = known leak risk, NOT undefined behavior
- Rich callbacks, UI graphs, game object graphs = not stable until cycle policy exists
- Future cycle policy: `weak<T>`, handles/arenas, constrained ownership graphs, or cycle collection
- Runtime must document which APIs can create cycles before they become official

## Managed Values

Tracked by ARC:
- `text`, `bytes`
- `list<T>`, `map<K, V>`
- `optional<T>` when T is managed
- `result<T, E>` when either side is managed
- Structs with managed fields
- Enums with managed payloads

Primitive list elements use contiguous value storage:
- `list<int>` stores `zt_int[]`
- `list<float>` stores `zt_float[]`
- `list<bool>` stores `zt_bool[]`
- `list<int8>`, `list<int16>`, `list<int32>` and `list<int64>` store fixed-width signed arrays
- `list<u8>`, `list<u16>`, `list<u32>` and `list<u64>` store fixed-width unsigned arrays
- Element operations copy scalar values directly
- There is no ARC retain/release per primitive element

Primitive optionals use stack/in-place value storage:
- `optional<int>` and `optional<int64>` use `zt_optional_i64`
- `optional<float>` uses `zt_optional_f64`
- `optional<bool>` uses `zt_optional_bool`
- `optional<int8>`, `optional<int16>` and `optional<int32>` use fixed-width signed payloads
- `optional<u8>`, `optional<u16>`, `optional<u32>` and `optional<u64>` use fixed-width unsigned payloads
- `none`, `is_some`/`is_none`, unwrap, and `or` do not allocate for these scalar payloads
- There is no ARC retain/release for primitive optional payloads

## Value Semantics Implementation

Assignment and parameter passing create semantic value copies.

Implementation may use:
- Direct copy for scalars
- Retain/release for immutable managed values
- **Copy-on-write (COW)** for mutable managed containers
- Internal move when source is provably unused

Observable rule:
```zt
var b: list<int> = a
b[0] = 10
-- a must NOT change
```

## Stack vs Heap

- `optional<T>` and `result<T,E>` should be stack/in-place when practical
- Scalar optionals/results should NOT allocate
- `result<void, E>` success should NOT allocate
- Managed payloads may store RC-managed pointers internally
- Heap allocation acceptable for escaping values, oversized payloads, managed payload internals
- Heap-first wrappers = performance debt (tracked explicitly)

## Ownership Requirements

Runtime/compiler must handle:
- Local scope exit
- Normal `return`
- Early return through `?`
- Branch exits
- Loop exits (`break`, `continue`)
- Temporaries until end of statement
- Function argument evaluation (left-to-right)
- Construction failure
- Contract panic (where cleanup is viable)

## Runtime Checks

| Check | Trigger |
|-------|---------|
| Bounds | List/text/bytes index out of range |
| Map key | Direct lookup with missing key |
| Division by zero | Arithmetic operation |
| Contract `where` | Field/parameter constraint violation |
| UTF-8 conversion | Invalid UTF-8 data |
| Allocation | Memory allocation failure |
| Platform | Host/runtime platform failure |

Expected failures become `result<T, E>` in stdlib APIs. Broken invariants and direct invalid access produce panic.

## Panic

Zenith injects **Panic with Unwinding** (or controlled abort) rather than undefined behavior.

Rules:
- Indexing out-of-bounds or arithmetic overflow inject unconditional panic
- Panic = fatal language control flow
- Panic output uses diagnostics model when source span and value context available
- Panic is NOT caught by result/optional flow
- Avoids segfaults and heap corruption

## Runtime Contracts

Value-level `where` contracts run at runtime:

```zt
struct User
    age: int where it >= 0
end

-- Constructor does NOT become result<T, E>
const user: User = User(age: 25)   -- panics if where fails
```

Required MVP sites:
- Struct field construction
- Struct field assignment
- Function/method parameter boundary

Failed contracts report `runtime.contract`.

### Recoverable Validation Pattern

```zt
struct Player
    hp: int where it >= 0
end

func try_create_player(hp: int) -> result<Player, text>
    if hp < 0
        return error("hp must be >= 0")
    end
    return success(Player(hp: hp))
end
```

## Const Collections

```zt
const items: list<Player> = [Player(name: "Julia", hp: 100)]
items[0] = Player(name: "Julia", hp: 80)   -- ERROR: const is immutable
items[0].hp = 80                            -- ERROR: deep mutation not allowed
```

`const` collections are observably immutable, including through indexed access.

## Concurrency Model (Deferred)

- Cross-thread work uses isolate/message-passing boundaries
- Non-atomic ARC cannot cross thread isolates
- Safe deep-transfer structure required for cross-isolate communication
- `Shared<T>` for explicit atomic RC wrappers
