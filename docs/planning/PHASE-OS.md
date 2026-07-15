# Phase OS — multi-OS staging for ECO packages

> **Status:** **scaffolding** (2026-07-15) — **non-blocking**  
> Linux product surface is complete for core + medium ports.  
> Multi-OS validation is **last**: do **not** block lib work or CI on Win/mac green.  
> Core packages (game stack) have real/stub Windows scripts from 2026-07-14;  
> medium M1–M6 packages have **deferred stubs only** (echo + exit 0).

## Goal

| Triple | Priority | Status |
|--------|----------|--------|
| `x86_64-unknown-linux-gnu` | done | **Linux** (core 5 + medium 0.1.0) |
| `x86_64-pc-windows-msvc` | **P0** | core scripts ready; medium = **deferred stubs** |
| `x86_64-apple-darwin` / `aarch64-apple-darwin` | P1 | not started |

## Prerequisites (Windows)

1. **Visual Studio 2022 Build Tools** (workload *Desktop development with C++*)
2. **cmake** on PATH (Box2D / optional real Jolt)
3. **ori.exe** ≥ 0.3.x with staged `runtime/x86_64-pc-windows-msvc/`
4. Shell: **x64 Native Tools Command Prompt** *or* scripts auto-load via `vswhere`

```powershell
$env:ORI_BIN = "C:\path\to\ori.exe"
$env:ORI_USE_SYSTEM_LINKER = "1"
```

## Naming (MSVC)

Package `native_libs = ["foo"]` → link looks for **`foo.lib`** under `lib/x86_64-pc-windows-msvc/`.

| Linux | Windows |
|-------|---------|
| `libori_raylib_shim.a` | `ori_raylib_shim.lib` |
| `libraylib.a` | `raylib.lib` |
| `libori_box2d_shim.a` | `ori_box2d_shim.lib` |
| `libbox2d.a` | `box2d.lib` |

## Per-package scripts (core — real/stub MSVC)

| Package | Build | Smoke |
|---------|-------|-------|
| **ori-game** | `tools/setup_raylib_windows.ps1` [`-Stub`] | `tools/smoke_windows.ps1` [`-Stub`] |
| **ori-box2d** | `tools/build_windows.ps1` | `tools/smoke_windows.ps1` |
| **ori-jolt** | `tools/build_windows.ps1` (stub default) | `tools/smoke_windows.ps1` |
| **ori-sqlite** | `tools/build_windows.ps1` | `tools/smoke_windows.ps1` |
| **ori-rres** | `tools/build_windows.ps1` | `tools/smoke_windows.ps1` |
| **ori-imgui** | `tools/build_windows.ps1` (stub host) | `tools/smoke_windows.ps1` |
| **ori-raygui** | `tools/build_windows.ps1` | `tools/smoke_windows.ps1` |
| **ori-enet** | `tools/build_windows.ps1` | `tools/smoke_windows.ps1` |

## Medium packages (M1–M6) — Phase OS **deferred**

Linux **0.1.0** is the product surface. Windows stubs exist so the gap is explicit;
they do **not** produce MSVC libs and are **not** required for CI.

| Package | `tools/build_windows.ps1` | Status |
|---------|---------------------------|--------|
| **ori-cgltf** | deferred stub (echo + exit 0) | Linux-only 0.1.0 |
| **ori-fast-obj** | deferred stub | Linux-only 0.1.0 |
| **ori-physfs** | deferred stub | Linux-only 0.1.0 |
| **ori-clay** | deferred stub | Linux-only 0.1.0 |
| **ori-lz4** | deferred stub | Linux-only 0.1.0 |
| **ori-recast** | deferred stub | Linux-only 0.1.0 |

Each package README has a short **Phase OS** section pointing here.

### Umbrella

```powershell
cd C:\path\to\ori-game
.\tools\smoke_eco_windows.ps1 -Stub   # recommended first run
# later: without -Stub if real raylib.lib is staged
```

## Checklist (execute on Windows)

| # | Package | Build | Smoke | Notes |
|---|---------|-------|-------|-------|
| 1 | ori-game | [ ] | [ ] | Start with `-Stub` |
| 2 | ori-box2d | [ ] | [ ] | needs cmake |
| 3 | ori-sqlite | [ ] | [ ] | amalgamation under vendor/ |
| 4 | ori-rres | [ ] | [ ] | |
| 5 | ori-jolt | [ ] | [ ] | stub ABI |
| 6 | ori-imgui | [ ] | [ ] | stub until GLFW full |
| 7 | ori-raygui | [ ] | [ ] | real raylib for GUI demos |

## ori-game details

- **Stub:** `tools/setup_raylib_windows.ps1 -Stub` compiles `tools/raylib_stub.c` → `ori_raylib_shim.lib` + tiny `raylib.lib`.
- **Real:** place `raylib.h` + large `raylib.lib`, re-run without `-Stub` to compile `native/ori_raylib_shim.c`.

## Acceptance for “5 (Linux+Win)”

- [ ] All seven package smokes green on Windows MSVC (stub OK for graphics packages)
- [ ] Matrix Table A: maturity **5 (Linux+Win)**
- [ ] Optional: integration demos staged for Win triples

## Linux reference (already green)

```bash
export ORI_BIN=ori ORI_USE_SYSTEM_LINKER=1
~/Documentos/Projetos/ori-game/tools/smoke_eco_linux.sh
```
