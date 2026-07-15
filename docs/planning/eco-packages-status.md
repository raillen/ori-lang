# ECO packages status — Ori sibling ports (`ori-*`)

> **Status:** active (2026-07-15)  
> **Linux-5 program:** **complete** for the game-stack packages below.  
> **Policy (2026-07-15):** **implement / mature / port libs on Linux first.**  
> Multi-OS validation (Windows/mac) is **last** — scripts may exist, but execution is deferred.  
> **Canonical paths:** `/home/raillen/Documentos/Projetos/ori-*`  
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

Content modules in **`ori-game`:** `game.tiled`, `game.ldtk`, `game.aseprite`, `game.spine`, `game.rres_assets`, `game.marching_cubes` (+ `marching_cubes_draw`), …

---

## Layout

```
Documentos/Projetos/
  ori-lang/
  ori-raylib/
  ori-game/          # path-dep → ori-raylib
  ori-imgui/
  ori-raygui/
  ori-box2d/
  ori-jolt/
  ori-rres/
  ori-sqlite/
  ori-enet/
  ori-freetype/
  ori-harfbuzz/
  ori-stb/
  ori-noise/
  ori-miniz/
  ori-nfd/
  ori-implot/
  ori-imnodes/
  ori-imguizmo/
  ori-tracy/
  ori-enkiTS/
```

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
~/Documentos/Projetos/ori-game/tools/smoke_eco_linux.sh
```

---

## Phase OS (last)

Scaffolding may exist (`build_windows.ps1`, `smoke_eco_windows.ps1`).  
**Do not block lib work on multi-OS.** Run only after Linux implement/mature/port queue is satisfied.

---

## Next work (Linux-only)

**Auto-implement remaining medium ports** (no need to re-prompt stages):

```bash
/execute-plan docs/planning/pr-plan-eco-ports-e2e.md --concurrency 3 \
  --instructions "Linux only. Sibling packages at /home/raillen/Documentos/Projetos/ori-*. Copy smoke pattern from ori-stb/ori-enkiTS. Use int64_t ABI. Smoke must print ok; ori test 0 failed. Update eco-library-ports-catalog.md and eco-packages-status.md when adding a package. Phase OS last. No flecs/EnTT. Do not re-implement packages already marked done in §2 of this design doc."
```

Design + DAG: [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) · Catalog: [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md)

Done recently: high ports (stb/noise/miniz/nfd/implot/imnodes/imguizmo/tracy/enkits) + deepen B2.15–19.

Residual / roadmap:
1. Medium ports via execute-plan above (cgltf, fast_obj, physfs, clay, lz4, recast)  
2. Optional `ori-game` wires (PR 8 of the plan)  
3. Studio app = separate product track  
4. Phase OS = **last**  

**Do not re-queue as open alta:** freetype, harfbuzz, stb, noise, miniz, nfd, implot, imnodes, imguizmo, tracy, enkits (see inventory + catalog §2).

**ECS:** no flecs/EnTT as default — see catalog §7 / roadmap § ECS.

---

## Implementation matrix

Full history: [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md).
