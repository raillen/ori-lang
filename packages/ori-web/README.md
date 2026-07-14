# web (`ori-web`)

Minimal HTTP **Library** layer for Ori (phase **A + B**).

- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) (D14–D20, §5.5)
- Phase B notes: [`docs/phase-b.md`](docs/phase-b.md)
- Learning course: [`docs/planning/web-framework-learning-course.md`](../../docs/planning/web-framework-learning-course.md)

## Features

| Area | API |
|------|-----|
| Types | `Request`, `Response`, `App`, `Handler`, `ActionResult` |
| Responses | `text`, `html`, `json`, `redirect`, `not_found`, `forbidden`, `bad_request`, `payload_too_large`, `too_many_requests` |
| Router | `get` / `post` / `put` / `patch` / `delete` + path params `:id` |
| Static | `static(app, url_prefix, dir)` with `..` path jail |
| Session | cookie `ori_sid` (HttpOnly, SameSite=Lax); `session_get` / `session_set` / flash / `session_regenerate` |
| Session store (B3) | `use_memory_sessions()` · `use_file_sessions(dir)` |
| Timeouts (A9) | `set_session_timeouts(app, idle_ms, absolute_ms)` (default 1h / 24h) |
| CSRF | form `csrf_token` or header `X-CSRF-Token` on mutations |
| Rate limit (B4) | `set_rate_limit(app, per_minute)` · `client_key` / `set_trust_proxy` |
| Headers (B6) | nosniff, frame, referrer, permissions-policy · optional `set_csp` |
| Secret (A7) | `ORI_WEB_SECRET` when `ORI_WEB_ENV=production` · `require_secret` |
| Dispatch | **`dispatch`** (not `handle` — reserved keyword) |
| Serve | `serve(host, port, app)` |
| Forms / htmx | `form_body`, `is_htmx` |
| Auth helper | `require_session_key(key, next)` |

## Demos

| Package | Port | Focus |
|---------|------|--------|
| `packages/ori-web/examples/hello_server` | 3456 | minimal |
| `packages/ori-web-demo` | 3457 | HTML-first notes + htmx |
| `packages/ori-web-demo-api` | 3458 | JSON API + CSRF header |
| `packages/ori-web-demo-auth` | 3459 | login + regenerate + file sessions |

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

## Not yet (phase C+)

Full CSP defaults for all apps, Redis session, argon2 passwords, keep-alive, in-process TLS, request read deadlines (B7), generators (`ori-web-app`).
