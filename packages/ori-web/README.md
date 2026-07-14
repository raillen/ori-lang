# web (`ori-web`)

Minimal HTTP **Library** layer for Ori (phases **A + B + C**, plus SEC8 helpers).

**Status:** web stack **feature freeze v1** — see [`../FREEZE-WEB.md`](../FREEZE-WEB.md).

- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) (D14–D20, §5.5)
- Phase B/C/D: [`docs/phase-b.md`](docs/phase-b.md) · [`phase-c.md`](docs/phase-c.md) · [`phase-d.md`](docs/phase-d.md)
- Stack index: [`../README.md`](../README.md)

## Features

| Area | API |
|------|-----|
| Types | `Request`, `Response`, `App`, `Handler`, `ActionResult`, `Middleware` |
| Responses | `text`, `html`, `json`, `json_string_map`, `redirect`, `not_found`, `forbidden`, `bad_request`, `payload_too_large`, `too_many_requests` |
| JSON helpers | flat: `json_string_map` · `parse_json_object` · nested: `json_string`/`json_int`/`json_object`/`json_array`/`json_pair` · `parse_json_nested` (dotted paths; no `ori.json` import) |
| Router | `get` / `post` / `put` / `patch` / `delete` + path params `:id` |
| Static | `static(app, url_prefix, dir)` with `..` path jail |
| Middleware | `use_middleware` · `clear_middleware` · catalog `mw_set_header` / `mw_timing` / `mw_request_id` ([docs/middleware.md](docs/middleware.md)) |
| Session | cookie `ori_sid` (HttpOnly, SameSite=Lax); `session_get` / `session_set` / flash / `session_regenerate` |
| Session store (B3) | `use_memory_sessions` · `use_file_sessions` · `use_kv_sessions` · **`use_custom_sessions`** · `clear_session_cache` · `purge_expired_sessions` · `session_backend` |
| Read timeout (B7) | `set_read_timeout(app, ms)` — socket deadline + full Content-Length body read in `serve` |
| Keep-alive | `set_keep_alive(app, enabled, max)` |
| Upload (C8) | `parse_multipart` · `form_file` / `form_part_value` · `save_upload(dir, part, max_bytes, "txt,png")` |
| Timeouts (A9) | `set_session_timeouts(app, idle_ms, absolute_ms)` (default 1h / 24h) |
| CSRF | form `csrf_token` or header `X-CSRF-Token` on mutations |
| Rate limit (B4) | `set_rate_limit(app, per_minute)` · `client_key` / `set_trust_proxy` |
| Headers (B6) | nosniff, frame, referrer, permissions-policy · optional `set_csp` |
| Secret (A7) | `ORI_WEB_SECRET` when `ORI_WEB_ENV=production` · `require_secret` |
| Dispatch | **`dispatch`** (not `handle` — reserved keyword) |
| Test hooks | `make_request` · `request_set_header` · `to_http` (wire cookies + security headers) |
| Serve | `serve(host, port, app)` |
| Forms / htmx | `form_body`, `is_htmx` |
| Auth helper | `require_session_key(key, next)` |

## Demos & tests

| Package | Port | Focus |
|---------|------|--------|
| `packages/ori-web/examples/hello_server` | 3456 | minimal |
| `packages/ori-web/examples/sec8_tests` | — | SEC8 smoke (XSS body, CSRF, path jail, cookies, JSON, middleware, kv sessions) — `ori run main.orl` |
| `packages/ori-web-demo` | 3457 | HTML-first notes + htmx |
| `packages/ori-web-demo-api` | 3458 | JSON API + CSRF header |
| `packages/ori-web-demo-auth` | 3459 | login + argon2 + **TOTP 2FA** + SQLite sessions |
| `packages/ori-web-demo-upload` | 3460 | C8 multipart upload |

## Use (path dependency)

```toml
[dependencies]
web = { path = "../ori-web", version = "0.1.0" }
```

```ori
var a: web.App = web.app()
a = web.set_rate_limit(a, 60)
a = web.set_cookie_secure(a, false)  -- true behind HTTPS
web.get(a, "/", home)
match web.serve("127.0.0.1", 3456, a)
case ok(_):
case err(msg):
    io.println(msg)
end
```

## Phase C

See [`docs/phase-c.md`](docs/phase-c.md): lockout, audit, re-auth, CSRF rotate, `__Host-` cookie.

## Middleware sketch

```ori
mw_log(next: web.Handler) -> web.Handler
    return (req: web.Request) => next(req)
end

a = web.use_middleware(a, mw_log)
```

Middleware runs after static/CSRF/rate-limit, around the matched route handler only.
Last registered is outermost.

## Session backends

```ori
web.use_memory_sessions()                 -- default, process-local
web.use_file_sessions("tmp/sessions")     -- one file per sid
web.use_kv_sessions("tmp/sessions.kv")    -- single flat file (multi-restart)
-- SQLite (package ori-web-session-sqlite + ori-sqlite):
-- web_session_sqlite.use_sqlite_sessions("var/sessions.db")
-- Custom: web.use_custom_sessions(load, save, remove)
```

## Upload sketch (C8)

```ori
match web.parse_multipart(req)
case ok(parts):
    match web.form_file(parts, "file")
    case some(f):
        match web.save_upload("var/uploads", f, 1048576, "png,jpg,pdf")
        case ok(path):
            -- store path outside webroot
        case err(e):
            return ok(web.bad_request(e))
        end
    case none:
    end
case err(e):
    return ok(web.bad_request(e))
end
```

## Templates (W5)

HTML rendering lives in **`web_app`** (`render` / `page_data` / `csrf_field`) so the
Library layer stays free of a templates package cycle. Path-depend both packages
in apps.

## Keep-alive

```ori
a = web.set_keep_alive(a, true, 32)  -- default on; max requests per connection
```

HTTP/1.1 reuse in `serve`. Prefer proxy idle timeouts in production.

## 2FA

Optional package **`ori-web-auth`** (TOTP via `ori.crypto.totp_*`).

## Not yet

Redis session driver, in-process TLS server (edge proxy recommended — phase D).
Password hashing / TOTP: **`ori.crypto`**. SQLite sessions:
`packages/ori-web-session-sqlite` + symlink `packages/ori-sqlite` (see
`packages/ori-sqlite.README.md`).
