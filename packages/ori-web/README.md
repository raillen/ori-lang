# web (`ori-web`)

Minimal HTTP **Library** layer for Ori.

- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) (D14–D20)
- Learning course: [`docs/planning/web-framework-learning-course.md`](../../docs/planning/web-framework-learning-course.md)

## Features (MVP)

| Area | API |
|------|-----|
| Types | `Request`, `Response`, `App`, `Handler`, `ActionResult` |
| Responses | `text`, `html`, `redirect`, `not_found`, `forbidden`, `bad_request`, `payload_too_large` |
| Router | `get` / `post` / `put` / `patch` / `delete` + path params `:id` |
| Static | `static(app, url_prefix, dir)` with `..` path jail |
| Session | cookie `ori_sid` (HttpOnly, SameSite=Lax); `session_get` / `session_set` / flash |
| CSRF | synchronizer token — form `csrf_token` or header `X-CSRF-Token` on mutations |
| Dispatch | `dispatch(app, req)` — **not** named `handle` (`handle` is a reserved keyword in Ori) |
| Serve | `serve(host, port, app)` — HTTP/1.1 accept loop, `Connection: close` |
| Auth helper | `require_session_key(key, next)` → wrapped `Handler` |

## Use (path dependency)

```toml
# ori.proj
[dependencies]
web = { path = "../packages/ori-web", version = "0.1.0" }
```

```ori
module app.main

import web = web
import ori.io = io

home(req: web.Request) -> web.ActionResult
    return ok(web.text(200, "hello"))
end

main()
    var a: web.App = web.app()
    web.get(a, "/", home)
    match web.serve("127.0.0.1", 3456, a)
    case ok(_):
    case err(msg):
        io.println(msg)
    end
end
```

## Smoke

```bash
cd packages/ori-web/examples/hello_server
ori get .
ori run main.orl
# other terminal:
curl -s http://127.0.0.1:3456/hello
curl -s -c /tmp/cj -b /tmp/cj http://127.0.0.1:3456/   # form + CSRF
# POST with csrf_token from the form (or expect 403 without it)
```

## Not yet (roadmap §5.5 phases B–D)

HTTPS in-process, Redis/session store, rate limit, full CSP, multi-connection HTTP/1.1 keep-alive, generators (`ori-web-app`).

## Pair with templates

Use the `templates` package (`packages/ori-templates`) for HTML views; this package only ships the HTTP app layer.
