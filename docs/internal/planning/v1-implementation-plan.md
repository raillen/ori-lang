# Zenith v1.0 — Implementation Plan (Dependency-Ordered)

> Created: 2026-04-29
> Source: v1-completeness-discussion.md (54 modules)
> Status: closed historical plan; superseded by `docs/spec/language/final-language-contract.md`

---

## 1. Coherence Analysis

### 1.1 Incoherences Found

| #            | Problem                                                                                                  | Impact                    | Action                                                          |
| ------------ | -------------------------------------------------------------------------------------------------------- | ------------------------- | --------------------------------------------------------------- |
| **I1** | `JsonValue` is a **recursive enum** (`Array(list<JsonValue>)`) — must verify compiler support | 🔴 Blocks M41 + M46       | ✅ Verified: cycle detector won't fire (list`<T>` is pointer) |
| **I2** | `std.toml` depends on JSON expansion, but `.ztproj` is parsed by compiler (C)                        | 🟡 Unnecessary complexity | Parse `.ztproj` in C, `std.toml` as convenience             |
| **I3** | `choice<T>` and `shuffle<T>` look generic, but v1 C backend has specialized list runtime paths | 🟢 Closed for v1          | Implement as compiler-lowered `list<int>`/`list<text>` intrinsics; keep full user-function monomorphization post-v1 |
| **I4** | `std.format.table(any<TR>)` needs trait dispatch + introspection                                       | 🟡 High complexity        | Implement after TextRepresentable                               |
| **I5** | `type aliases` is a parser+semantic change — if done early, benefits everything                       | 🟢 Opportunity            | ✅ Already implemented                                          |
| **I6** | Inline if expression likely already works (used in `console.zt`)                                       | 🟢 Just verify            | ✅ Verified: works                                              |
| **I7** | `std.env` already exists as `os.env(name)` in `std.os`                                             | 🟢 No new module needed   | ✅ Confirmed                                                    |
| **I8** | `map.entries` needs a small product type; `tuple` decision was reopened and accepted | Medium-high language work | Add tuple parser/type/emitter tasks before full map entries |

### 1.2 Critical Paths

```
Path A: M21 (Runtime Generic) -> M02 (list HOFs) -> M42 (random.choice/shuffle via v1 list specializations)
Path B: M31 (Enum AV) → M41 (JsonValue) → M46 (std.toml) → M28 (.ztproj)
Path C: M31b (Tuple) -> M03 (`map.entries`) -> collection HOF/API polish
```

---

## 2. Execution Plan

### Tier 0: Verification & Quick Wins (1-2 days)

| Task                                     | Module  | Type      | Est. | Status      |
| ---------------------------------------- | ------- | --------- | ---- | ----------- |
| Verify inline if expression works        | M52     | Verify    | 1h   | ✅ Done     |
| Verify enum associated values e2e        | M31     | Verify    | 2h   | ✅ Done     |
| Verify recursive enum support            | I1      | Verify    | 2h   | ✅ Done     |
| Check if `std.os` has env vars         | I7      | Verify    | 30m  | ✅ Done     |
| Remove duplicate type table entries      | M23.D1  | Quick fix | 30m  | ✅ Done     |
| Fix `std.console` default message      | M44     | Quick fix | 10m  | ✅ Done     |
| Accept `std.math` / `std.time` as-is | M39/M40 | Accept    | 0    | ✅ Accepted |
| Accept operator precedence               | M49     | Accept    | 0    | ✅ Accepted |
| Accept module visibility model           | M50     | Accept    | 0    | ✅ Accepted |
| Accept for loop variants                 | M53     | Accept    | 0    | ✅ Accepted |
| Accept string escapes                    | M54     | Accept    | 0    | ✅ Accepted |
| Accept circular imports                  | M45     | Accept    | 0    | ✅ Accepted |

---

### Tier 1: Compiler Foundations (1-2 weeks)

