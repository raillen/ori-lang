# Zenith v1 Surface Contract

> Audience: contributor, maintainer
> Status: historical baseline
> Surface: spec
> Source of truth: no
> Last updated: 2026-04-29

This document records the historical Zenith v1 baseline. Current final language
decisions may supersede this file, especially decisions recorded in
`post-v1-remaining-language-work.md` and later post-v1 closure artifacts.

Upstream sources: `surface-implementation-status.md`,
`language-readiness-surface-contract.md`, `language-reference.md`,
`post-v1-remaining-language-work.md`.

---

## Status Definitions

- **Shipped**: implemented, tested, documented, formatter/LSP aware.
- **Required**: must be shipped before v1.
- **Accepted**: decision made; implementation is tracked work.
- **Deferred**: explicitly not v1; will not block release.

---

## Language Surface

### Shipped (Conformant in Current Alpha)

| Feature | Evidence |
|---------|----------|
| Namespaces and qualified imports | behavior tests |
| Multifile packages with `zenith.ztproj` | behavior tests |
| Functions (`func`, `-> Type`, `return`) | behavior tests |
| Control flow (`if/else if/else`, `while`, `for in`, `repeat N times`, `match/case/case else`) | behavior tests |
| Structs with fields, construction, field access | behavior tests |
| Traits and `apply ... to ...` | behavior tests |
| Methods (`func` and `mut func` on types) | behavior tests |
| Collections: `list<T>`, `map<K,V>` (index, slice, len, get) | behavior tests |
| `optional<T>` and `result<T,E>` | behavior tests |
| `?` propagation | behavior tests |
| `f"..."` / `fmt "..."` string interpolation | behavior tests; `f` is canonical |
| `panic`, `todo`, `unreachable`, `check` | behavior tests |
| `core.Error` qualified construction | behavior tests |
| Unsigned aliases (`u8`â€“`u64`) | behavior tests |
| `public var` at namespace scope | behavior tests |
| Closures v1 (immutable capture) | behavior tests |
| Lambdas v1 + int HOFs | behavior tests |
| Single-expression closures | behavior tests |
| Closure return type inference | behavior tests |
| Explicit `lazy<int/float/bool/text>` | behavior tests |
| Runtime value contracts (`where`) | behavior tests |
| `using` statement (block, flat, custom cleanup) | behavior tests |
| `case some(name)` / `case none` destructuring | behavior tests |
| `if-else` as expression | behavior tests |
| `type` aliases | behavior tests |
| Struct literal shorthand `{ fields }` | superseded by current final decision: rejected as bare type-omission syntax |
| Enum dot shorthand `.Variant` | behavior tests |
| `any<Trait>` dynamic dispatch | behavior tests |
| `<T: Trait>` inline constraints | behavior tests |
| `given` clause for complex constraints | behavior tests |
| `match` with `:` delimiter and `case else` | behavior tests |
| `then` and `given` as contextual keywords | behavior tests |
| `capture` keyword in closures | behavior tests |
| Trait default implementations | behavior tests |

### Required Before v1 (Currently Accepted, Not Fully Implemented)

