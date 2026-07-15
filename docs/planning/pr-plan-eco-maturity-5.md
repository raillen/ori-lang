# Design + PR Plan — ECO packages to maturity **5 (Linux)**

**Status:** **PRs 1–19 complete** (2026-07-15) — W10 + Phase OS note (`/execute-plan` plan id `5b7bfbb0`)  
**Date:** 2026-07-15  
**Policy:** Linux first. Multi-OS (**Phase OS**) is **last** and does **not** block score 5.  
**Maturity gate for this plan:** score **5 (Linux)** = product engine-grade on Linux per matrix: **broad API + tests + real-use smoke** (polished windowed demos optional, not the gate). Full checklist: **§3 G1–G7**.  
**Cluster path:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-*`  
**Catalog:** [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md)  
**Inventory / Next work:** [`eco-packages-status.md`](eco-packages-status.md)  
**Matrix:** [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md)  
**Prior plan (ports 0.1.0):** [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) — **complete** (do not re-scaffold packages)  
**Linked from:** [`docs/planning/README.md`](README.md) · status Next work · catalog header · matrix W10

**Execute:**

```bash
/execute-plan docs/planning/pr-plan-eco-maturity-5.md --concurrency 4
```

**User instructions for implementers (inject / default):**

```text
Linux only. Packages at /home/raillen/Documentos/Projetos/game-engine-full/ori-*.
Each package keeps its own git repo — commit in that package root.
Path deps stay sibling-relative (../ori-X). int64_t ABI; milli-float where applicable.
Smoke must print ok; ori test 0 failed. Update package CHANGELOG + version bump (e.g. 0.1.0 → 0.2.0 or 0.3.0).
Score 5 gate = checklist §3 for that package — not “pretty demos”.
Do not re-port packages already maturity 5. Phase OS last / non-blocking.
No flecs/EnTT, no miniaudio, no dual physics.
```

---

## 1. Goal

Raise **every** ECO game package that is currently **&lt; 5** to **5 (Linux)**, then:

1. Deepen **ori-game** optional wires where product value is high.  
2. Mark matrix/status/catalog all **5 (Linux)**.  
3. Leave **Phase OS** as explicit final PR (scaffolding / docs only; no multi-OS CI gate).

One `/execute-plan` run should be enough to drive the DAG without “prossiga” between packages.

**Out of scope:** Studio Tauri product work, Marketplace, flecs/EnTT, miniaudio, OpenAL, ozz, Steamworks, dual physics, bgfx.

---

## 2. Baseline — already **5 (Linux)** (do not re-implement)

| Package | Repo | Ver. | Notes |
|---------|------|------|--------|
| raylib | `ori-raylib` | 0.1.0 | L0 |
| ori_game | `ori-game` | 0.3.0 | L1 hub (wires deepen in this plan only) |
| imgui | `ori-imgui` | 0.4.0 | Tier0–2 MVP |
| raygui | `ori-raygui` | 0.2.0 | |
| box2d | `ori-box2d` | 0.3.0 | |
| jolt | `ori-jolt` | 0.2.0 | |
| rres | `ori-rres` | 0.3.0 | |
| sqlite | `ori-sqlite` | 0.3.0 | |
| enet | `ori-enet` | 0.3.0 | |
| freetype | `ori-freetype` | 0.1.0 | + `game.font_atlas` |
| harfbuzz | `ori-harfbuzz` | 0.1.0 | |

These may receive **bugfix-only** touch-ups if a dependent PR needs them; no “maturity project” PR.

---

## 3. Score **5 (Linux)** — definition of done (all packages in §4)

A package reaches **5** when **all** of the following hold on Linux:

| # | Criterion | Evidence |
|---|-----------|----------|
| **G1** | **Broad product API** (not smoke-only) | Public Ori modules cover the checklist row for that package (§4) |
| **G2** | **≥ 4 automated tests** (or ≥ 3 if surface is tiny) | `ori test` 0 failed; happy + error + one edge |
| **G3** | **Green `tools/smoke_linux.sh`** | Prints `ok` / package smoke line; AOT or JIT path works with staged libs |
| **G4** | **README** documents API surface + Phase OS note | English README; version bumped |
| **G5** | **CHANGELOG** entry for the maturity bump | Keep a Changelog |
| **G6** | **No dual path-dep leaf collision** | Unique module leaves if multi-dep (see cgltf.loader / fast_obj.mesh lesson) |
| **G7** | **Version bump** | e.g. `0.1.0` → `0.2.0` (or `0.3.0` if large surface) in `ori.pkg.toml` |

**Not required for 5:** multi-OS, Marketplace, windowed eye-candy demos, Studio integration.

**Optional bonus (nice, not gate):** headless or short example under `examples/` that prints `ok`.

---

## 4. Target packages (current → **5**)

Paths: `/home/raillen/Documentos/Projetos/game-engine-full/<repo>/`.

| ID | Repo | Was → Now | Target API for G1 (minimum product surface) | Status |
|----|------|-----------|-----------------------------------------------|--------|
| **U1** | `ori-stb` | 3–4 → **5** @ **0.2.0** | image: load path + dims + free; perlin: 2D/3D; rect_pack: init/pack/get rect; optional write_png if cheap | **done** PR 2 |
| **U2** | `ori-noise` | 3–4 → **5** @ **0.2.0** | seed, set type (at least 2 noise kinds), get 2D/3D, fractal/octaves if FNL exposes easily | **done** PR 3 |
| **U3** | `ori-miniz` | 3–4 → **5** @ **0.2.0** | compress/decompress buffer; CRC32; **zip** create or extract **one** entry (or documented skip with inflate stream) | **done** PR 4 |
| **U4** | `ori-lz4` | 3 → **5** @ **0.2.0** | block + **frame** (or stream) compress/decompress; bound/size helpers; larger fixture test | **done** PR 5 |
| **U5** | `ori-nfd` | 3 → **5** @ **0.2.0** | open/save/folder; multi-open if pfd allows; cancel path returns clear false/empty; no GUI hang in smoke (skip interactive if headless CI) | **done** PR 6 |
| **U6** | `ori-implot` | 3 → **5** @ **0.2.0** | FULL default or dual path; line + scatter + bar; axis labels; clear/reset; depends on imgui path | **done** PR 7 |
| **U7** | `ori-imnodes` | 3 → **5** @ **0.2.0** | create nodes/pins/links; query link count; begin/end node editor; FULL link | **done** PR 8 |
| **U8** | `ori-imguizmo` | 3 → **5** @ **0.2.0** | translate **and** rotate **or** scale (at least 2 ops); manipulate matrix milli; FULL link | **done** PR 9 |
| **U9** | `ori-tracy` | 3 → **5** @ **0.2.0** | zone begin/end; frame mark; plot value; message; FULL client when flag set | **done** PR 10 |
| **U10** | `ori-enkiTS` | 3–4 → **5** @ **0.2.0** | add task, wait, parallel_for (or N independent tasks); shutdown clean; ≥2 cores smoke | **done** PR 11 |
| **U11** | `ori-cgltf` | 3 → **5** @ **0.2.0** | meshes/primitives counts; POSITION + NORMAL accessors (or document); node TRS milli; material base color if present; **export interleaved float mesh buffer** for raylib upload helper | **done** PR 12 |
| **U12** | `ori-fast-obj` | 3 → **5** @ **0.2.0** | positions + normals + texcoords + indices; material name count; **flatten to float mesh** helper | **done** PR 13 |
| **U13** | `ori-physfs` | 3 → **5** @ **0.2.0** | init/mount/exists/read; **write** to write-dir or user-dir; enumerate; unmount; multi-mount read | **done** PR 14 |
| **U14** | `ori-clay` | 3 → **5** @ **0.2.0** | multi-box nested layout; padding/gap; measure text hook already; **command list export** (rects) for a pure-Ori or raylib drawer (headless bounds sufficient for tests) | **done** PR 15 |
| **U15** | `ori-recast` | 3 → **5** @ **0.2.0** | build from **triangle soup** (not only plane); find_path; raycast or nearest poly; destroy; optional agent radius param | **done** PR 16 |

---

## 5. Integration targets (`ori-game`)

After package PRs, deepen optional modules (still optional path-deps; do not force every game to link everything):

| Wire | Uses | Goal for “engine grade” |
|------|------|-------------------------|
| `game.gltf` | cgltf | Load → counts + optional `to_mesh_milli` for `game.draw3d` / mesh upload if API exists |
| `game.obj` | fast_obj | Same pattern as gltf |
| `game.physfs_assets` | physfs | Mount assets dir; load_bytes / exists used by asset_loader path |
| `game.noise` *(new thin)* | noise | Optional: sample 2D for procedural |
| `game.compress` *(new thin)* | lz4 and/or miniz | Optional: compress_bytes helpers |
| `game.nav` *(new thin)* | recast | Optional: build plane or mesh path query wrapper |

Not every wire is mandatory if a package has no natural L1 home — package-level 5 still stands alone.

---

## 6. PR Plan (DAG for `/execute-plan`)

### PR 1: Maturity-5 plan lock-in + inventory refresh — **done** (2026-07-15)

**Description:** Point catalog/status/matrix **Next work** at this plan. Document score-5 gate (§3). List packages already 5 vs U1–U15. Layout note: `game-engine-full/`. Do not implement package code.

**Files/components affected:** `docs/planning/pr-plan-eco-maturity-5.md`, `docs/planning/eco-packages-status.md`, `docs/planning/eco-library-ports-catalog.md`, `docs/planning/game-ports-maturity-matrix.md`, `docs/planning/README.md`

**Dependencies:** None

**Done notes:** Inventory refresh + score-5 G1–G7 pointer + U1–U15 list in status/matrix/catalog; execute-plan entrypoint in status; cluster path `game-engine-full/`.

---

### PR 2: ori-stb → 5 (Linux) — **done**

**Description:** Expand `ori-stb` to §4 U1. ≥4 tests; smoke green; version bump; CHANGELOG; README API table. Path: `game-engine-full/ori-stb`.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-stb/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — perlin2, image roundtrip, tests.

---

### PR 3: ori-noise → 5 (Linux) — **done**

**Description:** Expand `ori-noise` to §4 U2. Seed + multiple noise types + 2D/3D; tests; smoke; version bump.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-noise/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — fractal/octaves, tests.

---

### PR 4: ori-miniz → 5 (Linux) — **done**

**Description:** Expand `ori-miniz` to §4 U3 (include zip one-entry or stream inflate). Tests + smoke + version bump.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-miniz/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — buffer deflate, CRC32, zip one entry.

---

### PR 5: ori-lz4 → 5 (Linux) — **done**

**Description:** Expand `ori-lz4` to §4 U4 (frame/stream + bounds). Tests + smoke + version bump.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-lz4/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — block + OLZ1 stream, bounds, large fixture.

---

### PR 6: ori-nfd → 5 (Linux) — **done**

**Description:** Expand `ori-nfd` to §4 U5. Headless smoke must not block: use cancel/default path or document `NFD_SMOKE_SKIP_UI=1` with unit tests for path marshalling. Version bump.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-nfd/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — multi-open, path marshalling, headless smoke.

---

### PR 7: ori-implot → 5 (Linux) — **done**

**Description:** Expand `ori-implot` to §4 U6. Ensure FULL draw path builds by default on Linux smoke (or dual artifact). Depends on sibling `ori-imgui` for link if required — do not break GLFW-only imgui. ≥4 tests where host allows; smoke green.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-implot/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — line/scatter/bar + axes + dual FULL.

---

### PR 8: ori-imnodes → 5 (Linux) — **done**

**Description:** Expand `ori-imnodes` to §4 U7. FULL path green; tests for graph bookkeeping without requiring human click when possible.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-imnodes/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — nodes/pins/links + editor API.

---

### PR 9: ori-imguizmo → 5 (Linux) — **done**

**Description:** Expand `ori-imguizmo` to §4 U8 (≥2 manipulate ops). FULL path; milli matrix API; tests + smoke.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-imguizmo/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — TRS milli + dual FULL Manipulate.

---

### PR 10: ori-tracy → 5 (Linux) — **done**

**Description:** Expand `ori-tracy` to §4 U9. Zone + frame + plot + message; FULL client when flag set; tests that call APIs without requiring Tracy GUI.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-tracy/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — zone/frame/plot/message + dual FULL.

---

### PR 11: ori-enkiTS → 5 (Linux) — **done**

**Description:** Expand `ori-enkiTS` to §4 U10. Parallel tasks; wait; clean shutdown; stress test with N tasks; C++ link pattern preserved.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-enkiTS/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — add/wait, parallel_for, N tasks.

---

### PR 12: ori-cgltf → 5 (Linux) — **done**

**Description:** Expand `ori-cgltf` to §4 U11. Mesh export buffer for engine use; richer metadata; ≥4 tests with fixtures; smoke; version bump. Keep module leaf unique (`cgltf.loader`).

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-cgltf/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — mesh export, TRS milli, materials.

---

### PR 13: ori-fast-obj → 5 (Linux) — **done**

**Description:** Expand `ori-fast-obj` to §4 U12. Flatten mesh; indices/normals/uvs; tests + smoke; leaf `fast_obj.mesh`.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-fast-obj/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — flatten mesh, normals/uvs/materials.

---

### PR 14: ori-physfs → 5 (Linux) — **done**

**Description:** Expand `ori-physfs` to §4 U13. Write path + multi-mount + enumerate; tests with temp dirs; smoke.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-physfs/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — write-dir, multi-mount, unmount tests.

---

### PR 15: ori-clay → 5 (Linux) — **done**

**Description:** Expand `ori-clay` to §4 U14. Nested layout + command/bounds export for drawers; headless tests ≥4; smoke.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-clay/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — nested layout, pad/gap, command export.

---

### PR 16: ori-recast → 5 (Linux) — **done**

**Description:** Expand `ori-recast` to §4 U15. Triangle-soup build; path + nearest/raycast; agent radius; destroy; ≥4 tests; smoke. Keep C++ ld-script pattern.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-recast/**`

