# blog_app

Scaffolded **ori-web-app** example: home, posts, notes + tasks REST, optional **SQLite sessions**.

## Run

```bash
# optional SQLite sessions (default ORI_WEB_SESSION=sqlite)
ln -sfn "$HOME/Documentos/Projetos/ori-sqlite" ../ori-sqlite
( cd ../ori-sqlite && ./tools/build_linux.sh )

cd packages/blog_app
ORI_USE_AOT=1 ori run main.orl
# http://127.0.0.1:3000/
```

| Env | Meaning |
|-----|---------|
| `ORI_WEB_SESSION` | `sqlite` (default) · `file` · `memory` |

DB file: `var/sessions.db` when using sqlite.
