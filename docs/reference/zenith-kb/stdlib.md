# Zenith Standard Library

## Module Set (MVP)

| Module | Purpose |
|--------|---------|
| `std.io` | Input/output, print, file handles |
| `std.bytes` | Byte manipulation utilities |
| `std.fs` | File system operations |
| `std.fs.path` | Path operations |
| `std.json` | JSON parsing and serialization |
| `std.math` | Math functions |
| `std.regex` | Simple regex validation and search |
| `std.text` | Text manipulation, UTF-8 conversion |
| `std.list` | Small helpers for built-in lists |
| `std.map` | Small helpers for built-in maps |
| `std.set` | Small helpers for built-in sets |
| `std.random` | Random number generation |
| `std.validate` | Validation helpers |
| `std.time` | Time, duration, instant |
| `std.format` | Formatting helpers |
| `std.os` | OS info, platform detection |
| `std.os.process` | Process management |
| `std.test` | Test utilities: fail, skip, bool checks and int/text comparisons |

Deferred: `std.net` (next wave after core stdlib)

## Import Style

```zt
import std.io as io
import std.text as text
import std.fs as fs
import std.fs.path as path
import std.json as json
import std.math as math
import std.regex as regex
import std.random as random
import std.time as time
import std.format as format
import std.os as os
import std.os.process as process
import std.validate as validate
import std.list as list
import std.map as map
import std.set as set
```

Rules:
- All stdlib modules imported explicitly
- Core language traits are implicit (no import needed)
- Use `as` alias matching last namespace segment

## Error Policy

Each side-effecting/fallible module owns its error type:

| Module | Error Type |
|--------|------------|
| `std.io` | `io.Error` |
| `std.fs` | `fs.Error` |
| `std.json` | `json.Error` |
| `std.regex` | `regex.Error` |
| `std.time` | `core.Error` for host sleep failures |
| `std.os` | `os.Error` |
| `std.os.process` | `process.Error` |

Rules:
- Expected failure returns `result<T, Module.Error>`
- Expected absence returns `optional<T>`
- Timeout is an error variant, NOT panic
- Invalid direct indexing may panic
- Safe lookup APIs return `optional<T>`

## Module-Owned Types

### std.io
- `io.Input`, `io.Output`, `io.Error`

### std.json
- `json.Value`, `json.Object`, `json.Array`, `json.Kind`, `json.Error`

### std.time
- `time.Instant`, `time.Duration`
- Clock helpers: `now`, `now_ms`
- Duration helpers: `milliseconds`, `seconds`, `minutes`, `hours`
- Difference helpers: `diff`, `elapsed`, `since`, `until`
- Sleep helpers: `sleep`, `sleep_ms`

### std.fs
- `fs.Metadata`, `fs.Error`
- Main helpers: `exists`, `is_file`, `is_dir`, `create_dir`, `create_dir_all`, `remove_file`, `remove_dir`, `copy`, `rename`, `list_dir`, `file_size`
- Compatibility names remain available: `copy_file`, `move`, `size`

### std.math
- Constants: `pi`, `e`, `tau`
- Core helpers: `abs`, `min`, `max`, `clamp`, `pow`, `sqrt`
- Rounding: `floor`, `ceil`, `round`, `trunc`
- Trigonometry: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- Logs and exponentials: `ln`, `log_ten`, `log2`, `log`, `exp`
- Float checks: `is_nan`, `is_infinite`, `is_finite`

### std.regex
- Types: `regex.Regex`, `regex.Error`
- Core helpers: `compile`, `is_valid`, `is_match`, `contains`, `matches`, `full_match`, `first`, `count`, `find_all`, `split`, `replace_all`, `escape`
- Current engine covers simple ASCII-oriented patterns: literals, `.`, `^`, `$`, `*`, `+`, `?`, character classes/ranges and `\d`, `\w`, `\s`
- Groups, captures, flags and full Unicode regex are deferred

### std.format
- `format.BytesStyle`

### std.os
- `os.Platform`, `os.Arch`, `os.Error`

### std.os.process
- `process.Output`, `process.Error`

## std.random (public var example)

`std.random` exposes read-only observable module state:

```zt
import std.random as random

-- Read-only state
random.seeded        -- bool: has the generator been seeded?
random.last_seed     -- int: the last seed value used
random.draw_count    -- int: total draws since seed
random.stats()       -- returns statistics
```

Rules:
- External write to `public var` is blocked
- Mutation only through explicit `public func` APIs
- Tests must reseed/reset state to avoid flakes

## Safe Collections API

```zt
const score_key: text = "Julia"
const scores: map<text, int> = {score_key: 10}
const values: list<int> = []

-- Direct access PANICS if key missing
const score: int = scores["Julia"]

-- Safe lookup returns optional
const maybe_score: optional<int> = scores.get(score_key)
const maybe_item: optional<int> = values.get(3)
const maybe_first: optional<int> = list.first(values)
const maybe_last: optional<int> = list.last(values)
const without_first: list<int> = list.rest(values)
const without_two: list<int> = list.skip(values, 2)
const has_julia: bool = map.has_key(scores, score_key)
const no_values: bool = list.is_empty(values)

import std.set as set

var tags: set<text> = set.empty()
set.add(tags, "docs")
set.add(tags, "language")

const more_tags: set<text> = set.of("docs", "stdlib")
const all_tags: set<text> = set.union(tags, more_tags)
const has_docs: bool = set.has(all_tags, "docs")
```

## Naming Policy

- Prefer clear verbs and nouns over abbreviated cleverness
- Mutating methods declared with `mut func`
- Safe lookup functions: `get`, `find`, `try_` prefix when distinction needed
- Path operations live in `std.fs.path`, NOT `std.fs`

## Namespace Mutable State Policy

For stdlib modules using `public var`:
- Prefer `public const` by default
- `public var` only at namespace top-level
- External read through qualified import
- Block external write outside owner namespace
- `public` = visibility only, not `global`
- Mutation exposed by explicit `public func` APIs
- Tests must reseed/reset state

## Implementation Gate

A stdlib function is complete only when it has:
- Public signature in docs
- Runtime/backend implementation
- Typed error behavior if fallible
- Behavior tests
- Diagnostic tests when misuse is likely
- Examples using canonical import style
