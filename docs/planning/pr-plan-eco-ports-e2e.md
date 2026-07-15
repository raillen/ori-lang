# Design + PR Plan — ECO library ports end-to-end (Linux-first)

**Status:** **PRs 1–10 complete** (medium M1–M6 packages **0.1.0** + catalog/status/matrix + umbrella smoke + Phase OS scaffolding last).  
**Date:** 2026-07-15  
 
**Policy:** implement / mature / port on **Linux first**. Multi-OS (**Phase OS**) is **last**.  
**Maturity gate:** **API + smoke + tests** (not polished demos).  
**Canonical catalog:** [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md)  
**Status inventory:** [`eco-packages-status.md`](eco-packages-status.md)  
**Maturity matrix:** [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md)  
**Package conventions:** [`package-ecosystem-guidelines.md`](package-ecosystem-guidelines.md)  
**Linked from:** [`docs/planning/README.md`](README.md) · catalog header · status Next work

---

## 1. Goal

Deliver the **remaining product-relevant ECO ports** as sibling packages under:

```text
/home/raillen/Documentos/Projetos/ori-<name>/
```

so that a single invocation:

```bash
/execute-plan docs/planning/pr-plan-eco-ports-e2e.md --concurrency 3
```

can implement **all medium-priority ports**, optional deepen wires into `ori-game`, catalog/status updates, and leave Phase OS as an explicit final PR — **without** the user having to re-prompt “prossiga” between stages.

**Out of scope for this plan:** Studio Tauri app, Marketplace publish, flecs/EnTT default ECS, dual physics, bgfx.

---

## 2. Baseline (already done — do not re-implement)

### 2.1 Linux-5 core

| Package | Repo | Maturity |
|---------|------|----------|
| raylib | `ori-raylib` | 5 Linux |
| ori_game | `ori-game` | 5 Linux |
| imgui | `ori-imgui` | 5 Linux |
| raygui | `ori-raygui` | 5 Linux |
| box2d | `ori-box2d` | 5 Linux |
| jolt | `ori-jolt` | 5 Linux |
| rres | `ori-rres` | 5 Linux |
| sqlite | `ori-sqlite` | 5 Linux |
| enet | `ori-enet` | 5 Linux |

### 2.2 Residual polish + deepen (sections 1–2)

| Item | Location | Maturity |
|------|----------|----------|
| FreeType + atlas | `ori-freetype` + `game.font_atlas` | 5 Linux |
| HarfBuzz + layout | `ori-harfbuzz` | 5 Linux |
| Marching Cubes + export/GPU path | `ori-game` `game.marching_cubes*` | 5 Linux |
| ImGui multi-context / Tier2 MVP | `ori-imgui` | done |
| B2.17 surfaces + B2.19 explore | `ori-game` | done |

### 2.3 High-priority ports (B2.18 high — done)

| Package | Repo | Notes |
|---------|------|-------|
| stb | `ori-stb` | image / perlin / rect_pack |
| noise | `ori-noise` | FastNoiseLite |
| miniz | `ori-miniz` | deflate / CRC |
| nfd | `ori-nfd` | portable-file-dialogs |
| implot | `ori-implot` | series buffer; `ORI_IMPLOT_FULL=1` for draw |
| imnodes | `ori-imnodes` | graph bookkeeping; `ORI_IMNODES_FULL=1` |
| imguizmo | `ori-imguizmo` | translate milli; `ORI_IMGUIZMO_FULL=1` |
| tracy | `ori-tracy` | zones/frames; `ORI_TRACY_FULL=1` |
| enkits | `ori-enkiTS` | parallel task sum |

**Templates to copy** (structure, smoke pattern, AOT ld-scripts for C++):

- Pure C: `ori-stb`, `ori-noise`, `ori-miniz`
- C++ deps: `ori-nfd` (ld-script → `-lstdc++`)
- Optional FULL ImGui clients: `ori-implot` / `ori-imnodes` / `ori-imguizmo`
- Job system: `ori-enkiTS`

---

## 3. Remaining work (this plan)

### 3.1 Medium ports (implement)

