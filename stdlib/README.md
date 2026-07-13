# Ori Standard Library

> Surface: **S3** + inference B + **`ok`/`err`**.  
> **M2 complete:** canonical public API is **`ori.X`** via `stdlib/X.orl`.  
> Nested `ori.X.utils` / `ori.X.algorithms` remain as **compat** modules (full implementations).

Policy: [`docs/planning/stdlib-merge-policy.md`](../docs/planning/stdlib-merge-policy.md).

---

## Three layers

| Layer | Role | Location |
|-------|------|----------|
| **1** | Hot path FFI / ARC / collections primitives | Rust `stdlib.rs` + `ori-runtime` |
| **2** | Ergonomic wrappers | Prefer `stdlib/X.orl` → `module ori.X` |
| **3** | Algorithms | Prefer same `ori.X` parent; heavy domains may keep `X/algorithms.orl` |

---

## Import (canonical)

```ori
import ori.io = io
import ori.fs = fs
import ori.string = str
import ori.queue = queue

import ori.string (is_empty)
import ori.fs (read_text_or)
const q = queue.from_list(["a", "b"])
```

### Compatibility (supported, not preferred)

```ori
import ori.fs.utils = fu
import ori.queue.utils = qu
```

---

## Layout on disk

| Prefer | When |
|--------|------|
| `stdlib/X.orl` | Default — helpers for domain `X` |
| `stdlib/X/utils.orl` | Compat alias of helpers (or remaining utils-only path) |
| `stdlib/X/algorithms.orl` | Compat / heavy algorithms still nested |
| `stdlib/math/vec2.orl` etc. | Multi-file domain assets |

Almost every domain now has a **parent** `X.orl` so `import ori.X` sees helpers without `.utils`.

---

## Residual notes

- Pure L1 symbols (e.g. `ori.fs.create_dir_all`) live only in the runtime — no redundant `.orl` wrapper of the same name.
- `path.relative` sequential calls: regression un-ignored (fixed with ARC `list_push`).
- Do not add new public APIs only under `ori.*.utils`.

---

## Adding a function

### Layer 1
Manifest + runtime + tests (see AGENTS.md).

### Layer 2/3
1. Add `public` to `stdlib/X.orl` (`module ori.X`).
2. Optionally mirror in `X/utils.orl` for compat (not required for new APIs).
3. Update `.oridoc` sidecar when user-facing.
