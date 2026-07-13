# Freeze + path to 1.0 (readiness checklist)

> **Status (2026-07-13): finalized as process.**  
> FREEZE-1 window is **open and in force** on `0.3.x`.  
> This is **not** a declaration of Ori 1.0 — it is the **contract** under which
> 0.3.x evolves until an explicit exit or 1.0 criteria are met.

Related: [`freeze-and-abi-gates.md`](freeze-and-abi-gates.md) · Spec [`19-abi.md`](../spec/19-abi.md) · [`BACKLOG.md`](BACKLOG.md)

---

## FREEZE-1 (language/API freeze) — **in force**

| Check | Status |
|-------|--------|
| Window opened 2026-07-13 | done |
| Scope: S3 surface, `ori.X` stdlib, manifests, `ori-native-abi-1` | done |
| Patch/`0.3.x` additive only without freeze exit | done |
| Breaking → `0.4+` + CHANGELOG freeze exit | rule in force |
| Documented in CHANGELOG Unreleased / gates file | done |

### Allowed on 0.3.x without freeze exit

- Bug fixes, diagnostics, tests, docs, examples
- Additive stdlib / CLI (non-breaking)
- Performance and packaging (Linux package, `.deb`, stage scripts)

### Not allowed without version bump + freeze note

- Breaking syntax/semantics
- Removing/renaming public `ori.X` APIs
- Incompatible manifest schema
- Breaking ABI without `ori-native-abi-N` bump

---

## ABI-1 — **enforced**

| Check | Status |
|-------|--------|
| Spec 19 normative | done |
| Tag `ori-native-abi-1` in runtime | done |
| `runtime-link.json` carries `abi_version` | done |
| Stage staticlib **and** cdylib on symbol changes | process |

---

## Language-first implementation (pre-1.0 content) — **closed**

| Area | Status |
|------|--------|
| S3 + inference B | done |
| M1 install path / M2 stdlib / M3 ABI | done |
| STDLIB async + net/fs streams | done |
| LANG-DOC + examples | done |
| LANG-PERF | done (living residual only) |
| LANG-RES | done (reopen on concrete blocker) |
| Active language backlog | empty |

---

## Path to **1.0** (not started as a gate)

1.0 is a **maturity** call, not a feature checklist:

1. Keep FREEZE-1 discipline through prolonged `0.3.x` use.
2. Real programs / feedback without prolonged intentional breaking.
3. Install path remains Rust-free for end users (M1).
4. Self-host (M4) is **optional and last** — not required for 1.0 utility.
5. Explicit CHANGELOG entry when declaring 1.0 (or when closing FREEZE-1 into 0.4/1.0).

Until then: ship **0.3.x** packages under freeze.

---

## Packaging under freeze (Linux)

| Artifact | Role |
|----------|------|
| `ori-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` | Portable tree |
| `ori_X.Y.Z_amd64.deb` | Debian/Ubuntu install (`tools/package_deb.sh`) |

Windows/macOS packages remain deferred (DIST multi-OS).
