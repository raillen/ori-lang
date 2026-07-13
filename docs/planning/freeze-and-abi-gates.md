# Freeze and ABI gates (FREEZE-1 / ABI-1)

> **Status (2026-07-13):** both gates **done / in force** — process **finalized**.  
> Calendar window: opened **2026-07-13** · remains open through pre-1.0 `0.3.x`
> until an explicit exit is recorded in CHANGELOG + this file.  
> 1.0 readiness checklist: [`freeze-1-0-readiness.md`](freeze-1-0-readiness.md).

---

## FREEZE-1 — **DONE** (window open)

**Opened:** 2026-07-13  
**Exit:** not closed yet (intentional long pre-1.0 window on `0.3.x`)

### Scope of freeze (no intentional breaking)

- Language surface S3 (`0.3.x` syntax/semantics)
- Canonical stdlib paths `ori.X` (nested utils/algorithms remain silent compat)
- Package manifests: `ori.proj` / `ori.pkg.toml` (see `manifest-schema.md`)
- Native ABI tag **`ori-native-abi-1`** (spec 19) — treat as stable; bump only for breaks

### Allowed during freeze

- Patch releases (`0.3.x`): bug fixes, docs, diagnostics, tests
- Additive APIs (new stdlib functions, new optional CLI flags)
- Performance and reliability of existing paths

### Not allowed without bumping to `0.4+` and explicit exit

- Breaking language syntax/semantics
- Removing or renaming public `ori.X` APIs
- Changing `ori.proj` / `ori.pkg.toml` required fields incompatibly
- Breaking `ori-native-abi-1` without ABI version bump

### How to exit freeze

1. Decide end date and next version line (`0.4+` or approach to 1.0).
2. Document in CHANGELOG (`FREEZE-1 closed YYYY-MM-DD`).
3. Update BACKLOG (already **done** for opening the gate).

---

## ABI-1 — **DONE** (enforcement in force)

With FREEZE-1 open, **ABI-1 is enforced**:

| Gate | Rule |
|------|------|
| Spec | `docs/spec/19-abi.md` is normative for native layouts / mangling / runtime symbols |
| Version | Tag **`ori-native-abi-1`** (`ORI_ABI_VERSION` in `ori-runtime`) |
| Link | `runtime-link.json` `abi_version` must match staged runtime |
| Break | Incompatible layout/symbol change → bump to `ori-native-abi-N`, update cap. 19, re-stage staticlib **and** cdylib |
| Additive | New `ori_*` symbols OK without bump; list in stdlib manifest |

### Enforcement checklist (maintainers)

- [x] Spec 19 published (`ori-native-abi-1`)
- [x] Freeze window open (FREEZE-1)
- [x] CHANGELOG documents ABI-1 in force
- [x] No silent layout changes on `0.3.x` without bump

## Explicitly out of freeze scope (not language work)

- Multi-OS packaging / marketplace / demos — **shelved** until language + docs +
  performance are done (see BACKLOG priority policy)
- Self-host (M4) — last
- C/debug async (LANG-3 wontfix for v1)