| Feature | Source | Complexity |
|---------|--------|------------|
| Complete `list<T>` explicit API beyond the current C backend `list<int>`/`list<text>` executable subset | v8 COL.01 | Medium |
| Complete `map<K,V>` explicit API beyond current C backend `map<text,text>` value-style subset (`map.entries`, generic `set/remove/keys/values/merge`) | v8 COL.02 | Medium |
| Complete `set<T>` explicit API (`set.add/remove/contains/union/intersection/difference`) | v8 COL.03 | Medium |
| Complete `text` API (`to_lower/to_upper/capitalize/split/replace`) | v8 COL.04 | Medium |
| Complete `bytes` safe API (`bytes.get/slice/index_of/concat`) | v8 COL.05 | Low |
| Generic HOFs beyond the current `list<int>` executable subset: `list.map/filter/find/any/all/count/sort_by` | v8 COL.06 | High |
| Collection diagnostics (assertive direct + safe API two-layer model) | v8 COL.07 | Medium |
| `tuple<...>` positional product types | v1 tuple reversal | High |
| `fs.read_bytes/write_bytes` | v8 IO.01 | Low |
| `fs.walk_dir` | v8 IO.02 | Medium |
| `path.extension -> optional<text>`, `path.parent -> optional<text>` | v8 IO.03 | Low |
| `/` as canonical path separator | v8 IO.04 | Medium |
| `std.io` returns `io.Error` (not `core.Error`) | v8 IO.06 | Medium |
| Structured diagnostics (`diagnostic.Diagnostic/Report`) | v8 DIA.01-06 | High |
| `regex.try_first/try_split/try_replace_all` | v8 REG.05 | Medium |

### Deferred (Explicitly Not v1)

| Feature | Reason | Source |
|---------|--------|--------|
| `std.net` TLS/UDP/server APIs | v1 ships blocking TCP client only | v1 M12 decision |
| `std.mem.Allocator`, arenas | Superseded: advanced allocation is accepted as explicit `std.mem` library API, not syntax | post-v1 remaining work |
| `async/await` keywords | Rejected; jobs/channels are library/runtime APIs | post-v1 concurrency closure |
| `owned<T>`, `borrow<T>`, lifetimes | Changes language identity | v7 DiÃ¡logo Futuro |
| Full type inference (`const x = 42`) | Requires inference engine | v7 DiÃ¡logo Futuro |
| Full local type inference remains rejected; argument-position generic inference is now accepted/implemented for the supported subset | Explicitness boundary | post-v1 monomorphization closure |
| LLVM backend | Post-v1 | v7 DiÃ¡logo Futuro |
| WASM backend | Post-v1 | v8 FUT.12 |
| JS backend | Post-v1 | v8 FUT.13 |
| Macros | Not in language philosophy | v7 Rejected |
| Broad method/function overloading | Rejected; fixed operator traits are a separate accepted Level 2 surface | post-v1 trait stability |
| `char` type | `text` with helpers suffices | v7 Rejected |
| `?.` safe navigation | Dense symbol | v7 Rejected |
| `try/catch` | Uses `result<T,E>` + `?` | v7 Rejected |

### Accepted Tuple Surface

`tuple` is the canonical name for fixed-size positional product types.

```zt
func min_max(values: list<int>) -> tuple<int, int>
    ...
end

const pair: tuple<text, int> = ("score", 10)
```

Rules:

- A tuple has a fixed arity.
- Position is part of the type: `tuple<int, text>` differs from `tuple<text, int>`.
- Each position may use any valid non-`void` type, including `list<T>`, `map<K,V>`, `set<T>`, structs, enums, `optional<T>`, `result<T,E>`, `any<Trait>`, and callable types once callable type syntax is stable.
- Tuples are for small temporary values, multiple returns, and collection entries.
- Public domain data should still prefer `struct`.
- Named tuple fields are not part of v1. Use `struct` when names are needed.
- `group` exists as an accepted alias in later closure work, while `tuple` remains the canonical contract name.
- Current implementation status: tuple literals and `tuple<...>` instantiations
  lower to generated C structs. Positional field access syntax is still a
  separate surface item.

---

## Standard Library v1 Surface

### Shipped

