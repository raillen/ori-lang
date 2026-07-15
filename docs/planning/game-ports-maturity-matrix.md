# Game / ECO ports — maturity & backlog matrix

> **Status:** active consult doc (implementation reference)  
> **Updated:** 2026-07-15  
> **Program:** Linux-5 maturity wave **completed**.  
> **Policy:** implement / mature / port libs **on Linux first**. Multi-OS (**Phase OS**) is **last**.  
> **Related:** [`eco-packages-status.md`](eco-packages-status.md) ·  
> [`eco-library-ports-catalog.md`](eco-library-ports-catalog.md) ·  
> [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) (medium ports `/execute-plan`) ·  
> `ori-game/docs/planning/ROADMAP-GAME-ECO.md`

### Maturity scale

| Score | Meaning |
|------:|---------|
| **1** | Skeleton / plan only |
| **2** | MVP links + minimal demo/smoke |
| **3** | Small real use / jam-viable on Linux |
| **4** | Broad surface + tests + several demos |
| **5** | Product engine-grade on **Linux** (this program’s target). Multi-OS = Phase OS |

---

## Wave progress (Linux-5 program)

| Wave | Focus | Status |
|------|--------|--------|
| **W1** | ori-game S3 demo hygiene + smoke expand | **done** |
| **W2** | ori-game gamepad + RenderTexture → 0.3.0 | **done** |
| **W3** | ori-box2d → 0.3.0 | **done** |
| **W4** | ori-jolt → 0.2.0 | **done** |
| **W5** | raygui 0.2.0 + imgui 0.3.0 | **done** |
| **W6** | rres 0.3.0 + sqlite 0.3.0 | **done** |
| **W7** | Matrix gate all **5 (Linux)** | **done** |
| **W8** | Integration demos + umbrella smoke | **done** (2026-07-14) |
| **W9+** | Deepen + new ports (Linux) | **active** — ImGui T2, surface 4→5, new `ori-*` |
| **Phase OS** | Win/mac stage + smoke | **last** (scripts may exist; not blocking) |

---

## Table A — Already ported (Linux-5)

| Package | Repo | Ver. | Maturity | Status |
|---------|------|------|----------|--------|
| `raylib` L0 | `ori-raylib` | **0.1.0** | **5 (Linux)** | split from ori-game |
| `ori_game` | `ori-game` | **0.3.0** | **5 (Linux)** | L1 `game.*` + content loaders |
| `box2d` | `ori-box2d` | **0.3.0** | **5 (Linux)** | joints, poly4, queries, contacts, materials |
| `jolt` | `ori-jolt` | **0.2.0** | **5 (Linux)** | layers, friction, torque, floor, hit body |
| `imgui` | `ori-imgui` | **0.4.0** | **5 (Linux)** | Tier0 dock/tables + Tier1 file/plot/nodes |
| `raygui` | `ori-raygui` | **0.2.0** | **5 (Linux)** | textbox, toggle, dropdown, style, … |
| `rres` | `ori-rres` | **0.3.0** | **5 (Linux)** | validate, list_names, read_bytes |
| `sqlite` | `ori-sqlite` | **0.3.0** | **5 (Linux)** | prepared + multi-row JSON |
| `enet` | `ori-enet` | **0.3.0** | **5 (Linux)** | channels, broadcast, protocol |
| `freetype` | `ori-freetype` | **0.1.0** | **5 (Linux)** | face + text + gray atlas — **do not re-queue** |
| `harfbuzz` | `ori-harfbuzz` | **0.1.0** | **5 (Linux)** | shape/layout + AOT tests — **do not re-queue** |
| `stb` | `ori-stb` | **0.1.0** | **3–4 (Linux)** | image / perlin / rect_pack — **alta done** |
| `noise` | `ori-noise` | **0.1.0** | **3–4 (Linux)** | FastNoiseLite — **alta done** |
| `miniz` | `ori-miniz` | **0.1.0** | **3–4 (Linux)** | deflate / CRC — **alta done** |
| `nfd` | `ori-nfd` | **0.1.0** | **3 (Linux)** | file dialogs — **alta done** |
| `implot` | `ori-implot` | **0.1.0** | **3 (Linux)** | series + FULL draw — **alta done** |
| `imnodes` | `ori-imnodes` | **0.1.0** | **3 (Linux)** | node graph + FULL — **alta done** |
| `imguizmo` | `ori-imguizmo` | **0.1.0** | **3 (Linux)** | gizmo translate + FULL — **alta done** |
| `tracy` | `ori-tracy` | **0.1.0** | **3 (Linux)** | zones/frames + FULL — **alta done** |
| `enkits` | `ori-enkiTS` | **0.1.0** | **3–4 (Linux)** | task scheduler — **alta done** |
| `cgltf` | `ori-cgltf` | **0.1.0** | **3 (Linux)** | glTF 2.0 — **medium done** |
| `fast_obj` | `ori-fast-obj` | **0.1.0** | **3 (Linux)** | Wavefront OBJ — **medium done** |
| `physfs` | `ori-physfs` | **0.1.0** | **3 (Linux)** | virtual FS — **medium done** |
| `clay` | `ori-clay` | **0.1.0** | **3 (Linux)** | IM layout — **medium done** |
| `lz4` | `ori-lz4` | **0.1.0** | **3 (Linux)** | LZ4 compress — **medium done** |
| `recast` | `ori-recast` | **0.1.0** | **3 (Linux)** | navmesh MVP — **medium done** |
| Studio | plan only | — | **0.5–1** | Separate product track |

