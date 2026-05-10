# Ori Language Specification — Chapter 12: Standard Library Contracts

> Status: normative (contracts); informative (usage examples)
> Audience: stdlib implementers, compiler implementers

---

## Overview

The Ori standard library lives in the `ori.*` namespace hierarchy.
It is available in every Ori program without explicit installation.

Stdlib modules are imported explicitly:

```ori
import ori.io as io
import ori.fs as fs
import ori.iter as iter
```

The stdlib is small and layered:
- **Core types** (`optional`, `result`, `list`, etc.) — always available, no import.
- **Foundation modules** — general-purpose utilities.
- **Domain modules** — specific areas (networking, JSON, etc.).

---

## Core (Always Available)

No import required. These types and functions are built into the language.

### Built-in Types

`bool`, `int`, `int8`–`int64`, `u8`–`u64`, `float`, `float32`–`float64`,
`string`, `bytes`, `void`, `list<T>`, `map<K,V>`, `set<T>`, `optional<T>`,
`result<T,E>`, `range<T>`, `lazy<T>`, `any<Trait>`, `tuple<...>`

### Built-in Functions

```ori
len(collection)              -- int: length of list, map, set, string, bytes
string(value)                -- string: convert any Displayable value (calls to_string())
int(value)                   -- int: convert float or numeric string to int
float(value)                 -- float: convert int or numeric string to float
u8(value)                    -- u8: explicit narrowing conversion
-- ... all numeric conversions follow the same pattern
```

### Built-in Traits (in `ori.core`)

`Displayable`, `Equatable`, `Comparable`, `Hashable`, `Disposable`,
`Iterable<Item>`, `Default`, `From<Other>`, `Error`, `Cloneable`

---

## `ori.io` — Basic Input/Output

```ori
import ori.io as io

io.print(value: any<Displayable>)                    -> result<void, ori.Error>
io.print(value: any<Displayable>, to: ori.io.Writer) -> result<void, ori.Error>
io.read_line()                                       -> result<string, ori.Error>
io.write(text: string)                               -> result<void, ori.Error>
io.write(text: string, to: ori.io.Writer)            -> result<void, ori.Error>

-- Standard writers
ori.io.stdout: ori.io.Writer
ori.io.stderr: ori.io.Writer
```

---

## `ori.fs` — File System

```ori
import ori.fs as fs

fs.read_text(path: string)             -> result<string, ori.fs.Error>
fs.read_bytes(path: string)            -> result<bytes, ori.fs.Error>
fs.write_text(path: string, content: string) -> result<void, ori.fs.Error>
fs.write_bytes(path: string, data: bytes)    -> result<void, ori.fs.Error>
fs.exists(path: string)                -> bool
fs.delete(path: string)                -> result<void, ori.fs.Error>
fs.list_dir(path: string)              -> result<list<string>, ori.fs.Error>
fs.open_read(path: string)             -> result<ori.fs.File, ori.fs.Error>
fs.open_write(path: string)            -> result<ori.fs.File, ori.fs.Error>
fs.read_all(file: ori.fs.File)         -> result<string, ori.fs.Error>

-- ori.fs.File implements Disposable; use with `using`
```

---

## `ori.string` — String Operations

```ori
import ori.string as string

string.len(s: string)                         -> int
string.concat(a: string, b: string)           -> string
string.split(s: string, sep: string)          -> list<string>
string.contains(s: string, sub: string)       -> bool
string.starts_with(s: string, prefix: string) -> bool
string.ends_with(s: string, suffix: string)   -> bool
string.trim(s: string)                        -> string
string.trim_start(s: string)                  -> string
string.trim_end(s: string)                    -> string
string.to_upper(s: string)                    -> string
string.to_lower(s: string)                    -> string
string.replace(s: string, from: string, to: string) -> string
string.slice(s: string, range: range<int>)    -> string
string.chars(s: string)                       -> list<string>
string.to_bytes(s: string)                    -> bytes
string.from_bytes(b: bytes)                   -> result<string, string>
string.parse_int(s: string)                   -> result<int, string>
string.parse_float(s: string)                 -> result<float, string>
```

---

## `ori.bytes` — Byte Operations

```ori
import ori.bytes as bytes

bytes.len(b: bytes)                          -> int
bytes.concat(a: bytes, b: bytes)             -> bytes
bytes.slice(b: bytes, range: range<int>)     -> bytes
bytes.to_hex(b: bytes)                       -> string
bytes.from_hex(s: string)                    -> result<bytes, string>
bytes.decode_utf8(b: bytes)                  -> result<string, string>
bytes.get(b: bytes, index: int)              -> u8
```

---

## `ori.iter` — Functional Collection Operations

Generic: works for any `list<T>` with the appropriate element type.

