# web_session_sqlite (`ori-web-session-sqlite`)

**B3** session store adapter: SQLite table `web_sessions` for `ori-web`.

Uses `web.use_custom_sessions` so the Library layer stays free of native deps.

## Depend

```toml
[dependencies]
web = { path = "../ori-web", version = "0.1.0" }
web_session_sqlite = { path = "../ori-web-session-sqlite", version = "0.1.0" }
sqlite = { path = "../ori-sqlite", version = "0.3.0" }
```

Link and build **ori-sqlite** first (see [`../ori-sqlite.README.md`](../ori-sqlite.README.md)):

```bash
ln -sfn "$HOME/Documentos/Projetos/ori-sqlite" ../ori-sqlite
( cd ../ori-sqlite && ./tools/build_linux.sh )
```

## Use

```ori
import web = web
import web_session_sqlite = ssql

match ssql.use_sqlite_sessions("var/sessions.db")
case ok(_):
case err(e):
    io.println(e)
end

-- normal web.session_get / session_set / CSRF / flash
```

## API

| Function | Role |
|----------|------|
| `use_sqlite_sessions(path)` | Open DB, create table, register custom backend |
| `close()` | Close DB + revert to memory sessions |
| `is_open` / `db_path` | Status |
| `session_count` / `clear_all` | Ops / tests |

## Smoke

```bash
./tools/qa/web_session_sqlite_smoke.sh
# or:
cd packages/ori-web-session-sqlite/examples/smoke && ori run main.orl
```

## Schema

```sql
CREATE TABLE web_sessions (
  sid TEXT PRIMARY KEY NOT NULL,
  blob TEXT NOT NULL,
  updated_ms INTEGER NOT NULL DEFAULT 0
);
```
