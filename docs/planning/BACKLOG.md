# Ori — single implementation backlog

> **This file is the only active “what remains to implement” list.**  
> Surface baseline: **S3 `0.3.0`** + inference B **`0.3.1`** + package **`0.3.2`**.  
> Last consolidated: **2026-07-13** (language-first focus).

---

## Priority policy (2026-07-13)

**Until language + docs/examples + performance are solid, do not prioritize:**

- Multi-OS packages / marketplace / registry marketing (DIST-*, TOOL marketplace, ECO demos)
- Self-host (M4)

**Active focus (in order):**

1. **Language completeness** — native backend, diagnostics, stdlib contracts, async/FFI correctness  
2. **Documentation & examples** — accurate S3 surface, install/tour/guides/examples  
3. **Performance** — compile, runtime ARC/I/O, JIT/AOT paths  
4. **Local DX only** — VS Code + Zed extensions (install from repo / `.vsix` / dev install; **no Marketplace publish**)

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
| DONE-FREEZE-1 / ABI-1 | Freeze window open; ABI-1 in force |
| DONE-LANG-DOC | User docs + examples aligned to S3 / current stdlib / editors local |
| DONE-LANG-PERF | AOT/JIT, stage release, mold/lld PATH, microbench + ARC bench; living JIT lower only |
| CANC-GAME / CANC-IMGUI | **Cancelled** — never product again |
| CANC-AUK9 | Archived |
| WONT-HM / WONT-LANG-3 | Global HM; C async v1 |

---

## 2. Active work (language-first)

| ID | Item | P | D | Status | Notes |
|----|------|---|---|--------|-------|
| **LANG-PERF** | Measure and improve hot paths (check/compile/run, ARC, net/fs) | 1 | L | **done** | Wave1: BundledRustLld + Cranelift flags. Wave2: mold/lld PATH, stage **release**, microbench. Wave3: ARC bench `tools/bench/arc_list_churn.orl`. Living: further JIT lower is Cranelift-bound (~40–50 ms tiny programs with release cdylib) — not a v1 gate. |
| **LANG-RES** | Native residuals only if they block real programs | 2 | M | **partial** | Spec 14; not invent features |

### Done this focus wave (DX + docs + perf)

| ID | Notes |
|----|-------|
| **LANG-DOC** | User docs EN/PT + root READMEs + examples catalog; living maintenance only after this |
| **LANG-PERF** | AOT/JIT measured and improved; stage release default; harness in `tools/microbench_lang_perf.sh` |
| **DX-VSCODE** | v0.3.2 local `.vsix` |
| **DX-ZED** | `extensions/zed-ori` dev install |

---

## 3. Shelved (after language is complete)

Do **not** pull these into “what’s next” until the user re-opens them:

| ID | Item |
|----|------|
| DIST-1 / DIST-2 / DIST-3 / DIST-4 | Multi-OS packages, smoke matrix, extra triples |
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
