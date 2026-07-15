# ECO packages status — Ori sibling ports (`ori-*`)

> **Status:** active (2026-07-15)  
> **Linux-5 core stack:** **complete** (raylib…harfbuzz).  
> **Active program:** raise remaining ports **3–4 → 5 (Linux)** —  
> [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md) (`/execute-plan`, plan id `5b7bfbb0`).  
> **Policy (2026-07-15):** **implement / mature / port libs on Linux first.**  
> Multi-OS validation (Windows/mac) is **last** — scripts may exist, but execution is deferred.  
> **Canonical paths:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-*`  
> **Matrix:** [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md) ·  
> **Catálogo de ports (canônico):** [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md) ·  
> **Roadmap:** `ori-game/docs/planning/ROADMAP-GAME-ECO.md`

---

## Score **5 (Linux)** gate (brief)

A package is **5 (Linux)** when plan §3 **G1–G7** hold on Linux:

| # | Criterion |
|---|-----------|
| **G1** | Broad product API (not smoke-only) — checklist in plan §4 |
| **G2** | ≥4 automated tests (or ≥3 if surface is tiny); `ori test` 0 failed |
| **G3** | Green `tools/smoke_linux.sh` (prints `ok`) |
| **G4** | README documents API + Phase OS note |
| **G5** | CHANGELOG entry for the maturity bump |
| **G6** | No dual path-dep leaf collision |
| **G7** | Version bump in `ori.pkg.toml` |

**Not required:** multi-OS, Marketplace, windowed demos, Studio. Full text: plan §3.

---

## Already **5 (Linux)** — do not re-implement

| Repo | Package | Ver. | Role |
|------|---------|------|------|
| **`ori-raylib`** | `raylib` | **0.1.0** | L0 raylib bindings (`ori_rl_*` shim) |
| **`ori-game`** | `ori_game` | **0.3.0** | L1 `game.*` (2D/3D, audio, content loaders) — wires deepen only |
| **`ori-imgui`** | `imgui` | **0.4.0** | Dear ImGui + Tier0/1 + optional raylib embed |
| **`ori-raygui`** | `raygui` | **0.2.0** | Immediate UI on raylib |
| **`ori-box2d`** | `box2d` | **0.3.0** | Box2D 3.x milli-unit physics |
| **`ori-jolt`** | `jolt` | **0.2.0** | Jolt 3D physics |
| **`ori-rres`** | `rres` | **0.3.0** | ORPK resource packs |
| **`ori-sqlite`** | `sqlite` | **0.3.0** | SQLite + prepared/multi-row |
| **`ori-enet`** | `enet` | **0.3.0** | ENet multiplayer (channels/protocol) |
| **`ori-freetype`** | `freetype` | **0.1.0** | FreeType face + text + gray atlas |
| **`ori-harfbuzz`** | `harfbuzz` | **0.1.0** | shape/layout + AOT tests (needs FreeType) |

Bugfix-only touch-ups allowed if a dependent PR needs them.

---

## U1–U15 — need work → **5 (Linux)**

Canonical API targets: plan §4. Paths under `game-engine-full/`.

| ID | Repo | Package | Ver. | Now | Plan PR |
|----|------|---------|------|-----|---------|
| **U1** | `ori-stb` | `stb` | 0.1.0 | 3–4 | PR 2 |
| **U2** | `ori-noise` | `noise` | 0.1.0 | 3–4 | PR 3 |
| **U3** | `ori-miniz` | `miniz` | 0.1.0 | 3–4 | PR 4 |
| **U4** | `ori-lz4` | `lz4` | 0.1.0 | 3 | PR 5 |
| **U5** | `ori-nfd` | `nfd` | 0.1.0 | 3 | PR 6 |
| **U6** | `ori-implot` | `implot` | 0.1.0 | 3 | PR 7 |
| **U7** | `ori-imnodes` | `imnodes` | 0.1.0 | 3 | PR 8 |
| **U8** | `ori-imguizmo` | `imguizmo` | 0.1.0 | 3 | PR 9 |
| **U9** | `ori-tracy` | `tracy` | 0.1.0 | 3 | PR 10 |
| **U10** | `ori-enkiTS` | `enkits` | 0.1.0 | 3–4 | PR 11 |
| **U11** | `ori-cgltf` | `cgltf` | 0.1.0 | 3 | PR 12 |
| **U12** | `ori-fast-obj` | `fast_obj` | 0.1.0 | 3 | PR 13 |
| **U13** | `ori-physfs` | `physfs` | 0.1.0 | 3 | PR 14 |
| **U14** | `ori-clay` | `clay` | 0.1.0 | 3 | PR 15 |
| **U15** | `ori-recast` | `recast` | 0.1.0 | 3 | PR 16 |

Content modules in **`ori-game`:** `game.tiled`, `game.ldtk`, `game.aseprite`, `game.spine`, `game.rres_assets`, `game.marching_cubes` (+ `marching_cubes_draw`), `game.gltf` / `game.obj` / `game.physfs_assets` (deepen in plan PR 17).

---

## Layout (`game-engine-full`)

ECO game packages live under **`/home/raillen/Documentos/Projetos/game-engine-full/`** (model A: one folder, **N git remotes**).  
`ori-lang` (compiler docs for this inventory) and `ori-game-studio` stay **siblings** of that folder under `Projetos/` — not inside the cluster.

```
Documentos/Projetos/
  ori-lang/                    # compiler (outside cluster)
  ori-game-studio/             # Tauri app (outside cluster)
  game-engine-full/            # ECO game libs — each keeps own git remote
    ori-raylib/                # L0
    ori-game/                  # L1 hub (path-dep → siblings)
    ori-box2d/  ori-jolt/  ori-recast/
    ori-imgui/  ori-raygui/  ori-clay/
    ori-implot/ ori-imnodes/ ori-imguizmo/
    ori-freetype/ ori-harfbuzz/
    ori-rres/ ori-cgltf/ ori-fast-obj/ ori-physfs/
    ori-stb/ ori-noise/ ori-miniz/ ori-lz4/
    ori-enet/ ori-sqlite/ ori-enkiTS/ ori-tracy/ ori-nfd/
