# Editor Checklist v1

> Audience: maintainer, contributor
> Status: active
> Surface: internal
> Last updated: 2026-05-09

This checklist is the execution source for the Zenith editor work.

Use this with `editor-roadmap-v1.md`.

Legend:

- `[x]` done
- `[ ]` pending
- `[~]` partial or needs validation
- `[!]` blocked or risky

## Track A - Keter Micro

Path: `tools/keter-micro/`

### A0 - Existing MVP Stabilization

- [x] A0.01 - Create Rust crate for `keter-micro`.
- [x] A0.02 - Add Ratatui/Crossterm terminal shell.
- [x] A0.03 - Detect `zenith.ztproj`.
- [x] A0.04 - Detect standalone `.zt` files.
- [x] A0.05 - Detect plain folders.
- [x] A0.06 - Open initial Zenith files.
- [x] A0.07 - Add rope-backed text buffer.
- [x] A0.08 - Support edit, cursor movement, save, dirty state, close file, and tab switching.
- [x] A0.09 - Add compact tabs.
- [x] A0.10 - Add split preview with terminal-size guard.
- [x] A0.11 - Run `zt check`, `zt run`, `zt build`, `zt test`, and `zt fmt`.
- [x] A0.12 - Parse diagnostics from command output.
- [x] A0.13 - Add Problems panel.
- [x] A0.14 - Add Output panel.
- [x] A0.15 - Add command palette.
- [x] A0.16 - Add settings menu with grouped rows.
- [x] A0.17 - Persist panel/theme settings to TOML.
- [x] A0.18 - Add `config.toml` template.
- [x] A0.19 - Add `keybinds.toml` template.
- [x] A0.20 - Add theme TOML directory.
- [x] A0.21 - Add language TOML directory.
- [x] A0.22 - Add mouse capture option.
- [x] A0.23 - Add footer command bar separate from command palette.
- [x] A0.24 - Replace broken `tui-menu` path with owned dropdown menu.
- [~] A0.25 - Validate menu, mouse, tabs, and double-click in real Windows Terminal.
- [x] A0.26 - Remove stale warnings and unused code.
- [x] A0.27 - Remove temporary patch/fix scripts from `tools/keter-micro/`.
- [x] A0.28 - Clean generated `target/` when no Windows process is locking `keter-micro.exe`.
- [x] A0.29 - Detect ancestor `zenith.ztproj` when launched from a nested folder.
- [x] A0.30 - Resolve `zt`, `zpm`, and `zt-lsp` from the detected project root.

### A1 - UI and Accessibility Baseline

- [x] A1.01 - Header shows app name and project context.
- [x] A1.02 - Header shows active file clearly.
- [x] A1.03 - Header works correctly in split mode.
- [x] A1.04 - Footer opens command bar.
- [x] A1.05 - Footer command bar supports core slash commands.
- [x] A1.06 - Footer command bar supports command suggestions by mouse.
- [x] A1.07 - Footer command bar supports `/help`.
- [x] A1.08 - Footer command bar supports `/keybinds` and `/shortcuts`.
- [x] A1.09 - Footer command bar supports `/settings`.
- [x] A1.10 - Footer command bar supports `/textmode`.
- [x] A1.11 - Footer command bar supports `/files`, `/problems`, `/output`, and `/git`.
- [x] A1.12 - Settings menu uses grouped topics.
- [x] A1.13 - Settings menu uses explicit panel state and mode columns.
- [x] A1.14 - Settings changes persist across launch.
- [ ] A1.15 - Map all user-facing UI strings through language TOML.
- [~] A1.16 - Keep Portuguese/English language packs readable and complete.
- [~] A1.17 - Theme TOML controls the main UI colors.
- [ ] A1.18 - Theme TOML controls all selection, severity, and panel colors.
- [ ] A1.19 - Validate high contrast theme manually.
- [x] A1.20 - Add short accessibility smoke checklist to README.
- [x] A1.21 - Make the main workspace render as an editor surface, not a session report.
- [~] A1.22 - Do a manual visual pass against Micro/LazyVim references.
- [x] A1.23 - Remove heavy border from the main editor surface.
- [x] A1.24 - Replace long footer help text with compact statusline.
- [x] A1.25 - Hide empty Problems and Output panels in minimal chrome.
- [x] A1.26 - Hide noisy dotfolders, caches, and build output in the explorer by default.
- [x] A1.27 - Move the top menu immediately after the `Keter Micro` header label.
- [x] A1.28 - Render a visible cursor cell inside the active editor line.
- [x] A1.29 - Use the real editor body area for mouse cursor placement.

