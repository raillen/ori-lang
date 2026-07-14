# Ori packages — web stack

**Status:** feature-complete for HTML-first v1 · **feature freeze** after S8 + nested JSON  
(path-local packages under this directory; not yet a published registry product.)

Outside the language FREEZE-1 *compiler* core. Apps depend via `ori.proj` path deps.

## Libraries (use these)

| Package | Role | Depends on |
|---------|------|------------|
| **ori-templates** | HTML templates `@{ }` · S8 trim `-` | — |
| **ori-web** | HTTP, session, CSRF, middleware, JSON, upload, B7 | runtime `ori.net` / `ori.crypto` |
| **ori-web-app** | App layer + generators (`bin/new`, scaffold) | web + templates |
| **ori-web-auth** | Optional TOTP 2FA | web + `ori.crypto` |
| **ori-web-session-sqlite** | Optional SQLite sessions | web + **ori-sqlite** (symlink) |

## Demos / examples

| Package | Port | Focus |
|---------|------|--------|
| `ori-web/examples/hello_server` | 3456 | minimal |
| `ori-web/examples/sec8_tests` | — | security smoke (`tools/qa/web_sec8.sh`) |
| `ori-web-demo` | 3457 | HTML-first notes + htmx |
| `ori-web-demo-api` | 3458 | JSON API + CSRF header |
| `ori-web-demo-auth` | 3459 | login + argon2 + TOTP + SQLite sessions |
| `ori-web-demo-upload` | 3460 | C8 multipart upload |
| `blog_app` | 3000 | scaffolded REST + SQLite sessions |

## Minimal app (Library only)

```toml
# ori.proj
[dependencies]
web = { path = "../ori-web", version = "0.1.0" }
templates = { path = "../ori-templates", version = "0.1.0" }
```

## Full App (generators + conventions)

```toml
[dependencies]
web = { path = "../ori-web", version = "0.1.0" }
templates = { path = "../ori-templates", version = "0.1.0" }
web_app = { path = "../ori-web-app", version = "0.1.0" }
```

```bash
./ori-web-app/bin/new myapp ./myapp
cd myapp
../ori-web-app/bin/generate-scaffold notes
ori get . && ori run main.orl
```

## Optional pieces

| Need | Package / env |
|------|----------------|
| TOTP 2FA | `web_auth` path dep |
| SQLite sessions | `web_session_sqlite` + symlink [`ori-sqlite.README.md`](ori-sqlite.README.md) |
| Session backend | `ORI_WEB_SESSION=sqlite\|file\|memory` (blog / demo-auth) |
| Runtime crypto | staged `ori-runtime` with argon2 + totp symbols |

```bash
# from packages/
ln -sfn "$HOME/Documentos/Projetos/ori-sqlite" ori-sqlite
( cd ori-sqlite && ./tools/build_linux.sh )
```

## QA (framework)

```bash
./tools/qa/web_sec8.sh                 # always
./tools/qa/web_auth_smoke.sh           # TOTP
./tools/qa/web_session_sqlite_smoke.sh # AOT + sqlite native
```

Hooked into `tools/qa/daily_full.sh` (S6b–S6d; sqlite may soft-skip).

## Docs map

| Doc | Content |
|-----|---------|
| [`docs/planning/web-templates-discussion-roadmap.md`](../docs/planning/web-templates-discussion-roadmap.md) | Design decisions |
| [`ori-web/docs/phase-b.md`](ori-web/docs/phase-b.md) … [`phase-d.md`](ori-web/docs/phase-d.md) | Security / ops |
| [`ori-web/docs/middleware.md`](ori-web/docs/middleware.md) | Middleware onion |
| [`FREEZE-WEB.md`](FREEZE-WEB.md) | What is frozen vs open |

## Packaging note (distribution)

You do **not** need one giant binary of all libs for every app.

- **Core site:** `ori-templates` + `ori-web` (+ runtime Ori with crypto if auth).  
- **Rails-like app:** add `ori-web-app`.  
- **2FA:** add `ori-web-auth`.  
- **SQLite sessions:** add `ori-web-session-sqlite` + native `ori-sqlite`.  

For an **official stack tarball/docs**, ship the **library packages together** in the monorepo (or one archive) and document path deps — demos are examples, not required installs. Registry publish can be **per package** later; a meta package is optional convenience, not a hard requirement.

## Feature freeze (web v1)

**In freeze:** surface of templates + web + web-app + auth + sqlite adapter + demos above.  
**Out of freeze (backlog C):** Redis, ORM, magic routes, WebAuthn/SMS, template `match` (S10), multi-OS dist of the *language* (separate track).

Bugfixes and docs polish are allowed; new feature surface needs a conscious unfreeze.