### Detail surfaces (ori-game)

| Surface | Maturity |
|---------|----------|
| `game.app` | 5 |
| `game.input` (+ gamepad) | 5 |
| `game.draw` (+ RenderTexture) | 5 |
| `game.audio` | **5** (buses, pool, seek) |
| 2D systems (tilemap, particles, inventory, dialogue, scene, …) | **5** |
| 3D / shaders / light bank | **5** (presets; no shadow) |
| Mechanics | **5** (i-frames, patrol/aggro) |

---

## Table B — Remaining (post Linux-5)

### B1 — Phase OS (multi-OS) — **last** (non-blocking)

| ID | Item | Priority | Notes |
|----|------|----------|-------|
| **B1.4** | Stage Win (+ mac) libs for all ECO packages | **last** | Core scripts ready; medium M1–M6 = deferred stubs only ([`PHASE-OS.md`](PHASE-OS.md)) |
| **B1.4b** | smoke_windows / CI multi-OS | **last** | **Do not** require green for product progress |

Scaffolding for medium packages (README + `build_windows.ps1` echo stubs): **done** (plan PR 10).

### B2 — Deepen + ports (Linux — **active**)

| ID | Item | Priority | Status |
|----|------|----------|--------|
| **B2.1** | Tiled + LDtk | P1 | **done** — `game.tiled` + `game.ldtk` |
| **B2.2** | enet / multiplayer | P1 | **done** — `ori-enet` **0.3.0** |
| **B2.3** | Aseprite + Spine | P1 | **done** — `game.aseprite` + `game.spine` |
| **B2.4** | 3D/audio deepen | P2 | **done** — pitch/pan/master; cylinder/capsule/billboard |
| **B2.5** | rres ↔ assets + physics debug draw | P2 | **done** — `game.rres_assets`; debug_draw |
| **B1.15** | ImGui inside raylib window | P2 | **done** — `imgui.init_raylib` |
| **B2.12** | Split `ori-raylib` | P3 | **done** — `raylib` 0.1.0 |
| **B2.13** | ImGui Tier 0 dock/tables | P2 | **done** |
| **B2.14** | ImGui Tier 1 file/plot/nodes | P2 | **done** — **0.4.0** |
| **B2.15** | ImGui multi-context (editor vs game) | P2 | **done** — create/set/destroy context |
| **B2.16** | ImGui Tier 2 (style, image, curves, timeline) | P2 | **done** MVP — pure Ori curves/timeline; style/image host |
| **B2.17** | Surface maturity 4→5 (3D/shaders, mechanics, audio edge) | P2 | **done** — buses/pool/seek; combat i-frames; fog presets; dialogue/inv |
| **B2.18** | New sibling ports (product-driven `ori-*`) | P2 | **done (high + medium)** — ALTA fechada (stb/noise/miniz/nfd/implot/imnodes/imguizmo/tracy/enkits **0.1.0**). MÉDIA M1–M6 done (cgltf/fast_obj/physfs/clay/lz4/recast **0.1.0**). Residual open: `ori-miniaudio` only if gap. **Do not re-queue** done ports. Plan: [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) |
| **B2.19** | In-`ori-game` exploration (camera, save, A\*, actions) | P2 | **done** — camera limits/shake; slots; pathfind; actions; cutscene; net_predict |
| **B2.20** | ECS (flecs/EnTT) | — | **declined as default** — optional only if measured need |

### B3 — Studio

| ID | Item | Priority |
|----|------|----------|
| **B3.1** | Ori Game Studio Tauri app | separate product track |

---

## Acceptance notes (Linux-5 definition used)

- Product surface for jam/mid-size games on **Linux**, not 100% C 1:1 parity.
- Each package: S3 sources, smoke green, README/CHANGELOG, version bump.
- Multi-OS deferred by explicit user decision (2026-07-14).

## How to update

After Phase OS work: raise maturity to **5 (Linux+Win)** etc., clear B1 rows.