### A2 - File and Project Workflow

- [x] A2.01 - Explorer can open project files by keyboard.
- [~] A2.02 - Explorer supports mouse selection.
- [~] A2.03 - Explorer supports double-click open.
- [x] A2.04 - Command palette can filter files.
- [x] A2.05 - Standalone `.zt` mode exists.
- [x] A2.06 - Standalone `.zt` can be converted into a project.
- [x] A2.07 - Add open file prompt.
- [x] A2.08 - Add open folder prompt.
- [x] A2.09 - Add dynamic path suggestions while typing.
- [x] A2.10 - Add recent files.
- [x] A2.11 - Add recent folders.
- [x] A2.12 - Add clear errors for invalid paths.
- [~] A2.13 - Add large workspace performance smoke test.
- [x] A2.14 - Wire open folder through File menu, command bar, and command palette.
- [x] A2.15 - Rebuild file explorer and LSP session after opening a new folder.

### A3 - Language Intelligence

- [x] A3.01 - Start `zt-lsp` in project mode.
- [x] A3.02 - Sync active document with LSP.
- [x] A3.03 - Add completion action.
- [x] A3.04 - Add hover action.
- [x] A3.05 - Add go to definition action.
- [x] A3.06 - Add references action.
- [x] A3.07 - Add rename availability check.
- [ ] A3.08 - Add rename prompt.
- [ ] A3.09 - Apply rename edits safely.
- [x] A3.10 - Add document symbols if `zt-lsp` exposes them.
- [x] A3.11 - Add workspace symbols if `zt-lsp` exposes them.
- [x] A3.12 - Add signature help if `zt-lsp` exposes it.
- [x] A3.13 - Keep single-file fallback through `zt check`.
- [x] A3.14 - Improve single-file LSP fallback messaging.
- [x] A3.15 - Keep lexical syntax highlighting fallback.
- [ ] A3.16 - Add tree-sitter when Zenith grammar is ready.
- [x] A3.17 - Show LSP completion as an editor popup.
- [x] A3.18 - Add automatic completion trigger while typing.

### A4 - Panels and Workflow Tools

- [x] A4.01 - Files panel supports fixed mode.
- [x] A4.02 - Files panel supports floating mode.
- [x] A4.03 - Problems panel supports fixed mode.
- [x] A4.04 - Problems panel supports floating mode.
- [x] A4.05 - Output panel supports fixed mode.
- [x] A4.06 - Output panel supports floating mode.
- [x] A4.07 - Mouse scroll works in editor and panels.
- [x] A4.08 - Output panel supports clear output.
- [x] A4.09 - Output panel supports command history.
- [x] A4.10 - Output panel supports copy selected text.
- [ ] A4.11 - Problems panel supports filtering.
- [x] A4.12 - Problems panel supports grouping by severity.
- [x] A4.13 - Git status read-only summary exists.
- [x] A4.14 - Git diff read-only summary exists.
- [x] A4.15 - Add navigable Git panel.
- [ ] A4.16 - Add stage/unstage later.
- [ ] A4.17 - Add commit later.
- [ ] A4.18 - Add real PTY terminal panel.

### A5 - Local Plugins and Extension Model

- [x] A5.01 - Discover local plugin manifests.
- [x] A5.02 - Show plugin manifest summary.
- [x] A5.03 - Execute local plugin commands safely.
- [x] A5.04 - Load snippets from plugin manifests.
- [~] A5.05 - Load themes from plugin manifests.
- [~] A5.06 - Load language packs from plugin manifests.
- [x] A5.07 - Show plugin permissions clearly.
- [x] A5.08 - Add plugin validation errors.
- [x] A5.09 - Document that marketplace and stable ABI are out of scope.

### A6 - Packaging, Dogfood, and Quality