| # | Package | Upstream | Role | Status |
|---|---------|----------|------|--------|
| M1 | **ori-cgltf** | [cgltf](https://github.com/jkuhlmann/cgltf) | glTF 2.0 load (meshes, nodes, materials, animations metadata) | **done 0.1.0** |
| M2 | **ori-fast-obj** | [fast_obj](https://github.com/thisistherk/fast_obj) | Wavefront OBJ load (complements cgltf) | **done 0.1.0** |
| M3 | **ori-physfs** | [PhysFS](https://github.com/icculus/physfs) | Virtual FS / multi-archive (with rres/ORPK) | **done 0.1.0** |
| M4 | **ori-clay** | [Clay](https://github.com/nicbarker/clay) | Immediate-mode UI layout (not Yoga) | **done 0.1.0** |
| M5 | **ori-recast** | [Recast Navigation](https://github.com/recastnavigation/recastnavigation) | Navmesh build + path query (3D) | **done 0.1.0** |
| M6 | **ori-lz4** | [lz4](https://github.com/lz4/lz4) | Fast compression (when miniz not enough) | **done 0.1.0** |
| M7 | **ori-miniaudio** *(conditional)* | [miniaudio](https://github.com/mackron/miniaudio) | Only if raylib `game.audio` still fails a measured gap | **open (skip default)** |

### 3.2 Integration + docs

| Item | Description |
|------|-------------|
| Game wires | Optional thin modules in `ori-game` that **consume** new packages (path deps) without swallowing them |
| Catalog / matrix / status | Move each package from “proposed” → “done 0.1.0” with maturity 3–4 Linux |
| Umbrella smoke | Extend or document `ori-game/tools/smoke_eco_linux.sh` to include new packages when present |

### 3.3 Explicitly deferred (separate PR only if requested)

| Item | When |
|------|------|
| Phase OS (Win/mac stage + smoke) | **Last** — after all Linux PRs |
| Low priority: OpenAL Soft, ozz, cute_c2, Steam/Discord, Lua host | Only with explicit product need |
| Declined: Yoga, cglm/HMM, bgfx, ejson, flecs/EnTT default | Never in this plan |

---

## 4. Package template (mandatory for every new port)

Each package **must** live at:

```text
/home/raillen/Documentos/Projetos/ori-<name>/
```

### 4.1 Files

```text
ori-<name>/
├── ori.pkg.toml          # name WITHOUT ori- prefix; native_libs = ["ori_<name>_shim"]
├── README.md             # EN: build/smoke, API table, deps
├── CHANGELOG.md          # [0.1.0] Added
├── <name>/               # Ori modules (module <name>....)
│   └── *.orl
├── native/
│   └── ori_<name>_shim.c|.cpp
├── vendor/               # upstream single-header or git-vendored sources
├── lib/x86_64-unknown-linux-gnu/
│   ├── libori_<name>_shim.so   # JIT
│   └── libori_<name>_shim.a    # AOT (real ar OR GNU ld script → objs + -l*)
├── examples/smoke_*.orl  # prints exactly "ok" on success
├── tests/test_*.orl      # @test functions; AOT when possible
└── tools/
    ├── build_linux.sh
    └── smoke_linux.sh    # build → check → run smoke → ori test
```

### 4.2 Conventions

| Rule | Detail |
|------|--------|
| Language | Ori S3 (`module`, no declaration `func`, `end` blocks) |
| Identifiers / code comments | English |
| User docs | English primary; optional `*.pt-BR.md` later |
| ABI | Ori `int` is 64-bit → C shims use `int64_t` for pointers and lengths |
| JIT | Stage `libori_*_shim.so` under `lib/<triple>/` |
| AOT | Stage `.a`; if C++ or system libs missing as static, use **GNU ld script** named `libori_*_shim.a` that `INPUT(…objs.a -lstdc++ …)` (see `ori-nfd`, `ori-enkiTS`) |
| Smoke env | `ORI_USE_SYSTEM_LINKER=1`, `ORI_USE_JIT=1`, `LD_LIBRARY_PATH` includes package `lib/triple` |
| Maturity | 3 Linux = API + smoke + tests; 4 = broader surface; 5 = product-grade documented |

### 4.3 `ori.pkg.toml` skeleton

```toml
[package]
name = "<name>"          # e.g. cgltf — NOT ori-cgltf
version = "0.1.0"
entry = "<name>/....orl"
ori_version = "0.3.0"
description = "..."
license = "MIT"          # or upstream license
native_libs = ["ori_<name>_shim"]

# Only if Ori modules import another package:
# [dependencies]
# foo = { path = "../ori-foo", version = "0.1.0" }
```

### 4.4 Acceptance per package (Definition of Done)

1. `./tools/build_linux.sh` succeeds on Linux x86_64.  
2. `./tools/smoke_linux.sh` prints `smoke ok` (and smoke example prints `ok`).  
3. `ori test tests/...` has **0 failed** (use AOT; if AOT needs ld-script, add it).  
4. README documents API + FULL flags if any.  
5. Catalog + `eco-packages-status.md` updated in the **docs PR** (or same PR if single-package slice).  
6. No secrets committed; vendor licenses noted in README.

### 4.5 Validation environment (implementers)

```bash
export ORI_BIN="${ORI_BIN:-/home/raillen/.grok/worktrees/projetos-ori-lang/game-engine-final/compiler/target/debug/ori}"
export ORI_RUNTIME_CDYLIB="${ORI_RUNTIME_CDYLIB:-/home/raillen/.grok/worktrees/projetos-ori-lang/game-engine-final/runtime/x86_64-unknown-linux-gnu/libori_runtime.so}"
export ORI_RUNTIME_LIB="${ORI_RUNTIME_LIB:-/home/raillen/.grok/worktrees/projetos-ori-lang/game-engine-final/runtime/x86_64-unknown-linux-gnu/libori_runtime.a}"
export ORI_USE_SYSTEM_LINKER=1
export ORI_USE_JIT=1
```

If debug `ori` is missing, build:  
`cd …/game-engine-final/compiler && cargo build -p ori-driver` and stage runtime cdylib/staticlib as above.

---

## 5. Per-package implementation recipes

### 5.1 ori-cgltf

**Upstream:** single-header `cgltf.h` (+ optional write).  
**Shim focus:**

- `load_file(path)` → handle  
- counts: meshes, nodes, materials, animations  
- mesh accessors: vertex count, index count (milli or raw floats via opaque buffer + getters)  
- free  

**Tests:** load a tiny fixture glTF (commit a minimal `.gltf` under `tests/fixtures/` or generate in smoke).  
**Optional game wire (later PR):** `game.gltf` path-dep helper → mesh bake.

### 5.2 ori-fast-obj

**Upstream:** `fast_obj.h` single-header.  
**Shim:** load path → positions/normals/uv counts + sample getters; free.  
**Tests:** load minimal `.obj` fixture.  
**Dep:** none (complements cgltf; independent).

### 5.3 ori-physfs

**Upstream:** PhysFS (build static from source or system `libphysfs-dev`).  
**Shim:** `init` / `deinit`, `mount(path, mountPoint)`, `exists`, `read_bytes` length + sum, `enumerate` count.  
**Tests:** mount a temp dir with a known file; read content length.  
**Wire:** optional `game.physfs_assets` that feeds `asset_loader` / rres extract paths.

### 5.4 ori-clay

**Upstream:** Clay (C header library).  
**Shim:** begin layout, open element, text measure stub, end → compute box count + sample x/y/w/h milli.  
**Tests:** layout 2 boxes non-overlapping or parent/child sizes > 0.  
**Note:** Prefer Clay over Yoga (declined). Do **not** depend on raylib for core tests; draw helpers optional in game.

### 5.5 ori-recast

**Upstream:** Recast Navigation (Recast + Detour).  
**MVP surface (do not boil ocean):**

1. Build navmesh from a simple triangle soup (input as milli verts + indices lists or baked fixture).  
2. Find path: start/end positions → waypoint count > 0.  

**Shim strategy:** C++ wrapper around DetourNavMeshQuery; keep Ori API milli-int.  
**Tests:** square walkable plane path from (0,0) to (10,0).  
**Note:** 2D grid A\* already lives in `game.pathfind` — Recast is **3D navmesh**, not a replacement.

### 5.6 ori-lz4

**Upstream:** lz4 (frame or block API).  
**Shim:** compress/decompress buffer or cstring round-trip length (same pattern as miniz).  
**Tests:** round-trip length equality; compressed size < raw for compressible text.  
**Skip condition:** if PR triage decides miniz is enough for 12 months, mark PR as skipped in report — but default is **implement**.

### 5.7 ori-miniaudio (conditional)

**Only implement if** PR 0 (policy) records a measured gap in `game.audio` (e.g. missing device enumeration, decode without raylib, or multi-bus beyond current buses).  

**If skipped:** document in status “deferred — raylib audio sufficient”.  
**If done:** thin decoder + device list + play one buffer; do not dual-stack OpenAL.

### 5.8 Optional FULL flag polish (small PRs or fold into integration)

| Flag | Package | Effect |
|------|---------|--------|
| `ORI_IMPLOT_FULL=1` | ori-implot | Link ImPlot draw against ImGui |
| `ORI_IMNODES_FULL=1` | ori-imnodes | Full editor widgets |
| `ORI_IMGUIZMO_FULL=1` | ori-imguizmo | Manipulate() |
| `ORI_TRACY_FULL=1` | ori-tracy | TracyClient |

Default CI remains FULL=0. Document flags in each README (already started).

---

## 6. Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Location | Sibling repos under `Documentos/Projetos/ori-*`, not monorepo `packages/` | Matches ECO layout; path deps; independent versioning |
| OS order | Linux only in this plan; Phase OS last | User policy 2026-07-15 |
| Maturity | API + smoke + tests | Demos do not gate maturity |
| Package shape | Template §4 + copy from `ori-stb` / `ori-enkiTS` | Proven green smoke pattern |
| C++ AOT | GNU ld script `.a` → objs + `-lstdc++` | Works without distro static libstdc++ packaged |
| ECS | No flecs/EnTT | Catalog §7 |
| Physics | No second engine | box2d + jolt already 5 |
| Recast vs A\* | Both: pathfind 2D pure Ori; Recast 3D navmesh | Different problem domains |
| Clay vs Yoga | Clay only | Catalog decline Yoga |
| Assimp | No | cgltf + fast_obj |
| miniaudio | Conditional | Avoid dual audio stack unless gap measured |
| execute-plan multi-repo | Implementers write under absolute sibling paths; monorepo docs PR updates catalog | Packages are not all one git root |

---

## 7. Open Questions (defaults for auto-execution)

| Q | Default if user silent |
|---|------------------------|
| Implement miniaudio? | **Skip** unless `game.audio` gap written in PR 0 notes |
| Implement lz4? | **Yes** (small; completes media compression story) |
| Commit fixtures under package `tests/fixtures/`? | **Yes** (tiny synthetic assets only) |
| Create GitHub remotes automatically? | **No** — local sibling trees only |
| Phase OS in same execute-plan run? | **Yes as last PR**, scaffolding only if scripts missing; no multi-OS CI green required beyond documenting scripts |

---

## 8. Skills / implementer instructions

When executing each PR, implementers **must**:

1. Follow `clean-code` + Ori S3 conventions (project `AGENTS.md`).  
2. Prefer `c-secure` for C shims (bounds, free paths).  
3. Prefer `living-docs` for README/CHANGELOG/catalog.  
4. Prefer `ori-testing` style: check → smoke run → `ori test`.  
5. Not invent Ori syntax; match existing packages.  
6. Not break reserved keywords (`map`, `bind`, `ok` as function names).  
7. Keep diffs focused; no drive-by refactors of unrelated ECO packages.

**Cross-cutting user instruction string for `/execute-plan`:**

```text
--instructions "Linux only. Sibling packages at /home/raillen/Documentos/Projetos/ori-*. Copy smoke pattern from ori-stb/ori-enkiTS. Use int64_t ABI. Smoke must print ok; ori test 0 failed. Update eco-library-ports-catalog.md and eco-packages-status.md when adding a package. Phase OS last. No flecs/EnTT. Do not re-implement packages already marked done in §2 of this design doc."
```

---

## 9. How to run automatically

### Full remaining medium stack

```bash
/execute-plan /home/raillen/Documentos/Projetos/ori-lang/docs/planning/pr-plan-eco-ports-e2e.md \
  --concurrency 3 \
  --instructions "Linux only. Sibling packages at /home/raillen/Documentos/Projetos/ori-*. Copy smoke pattern from ori-stb/ori-enkiTS. Use int64_t ABI. Smoke must print ok; ori test 0 failed. Update eco-library-ports-catalog.md and eco-packages-status.md when adding a package. Phase OS last. No flecs/EnTT. Do not re-implement packages already marked done in §2 of this design doc."
```

### Dry-run (validate DAG only)

```bash
/execute-plan /home/raillen/Documentos/Projetos/ori-lang/docs/planning/pr-plan-eco-ports-e2e.md --dry-run
```

### Resume after failure

```bash
/execute-plan --resume <PLAN_ID>
```

**Note:** Packages are multi-repo. Graphite stack may only apply cleanly to `ori-lang` docs PRs; package trees may be plain directories without a single remote. Implementers still produce commits if those directories are git repos; if not, they create local git init **only if** the directory is not already a repo — prefer existing sibling layout without rewriting history of unrelated projects.

---

## PR Plan

### PR 1: Eco ports plan lock-in + remaining inventory

**Description:** Ensure this document is linked from `docs/planning/README.md` and `eco-library-ports-catalog.md` (pointer to e2e plan). Refresh status tables so **done** high ports are not re-queued. Add “execute-plan entrypoint” one-liner to `eco-packages-status.md` Next work section. No package code.

**Files/components affected:** `docs/planning/pr-plan-eco-ports-e2e.md`, `docs/planning/README.md`, `docs/planning/eco-library-ports-catalog.md`, `docs/planning/eco-packages-status.md`, `docs/planning/game-ports-maturity-matrix.md`

**Dependencies:** None

---

### PR 2: ori-cgltf package 0.1.0

**Description:** Create `/home/raillen/Documentos/Projetos/ori-cgltf` from template §4. Vendor `cgltf.h`. Shim: load/free, mesh/node/material/animation counts, minimal vertex count for mesh 0. Fixture glTF under `tests/fixtures/`. Smoke + tests green.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-cgltf/**` (new), optional catalog mention

**Dependencies:** None

---

### PR 3: ori-fast-obj package 0.1.0

**Description:** Create `ori-fast-obj` with `fast_obj` single-header. Load path → position count / face count; free. Fixture `.obj`. Smoke + tests.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-fast-obj/**` (new)

**Dependencies:** None

---

### PR 4: ori-physfs package 0.1.0

**Description:** Create `ori-physfs`. Build/link PhysFS (system dev package preferred: `libphysfs-dev`, else vendor+cmake). Shim init/mount/exists/read. Smoke mounts a temp directory written by the smoke script. Tests.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-physfs/**` (new)

**Dependencies:** None

---

### PR 5: ori-clay package 0.1.0

**Description:** Create `ori-clay`. Vendor Clay. Shim: simple layout compute → element count + bounds milli for root/children. Headless tests (no window). Smoke.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-clay/**` (new)

**Dependencies:** None

---

### PR 6: ori-lz4 package 0.1.0

**Description:** Create `ori-lz4` using lz4 block or frame API (prefer single amalgamation or system liblz4). Round-trip compress/decompress length tests; smoke.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-lz4/**` (new)

**Dependencies:** None

---

### PR 7: ori-recast package 0.1.0 (navmesh MVP)

**Description:** Create `ori-recast`. Vendor Recast+Detour (or subset). C++ shim: build navmesh from simple plane mesh; path query returns ≥2 waypoints. Milli-int Ori API. Build may be longer; keep MVP tight. Smoke + tests.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-recast/**` (new)

**Dependencies:** None

---

### PR 8: ori-game optional path-dep wires (Linux)

**Description:** Add optional path dependencies and thin modules only where valuable:

- `game.gltf` or load helper using `cgltf` if PR 2 done  
- `game.obj` helper using `fast_obj` if PR 3 done  
- optional PhysFS-backed open path for assets  

Keep modules optional (compile when path dep present). Add unit tests that skip gracefully or use fixtures. Do **not** force all games to link every native lib.

**Files/components affected:** `/home/raillen/Documentos/Projetos/ori-game/ori.pkg.toml`, `/home/raillen/Documentos/Projetos/ori-game/game/**`, `/home/raillen/Documentos/Projetos/ori-game/tests/**`, `/home/raillen/Documentos/Projetos/ori-game/CHANGELOG.md`

**Dependencies:** PR 2, PR 3

---

### PR 9: Catalog + matrix + status + umbrella smoke

**Description:** Mark all implemented medium ports as **done 0.1.0** in catalog §2/§4, status table, matrix B2.18 (medium done). Extend `ori-game/tools/smoke_eco_linux.sh` (or sibling `smoke_ports_linux.sh`) to call each package `smoke_linux.sh` if directory exists. Update this plan’s status line if needed.

**Files/components affected:** `docs/planning/eco-library-ports-catalog.md`, `docs/planning/eco-packages-status.md`, `docs/planning/game-ports-maturity-matrix.md`, `docs/planning/pr-plan-eco-ports-e2e.md`, `/home/raillen/Documentos/Projetos/ori-game/tools/smoke_eco_linux.sh` or new smoke script

**Dependencies:** PR 2, PR 3, PR 4, PR 5, PR 6, PR 7

---

### PR 10: Phase OS scaffolding (last) — **done**

**Description:** **Do not** block on multi-OS CI green. Ensure each new package has documented `tools/build_windows.ps1` stub **or** a single shared note in catalog that Phase OS is deferred. If scripts already exist for older packages, add one-line pointers. No requirement to run Windows/mac builds in this PR.

**Files/components affected:** `docs/planning/PHASE-OS.md` (if exists in ori-lang or ori-game), package READMEs (Phase OS section), `docs/planning/eco-packages-status.md`

**Dependencies:** PR 9

**Landed (2026-07-15):** medium packages `ori-cgltf` / `ori-fast-obj` / `ori-physfs` / `ori-clay` / `ori-lz4` / `ori-recast` each have README **Phase OS** section + deferred `tools/build_windows.ps1` (echo + exit 0). Docs: `PHASE-OS.md` medium table, `eco-packages-status.md` residual, this plan status **1–10 complete**. Phase OS remains **non-blocking**.

---

## 10. Suggested parallelism (levels)

| Level | PRs | Notes |
|-------|-----|-------|
| 0 | PR 1, PR 2, PR 3, PR 4, PR 5, PR 6, PR 7 | Independent packages + docs lock-in |
| 1 | PR 8 | Needs cgltf + fast_obj |
| 2 | PR 9 | Needs all medium packages |
| 3 | PR 10 | Phase OS last |

Linearized stack order for Graphite/plain-git:  
`PR1 → PR2 → PR3 → PR4 → PR5 → PR6 → PR7 → PR8 → PR9 → PR10`  
(within level 0, numeric order is fine even though 2–7 are parallelizable).

---

## 11. Risk register

| Risk | Mitigation |
|------|------------|
| Multi-repo git / no single remote | Work on local sibling paths; docs PRs in ori-lang; packages may be uncommitted trees until user pushes |
| PhysFS / Recast build complexity | Prefer system packages when available; vendor fallback; keep MVP APIs tiny |
| AOT C++ link | ld-script pattern from nfd/enkiTS |
| execute-plan worktree vs sibling paths | Instruct implementers to write absolute paths outside worktree when package is sibling; or copy tree into worktree if required by isolation |
| Recast compile time | Cap to Detour navmesh query + minimal Recast build path |
| miniaudio dual stack | Default skip |

---

## 12. Success criteria for the whole plan

1. Packages **ori-cgltf, ori-fast-obj, ori-physfs, ori-clay, ori-lz4, ori-recast** exist with green `smoke_linux.sh`.  
2. Catalog ALTA empty; MÉDIA only miniaudio (if skipped) + any deferred.  
3. `eco-packages-status.md` lists all new packages with maturity ≥3 Linux.  
4. Matrix B2.18 medium portion marked done (or partial only if a PR failed).  
5. User can re-run any package smoke without manual re-scaffolding.  
6. Phase OS remains last and non-blocking — **scaffolding done** (deferred stubs + docs); multi-OS CI green **not** required.

---

## Appendix A — Copy-paste package bootstrap (implementer)

```bash
NAME=cgltf   # example
ROOT=/home/raillen/Documentos/Projetos/ori-$NAME
mkdir -p "$ROOT"/{native,vendor,tools,examples,tests,$NAME,lib/x86_64-unknown-linux-gnu}
# then write ori.pkg.toml, shim, modules, build_linux.sh, smoke_linux.sh
# vendor upstream
# ./tools/smoke_linux.sh
```

## Appendix B — Related done packages (do not reopen)

`ori-freetype`, `ori-harfbuzz`, `ori-stb`, `ori-noise`, `ori-miniz`, `ori-nfd`, `ori-implot`, `ori-imnodes`, `ori-imguizmo`, `ori-tracy`, `ori-enkiTS`, core Linux-5 set.

---

*End of design document. The `## PR Plan` section above is the DAG consumed by `/execute-plan`.*
