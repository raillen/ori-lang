# Zenith IDE Pragmatico

Zenith IDE is the desktop track for the Zenith editor work.

Stack:

- Tauri 2 desktop shell
- Rust backend command bridge
- CodeMirror 6 editor surface
- Local Zenith syntax and completion fallback

Current MVP:

- Open a project folder by path.
- Open `.zt` files from the project list.
- Open and save a single file by path.
- Keep multiple opened files as tabs.
- Run `zt check`, `zt run`, `zt build`, `zt test`, and `zt fmt`.
- Show command output and a simple Problems list.
- Switch dark, light, and high contrast themes.
- Use a reduced-distraction focus mode.
- Open a simple command palette with `Ctrl+P`.

Run:

```powershell
cd tools/zenith-ide
npm install
npm run tauri dev
```

Build frontend only:

```powershell
npm run build
```

Check Rust backend:

```powershell
cargo check --manifest-path src-tauri/Cargo.toml
```

Out of MVP:

- Marketplace.
- Stable plugin ABI.
- Custom terminal emulator.
- Full visual debugger.
- Real-time collaboration.