| Module | Key APIs | Status |
|--------|----------|--------|
| `std.io` | `read_line`, `read_all`, `write`, `print`, `to_core_error` | Shipped (`io.Error`) |
| `std.fs` | `read_text/write_text/append_text`, `read_bytes/write_bytes`, `exists/is_file/is_dir`, `create_dir/_all`, `remove_file/dir/_all`, `copy_file/move`, `list_dir/walk_dir`, `metadata/size` | Shipped |
| `std.fs.path` | `join/normalize/is_absolute/is_relative/absolute/relative/base_name/name_without_extension/extension/parent/has_extension/change_extension` | Shipped (`extension`/`parent` return `optional<text>`, `/` and `\` supported) |
| `std.math` | `abs/abs_int/min/max/clamp/pow/sqrt/floor/ceil/round/trunc/sin/cos/tan/asin/acos/atan/atan2/ln/log_ten/log2/log/exp/infinity/nan/is_nan/is_infinite/is_finite/deg_to_rad/rad_to_deg/approx_equal`, constants `pi/e/tau` | Shipped |
| `std.time` | `now/now_ms/elapsed/sleep/sleep_ms` | Shipped |
| `std.json` | `parse/stringify/pretty/read/write` (limited to `map<text,text>`) | Shipped |
| `std.format` | `number/percent/date/datetime/date_pattern/datetime_pattern/bytes/hex/bin` | Shipped |
| `std.validate` | Validation helpers | Shipped |
| `std.random` | `seed/next/between/float_between/choice/shuffle/stats` | Shipped (`choice`/`shuffle` support `list<int>` and `list<text>` in v1 C backend) |
| `std.int`, `std.float`, `std.bool` | Explicit primitive conversions and parse helpers | Shipped |
| `std.debug` | `size_of(value)`, `type_name(value)` | Shipped as compiler-known helpers for typed values |
| `std.regex` | `compile/is_match/matches/full_match/first/count/find_all/split/replace_all/escape`, plus `try_first/try_find_all/try_split/try_replace_all` | Shipped |
| `std.test` | `fail/skip/is_true/is_false/equal_int/equal_text/not_equal_int/not_equal_text` | Shipped |
| `std.os` | `args/env/pid/platform/arch/current_dir/change_dir` | Shipped |
| `std.os.process` | `run/run_capture` | Shipped |
| `std.net` | `connect/read_some/write_all/close/is_closed/kind/to_core_error` | Shipped (blocking TCP client, `net.Error`) |
| `std.http` | `get/post`, `Response`, `Error`, `ErrorKind` | Shipped (blocking HTTP only, no TLS) |
| `std.bytes` | `empty/from_list/to_list/join/concat/starts_with/ends_with/contains/get/slice/index_of/len/is_empty` | Shipped |
| `std.collections` | Queue/stack/grid/pqueue/circbuf/btreemap/btreeset + int HOFs | Shipped |
| `std.concurrent` | `copy_*` boundary helpers | Shipped |
| `std.lazy` | `once_*`/`force_*`/`is_consumed_*` for `int`, `float`, `bool`, and `text` | Shipped |
| `std.console` | Interactive terminal helpers | Shipped |

---

## Runtime v1 Contract

| Property | Current Status | v1 Target |
|----------|---------------|-----------|
| Memory model | ARC (non-atomic) | Same |
| Concurrency model | Single-isolate by default | Same |
| Value semantics | Copy-on-write for managed types | Same |
| RC cycles | Leak risk, no collector | Documented limitation |
| Panic model | Fatal, not caught by result/optional | Same |
| Overflow checking | Checked by default | Same |
| Code organization | Monolithic `zenith_rt.c` | Componentized (Unity Build) |
| C backend organization | Split backend internals under `compiler/targets/c/` | Modular C backend with stable `emitter.h` facade |

---

## Tooling v1 Surface

| Tool | Status |
|------|--------|
| `zt check/build/run` | Shipped |
| `zt test` with `attr test` | Shipped |
| `zt fmt` / `zt fmt --check` | Shipped |
| `zt doc check/show` | Shipped |
| `zt emit-c` | Shipped |
| `zt summary` | Shipped |
| `zt perf` | Shipped |
| Single-file mode | Shipped |
| `zpm init/add/install` | Shipped |
| VSCode extension on Marketplace | Pending (T.01) |
| LSP | Beta |

