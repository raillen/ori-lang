# Editor Roadmap v1

> Audience: maintainer, contributor
> Status: active
> Surface: internal
> Last updated: 2026-05-09

This roadmap is the active plan for the Zenith editor/tooling work.

The editor project has two tracks:

- **Track A - Keter Micro**: terminal editor focused on speed, keyboard flow, low visual noise, and direct Zenith workflow.
- **Track B - Zenith IDE Pragmatico**: desktop IDE using Tauri, CodeMirror 6, and a Rust backend.

The goal is not VS Code parity. The goal is a readable, accessible, useful editor
for Zenith projects and standalone `.zt` files.

## Product Principles

- Keep the interface minimal and explicit.
- Prefer short labels and grouped menus.
- Keep keyboard navigation complete.
- Support mouse where terminal or platform allows it.
- Avoid hidden behavior when a visible command or menu is clearer.
- Persist user choices in readable TOML files.
- Keep Track A and Track B sharing project discovery, commands, diagnostics, LSP, settings, and language assets.

## Shared Scope

Both tracks should eventually share these services:

| Service | Purpose | Status |
| --- | --- | --- |
| `ProjectService` | Find `zenith.ztproj`, standalone `.zt` files, roots, tools, and manifests. | Active in Track A |
| `CommandService` | Run `zt check`, `zt run`, `zt build`, `zt test`, and `zt fmt`. | Active in Track A; initial bridge in Track B |
| `DiagnosticsService` | Normalize CLI/LSP diagnostics into Problems. | Active in Track A; command-output fallback in Track B |
| `LspService` | Start and communicate with `zt-lsp`. | Active in Track A |
| `SettingsService` | Load and persist config, keybinds, themes, languages, and accessibility options. | Partial in Track A |
| `LanguageAssets` | Highlight rules, snippets, language labels, and text mode mappings. | Partial in Track A |
| `TerminalService` | PTY and terminal integration. | Not started |
| `PluginService` | Local plugin discovery and safe execution. | Local manifest commands/snippets in Track A |

## Track A - Keter Micro

Stack:

- Rust
- Ratatui
- Crossterm
- ropey
- `zt-lsp`
- TOML config, keybinds, themes, and language packs
- tree-sitter later, after a stable Zenith grammar is available

Current state:

- Executable MVP exists in `tools/keter-micro/`.
- Unit tests pass.
- Probe mode passes.
- Probe mode also works when launched from `tools/keter-micro/`; the editor now discovers the ancestor Zenith project and resolves root-level `zt`, `zpm`, and `zt-lsp`.
- Header now shows app, project context, active file, split state, and compact LSP status.
- Footer command bar is separate from the full command palette and supports the core slash commands.
- Main workspace now prioritizes the active editor buffer instead of showing a long session/config report.
- Main editor chrome is now minimal: no outer editor border, compact tabs, short statusline, and empty Problems/Output panels hidden.
- Explorer hides dotfolders, caches, build outputs, and generated folders by default.
- LSP completion is visible as an editor popup and can insert the selected item.
- File/folder opening is available through the top menu, footer command bar, and command palette.
- Recent files and folders persist in `config.toml`.
- Automatic LSP completion, signature help, document symbols, and workspace symbols are wired.
- Output supports clear, selected-line copy, and command history.
- Local plugin manifests support safe `zt` command execution and snippet insertion.
- First run creates user config, keybind, theme, and language TOML files.
- Temporary patch/fix scripts have been removed from `tools/keter-micro/`.
- UI needs terminal-level validation and cleanup.
- The code has useful modules, but `main.rs` still owns too much rendering and orchestration.
- Current Rust checks are clean; keep warnings from returning as new work lands.

### A0 - Stabilize Existing MVP

Goal: make the current TUI reliable enough for daily dogfood.

Scope:

- Validate menu dropdown behavior in a real terminal.
- Validate mouse click, scroll, tab selection, and file double click.
- Keep command palette separate from footer command bar.
- Keep settings persistence stable.
- Remove temporary patch scripts and stale generated artifacts.
- Reduce warnings that hide real problems.

Exit criteria:

- `cargo test --manifest-path tools/keter-micro/Cargo.toml` passes.
- `cargo run --manifest-path tools/keter-micro/Cargo.toml -- --probe .` passes.
- Manual smoke test confirms menu, footer command bar, tabs, files, Problems, and Output.

### A1 - Finish UI and Accessibility Baseline

Goal: make Keter Micro pleasant and predictable.

Scope:

- Header starts with app name, then top menu, then short project/file/LSP context.
- Main editor surface shows the active file first, with syntax highlighting, visible cursor cell, and compact status.
- Footer command bar supports `/help`, `/keybinds`, `/settings`, `/textmode`, `/files`, `/problems`, `/output`, and `/git`.
- Settings menu keeps grouped columns with explicit state and mode.
- All visible text is mapped through language TOML where practical.
- Theme TOML files control foreground, background, accent, muted, severity, and selection colors.
- High contrast theme is verified in real terminal.

Exit criteria:

- UI text has a clear mapping path.
- Theme files can be edited without recompiling.
- Footer command bar is usable without opening the full command palette.

### A2 - File and Project Workflow

Goal: make opening workspaces and files explicit.

Scope:

- Add open file prompt.
- Add open folder prompt through menu, command bar, and palette.
- Support path typing with dynamic path suggestions.
- Support recent folders and recent files.
- Support standalone `.zt` flow and project conversion flow.
- Improve workspace file search for large folders.

Exit criteria:

- User can open a file or folder without restarting the editor.
- Opening a folder rebuilds the project context, file explorer, and LSP session.
- Standalone `.zt` files remain first-class.
- Path errors are short and actionable.

### A3 - Language Intelligence

Goal: make `zt-lsp` the main language intelligence source.

Scope:

- Keep completion, hover, definition, references, and diagnostics wired.
- Add explicit rename prompt and safe edit application.
- Add document symbols and workspace symbols if `zt-lsp` exposes them.
- Add signature help UI if `zt-lsp` exposes it.
- Improve single-file fallback messaging.
- Add tree-sitter highlight when Zenith grammar is available.

Exit criteria:

- Rename either works safely or is clearly unavailable.
- LSP errors do not break editing.
- Syntax highlighting is more accurate than the current lexical fallback.

### A4 - Panels and Workflow Tools

Goal: make the editor useful for daily development.

Scope:

- Problems panel: scrolling, selection, jump, grouping, and filter.
- Output panel: scrolling, command history, copy text, clear output.
- Git panel: status, diff, stage/unstage, and commit later.
- Logs panel: readable internal logs, not noisy by default.
- Terminal panel: PTY integration after editor basics are stable.

Exit criteria:

- Panels can be fixed or floating.
- Panel state persists.
- Panels are keyboard and mouse reachable.

### A5 - Plugin and Extension Local Model

Goal: support local extension points without promising marketplace or stable ABI.

Scope:

- Keep local manifest discovery.
- Add safe command execution from local plugin manifests.
- Add snippets from local manifests.
- Add language packs from local manifests.
- Add theme packs from local manifests.
- Keep permissions explicit.

Exit criteria:

- Local plugins can add visible commands and snippets.
- No public marketplace promise.
- No stable plugin ABI promise before the model is proven.

### A6 - Packaging and Dogfood

Goal: make Keter Micro easy to run outside the source tree.

Scope:

- Windows binary smoke test.
- Linux smoke test.
- Config directory creation on first run.
- Clear troubleshooting output.
- Minimal release notes.

Exit criteria:

- A user can download or build Keter Micro and open a Zenith project.
- Common failure states show short messages.

## Track B - Zenith IDE Pragmatico

Stack:

- Rust backend
- Tauri desktop shell
- CodeMirror 6 editor surface
- `zt-lsp`
- Shared services from Track A where possible

Current state:

- Implemented as an initial runnable scaffold in `tools/zenith-ide/`.
- Uses Tauri 2, CodeMirror 6, and a Rust command bridge.
- Can open a folder by path, list `.zt` files, open files from the project list, save files, and keep opened files as tabs.
- Can run `zt check`, `zt run`, `zt build`, `zt test`, and `zt format` through the backend bridge.
- Shows command output and a simple Problems list parsed from compiler-style output.
- Has local Zenith syntax highlighting and local keyword/type completion fallback.
- Has dark, light, and high contrast theme selection persisted in browser storage.
- Has a reduced-distraction focus mode and a simple `Ctrl+P` command palette.
- Builds successfully with `npm run build`, `cargo check --manifest-path src-tauri/Cargo.toml`, and `npm run tauri build -- --no-bundle`.
- Does not yet use the Track A shared services; project and command logic are duplicated as a temporary scaffold.
- Does not yet connect to `zt-lsp`, so hover, definition, references, and real LSP completion remain pending.

### B0 - Shared Core Extraction

Goal: avoid duplicating Track A behavior.

Scope:

- Extract reusable project discovery.
- Extract command execution.
- Extract diagnostics normalization.
- Extract config schema.
- Extract language asset loading.

Exit criteria:

- IDE backend can use the same logic as Keter Micro.
- Track A keeps working after extraction.

### B1 - Desktop Shell

Goal: create the first runnable IDE shell.

Scope:

- Create `tools/zenith-ide/`.
- Add Tauri app skeleton.
- Add Rust backend commands.
- Add CodeMirror 6 editor view.
- Open file and folder from desktop UI.

Exit criteria:

- App builds and launches.
- User can open and edit a `.zt` file.
- User can save file.

### B2 - Language Integration

Goal: connect CodeMirror 6 to Zenith language services.

Scope:

- Add syntax highlighting.
- Connect completion, hover, diagnostics, definition, and references.
- Add Problems panel.
- Add Output panel.
- Add command palette.

Exit criteria:

- IDE can run `zt check`.
- Diagnostics appear in editor and Problems.
- LSP failures are visible and recoverable.

### B3 - Settings, Themes, and Accessibility

Goal: give the IDE the same philosophy as Micro.

Scope:

- Settings UI.
- Theme selection.
- High contrast theme.
- Keyboard shortcuts.
- Reduced distraction mode.
- Readability options allowed by GUI.

Exit criteria:

- Settings persist.
- Themes are inspectable and editable.
- Keyboard-only workflow works for core actions.

### B4 - Practical IDE Features

Goal: add the features expected from a pragmatic desktop editor.

Scope:

- Tabs and open editors.
- Explorer/sidebar.
- Integrated terminal.
- Git panel.
- Search in files.
- Snippets.
- Local plugin discovery.

Exit criteria:

- The IDE can handle common project work without switching tools constantly.

## Explicitly Out Of Initial Scope

- GPUI implementation.
- Public extension marketplace.
- Stable plugin ABI.
- Real-time collaboration.
- Full visual debugger.
- Custom terminal emulator.
- Full VS Code/Zed parity.

## Validation

Minimum validation per milestone:

- `cargo test --manifest-path tools/keter-micro/Cargo.toml`
- `cargo run --manifest-path tools/keter-micro/Cargo.toml -- --probe .`
- Manual TUI smoke test in Windows Terminal.
- When Track B starts: Tauri build and manual desktop smoke test.
- Current Track B automated gates:
  - `npm --prefix tools/zenith-ide run build`
  - `npm --prefix tools/zenith-ide audit --audit-level=moderate`
  - `cargo check --manifest-path tools/zenith-ide/src-tauri/Cargo.toml`
  - `npm --prefix tools/zenith-ide run tauri build -- --no-bundle`