| #   | Task                                                      | Module | Depends On | Est. | Status             |
| --- | --------------------------------------------------------- | ------ | ---------- | ---- | ------------------ |
| 1.1 | Fix `call_indirect` double evaluation bug               | M22.A  | —         | 2d   | ✅ Done            |
| 1.2 | Static immortal closures for func refs                    | M22.E  | —         | 2d   | ✅ Done            |
| 1.3 | Add number literals (`0xFF`, `0b1010`, `1_000_000`) | M47    | —         | 2d   | ✅ Already existed |
| 1.4 | Add multiline strings (`"""`)                           | M48    | —         | 2d   | ✅ Already existed |
| 1.5 | Implement `type Alias = T`                              | M51    | —         | 3d   | ✅ Already existed |
| 1.6 | Implement/verify inline if expression                     | M52    | —         | 1d   | ✅ Already existed |
| 1.7 | Implement `with` expression for structs                 | M33a   | —         | 3d   | ✅ Done            |
| 1.8 | Implement default field values for structs                | M33b   | —         | 2d   | ✅ Already existed |
| 1.9 | Implement tuple type/literal front-end                    | M31b   | -          | 3d   | ✅ Done via 2.14 |
| 1.10 | Implement tuple semantic checks and diagnostics          | M31b   | 1.9        | 3d   | ✅ Done via 2.14 |

Tier 1 closure note:

- 1.9 and 1.10 are closed through the tuple work finalized in 2.14. The parser recognizes tuple literals, `tuple<...>` resolves through semantic checks, HIR/ZIR carry tuple literals, diagnostics cover invalid tuple use through the type checker, and generated tuple structs are validated by `tuple_generated_struct_callbacks`.

---

### Tier 2: Runtime Architecture (2-3 weeks)

| #    | Task                                                           | Module | Depends On | Est. | Status |
| ---- | -------------------------------------------------------------- | ------ | ---------- | ---- | ------ |
| 2.1  | Refactor `zt_dyn_text_repr` to C union                       | M21.F3 | —         | 1d   | Done |
| 2.2  | Extend `ZT_DECLARE_*` macros for outcome types               | M21.F2 | —         | 2d   | Done |
| 2.3  | Implement `zt_list_generic` (void* + callbacks)              | M21.F1 | —         | 5d   | Done |
| 2.4  | Implement `zt_map_generic`                                   | M21.F1 | 2.3        | 5d   | Done |
| 2.5  | Implement `zt_set_generic`                                   | M21.F1 | 2.3        | 3d   | Done |
| 2.6  | Emitter: generate callback statics per type                    | M15.B  | 2.3-2.5    | 3d   | Done |
| 2.7  | Emitter: wire generic fallback in type resolution              | M15    | 2.6        | 2d   | Done |
| 2.8  | Remove `zt_list_dyn_text_repr` (use `zt_list_dyn`)         | M23.D6 | 2.3        | 1d   | Done |
| 2.9  | Extract specialized collections to `zenith_collections_rt.c` | M23.D5 | 2.3-2.5    | 2d   | Done |
| 2.10 | Remove sub-integer surface types (keep FFI)                    | M24    | —         | 2d   | Superseded by v1 contract |
| 2.11 | Componentize runtime (Unity Build): Borealis, Net, Json        | M55    | —         | 2d   | Done |
| 2.12 | Componentize runtime (Unity Build): Outcome boilerplate        | M55    | 2.11       | 1d   | Done |
| 2.13 | Modularize C emitter backend while keeping `emitter.h` stable  | M23.D8 | 2.6-2.7    | 3d   | Done |
| 2.14 | Lower tuple instantiations to generated C structs + callbacks  | M31b   | 1.10, 2.6  | 5d   | Done |

Tier 2 execution notes:

- Runtime generic foundation is in place: `zt_list_generic`, `zt_map_generic`, and `zt_set_generic` are linked into the unity runtime and covered by a focused C runtime test.
- ARC now knows the generic heap kinds for release and deep copy.
- Outcome declarations now use reusable macros across the text and `core.Error` outcome families. The text-error implementation macro also covers `failure_message` and `dispose`; `core.Error` implementations remain explicit because their lifecycle is custom.
- Emitter fallback covers `list<Struct>` for both plain user structs and structs with managed fields. The C emitter maps unsupported `list<T>` forms to `zt_list_generic *`, emits `zt_elem_ops` helpers per element type, and auto-generates per-struct `copy`/`destroy` callbacks that retain/release managed fields. Coverage: `tests/behavior/list_struct_generic` (plain) and smoke behavior coverage.
- Generic runtime fallback is wired for v1 scope: type resolution covers generic `list<T>`, `map<K,V>`, and `set<T>` forms; generated helper bodies and `zt_elem_ops` callbacks now live outside the emitter facade.
- `zt_list_dyn_text_repr` is removed from the active runtime/emitter path. Public `list<any<TextRepresentable>>` uses the internal `zt_list_dyn` runtime shape.
- 2.10 was superseded by the current v1 surface contract: `u8`, `u16`, `u32`, and `u64` are shipped surface types, not removed types. The active compiler, docs, and behavior tests still cover them (`u_alias_basic`, `edge_boundaries_empty`, `optional_primitive_specialized`, and `list_primitive_numeric_matrix`). The old removal task should not count as implemented.
- 2.13 is an architecture step for maintainability. It should be a mechanical split first: keep generated C output stable, keep `emitter.h` stable, and move responsibilities into focused backend files before changing behavior.
- 2.13 is complete for the v1 mechanical split: buffer/output moved to `emitter_buffer.c`, C naming/sanitization moved to `emitter_names.c`, type canonicalization/mapping moved to `emitter_types.c`, closure context emission moved to `emitter_closure.c`, generated optional/map/outcome/generic list/map/set helpers moved to `emitter_helpers.c`, and trait vtable registry/emission moved to `emitter_vtable.c`. `emitter.h` remains stable; backend-only declarations live in `emitter_internal.h`. Deep ZIR-expression and cleanup/ARC splitting can continue post-v1 without blocking Tier 2. Validation: `python build.py`, `zt.exe check zenith.ztproj --all --ci`, `python run_suite.py smoke --no-perf`, and the focused C emitter stream test.
- 2.14 is complete for tuple instantiation lowering: the parser recognizes `(a, b)` as a tuple literal, `tuple<...>` resolves as a fixed positional product type, HIR/ZIR carry tuple literals, and ZIR generates synthetic struct declarations (`item0`, `item1`, ...) for each canonical tuple instantiation. The C backend then reuses normal struct emission and generic collection callbacks, including managed-field copy/destroy for `list<tuple<...>>`. Validation: `python build.py`, focused `check/build/run` for `tests/behavior/tuple_generated_struct_callbacks`, `zt.exe check zenith.ztproj --all --ci`, `python run_suite.py smoke --no-perf`.

---

### Tier 3: Error Model & Type Conversions (1 week)

| #    | Task                                                             | Module | Depends On | Est. | Status |
| ---- | ---------------------------------------------------------------- | ------ | ---------- | ---- | ------ |
| 3.1  | Migrate `std.io` from `core.Error` to `io.Error`           | M07    | 2.2        | 2d   | Done   |
| 3.2  | Fix `std.net` to use `net.Error` + `optional<int>` timeout | M12    | 3.1        | 1d   | Done   |
| 3.3  | Implement `std.http` minimal blocking client                   | M13    | 3.2        | 3d   | Done   |
| 3.4  | Migrate `std.random.between` error type                        | M42a   | 3.1        | 1h   | Done   |
| 3.5  | Remove unused `outcome<*,text>` specializations                | M23.D2 | 3.1-3.4    | 1d   | Done   |
| 3.6  | Replace FS outcome helpers with macro                            | M23.D3 | 2.2        | 2h   | Done   |
| 3.7  | Add type conversion functions (int.to_float, etc.)               | M24    | —         | 2d   | Done   |
| 3.8  | Optimize `for i in range()` to C for-loop                      | M24    | —         | 3d   | Done   |
| 3.9  | Remove `assert` builtin (keep `check`)                       | M24    | —         | 1h   | Done   |
| 3.10 | Move `size_of` to `std.debug`                                | M24    | —         | 1h   | Done   |

Tier 3 execution notes:

- 3.1 keeps the C host bridge returning `core.Error` internally because the runtime ABI already exposes `zt_host_*` helpers that way. The public `std.io` surface now returns `io.Error`, maps host errors at the module boundary, and exposes `io.to_core_error(err)` for callers that still return `core.Error`.
- 3.1 validation: `zt.exe check tests/behavior/std_io_basic/zenith.ztproj --all --ci`.
- 3.2 keeps the socket runtime bridge returning `core.Error` internally and maps it to `net.Error` in `std.net`. Public timeouts now use `optional<int>`; `none` maps to the runtime's internal no-timeout sentinel, so user code no longer passes `-1`.
- 3.2 validation: `zt.exe check tests/behavior/std_net_basic/zenith.ztproj --all --ci`.
- 3.3 ships an HTTP-only blocking client. The runtime bridge returns raw HTTP text via `zt_http_get_core` / `zt_http_post_core`; `std.http` parses status/body into `http.Response` and maps `core.Error` into `http.Error`. TLS, redirects, chunked encoding, and auth stay post-v1.
- 3.3 validation: `zt.exe check tests/behavior/std_http_basic/zenith.ztproj --all --ci`, `zt.exe build tests/behavior/std_http_basic/zenith.ztproj -o tests/behavior/std_http_basic/build/std-http-basic.exe --ci --native-raw`, and `tests/behavior/std_http_basic/run-loopback.ps1`.
- 3.4 changes `std.random.between` from `result<int,text>` to `result<int,core.Error>` with stable code `random.invalid_range`.
- 3.4 validation: `zt.exe check tests/behavior/std_random_between_branches/zenith.ztproj --all --ci`.
- 3.5 was narrowed to the safe v1 cut: `result<T,text>` remains active language coverage (`optional_result_*`, `result_question_basic`, `std_text_basic`) and `std.text.from_utf8` still exposes `result<text,text>`. The completed cleanup removes unused runtime/emitter specializations for `outcome<bytes,text>`, `outcome<optional<text>,text>`, `outcome<optional<bytes>,text>`, and `outcome<net.connection,text>`. Active `text` error variants stay until their public APIs migrate.
- 3.6 replaces the repeated FS outcome failure wrappers with `ZT_DEFINE_FS_FAILURE_HELPER`, keeping ownership behavior unchanged: each helper builds the failure outcome and then disposes the temporary `core.Error`.
- 3.6 validation: `python build.py` and `zt.exe check tests/behavior/std_fs_ops_basic/zenith.ztproj --all --ci`.
- 3.7 adds `std.int`, `std.float`, and `std.bool` for explicit primitive conversions. The short surface (`int.to_float`, `float.parse`, `bool.to_text`) is available through normal import aliases, for example `import std.int as int`.
- 3.7 validation: `zt-next.exe check tests/behavior/type_conversions_basic/zenith.ztproj --all --ci`, `zt-next.exe build tests/behavior/type_conversions_basic/zenith.ztproj -o tests/behavior/type_conversions_basic/build/type-conversions-basic.exe --ci --native-raw`, and the built fixture executable. Full `python build.py` is currently blocked because the existing `zt.exe` file is held open by another process.
- 3.8 lowers `for ... in range(...)` into an allocation-free counter loop before C emission. Generated C no longer calls `zt_builtin_range*`, `zt_list_i64_len`, or `zt_list_i64_get` for this pattern. The C backend still prints structured goto blocks rather than literal `for (...)` syntax, matching the current ZIR block emitter.
- 3.8 validation: `zt-next.exe check tests/behavior/range_builtin_basic/zenith.ztproj --all --ci`, `zt-next.exe build tests/behavior/range_builtin_basic/zenith.ztproj -o tests/behavior/range_builtin_basic/build/range-after.exe --ci --native-raw`, the built fixture executable, and `rg` confirmation that the generated C contains no `zt_builtin_range`, `zt_list_i64_len`, or `zt_list_i64_get` calls.
- 3.9 removes the remaining runtime `zt_assert` API and `ZT_ERR_ASSERT` diagnostic kind. Existing source fixtures were migrated to `check(...)`; `runtime.check` is the sole assertion diagnostic surface.
- 3.10 removes `size_of` from the global builtin path and ships it as `std.debug.size_of(value: text)`. The runtime bridge was renamed from `zt_builtin_size_of` to `zt_debug_size_of`; `std_debug_basic` covers the public import path, and `size_of_builtin_removed_error` covers the removed global path.