```ori
import ori.iter as iter

iter.map<T, R>(values: list<T>, mapper: func(T) -> R) -> list<R>
iter.filter<T>(values: list<T>, predicate: func(T) -> bool) -> list<T>
iter.reduce<T, R>(values: list<T>, initial: R, reducer: func(R, T) -> R) -> R
iter.flat_map<T, R>(values: list<T>, mapper: func(T) -> list<R>) -> list<R>
iter.find<T>(values: list<T>, predicate: func(T) -> bool) -> optional<T>
iter.any<T>(values: list<T>, predicate: func(T) -> bool) -> bool
iter.all<T>(values: list<T>, predicate: func(T) -> bool) -> bool
iter.count_where<T>(values: list<T>, predicate: func(T) -> bool) -> int
iter.zip<A, B>(a: list<A>, b: list<B>) -> list<tuple<A, B>>
iter.flatten<T>(nested: list<list<T>>) -> list<T>
iter.take<T>(values: list<T>, n: int) -> list<T>
iter.skip<T>(values: list<T>, n: int) -> list<T>
iter.partition<T>(values: list<T>, predicate: func(T) -> bool) -> tuple<list<T>, list<T>>
iter.sort<T>(values: list<T>) -> list<T> where T is Comparable
iter.sort_by<T>(values: list<T>, compare: func(T, T) -> Order) -> list<T>
iter.reverse<T>(values: list<T>) -> list<T>
iter.unique<T>(values: list<T>) -> list<T> where T is Equatable
iter.group_by<T, K>(values: list<T>, key: func(T) -> K) -> map<K, list<T>>
    where K is Hashable and K is Equatable
```

All `iter.*` functions are **eager**: they return a new `list<T>` immediately.
For lazy evaluation, wrap with `lazy<T>`.

---

## `ori.math` — Mathematics

```ori
import ori.math as math

math.abs(x: int) -> int
math.abs(x: float) -> float
math.min(a: int, b: int) -> int
math.max(a: int, b: int) -> int
math.min(a: float, b: float) -> float
math.max(a: float, b: float) -> float
math.clamp(value: int, min: int, max: int) -> int
math.floor(x: float) -> float
math.ceil(x: float) -> float
math.round(x: float) -> float
math.sqrt(x: float where x >= 0.0) -> float
math.pow(base: float, exp: float) -> float
math.log(x: float where x > 0.0) -> float
math.log2(x: float where x > 0.0) -> float
math.sin(x: float) -> float
math.cos(x: float) -> float
math.pi: float
math.e: float
math.infinity: float
math.nan: float
math.is_nan(x: float) -> bool
math.is_infinite(x: float) -> bool
```

---

## `ori.format` — Presentation Formatting

```ori
import ori.format as format

format.number(value: float, decimals: int = 0) -> string
format.percent(value: float, decimals: int = 0) -> string
format.date(millis: int, style: string = "iso") -> string
format.datetime(millis: int, style: string = "short", locale: string = "") -> string
format.hex(value: int) -> string
format.binary(value: int) -> string
format.bytes_size(bytes: int, style: ori.format.BytesStyle = .Decimal) -> string

enum BytesStyle
    Binary     -- KiB, MiB, GiB
    Decimal    -- KB, MB, GB
end
```

---

## `ori.time` — Time

```ori
import ori.time as time

time.now() -> int               -- Unix timestamp in milliseconds
time.sleep(millis: int)         -- block current thread
time.duration_ms(start: int, end: int) -> int
```

---

## `ori.random` — Random Numbers

```ori
import ori.random as random

random.int(min: int, max: int) -> int       -- inclusive range
random.float(min: float, max: float) -> float
random.bool() -> bool
random.shuffle<T>(items: list<T>) -> list<T>
random.choice<T>(items: list<T>) -> optional<T>
```

---

## `ori.json` — JSON

```ori
import ori.json as json

json.parse(text: string) -> result<ori.json.Value, string>
json.stringify(value: ori.json.Value) -> string
json.stringify(value: ori.json.Value, pretty: bool) -> string
```

---

## `ori.test` — Testing

```ori
import ori.test as test

-- Test functions are marked with attr:
@test
func test_addition()
    check 1 + 1 == 2
end

-- Assertions:
test.assert(condition: bool, message: string)
test.assert_eq<T>(a: T, b: T) where T is Equatable and T is Displayable
test.assert_ne<T>(a: T, b: T) where T is Equatable and T is Displayable
test.fail(message: string)
```

Run tests with `ori test`.

---

## `ori.os` — Operating System

```ori
import ori.os as os

os.args() -> list<string>        -- command-line arguments
os.env(name: string) -> optional<string>
os.exit(code: int)
os.pid() -> int
os.platform() -> string          -- "linux", "windows", "macos"
os.arch() -> string              -- "x86_64", "aarch64", etc.
```

---

## `ori.Error` — Standard Error Type

`ori.Error` is the standard error type for stdlib operations.

```ori
struct ori.Error
    message: string
    cause: optional<any<Error>>
end

implement Error for ori.Error
    func message() -> string
        return self.message
    end
    func cause() -> optional<any<Error>>
        return self.cause
    end
end
```

Most `ori.*` functions that can fail return `result<T, ori.Error>`.
