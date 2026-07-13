# Ori Standard Library

> Surface: **S3 (`0.3.0`)** + local inference **0.3.1 / option B**.  
> Modules use `module ori.…`, no declaration `func`, types with `[]`, imports
> `import path = alias` / `import path (names)`.  
> **Auk9** is archived as a product; Ori owns the living surface.

**Merge policy (M2):** canonical public API is **`ori.X`** (one domain module).
Prefer a single `stdlib/X.orl` on disk. Legacy `ori.X.utils` /
`ori.X.algorithms` remain as **silent compatibility** aliases — do **not** use
them in new examples. Full decision: [`docs/planning/stdlib-merge-policy.md`](../docs/planning/stdlib-merge-policy.md).

---

## Three layers

| Layer | Role | Location |
|-------|------|----------|
| **1** | Hot path, FFI, ARC, I/O, collections primitives | Rust: `ori-types/src/stdlib.rs` + `ori-runtime` |
| **2** | Safe wrappers / ergonomics in `.orl` | Prefer `stdlib/X.orl` → `ori.X` |
| **3** | Pure algorithms in `.orl` | Same `ori.X` when small; optional subfiles for heavy domains |

Layer 1 is **not** scheduled for a full port to `.orl`. The C ABI of Layer 1 is
the long-term runtime contract (formal freeze is **M3**, after feature work).

---

## How to import (canonical)

```ori
import ori.io = io
import ori.fs = fs
import ori.string = str
import ori.path = path
import ori.list = lists

-- selective helpers already exposed on the parent module
import ori.string (is_empty)
import ori.fs (read_text_or)
```

### Compatibility (supported, not preferred)

```ori
import ori.fs.utils = fu
import ori.string.utils = su
```

These still resolve while the physical merge of `X/utils.orl` into `X.orl`
(or stable re-exports) finishes. Prefer `ori.X` in all new code and docs.

---

## On-disk layout (target)

| Prefer | When |
|--------|------|
| `stdlib/name.orl` → `module ori.name` | Default for almost every domain |
| `stdlib/name/…` subfiles | Heavy algorithms (`graph`, `tree`), multi-file math (`vec2`, `mat3`), or a parent that would exceed ~400 lines *and* splits by theme |

Avoid duplicating the same helper in both `name.orl` and `name/utils.orl`
(known debt: `fs` — fix in M2 code lot 1).

Path rule: `ori.X.Y` → `stdlib/X/Y.orl` when a nested module still exists.
Flatten (loading `utils`/`algorithms` under a parent import) is a **compat
bridge**, not the long-term mental model for authors.

---

## Major modules (public names)

### Hybrid L1 + `.orl` parent (preferred style)

| Module | Typical file | Notes |
|--------|--------------|--------|
| `ori.string` | `string.orl` | Text helpers + algorithms in the parent |
| `ori.list` | `list.orl` | List helpers / int algorithms |
| `ori.map` | `map.orl` | Map helpers |
| `ori.fs` | `fs.orl` | FS convenience; `fs/utils.orl` still compat |
| `ori.io` | `io.orl` | Streams + helpers |
| `ori.net` | `net.orl` | TCP/TLS/UDP surface |
| `ori.time` | `time.orl` | Instant/Duration helpers |
| `ori.path` | `path.orl` | Join, normalize, relative, … |
| `ori.args` | `args.orl` | CLI args |
| `ori.config` | `config.orl` | Config helpers |
| `ori.log` | `log.orl` | Minimal logging |
| `ori.validate` | `validate.orl` | Validation helpers |

### L1-heavy / collections (import `ori.X`; helpers may still live under `X/utils` until M2 code lots)

| Module | Notes |
|--------|--------|
| `ori.os`, `ori.process` | Platform / process L1; some helpers still in `*/utils.orl` |
| `ori.queue`, `ori.stack`, `ori.deque`, `ori.heap`, … | Collection L1 + optional `.orl` utils |
| `ori.math` | L1 + `math/utils`, `algorithms`, `vec2`/`vec3`/`mat3` |
| `ori.bytes`, `ori.set`, `ori.json`, `ori.iter`, … | Mix of L1 and `.orl` |

Known limitations: map/set/graph wrappers often use concrete key types until
broader trait gates; `repeat` is a keyword — use string helpers such as
`replicate` / `repeated` instead of a `repeat` function name.

---

## Adding a function

### Layer 1 (runtime FFI)

1. Entry in `STDLIB_RUNTIME_FUNCTIONS` (`stdlib.rs`).
2. `stdlib_func_sig` + `stdlib_native_abi`.
3. `extern "C"` in `ori-runtime`.
4. Regression test in `ori-driver` tests.

### Layer 2 / 3 (`.orl`)

1. Prefer extending **`stdlib/<module>.orl`** (`module ori.<module>`).
2. Import L1 with `import ori.<mod> = …` as needed.
3. Export with `public`.
4. Avoid local name `len` (collides with runtime symbol habits).
5. Avoid keywords as function names.
6. Update `.oridoc` sidecar when the public surface changes.
7. Do **not** add new public APIs only under `ori.*.utils` unless there is a
   temporary technical reason — plan to land them on `ori.X`.

---

## Specs and plans

| Doc | Role |
|-----|------|
| [`docs/spec/12-stdlib.md`](../docs/spec/12-stdlib.md) | Normative contracts + architecture |
| [`docs/planning/stdlib-merge-policy.md`](../docs/planning/stdlib-merge-policy.md) | **M2 merge decision** |
| [`docs/planning/PENDENTES.md`](../docs/planning/PENDENTES.md) | Tactical backlog (M2 → M3 → M1 → M4) |
| [`docs/planning/historico/stdlib-gap-parity.md`](../docs/planning/historico/stdlib-gap-parity.md) | Historical gap matrix |

---

## Residual technical debt (code, not layout theory)

- `path.relative` sequential calls: test still ignored (memory/ARC suspicion).
- Duplicate helpers between some parents and `*/utils.orl` (start with `fs`).
- Physical consolidation of many `*/utils.orl` into parents — **M2 code lots**.
