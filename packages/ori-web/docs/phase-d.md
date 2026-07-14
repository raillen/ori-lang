# Phase D — ops / edge checklist

Outside the Ori process. Pair with phase B (TLS at edge) and C (app hardening).

| ID | Item | Done when |
|----|------|-----------|
| **D1** | Reverse proxy (Caddy / nginx / Traefik) | App listens on localhost; proxy terminates TLS |
| **D2** | HSTS | HTTPS stable; `Strict-Transport-Security` on proxy |
| **D3** | Backups | Session store files / DB / audit.log scheduled |
| **D4** | Least privilege | Process user non-root; filesystem jailed |
| **D5** | Secrets | `ORI_WEB_SECRET` / DB URLs only in env or secret manager — never git |

## Minimal Caddy

```caddy
example.com {
  encode gzip
  reverse_proxy 127.0.0.1:3000
  header {
    Strict-Transport-Security "max-age=31536000; includeSubDomains"
  }
}
```

## App env (production)

```bash
export ORI_ENV=production
export ORI_WEB_SECRET='…at-least-16-chars…'
# then:
ori run main.orl
```

With proxy:

```ori
a = web.set_cookie_secure(a, true)
a = web.set_trust_proxy(a, true)
a = web.set_host_cookie(a, true)  -- optional __Host- prefix
```

## Password hashes (C10)

Use `ori.crypto.password_hash` / `password_verify` (argon2id). Never store plain passwords.

## TLS (in-process vs edge)

**Recommended:** terminate TLS at Caddy/nginx/Traefik (this doc). Ori `serve` is
plain HTTP on localhost.

**Not in core v1:** in-process TLS listener (rustls server) — client
`connect_tls` already exists for outbound HTTPS. Prefer the reverse proxy for
certificates, HTTP/2, and HSTS.

## B7 + keep-alive

```ori
a = web.set_read_timeout(a, 30000)   -- socket read/write deadline
a = web.set_keep_alive(a, true, 32)
```

Proxy idle timeouts should still be shorter or equal than app deadlines.
