# Zenith Standard Library Reference

> Audience: user
> Status: current
> Surface: public

## Import Model

Zenith uses explicit namespace imports.

```zt
import std.io as io
import std.text as text
```

Then call through the alias:

```zt
io.write("hello")?
```

## Core Modules

| Area | Modules | Use |
| --- | --- | --- |
| text and bytes | `std.text`, `std.bytes`, `std.format` | strings, binary data, display helpers |
| IO and filesystem | `std.io`, `std.fs`, `std.fs.path` | terminal, files, paths |
| data | `std.json`, `std.list`, `std.map`, `std.set`, `std.collections` | structured values and collections |
| validation | `std.validate`, `std.regex` | input checks and simple pattern work |
| math and time | `std.math`, `std.random`, `std.time` | numbers, randomness, clocks |
| process and OS | `std.os`, `std.os.process` | args, env, command execution |
| tests | `std.test` | test helpers for `attr test` |
| concurrency | `std.jobs`, `std.channels`, `std.shared`, `std.atomic`, `std.concurrent` | typed handles and explicit transfer |
| memory | `std.mem`, `std.orc`, `std.unsafe` | advanced low-level helpers |
| network | `std.net`, `std.http` | current network foundation |

## Common Tasks

### Write Text

```zt
import std.io as io

io.write("hello")?
io.write("\n")?
```

### Work With Text

```zt
import std.text as text

const cleaned: text = text.trim(raw)
const lower: text = text.to_lower(cleaned)
const found: optional<int> = text.index_of(lower, "zen")
```

### Work With JSON

```zt
import std.json as json

const value: json.Value = json.parse_value(raw)?
const pretty: text = json.pretty_value(value)
```

### Test Code

```zt
import std.test as test

attr test
func addition_works() -> void
    test.equal_int(2 + 2, 4)
end
```

## Appendix A API Notes

These notes summarize the current public stdlib correction boundary.

Text:

- `std.text.index_of` and `std.text.last_index_of` return `optional<int>`.
- Compatibility helpers keep sentinel behavior explicit: `index_of_or_minus_one` and `last_index_of_or_minus_one`.

Lists, maps, and collections:

- `std.list` value helpers are the normal public shape for `list<T>`.
- The executable backend supports the current primitive/text subset documented in the reference docs.
- `std.map` value helpers work for generated maps where keys are `int` or `text` and values are primitive or `text`.
- Safe structural map/set keys are supported when the key fields are limited to
  `bool`, integer types, and `text`.
- `std.collections` exposes iterable/snapshot helpers where the runtime has stable ordering.
- Advanced `std.collections` structures are a v1 subset, not arbitrary
  generics: `grid2d/grid3d`, `pqueue`, and `circbuf` support `int` and `text`;
  `btreemap` supports `text,text`; `btreeset` supports `text`.
- Fully generic `grid2d<T>`, `circbuf<T>`, `pqueue<T>`, `btreemap<K,V>`, and
  `btreeset<T>` are tracked as post-RC technical debt.
- Nested managed payloads in advanced collections, such as
  `grid2d<list<text>>`, are post-RC work and fail during `zt check`.

Math, OS, and validation:

- `std.math.nan()` and `std.math.infinity()` are functions, not constants, because the current constant model safely exposes finite float literals only.
- `std.os.args()` is the canonical terminal-argument API. With `zt run`, arguments after `--` are forwarded to the program.
- `std.validate` includes the baseline `int`/`text` helpers plus the current broader executable helpers for float, bool, supported optional/result state, list length, and supported map-size families.

## Concurrency Surface

Teach typed facades first:

- `Job<T>`
- `Channel<T>`
- `Shared<T>`
- `Atomic<T>`

The current runtime has a restricted executable subset.

Unsupported payload types should fail with clear diagnostics instead of falling into hidden runtime behavior.

Current executable boundary:

- `std.concurrent.copy_*` supports `int`, `bool`, `float`, `text`, `bytes`, `list<int>`, `list<text>`, and `map<text,text>`.
- `std.jobs`, `std.channels`, `std.shared`, and `std.atomic` expose typed handles, but the executable runtime path is `int` payloads only.
- `Shared<text>` and `Atomic<bool>` are intentionally rejected until typed runtime storage is widened.

## Lazy, HTTP And Debug

- `std.lazy` is one-shot and currently executable for `int`, `float`, `bool`, and `text`.
- Fully generic `lazy<T>` and lazy iterators are post-RC work; unsupported lazy
  payloads fail during `zt check`.
- `std.http` is a small blocking HTTP client: `get`, `post`, `Response.status`, `Response.body`, and typed errors are the stable v1 core. HTTPS, redirects, custom headers, streaming, and bytes bodies are future work.
- `std.debug.size_of(value)` and `std.debug.type_name(value)` are compiler-known helpers for typed values.

## Advanced Modules

Use these only when you need their exact contract:

- `std.mem`
- `std.orc`
- `std.unsafe`
- `std.shared`
- `std.atomic`

They are not the first path for normal app code.

Current executable boundary:

- `std.mem` supports concrete `text`/`list<text>` ownership helpers plus compiler-known `mem.own/view/edit` for the finalized Appendix B safe subset: primitive scalars, text, safe tuples/structs, primitive/text lists, list of safe tuples/structs, `set<int/text>`, and primitive/text-key maps with scalar/text values.
- `std.orc` supports text and `list<text>` inspection plus the current cycle-collection hook.
- `std.unsafe` supports text and `list<text>` heap/retain helpers.
- Enums, optional/result payloads, nested mutable managed values, tuple/struct set keys, managed map values, and allocator resources remain tracked in Appendix B of `docs/spec/language/implementation-plan.md`.
- `mem.Temp` and `mem.Pool<T>` are reserved future library resources, not usable APIs in this release.

## Deeper References

- `docs/reference/stdlib/`
- `stdlib/zdoc/`
- `docs/spec/language/stdlib-reference-by-topic.md`
