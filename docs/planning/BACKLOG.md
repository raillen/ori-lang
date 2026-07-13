# Ori — single implementation backlog

> **This file is the only active “what remains to implement” list.**  
> Surface baseline: **S3 `0.3.0`** + inference B **`0.3.1`** + package **`0.3.3`**.  
> Last consolidated: **2026-07-13** (language-first closed; FREEZE-1 + Linux deb).

---

## Priority policy (2026-07-13)

**Until language + docs/examples + performance are solid, do not prioritize:**

- Multi-OS packages / marketplace / registry marketing (DIST-*, TOOL marketplace, ECO demos)
- Self-host (M4)

**Language-first queue is empty.** Ongoing work is **living maintenance** only:

1. Bugs / diagnostics from real programs  
2. Docs + examples drift  
3. Package/CI reliability (Linux tar.gz + deb already shipped)  
4. Local DX (VS Code / Zed — **no** store publish)

**Do not prioritize unless reopened:** multi-OS DIST, Marketplace, ECO demos, M4 self-host.

**Discontinued forever in product conversations:** `ori-game`, `ori-imgui` (removed; do not re-open).

---

## 0. How to read this list

| Field | Meaning |
|-------|---------|
| **ID** | Stable handle |
| **P** | Priority **1** = next · **2** = soon · **3** = later · **4** = after language freeze |
| **D** | **S** small · **M** medium · **L** large · **XL** multi-month |
| **Status** | `todo` · `partial` · `done` · `shelved` · `cancelled` |

---

## 1. Already done (language / stdlib / process)

| ID | What |
|----|------|
| DONE-S3 / INF / M1 / M2 / M3 | Surface, inference, install path, stdlib parents, ABI |
| DONE-STDLIB-1…5 / 4b / 4k | Canonical stdlib + async I/O + poll reactor |
| DONE-LANG-1 / LANG-2 | Native async subset + C/debug sync matrix slice |
| DONE-PKG-1…4 | Path/git/registry (code exists; not market push) |
| DONE-FREEZE-1 / ABI-1 | Freeze window open; ABI-1 in force; readiness checklist finalized |
| DONE-DIST-LINUX-DEB | Linux `.tar.gz` + `.deb` via `package_native_release` / `package_deb`; CI release assets |
| DONE-LANG-DOC | User docs + examples aligned to S3 / current stdlib / editors local |
| DONE-LANG-PERF | AOT/JIT, stage release, mold/lld PATH, microbench + ARC bench; living JIT lower only |
| DONE-LANG-RES | Native residual inventory Spec 14; product surface gate test; reopen only on concrete blocker |
| CANC-GAME / CANC-IMGUI | **Cancelled** — never product again |
| CANC-AUK9 | Archived |
| WONT-HM / WONT-LANG-3 | Global HM; C async v1 |

---

## 2. Active work (language-first)

| ID | Item | P | D | Status | Notes |
|----|------|---|---|--------|-------|
| *(none)* | Language-first implementation queue empty | — | — | — | Living maintenance only. |
| **LIVE-LINK** | Package smoke uses **SystemLinker only** (not RustcDriver) | 2 | S | **done** | RustcDriver double-links libstd vs `ori-runtime` staticlib (`rust_eh_personality`). |

### Done this focus wave (DX + docs + perf + residual)

| ID | Notes |
|----|-------|
| **LANG-DOC** | User docs EN/PT + root READMEs + examples catalog; living maintenance only after this |
| **LANG-PERF** | Closed — waves 1–3; see `perf-baseline-2026-07-13.md` |
| **LANG-RES** | Closed — Spec 14 inventory + `compile_runs_lang_res_product_surface_native`; see `lang-res-closure.md` |
| **DX-VSCODE** | v0.3.2 local `.vsix` |
| **DX-ZED** | `extensions/zed-ori` dev install |

---

## 3. Shelved (after language is complete)

Do **not** pull these into “what’s next” until the user re-opens them:

| ID | Item |
|----|------|
| DIST-1 / DIST-2 / DIST-3 / DIST-4 | Multi-OS packages (Win/macOS), smoke matrix, extra triples — Linux ship + deb **done** |
| TOOL-MP | VS Code Marketplace / Open VSX publish |
| ECO-1 / ECO-2 | External demos (raylib/sqlite community packages) |
| M4 | Self-hosting |

---

## 4. Sources

| Doc | Role |
|-----|------|
| This file | Only open-work list |
| `docs/spec/` | Normative language |
| `docs/install.md` + guides + examples | User docs |
| `extensions/vscode-orl` | VS Code DX |
| `extensions/zed-ori` | Zed DX |

When an item finishes: set status, update CHANGELOG if user-facing.