- [x] A6.01 - Add README with current run commands.
- [x] A6.02 - Add `--probe`.
- [x] A6.03 - Add `--lsp-probe`.
- [x] A6.04 - Add `--exec`.
- [x] A6.05 - Unit tests pass.
- [x] A6.06 - Clean current Rust warnings.
- [x] A6.07 - Add integration test plan.
- [x] A6.08 - Add manual Windows Terminal smoke checklist.
- [x] A6.09 - Run Windows binary smoke test outside source tree.
- [ ] A6.10 - Run Linux smoke test.
- [x] A6.11 - Validate first-run config directory creation.
- [x] A6.12 - Add short troubleshooting page.

## Track B - Zenith IDE Pragmatico

Path: `tools/zenith-ide/`

### B0 - Shared Core Extraction

- [ ] B0.01 - Define shared crate/module boundary.
- [ ] B0.02 - Extract project discovery from Keter Micro.
- [ ] B0.03 - Extract command execution.
- [ ] B0.04 - Extract diagnostics normalization.
- [ ] B0.05 - Extract config schema.
- [ ] B0.06 - Extract theme schema.
- [ ] B0.07 - Extract language pack schema.
- [ ] B0.08 - Keep Keter Micro compiling after extraction.

### B1 - Desktop Shell

- [x] B1.01 - Create `tools/zenith-ide/`.
- [x] B1.02 - Add Tauri app skeleton.
- [x] B1.03 - Add Rust backend command bridge.
- [x] B1.04 - Add frontend build setup.
- [x] B1.05 - Add CodeMirror 6 editor surface.
- [x] B1.06 - Add open file.
- [x] B1.07 - Add open folder.
- [x] B1.08 - Add save file.
- [x] B1.09 - Add tabs.
- [~] B1.10 - Add open editors list as optional side view.

### B2 - Zenith Language Integration

- [x] B2.01 - Add Zenith syntax highlighting in CodeMirror 6.
- [~] B2.02 - Connect diagnostics.
- [x] B2.03 - Connect `zt check`.
- [x] B2.04 - Connect `zt run`, `zt build`, `zt test`, and `zt fmt`.
- [ ] B2.05 - Connect `zt-lsp`.
- [~] B2.06 - Add completion.
- [ ] B2.07 - Add hover.
- [ ] B2.08 - Add go to definition.
- [ ] B2.09 - Add references.
- [ ] B2.10 - Add rename when backend supports safe edits.

### B3 - IDE UI and Accessibility

- [x] B3.01 - Add project explorer.
- [x] B3.02 - Add Problems panel.
- [x] B3.03 - Add Output panel.
- [x] B3.04 - Add command palette.
- [~] B3.05 - Add settings UI.
- [x] B3.06 - Add theme selector.
- [x] B3.07 - Add light, dark, and high contrast themes.
- [ ] B3.08 - Add keyboard shortcut editor or config.
- [x] B3.09 - Add reduced distraction mode.
- [~] B3.10 - Validate keyboard-only core workflow.

### B4 - Practical IDE Workflow

- [ ] B4.01 - Add integrated terminal.
- [ ] B4.02 - Add Git status panel.
- [ ] B4.03 - Add Git diff view.
- [ ] B4.04 - Add search in files.
- [~] B4.05 - Add snippets.
- [ ] B4.06 - Add local plugin discovery.
- [ ] B4.07 - Add local plugin commands after Track A proves model.
- [~] B4.08 - Add packaging flow.
- [~] B4.09 - Add Windows smoke test.
- [ ] B4.10 - Add Linux smoke test if Tauri target is supported.

## Shared Validation Gates

- [x] V0.01 - Keter Micro tests pass.
- [x] V0.02 - Keter Micro probe passes.
- [x] V0.03 - Keter Micro LSP probe passes when `zt-lsp` is available.
- [ ] V0.04 - Keter Micro manual TUI smoke test passes.
- [x] V0.05 - IDE build passes after Track B starts.
- [ ] V0.06 - IDE manual desktop smoke test passes after Track B starts.

## Known Current Gaps

- [ ] G0.01 - `main.rs` is still too large.
- [x] G0.02 - Temporary patch scripts removed from `tools/keter-micro/`.
- [x] G0.03 - Current Rust checks are warning-clean.
- [ ] G0.04 - UI text is not fully internationalized.
- [ ] G0.05 - Tree-sitter is not integrated.
- [ ] G0.06 - Terminal/PTTY is not integrated.
- [x] G0.07 - Track B scaffold exists.
- [ ] G0.08 - Visual quality still needs a real terminal pass against Micro/LazyVim references.
