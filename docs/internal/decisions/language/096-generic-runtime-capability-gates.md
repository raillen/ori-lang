# Decision 096 - Generic Runtime Capability Gates

- Status: accepted
- Date: 2026-05-10
- Type: implementation-only
- Scope: `0.4.2-beta.rc1`, generic runtime capability checks
- Upstream: `docs/internal/planning/0.4.2-beta.rc1-language-gap-implementation-plan.md`

## Context

The language already has public semantic traits such as `Equatable`,
`Hashable`, `Comparable`, and `Transferable`.

The C backend, however, only has materialized collection helpers for a smaller
runtime subset. Before this decision, some shapes could pass type checking and
then fail later during lowering or C emission.

That is hard to understand and it makes the generic implementation less safe.

## Decision

For `0.4.2-beta.rc1`, the checker owns an internal runtime capability gate.

This gate is separate from public traits.

The first implemented capabilities are:

- copy;
- destroy;
- equality;
- stable hash;
- stable ordering;
- transfer;
- materialized hash key;
- materialized order key;
- materialized payload.

For the first implementation slice, materialized collection keys and payloads
match the backend that exists today:

- `map<K, V>` keys: `int` and `text`;
- `set<T>` elements: `int` and `text`;
- `grid2d<T>`, `grid3d<T>`, and `circbuf<T>` payloads: `int` and `text`;
- `pqueue<T>` ordering: `int` and `text`;
- `btreemap<K, V>` and `btreeset<T>` stay on the existing `text` runtime path.

Unsupported materialized shapes fail during `check`.

The second implementation slice expands materialized hash keys to safe
structural values where the C backend now emits concrete `zt_elem_ops`
callbacks:

- `set<Struct>` elements with fields limited to `bool`, integral types, and
  `text`;
- `map<Struct, V>` keys with fields limited to `bool`, integral types, and
  `text`, for currently materialized value payloads;
- tuple-backed generated structs with the same field subset;
- generated `hash` and `equals` callbacks for those structs;
- generated `std.set` wrappers for `add`, `has`, and `remove` on that generic
  path.
- generated `std.map` wrappers for `set`, `has_key`/`contains`, and `remove`
  on that generic path.

Standalone `set<bool>`, `set<bytes>`, `map<bool, V>`, and `map<bytes, V>`
remain closed until the backend exposes those shapes deliberately.

Maps that contain a user struct in the key or value use generic runtime
storage consistently. Direct indexing still returns `V` or panics on a missing
key. Recoverable lookup remains `map.get`, which returns `optional<V>`.

Generated helper names for generic runtime shapes must be collision-safe.

For this release train, C helper names use the readable sanitized type shape
plus a stable hash suffix. The hash input is the canonical compiler/runtime
type identity, not only the last visible source segment.

This is required because two namespaces may both define a struct named `Flag`.
`list<left.Flag>`, `list<right.Flag>`, `set<left.Flag>`, `set<right.Flag>`,
`map<left.Flag, int>`, and `map<right.Flag, int>` must lower to distinct C
helpers even when their simple names look similar.

## Rationale

The checker is the earliest place where the compiler has enough type context to
explain the problem clearly.

This also prevents the emitter from accidentally accepting a shape that the
runtime cannot materialize yet.

Keeping this as an internal gate lets the language grow without promising a new
public trait API too early.

## Not Decided Yet

These are still Topic 2 follow-up work:

- materialized `bytes` keys;
- standalone materialized `bool` keys;
- structural `map.keys`, `map.values`, and `map.merge`;
- ordered collection support beyond the current `int` and `text` backend;
- public derive syntax;
- public capability traits such as `Cloneable` or `Orderable`.
- a public spelling for generated helper names. The current hash suffix is an
  internal C backend detail.

## User-Facing Diagnostic Rule

Diagnostics should describe the missing operation in plain language.

Example:

```text
map key type 'bool' needs a materialized stable hash/equality capability;
supported key shapes in this backend subset are int, text, and safe structs/tuples with bool/int/text fields
```

The diagnostic may mention a capability, but it should not imply that a new
public trait must already exist.

## Validation

Validated in the first implementation slice:

- `python build.py`
- `.\zt.exe check zenith.ztproj --all --ci`
- `python run_suite.py smoke --no-perf`
- `python tools\check_docs_paths.py`
- `python tools\check_docs_current_syntax.py`

Validated in the second implementation slice:

- `python build.py`
- `.\zt.exe check tests\behavior\set_struct_key_basic\zenith.ztproj --all --ci`
- `.\zt.exe run tests\behavior\set_struct_key_basic\zenith.ztproj --ci --native-raw`
- `.\zt.exe run tests\behavior\map_struct_key_basic\zenith.ztproj --ci --native-raw`
- `.\zt.exe check tests\behavior\map_struct_unsupported_key_error\zenith.ztproj --all --ci`
- `.\zt.exe run tests\behavior\map_struct_expected_type\zenith.ztproj --ci --native-raw`

Validated for collision-safe helper identity:

- `.\zt.exe run tests\behavior\generic_helper_name_collision_safe\zenith.ztproj --ci --native-raw`
