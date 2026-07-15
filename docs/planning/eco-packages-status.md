# ECO packages status — Ori sibling ports (`ori-*`)

> **Status:** active (2026-07-15)  
> **Linux-5 program:** **complete** for the game-stack packages below.  
> **Policy (2026-07-15):** **implement / mature / port libs on Linux first.**  
> Multi-OS validation (Windows/mac) is **last** — scripts may exist, but execution is deferred.  
> **Canonical paths:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-*`  
> **Matrix:** [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md) ·  
> **Catálogo de ports (canônico):** [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md) ·  
> **Roadmap:** `ori-game/docs/planning/ROADMAP-GAME-ECO.md`

---

## Inventory (Linux product surface)

| Repo | Package | Ver. | Role | Maturity |
|------|---------|------|------|----------|
| **`ori-raylib`** | `raylib` | **0.1.0** | L0 raylib bindings (`ori_rl_*` shim) | **5 (Linux)** |
| **`ori-game`** | `ori_game` | **0.3.0** | L1 `game.*` (2D/3D, audio, content loaders) | **5 (Linux)** |
| **`ori-imgui`** | `imgui` | **0.4.0** | Dear ImGui + Tier0/1 + optional raylib embed | **5 (Linux)** |
| **`ori-raygui`** | `raygui` | **0.2.0** | Immediate UI on raylib | **5 (Linux)** |
| **`ori-box2d`** | `box2d` | **0.3.0** | Box2D 3.x milli-unit physics | **5 (Linux)** |
| **`ori-jolt`** | `jolt` | **0.2.0** | Jolt 3D physics | **5 (Linux)** |
| **`ori-rres`** | `rres` | **0.3.0** | ORPK resource packs | **5 (Linux)** |
| **`ori-sqlite`** | `sqlite` | **0.3.0** | SQLite + prepared/multi-row | **5 (Linux)** |
| **`ori-enet`** | `enet` | **0.3.0** | ENet multiplayer (channels/protocol) | **5 (Linux)** |
| **`ori-freetype`** | `freetype` | **0.1.0** | FreeType face + text + gray atlas | **5 (Linux)** |
| **`ori-harfbuzz`** | `harfbuzz` | **0.1.0** | shape/layout + AOT tests (needs FreeType) | **5 (Linux)** |
| **`ori-stb`** | `stb` | **0.1.0** | image / perlin / rect_pack | **3–4 (Linux)** |
| **`ori-noise`** | `noise` | **0.1.0** | FastNoiseLite | **3–4 (Linux)** |
| **`ori-miniz`** | `miniz` | **0.1.0** | deflate / CRC32 | **3–4 (Linux)** |
| **`ori-nfd`** | `nfd` | **0.1.0** | file dialogs (pfd) | **3 (Linux)** |
| **`ori-implot`** | `implot` | **0.1.0** | ImPlot series + draw (FULL flag) | **3 (Linux)** |
| **`ori-imnodes`** | `imnodes` | **0.1.0** | node graph (FULL flag) | **3 (Linux)** |
| **`ori-imguizmo`** | `imguizmo` | **0.1.0** | gizmo translate (FULL flag) | **3 (Linux)** |
| **`ori-tracy`** | `tracy` | **0.1.0** | zones/frames (FULL TracyClient) | **3 (Linux)** |
| **`ori-enkiTS`** | `enkits` | **0.1.0** | task scheduler (parallel sum) | **3–4 (Linux)** |
| **`ori-cgltf`** | `cgltf` | **0.1.0** | glTF 2.0 load (meshes/nodes/materials/anim) | **3 (Linux)** |
| **`ori-fast-obj`** | `fast_obj` | **0.1.0** | Wavefront OBJ load | **3 (Linux)** |
| **`ori-physfs`** | `physfs` | **0.1.0** | virtual FS / multi-archive | **3 (Linux)** |
| **`ori-clay`** | `clay` | **0.1.0** | immediate-mode UI layout | **3 (Linux)** |
| **`ori-lz4`** | `lz4` | **0.1.0** | LZ4 compress/decompress | **3 (Linux)** |
| **`ori-recast`** | `recast` | **0.1.0** | Recast+Detour navmesh MVP | **3 (Linux)** |

Content modules in **`ori-game`:** `game.tiled`, `game.ldtk`, `game.aseprite`, `game.spine`, `game.rres_assets`, `game.marching_cubes` (+ `marching_cubes_draw`), …

---

## Layout

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

Path deps between packages stay sibling-relative (`../ori-raylib`, …) inside `game-engine-full/`.

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

Canonical write-up: [`PHASE-OS.md`](PHASE-OS.md). Umbrella: `ori-game/tools/smoke_eco_windows.ps1` (core only).

---

## Layout (2026-07-15)

ECO game packages live under **`Documentos/Projetos/game-engine-full/`** (model A: one folder, N git remotes). `ori-lang` and `ori-game-studio` stay siblings of that folder under `Projetos/`.

## Next work (Linux-only)

**Medium ports (M1–M6):** **done 0.1.0** — `ori-cgltf`, `ori-fast-obj`, `ori-physfs`, `ori-clay`, `ori-lz4`, `ori-recast`.  
**Execute-plan** [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md): **PRs 1–10 complete** (PR 10 = Phase OS scaffolding only).  
Catalog: [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md)

Done recently: high ports (stb/noise/miniz/nfd/implot/imnodes/imguizmo/tracy/enkits) + medium M1–M6 + deepen B2.15–19 + Phase OS stubs for medium.

Residual / roadmap (2026-07-15 post e2e):
1. **`ori-miniaudio` skipped** — gap measured: `game.audio` via raylib covers SFX, music streams, buses, seek/pitch/pan, sound pools. Revisit only for non-raylib backends or spatial 3D (then prefer catalog §5 OpenAL, not dual-stack).  
2. **`ori-game` wires landed** — `game.gltf` / `game.obj` / `game.physfs_assets` + path-deps (execute-plan PR8).  
3. **Git local** — medium packages M1–M6 have initial `master` commits (no remotes yet).  
4. Studio app = separate product track  
5. Phase OS **execution** on real MSVC host = **last** (scaffolding done; non-blocking)  

**Do not re-queue as open alta/média:** freetype, harfbuzz, stb, noise, miniz, nfd, implot, imnodes, imguizmo, tracy, enkits, cgltf, fast_obj, physfs, clay, lz4, recast (see inventory + catalog §2).

**ECS:** no flecs/EnTT as default — see catalog §7 / roadmap § ECS.

---

## Implementation matrix

Full history: [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md).
