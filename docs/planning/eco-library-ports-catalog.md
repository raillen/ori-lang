# Catálogo canônico — tradução de bibliotecas nativas para Ori (ECO)

> **Status:** canônico (2026-07-15) · **alta fechada** · **médios M1–M6 done 0.1.0** · miniaudio **skipped** · **maturidade → 5 active** · Phase OS last  

> **Âmbito:** packages irmãos `ori-*` (bindings / ports C·C++ → Ori S3), **não** stdlib do monorepo.  
> **Cluster path:** `/home/raillen/Documentos/Projetos/game-engine-full/ori-*` (model A: pasta única, N remotes).  
> **Política:** Linux implement / mature / port **primeiro**. Multi-OS (**Phase OS**) por **último**.  
> **Maturidade de superfície:** por **API + smoke da lib**, não por exemplos (exemplos podem vir depois).  
> **Score 5 (Linux) gate (resumo):** G1 broad API · G2 ≥4 tests · G3 smoke ok · G4 README · G5 CHANGELOG · G6 leaf único · G7 version bump — detalhe em [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md) §3.  
> **Execute-plan (ports médios 0.1.0):** [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) — **complete**  
> **Execute-plan (maturidade → 5 Linux) — active Next work:** [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md) — DAG U1–U15 + wires + docs  

> **Relacionados:**  
> - Inventário vivo + Next work: [`eco-packages-status.md`](eco-packages-status.md)  
> - Matriz de maturidade: [`game-ports-maturity-matrix.md`](game-ports-maturity-matrix.md)  
> - Fila de implementação game: `ori-game/docs/planning/ROADMAP-GAME-ECO.md`  
> - Convenções de package: [`package-ecosystem-guidelines.md`](package-ecosystem-guidelines.md)

**Este arquivo é a fonte de verdade** para *o que* portar, *qual* upstream usar e *com que prioridade*.  
O plano antigo [`porting-raylib-sqlite-cimgui.md`](porting-raylib-sqlite-cimgui.md) ficou histórico e aponta para cá.

---

## 1. Regras de port (quando vale traduzir)

Só abrir um `ori-<nome>` quando:

1. Há **necessidade de produto** (game, tools ou Studio).  
2. A lib tem **ABI amigável** (C, single-header, ou C++ com wrapper fino).  
3. **Não duplica** uma superfície que já está em maturidade **5 (Linux)** sem gap real.  
4. Preferir **módulo pure Ori** em `ori-game` / stdlib quando não houver FFI nativo.

| Nomenclatura | Regra |
|--------------|--------|
| Repo GitHub | `ori-<lib>` |
| `ori.pkg.toml` `name` | **sem** prefixo `ori-` (ex.: `name = "imgui"`) |
| Import | `import imgui.ui`, `import enet.host`, … |

---

## 2. Já traduzidos (inventário)

Repos sob **`game-engine-full/ori-*`**. Maturidade **5** = gate G1–G7 (plan §3); packages **&lt; 5** sobem via U1–U15 (não re-scaffold).

### 2.1 Já **5 (Linux)** — não reabrir como projeto de maturidade

