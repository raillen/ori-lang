# packages/ori-sqlite (local link)

Apps under `packages/` depend on `sqlite = { path = "../ori-sqlite", … }`.

Create a **symlink** (not committed) to your [ori-sqlite](https://github.com/raillen/ori-sqlite) checkout:

```bash
# from packages/
ln -sfn "$HOME/Documentos/Projetos/ori-sqlite" ori-sqlite
# or any clone:
# ln -sfn /path/to/ori-sqlite ori-sqlite

cd ori-sqlite && ./tools/build_linux.sh
```

Override discovery for QA:

```bash
export ORI_SQLITE_ROOT=/path/to/ori-sqlite
./tools/qa/web_session_sqlite_smoke.sh
```

`packages/ori-sqlite` is gitignored.