**Dependencies:** None

**Done notes:** **0.2.0** maturity 5 — triangle soup, nearest, raycast.

---

### PR 17: ori-game wires deepen (Linux) — **done**

**Description:** After U11–U15 land as needed:

- Deepen `game.gltf` / `game.obj` to use mesh export helpers if available.  
- Deepen `game.physfs_assets` (write/mount helpers as thin re-exports).  
- Add thin optional modules only if path-dep clean: `game.noise`, `game.compress` (lz4/miniz), `game.nav` (recast).  
- Tests with fixtures; skip if dep missing.  
- Do **not** list raylib `native_libs` on ori_game (path-dep only — dual-link lesson).  
- CHANGELOG.

**Files/components affected:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-game/ori.pkg.toml`, `/home/raillen/Documentos/Projetos/game-engine-full/ori-game/game/**`, `/home/raillen/Documentos/Projetos/game-engine-full/ori-game/tests/**`, `/home/raillen/Documentos/Projetos/game-engine-full/ori-game/CHANGELOG.md`

**Dependencies:** PR 12, PR 13, PR 14, PR 15, PR 16

**Done notes:** Wires landed — `game.gltf` / `game.obj` / `game.physfs_assets` / `game.noise` / `game.compress` / `game.navmesh` (commit `2adfbe9`).

---

### PR 18: Catalog + matrix + status + umbrella gate — **done** (2026-07-15)

**Description:** Mark U1–U15 as **5 (Linux)** in catalog §2, status inventory, matrix Table A. Add wave row **W10 maturity-5** done. Ensure `smoke_eco_linux.sh` still lists all packages; document `ECO_SMOKE_SKIP_*`. Update this plan status line to complete when all green. Fix any stale `Documentos/Projetos/ori-*` paths → `game-engine-full`.

**Files/components affected:** `docs/planning/eco-library-ports-catalog.md`, `docs/planning/eco-packages-status.md`, `docs/planning/game-ports-maturity-matrix.md`, `docs/planning/pr-plan-eco-maturity-5.md`, `/home/raillen/Documentos/Projetos/game-engine-full/ori-game/tools/smoke_eco_linux.sh`

**Dependencies:** PR 2, PR 3, PR 4, PR 5, PR 6, PR 7, PR 8, PR 9, PR 10, PR 11, PR 12, PR 13, PR 14, PR 15, PR 16

**Done notes:** All U1–U15 marked **5 (Linux)** @ **0.2.0** in catalog/status/matrix; W10 **done**; plan status **PRs 1–18 complete**; `smoke_eco_linux.sh` lists all packages under `proj_root` = parent of ori-game (`game-engine-full/`); `ECO_SMOKE_SKIP_GAME` / `ECO_SMOKE_SKIP_DEMOS` documented in status + smoke header. Smoke script unchanged (already complete).

---

### PR 19: Phase OS note refresh (last, non-blocking) — **done**

**Description:** Confirm each package that reached 5 still documents Phase OS deferred or has real Win stubs. **Do not** require Windows/mac CI green. Update `PHASE-OS.md` table: maturity-5 packages Linux-complete; multi-OS still last.

**Files/components affected:** `docs/planning/PHASE-OS.md`, `docs/planning/eco-packages-status.md`, package READMEs under `game-engine-full/ori-*/README.md` (only if missing Phase OS section)

**Dependencies:** PR 18

**Done notes:** `PHASE-OS.md` lists U1–U15 as Linux-complete with deferred Win stubs; status residual cleared (**plan complete**); missing package Phase OS sections + `tools/build_windows.ps1` deferred stubs added where absent. No Win/mac CI required.

---

## 7. Suggested parallelism (levels)

| Level | PRs | Notes |
|-------|-----|-------|
| 0 | PR 1 + PR 2…PR 16 | Docs lock-in + **15 independent package deepens** |
| 1 | PR 17 | ori-game wires (needs cgltf, fast_obj, physfs, clay, recast) |
| 2 | PR 18 | Catalog/matrix after all packages |
| 3 | PR 19 | Phase OS last |

Linearized stack (docs assembly order):

`PR1 → PR2 → … → PR16 → PR17 → PR18 → PR19`

(Within level 0, PRs 2–16 are fully parallel; concurrency 4–6 recommended.)

---

## 8. Implementer playbook (multi-repo)

1. **cwd / absolute paths:** always edit  
   `/home/raillen/Documentos/Projetos/game-engine-full/ori-<pkg>/`  
   Do **not** assume monorepo worktree contains packages.  
2. **Env:**

```bash
export ORI_BIN=/home/raillen/.grok/worktrees/projetos-ori-lang/game-engine-final/compiler/target/debug/ori
# or: $(command -v ori)
export ORI_RUNTIME_CDYLIB=…/libori_runtime.so
export ORI_RUNTIME_LIB=…/libori_runtime.a
export ORI_USE_SYSTEM_LINKER=1
export ORI_USE_JIT=1   # for ori run; tests may AOT
```

3. **After raylib stub smoke:** restore full `libraylib.a` into `ori-raylib/lib/...` if stub overwrote (see `smoke_eco_linux.sh` restore helper).  
4. **C++ packages:** ld-script `.a` + `cdylib`/`.so` for JIT as in nfd/enkiTS/recast.  
5. **Commit** inside the package git root; push optional.  
6. **Summary file:**  
   `/tmp/grok-$(id -u)/grok-exec-summary-<PLAN_ID>-pr-<n>.md`

---

## 9. Risk register

| Risk | Mitigation |
|------|------------|
| Scope creep to “full upstream API” | Cap G1 to §4 checklist only |
| ImGui FULL flags break headless CI | Keep headless stub path; test bookkeeping without window when needed |
| Dual native_libs on ori_game + raylib | Never re-list raylib libs on ori_game |
| Module leaf collision | Unique filenames across path-deps |
| Recast/PhysFS compile time | Incremental expand; keep smoke &lt; 2 min |
| execute-plan worktree isolation | Implementers write absolute paths under game-engine-full |
| NFD needs display | Skip UI in smoke; unit-test string/path ABI |

---

## 10. Success criteria (whole plan)

1. All U1–U15 packages meet §3 G1–G7 on Linux. — **met**  
2. Status + matrix + catalog show **5 (Linux)** for every package in `game-engine-full` (core already 5 + U1–U15). — **met** (PR 18)  
3. `smoke_eco_linux.sh` can run ports (with skip flags) without missing dirs. — **met**  
4. ori-game wires PR landed or explicitly partial with documented residual. — **met** (PR 17)  
5. Phase OS still last / non-blocking. — **met** (PR 19 docs/stubs; multi-OS execution still deferred)  
6. No new open alta/média port (miniaudio stays skipped). — **met**

---

## 11. Effort sketch (planning only)

| Band | Packages | Rough effort each |
|------|----------|-------------------|
| Small | nfd, tracy, noise, lz4 | 0.5–1 d |
| Medium | stb, miniz, enkits, clay, physfs, implot/imnodes/imguizmo | 1–2 d |
| Large | cgltf, fast_obj, recast | 2–3 d |
| Integration | PR 17 | 1–2 d |
| Docs | PR 1, 18, 19 | 0.5 d total |

Parallelism can compress calendar time to ~1 week wall-clock with concurrency 4+.

---

## Appendix A — Packages **not** in this plan

| Item | Reason |
|------|--------|
| ori-raylib / ori-game core / imgui / box2d / jolt / … | Already 5 |
| ori-miniaudio | Skipped — game.audio |
| ori-openal, ozz, cute_c2, Steam | Catalog §5 conditional only |
| ori-game-studio | Separate product track |
| flecs/EnTT | Declined as default ECS |

---

## Appendix B — Acceptance snippet (per package PR)

Implementer must end with evidence like:

```text
cd $PKG && ./tools/smoke_linux.sh   # → smoke ok
ori test tests/...                  # → N passed, 0 failed (N≥3 or ≥4)
grep version ori.pkg.toml           # bumped
```

And a one-line maturity claim: `maturity 5 (Linux) — G1..G7 met`.