| Package Ori | Repo | Upstream / papel | Ver. | Maturidade |
|-------------|------|------------------|------|------------|
| `raylib` | `ori-raylib` | [raylib](https://www.raylib.com/) L0 + shim | 0.1.0 | **5 (Linux)** |
| `ori_game` | `ori-game` | helpers L1 + content loaders (tiled, ldtk, aseprite, spine, …) | 0.3.0 | **5 (Linux)** (wires deepen = plan PR 17) |
| `imgui` | `ori-imgui` | [Dear ImGui](https://github.com/ocornut/imgui) / cimgui | 0.4.0 | **5 (Linux)** |
| `raygui` | `ori-raygui` | [raygui](https://github.com/raysan5/raygui) | 0.2.0 | **5 (Linux)** |
| `box2d` | `ori-box2d` | [Box2D](https://box2d.org/) 3.x milli-int | 0.3.0 | **5 (Linux)** |
| `jolt` | `ori-jolt` | [Jolt Physics](https://github.com/jrouwe/JoltPhysics) | 0.2.0 | **5 (Linux)** |
| `rres` | `ori-rres` | packs ORPK (espírito [rres](https://github.com/raysan5/rres)) | 0.3.0 | **5 (Linux)** |
| `sqlite` | `ori-sqlite` | [SQLite](https://sqlite.org) amalgamation | 0.3.0 | **5 (Linux)** |
| `enet` | `ori-enet` | [ENet](https://github.com/lsalzman/enet) | 0.3.0 | **5 (Linux)** |
| `freetype` | `ori-freetype` | FreeType face + text + **atlas** | **0.1.0** | **5 (Linux)** |
| `harfbuzz` | `ori-harfbuzz` | shape/layout + **AOT tests** | **0.1.0** | **5 (Linux)** |
| *(module)* | `ori-game` | MC + export_obj + GPU bake path | — | **5 (Linux)** |

### 2.2 Portados 0.1.0 — **U1–U15** (need → **5 Linux**)

| ID | Package Ori | Repo | Upstream / papel | Ver. | Now | Plan PR |
|----|-------------|------|------------------|------|-----|---------|
| **U1** | `stb` | `ori-stb` | [stb](https://github.com/nothings/stb) image/perlin/rect_pack | 0.1.0 | 3–4 | PR 2 |
| **U2** | `noise` | `ori-noise` | [FastNoiseLite](https://github.com/Auburn/FastNoiseLite) | 0.1.0 | 3–4 | PR 3 |
| **U3** | `miniz` | `ori-miniz` | [miniz](https://github.com/richgel999/miniz) deflate/CRC | 0.1.0 | 3–4 | PR 4 |
| **U4** | `lz4` | `ori-lz4` | [lz4](https://github.com/lz4/lz4) compression | 0.1.0 | 3 | PR 5 |
| **U5** | `nfd` | `ori-nfd` | portable-file-dialogs | 0.1.0 | 3 | PR 6 |
| **U6** | `implot` | `ori-implot` | [implot](https://github.com/epezent/implot) (+ FULL draw) | 0.1.0 | 3 | PR 7 |
| **U7** | `imnodes` | `ori-imnodes` | [imnodes](https://github.com/Nelarius/imnodes) (+ FULL) | 0.1.0 | 3 | PR 8 |
| **U8** | `imguizmo` | `ori-imguizmo` | [ImGuizmo](https://github.com/CedricGuillemet/ImGuizmo) (+ FULL) | 0.1.0 | 3 | PR 9 |
| **U9** | `tracy` | `ori-tracy` | [Tracy](https://github.com/wolfpld/tracy) zones/frames | 0.1.0 | 3 | PR 10 |
| **U10** | `enkits` | `ori-enkiTS` | [enkiTS](https://github.com/dougbinks/enkiTS) task scheduler | 0.1.0 | 3–4 | PR 11 |
| **U11** | `cgltf` | `ori-cgltf` | [cgltf](https://github.com/jkuhlmann/cgltf) glTF 2.0 | 0.1.0 | 3 | PR 12 |
| **U12** | `fast_obj` | `ori-fast-obj` | [fast_obj](https://github.com/thisistherk/fast_obj) OBJ | 0.1.0 | 3 | PR 13 |
| **U13** | `physfs` | `ori-physfs` | [PhysFS](https://github.com/icculus/physfs) virtual FS | 0.1.0 | 3 | PR 14 |
| **U14** | `clay` | `ori-clay` | [Clay](https://github.com/nicbarker/clay) IM layout | 0.1.0 | 3 | PR 15 |
| **U15** | `recast` | `ori-recast` | [Recast Navigation](https://github.com/recastnavigation/recastnavigation) navmesh | 0.1.0 | 3 | PR 16 |

Detalhe de superfícies **5** dentro de `ori-game` (audio, 2D, content, 3D, mechanics): ver matriz + ROADMAP do game — **não** são packages novos.

**Do not re-queue as new ports:** todos os repos acima já existem — deepen via [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md), não re-scaffold.

---

## 3. Prioridade **alta** — **fila fechada** (2026-07-15)

Nada em aberto. Ports de alto valor (nfd, implot, imnodes, imguizmo, stb, noise, miniz, tracy, enkiTS) estão em §2.

| Package Ori | Status |
|-------------|--------|
| *(vazio)* | **Não reabrir** como fila alta — ver §2 |

**Next work (não é port novo):** maturidade U1–U15 → **5 (Linux)** — [`pr-plan-eco-maturity-5.md`](pr-plan-eco-maturity-5.md).  
Fila média de **novos** ports: **vazia** (`ori-miniaudio` **skipped** — `game.audio`+raylib fecha o gap). Plan e2e 0.1.0: [`pr-plan-eco-ports-e2e.md`](pr-plan-eco-ports-e2e.md) (PRs 1–10 done).

---

## 4. Prioridade **média** (portar sob necessidade de produto)

| Package Ori (proposto) | Upstream canônico | Papel / condição |
|------------------------|-------------------|------------------|
| **`ori-miniaudio`** | [miniaudio](https://github.com/mackron/miniaudio) | **Skipped (2026-07-15)** — gap medido: `game.audio` (raylib) cobre SFX/music/buses/pool; reabrir só sem raylib ou 3D espacial (§5 OpenAL) |

**Done 0.1.0 (moved to §2):** `ori-cgltf`, `ori-fast-obj`, `ori-physfs`, `ori-clay`, `ori-lz4`, `ori-recast` — execute-plan medium M1–M6 (2026-07-15).

**Não listar de novo:** FreeType, HarfBuzz, Marching Cubes, medium M1–M6 above — **done** em §2 / §8.1.

**Ordem sugerida (médio):** *vazio* — miniaudio skipped. Baixa = §5.

---

## 5. Prioridade **baixa / condicional**

Portar **somente** com requisito medido ou produto explícito.

| Package Ori (proposto) | Upstream canônico | Quando |
|------------------------|-------------------|--------|
| **`ori-openal`** | [OpenAL Soft](https://github.com/kcat/openal-soft) | Áudio **3D espacial** (HRTF); não dual-stack com miniaudio/raylib por padrão |
| **`ori-ozz`** | [ozz-animation](https://github.com/guillaumeblanc/ozz-animation) | Pipeline esquelético além de raylib anim + `game.spine` bones-only |
| **`ori-cute-c2`** | [cute_c2](https://github.com/RandyGaul/cute_headers) (cute_headers) | Collision 2D leve; **não** substitui box2d |
| **cute_tiled / cute_sound / cute_path** | cute_headers | Baixa: já cobertos por tiled / audio / A\* Ori |
| **Steamworks / Discord RPC** | SDKs oficiais | Fase ship / distribuição — não bloqueia engine |
| **Lua (host de mods)** | Lua PUC-Rio | Só se produto de mods for meta; não default |

---

## 6. **Não** portar por padrão (evitar / declined)

| Ideia | Motivo |
|-------|--------|
| **Yoga** | Preferir **Clay** (C, IM); Yoga = flex C++ mais pesado |
| **cglm / HandmadeMath** | Preferir `ori.math` + vec Ori / math raylib; evita segundo ABI de math |
| **bgfx** | Backend de **render completo** — conflita com raylib como caminho principal |
| **ejson** | Preferir **`ori.json`** |
| **Chipmunk / Bullet** | Não dual-stack physics enquanto box2d/jolt são Linux-5 |
| **Assimp** | Pesado; preferir cgltf + fast_obj |
| **flecs / EnTT (ECS)** | **Declined como default** — composition via structs + systems Ori; ver §7 |

---

## 7. ECS (flecs / EnTT)

**Decisão:** não traduzir flecs/EnTT como modelo default da engine ou do Studio.

- Filosofia Ori: composition **explícita**, reading-first, cena = dados, lógica = módulos `.orl`.  
- Já há “ECS-lite” com `ogame`, `game.mechanics.*`, loaders.  
- Port opcional **só** se Tracy/perfil mostrar necessidade de escala massiva de entidades.

---

## 8. Exploração **sem** package novo (dentro de `ori-game`)

Usar stack já existente (stdlib + raylib/ori_game + opcional sqlite/enet/raygui/imgui):

| Área | Depende de |
|------|------------|
| Input rebinding / actions | `game.input` + raylib |
| Save slots | `game.save` + `ori.fs` / sqlite |
| Camera 2D (follow, shake, bounds) | `game.camera` |
| UI layout 2D HUD | `game.draw` / raygui / imgui |
| A\* 2D grid | pure Ori + tilemap/LDtk |
| Cutscene runner | dialogue + scene |
| Net prediction helpers | pure Ori + enet |
| **Marching Cubes** | pure Ori (`game.marching_cubes`) + `game.draw3d` / mesh upload; ver §4 |

### 8.1 Notas de port — FreeType · HarfBuzz · Marching Cubes

| Item | Forma | Dependências | Superfície completa (0.1.0 / module) |
|------|--------|--------------|--------------------------------------|
| **FreeType** | Package `ori-freetype` + shim C | libfreetype (system; staged static/shared) | face + text metrics, kerning, glyph index, PGM, `face_ptr`; **4 tests** |
| **HarfBuzz** | Package `ori-harfbuzz` | **FreeType face** (`hb_ft_font_create_referenced`) | `shape` / `shape_dir`, cluster, advances, cursor positions; **JIT smoke** gate |
| **Marching Cubes** | `game.marching_cubes` pure Ori | `ori.list` + optional `game.marching_cubes_draw` | `isosurface`/`fill_box` + bounds/flatten/wire export + draw wires/solid |

**Status 2026-07-15:** as **3 etapas** (FT → HB → MC) estão **completas** em Linux (API + smoke/tests). Atlas FreeType→raylib e Phase OS ficam para depois.


---

## 9. Como atualizar este catálogo

1. Novo port **acordado** → mover da tabela de prioridade para §2 (Já traduzidos) com versão + link do repo.  
2. Mudança de prioridade → editar só a seção 3/4/5 e a data no topo.  
3. Não duplicar status de build/smoke aqui — isso fica em `eco-packages-status.md` e READMEs dos repos.

---

## 10. Resumo visual

```text
ALTA     (vazia — não reabrir ports)
MÉDIA    (vazia — miniaudio skipped)
BAIXA    OpenAL Soft · ozz · cute_c2 · steam/discord · Lua host
EVITAR   Yoga · cglm/HMM core · bgfx · ejson · 2º physics · flecs/EnTT default
FEITO-5  raylib · ori_game · imgui · raygui · box2d · jolt · rres · sqlite · enet
         · freetype · harfbuzz · MC (in ori-game)
U1–U15   stb · noise · miniz · lz4 · nfd · implot · imnodes · imguizmo
         · tracy · enkits · cgltf · fast_obj · physfs · clay · recast
         → plan pr-plan-eco-maturity-5.md (G1–G7)
LAYOUT   Documentos/Projetos/game-engine-full/ori-*
```
