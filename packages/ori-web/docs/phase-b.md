# ori-web — Phase B (production floor)

Design: [`web-templates-discussion-roadmap.md`](../../../docs/planning/web-templates-discussion-roadmap.md) §5.5.

| ID | Feature | API / practice |
|----|---------|----------------|
| **B1 HTTPS** | TLS at reverse proxy (recommended) or runtime later | See Caddy/nginx snippet below. Local HTTP is for dev only. |
| **B2 Secure cookie** | `set_cookie_secure(app, true)` behind HTTPS | Always on in production. |
| **B3 Session store** | `use_memory_sessions` / `use_file_sessions` / `use_kv_sessions` / `use_custom_sessions` | Memory / file / kv built-in. Custom hooks for SQLite (`ori-web-session-sqlite`) or Redis. Helpers: `clear_session_cache`, `purge_expired_sessions`, `session_backend`. |
| **B4 Rate limit** | `set_rate_limit(app, per_minute)` | Applied to mutations; key = `client_key` (`X-Forwarded-For` if `set_trust_proxy`). |
| **B5 Flash + PRG** | `flash` / `take_flash` + `redirect(303, …)` | Already in MVP demos. |
| **B6 Security headers** | Always: nosniff, `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy` | Optional CSP: `set_csp(app, policy)`. |
| **B7 Request timeout** | Socket + body assembly | `set_read_timeout(app, ms)` → `ori.net.set_read_timeout_ms` / write timeout; `serve` reads headers then full `Content-Length` body (capped by `max_body`). Still pair with proxy idle timeouts (phase D). |

### Implementation note (App config)

`set_max_body` / `set_rate_limit` / `set_csp` / … mutate **module globals** and
return the same `App` value. Rebuilding `App { routes: a.routes, … }` (struct
copy with embedded lists) currently triggers a runtime ARC crash
(`malloc(): unaligned tcache chunk detected`). Keep `App` as routes + static
mounts only until that is fixed in the language runtime.
| **A7 Secret** | `ORI_WEB_SECRET` when `ORI_WEB_ENV=production\|prod` | `require_secret` / auto in `serve`. Min length 16. |
| **A8 Regenerate** | `session_regenerate(req) -> string` | Call after login; cookie follows via sid alias. |
| **A9 Timeouts** | `set_session_timeouts(idle_ms, absolute_ms)` | Defaults 1h idle / 24h absolute. |

## Edge TLS (B1 + D1)

Example **Caddy**:

```caddy
example.com {
  reverse_proxy 127.0.0.1:3457
}
```

Example **nginx**:

```nginx
server {
  listen 443 ssl http2;
  server_name example.com;
  # ssl_certificate …;
  location / {
    proxy_pass http://127.0.0.1:3457;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
```

With proxy, enable:

```ori
a = web.set_cookie_secure(a, true)
a = web.set_trust_proxy(a, true)
```

## Env

| Variable | Meaning |
|----------|---------|
| `ORI_WEB_ENV` | `dev` (default) or `production` / `prod` |
| `ORI_WEB_SECRET` | Required in production; min 16 chars |