```

Path deps stay sibling-relative (`../ori-raylib`, …) inside `game-engine-full/`.

```toml
[dependencies]
raylib   = { path = "../ori-raylib", version = "0.1.0" }
ori_game = { path = "../ori-game", version = "0.3.0" }
imgui    = { path = "../ori-imgui", version = "0.4.0" }
box2d    = { path = "../ori-box2d", version = "0.3.0" }
enet     = { path = "../ori-enet", version = "0.3.0" }
```

---

## Smoke (Linux)

```bash
export ORI_BIN=$(command -v ori) ORI_USE_SYSTEM_LINKER=1
~/Documentos/Projetos/game-engine-full/ori-game/tools/smoke_eco_linux.sh
```

---

## Phase OS (last — **non-blocking**)

**Policy:** do **not** block lib work or multi-OS CI green on Windows/mac.

| Tier | Scripts | Status |
|------|---------|--------|
| Core (game, box2d, jolt, sqlite, rres, imgui, raygui, enet) | real/stub `build_windows.ps1` + smoke | scripts ready — execute on MSVC host |
| Medium M1–M6 (cgltf, fast_obj, physfs, clay, lz4, recast) | **deferred** `tools/build_windows.ps1` (echo only) | documented Linux-only 0.1.0 |
| U1–U15 maturity deepens | Phase OS still last | score 5 does **not** require Win/mac |

Canonical write-up: [`PHASE-OS.md`](PHASE-OS.md). Umbrella: `ori-game/tools/smoke_eco_windows.ps1` (core only).

---

## Next work (Linux-only)

### Execute-plan entrypoint (active)

```bash
/execute-plan docs/planning/pr-plan-eco-maturity-5.md --concurrency 4 \
  --instructions "Linux only. Packages at /home/raillen/Documentos/Projetos/game-engine-full/ori-*. Each package keeps its own git repo. Path deps sibling-relative. int64_t ABI; milli-float. Smoke ok; ori test 0 failed. Version bump. Score 5 = plan §3 G1-G7. No re-port maturity-5 packages. Phase OS last. No flecs/miniaudio/dual physics."
```

**Active plan:** [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md) — U1–U15 → 5, then ori-game wires (PR 17), catalog (PR 18), Phase OS note (PR 19 last).

Prior ports plan [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md): **PRs 1–10 complete** (0.1.0 scaffolds; do not re-scaffold).  
Catalog: [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md)

Residual / roadmap (2026-07-15):
1. **Maturity → 5 (Linux)** for U1–U15 — **in progress** via maturity-5 plan.  
2. **`ori-miniaudio` skipped** — gap measured: `game.audio` via raylib covers SFX/music/buses/pool. Revisit only for non-raylib or spatial 3D (catalog §5 OpenAL).  
3. **`ori-game` wires** — gltf/obj/physfs landed; deepen in maturity-5 plan PR 17.  
4. Studio app = separate product track (`ori-game-studio` outside `game-engine-full`)  
5. Phase OS **execution** on real MSVC host = **last** (scaffolding done; non-blocking)  

**Do not re-queue as open alta/média ports:** packages already exist — use maturity-5 plan to deepen, not re-scaffold.

**ECS:** no flecs/EnTT as default — see catalog §7 / roadmap § ECS.

---

## Implementation matrix

Full history: [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md).