---

### Tier 4: Collection APIs & HOFs (2 weeks)

| #    | Task                                                         | Module | Depends On | Est. | Status |
| ---- | ------------------------------------------------------------ | ------ | ---------- | ---- | ------ |
| 4.1  | Implement full `list<T>` API (value-style)                 | M02    | T2         | 3d   | Done for C backend `list<int>`/`list<text>` |
| 4.2  | Implement `list<T>` HOFs (map, filter, reduce, etc.)       | M02    | 4.1, T1.1  | 3d   | Done for C backend `list<int>` subset |
| 4.3  | Implement full `map<K,V>` API                              | M03    | T2         | 3d   | Done for C backend `map<text,text>` value-style subset; generic `entries`/HOFs remain follow-up |
| 4.4  | Implement full `set<T>` API                                | M04    | T2         | 2d   | Done for existing C backend set surface: `contains`/`intersection` aliases landed; deeper `to_list`/HOF expansion remains tied to generic collection work |
| 4.5  | Implement `std.text` new APIs (split, to_lower, pad, etc.) | M05    | —         | 2d   | Done: split/chars/get/slice/concat/case/capitalize/pad/replace alias; `repeat` exposed as `repeat_text` because `repeat` is parser-reserved |
| 4.6  | Implement `std.bytes` new APIs (get, slice, index_of)      | M06    | —         | 1d   | Done: safe get/slice/index_of/concat/len/is_empty plus `from_list -> result<bytes, core.Error>` |
| 4.7  | Add `std.fs` read_bytes, write_bytes, walk_dir             | M08    | 3.1        | 2d   | Done: runtime-backed byte IO and recursive walk API |
| 4.8  | Fix `std.fs.path` optional returns + backslash             | M09    | —         | 1d   | Done: `extension`/`parent -> optional<text>` and `/`/`\` separator handling |
| 4.9  | Add `std.regex` try_* variants                             | M11    | —         | 1d   | Done: `try_first`, `try_find_all`, `try_split`, `try_replace_all` |
| 4.10 | Add `std.math.abs_int`                                     | M39    | —         | 30m  | Done |

---

### Tier 5: New Stdlib Modules & Language Features (2 weeks)

| #   | Task                                                     | Module | Depends On | Est. | Status |
| --- | -------------------------------------------------------- | ------ | ---------- | ---- | ------ |
| 5.1 | Implement `std.encoding` (base64, hex)                 | M46    | -          | 3d   | Done |
| 5.2 | Implement `std.hash` (sha256, md5)                     | M46    | -          | 3d   | Done |
| 5.3 | Implement `std.json` full JsonValue support            | M41    | T2, M31    | 5d   | Done |
| 5.4 | Implement `.ztproj` parser in C (compiler-internal)    | M46/I2 | -          | 3d   | Done / existing compiler path |
| 5.5 | Implement `random.float_between`                       | M42    | -          | 1d   | Done |
| 5.6 | Implement `random.choice<T>`, `random.shuffle<T>`    | M42    | T2         | 2d   | Done: v1 compiler-lowered `list<int>` and `list<text>` surface |
| 5.7 | Implement `TextRepresentable` auto-derive              | M30    | T2         | 3d   | Done: user structs accepted with fallback text rendering |
| 5.8 | Implement `std.format` expansion (table, pretty, etc.) | M43    | 5.7        | 5d   | Done for v1 `any<TextRepresentable>` surface |

---

Tier 5 execution notes:

- Status update: 5.1 `std.encoding`, 5.2 `std.hash`, 5.3 `std.json.Value`, 5.4 `.ztproj` C parser, 5.5 `random.float_between`, 5.6 `random.choice/shuffle`, 5.7 `TextRepresentable` auto-derive, and 5.8 `std.format` v1 `any<TextRepresentable>` surface are implemented.
- 5.3 `std.json.Value` is closed: focused fixture validates `parse_value`, `kind`, `get`, `at`, `as_text`, `as_bool`, `as_int`, `len`, `stringify_value`, and `pretty_value`.
- 5.6 `random.choice<T>` / `random.shuffle<T>` is closed for v1: calls are type-inferred by the checker and lowered to C runtime specializations for `list<int>` and `list<text>`. Full user-defined generic function monomorphization remains a post-v1 compiler feature, not a Tier 5 blocker.
- Validation: `python build.py` passed; focused `check` and `run` passed for `std_encoding_hash_basic`, `std_json_value_basic`, and `std_random_format_tier5`.

---

### Tier 6: Tooling & Diagnostics (1-2 weeks)

| #   | Task                                                | Module | Depends On | Est. | Status |
| --- | --------------------------------------------------- | ------ | ---------- | ---- | ------ |
| 6.1 | Implement `zt create` command                     | M25    | 5.4        | 2d   | Done   |
| 6.2 | Refactor CLI error output (✗/⚠/ℹ format)         | M27    | —         | 3d   | Done   |
| 6.3 | Implement structured diagnostics (DIA.01-06)        | M14    | 6.2        | 5d   | Done   |
| 6.4 | Implement match exhaustiveness warnings             | M29    | 6.3        | 2d   | Done   |
| 6.5 | Implement naming convention warnings                | M34    | 6.3        | 1d   | Done   |
| 6.6 | Implement new attributes (deprecated, todo, skip)   | M36    | —         | 2d   | Done   |
| 6.7 | Performance corpus setup                            | M16    | T4         | 2d   | Done   |
| 6.8 | Test runner improvements (golden/snapshot/negative) | M17    | 6.6        | 3d   | Done   |

- Status update: 6.1 `zt create` is implemented and validated by the scaffold driver test. 6.2 and 6.3 are covered by action-first diagnostics, stable diagnostic codes, CI rendering, profile limits, and clean CLI output checks. 6.4 and 6.5 are implemented through match exhaustiveness/default-case diagnostics and readability name warnings. 6.6 now supports `attr deprecated("...")`, `attr todo("...")`, and `attr skip("...")`. 6.7 has the performance corpus under `tests/perf`, with quick/nightly scenarios, budgets, and baselines. 6.8 is covered by runner filtering, skip/fail/pass reporting, formatter golden tests, snapshots, and negative fixture diagnostics.
- Validation: `python build.py`, `python tests/driver/test_attributes_v1.py`, `python tests/driver/test_create_scaffold.py`, `python tests/driver/test_cli_output_clean.py`, and `python tests/driver/test_zt_test_filter.py` passed.

---

### Tier 7: Documentation Reset & Canonical Spec (2-3 weeks)

Reference plan: `docs/internal/planning/tier-7-documentation-reset-plan.md`.
Initial inventory: `docs/internal/planning/tier-7-documentation-inventory.md`.
Decision reconciliation: `docs/internal/planning/tier-7-decision-reconciliation.md`.

Goal: replace the fragmented documentation set with one coherent source of truth
for the implemented language. Current docs must be inventoried and archived
before removal. The final public model should follow the content shape of
`lume-language-spec.html`: philosophy, syntax, semantics, examples, comparisons,
and practical guides in one readable flow.

| #    | Task                                                                 | Module | Depends On | Est. | Status |
| ---- | -------------------------------------------------------------------- | ------ | ---------- | ---- | ------ |
| 7.1  | Inventory all docs and classify as keep, rewrite, archive, or remove | M18    | ALL        | 2d   | Done |
| 7.2  | Create decision index with Current/Superseded/Historical status      | M18    | 7.1        | 3d   | Done |
| 7.3  | Reconcile decisions against the current compiler/spec/tests          | M18    | 7.2        | 3d   | Done |
| 7.4  | Write canonical language spec: syntax, types, semantics, examples    | M18    | 7.3        | 5d   | Done |
| 7.5  | Rewrite semantic guides: composition, errors, absence, ownership     | M37    | 7.4        | 3d   | Done |
| 7.6  | Write coming-from-X and language comparison guides                   | M38    | 7.4        | 3d   | Done |
| 7.7  | Rebuild public docs/tutorial/cookbook from the canonical spec        | M18    | 7.4        | 4d   | Done |
| 7.8  | Rebuild stdlib and `.zdoc` reference from implemented APIs           | M35    | 7.4        | 4d   | Done |
| 7.9  | Add docs validation gate for stale syntax and untested examples      | M18    | 7.4        | 2d   | Done |
| 7.10 | Archive/remove obsolete docs after the new canonical set lands       | M18    | 7.9        | 2d   | Done |

- Historical status update: Tier 7 created a canonical language spec (`docs/spec/language/zenith-language-spec.md`), a decision status index (`docs/internal/decisions/language/INDEX.md`), documentation reset plan/inventory/reconciliation artifacts, public learning/language/cookbook entry points, coming-from-X guides, and a docs current-syntax validation gate wired into `run_suite.py`. The public docs were later deleted for rewrite from `docs/spec/language/final-language-contract.md`. Superseded surface specs (`surface-syntax.md`, `closures.md`, `dyn-dispatch.md`, `callables.md`) are historical support material.
- Validation: `python -m py_compile tools/check_docs_current_syntax.py run_suite.py` passed. `python tools/check_docs_current_syntax.py` passed. `python tools/check_docs_paths.py` is not wired into the PR gate because it currently fails on pre-existing stale links outside this Tier 7 slice.

---

## 3. Summary

| Tier         | Focus                 | Est. Duration          |
| ------------ | --------------------- | ---------------------- |
| **T0** | Verify & Quick Wins   | ✅ Complete            |
| **T1** | Compiler Foundations  | ✅ 10/10 done          |
| **T2** | Runtime Architecture  | ✅ 13/13 active tasks done; 2.10 superseded |
| **T3** | Error Model           | ✅ 10/10 done          |
| **T4** | Collection APIs       | ✅ 10/10 v1 scope done |
| **T5** | New Stdlib + Features | ✅ 8/8 done |
| **T6** | Tooling & Diagnostics | ✅ 8/8 done            |
| **T7** | Documentation Reset   | ✅ 10/10 done          |
|              | **Total**       | **~10-14 weeks** |

---

## 4. Risk Register

### Resolved Risks

| Risk | Original Probability | Original Impact | Resolution |
| --- | --- | --- | --- |
| Recursive enums don't work | Medium | High | Verified safe for v1 because `list<T>` is pointer indirection and recursive enum checks passed. |
| Tuple front-end not represented in Tier 1 | Medium | Medium | Closed through 2.14; tuple literals, semantic resolution, HIR/ZIR carriage, and generated C structs are implemented. |
| Documentation syntax drift for `dyn`/`fmt`/`case default` | Medium | Medium | Tier 7 created canonical spec and `tools/check_docs_current_syntax.py`; PR suites now run the current-syntax docs gate. |

### Active Residual Risks

| Risk | Probability | Impact | Mitigation | Owner/Phase |
| --- | --- | --- | --- | --- |
| Generic collections performance regression | Medium | Medium | Keep specialized hot paths, keep `tests/perf` budgets, and compare generic fallback changes against quick perf gate. | Post-v1 perf hardening |
| Deep JSON parser/runtime recursion pressure | Medium | High | Add depth limit or iterative parser path before claiming unbounded nested JSON support. Current stdlib JSON is alpha. | Post-v1 runtime hardening |
| `with` expression managed-field copy complexity | Medium | Medium | Add focused behavior tests for `with` on structs containing `text`, `list`, `map`, `optional`, and `result` before expanding semantics. | Post-v1 value semantics |
| Documentation path debt | High | Low | `tools/check_docs_paths.py` currently fails on pre-existing stale links; keep it out of PR gate until old links are archived or fixed. | Docs cleanup |
| Public examples not all executable | Medium | Medium | Expand docs tooling to extract fenced `zt` examples and run `zt check`, or require `illustrative` labels. | Docs tooling |
| Translation drift | Medium | Low | `docs/spec/language/final-language-contract.md` is canonical; future public English docs and translations should be regenerated after the public docs reset. | Docs localization |

### Closure Statement

The v1 implementation plan is closed for the active v1 scope.

Remaining rows in this register are post-v1 hardening or documentation-debt
items. They do not reopen Tier 1-7 implementation status.
