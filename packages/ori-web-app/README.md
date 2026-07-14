# web_app (`ori-web-app`)

**Level 3 — App** layer for Ori: Rails-like conventions without magic.

- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) §12 (D21–D27, APP*)
- Builds on: `web` + `templates`

## What you get

| Piece | Role |
|-------|------|
| `standard_app()` | Batteries: rate limit, CSP, Secure cookie in production |
| `page_data` / `render` / `render_partial` | CSRF + flash + optional user in template ctx |
| `csrf_field(req)` | HTML hidden input |
| `run(app)` | Listen + secret check when `ORI_ENV=production` |
| `bin/new` | Scaffold app tree |
| `bin/generate-controller` | Controller + index view + route stub |
| `bin/generate-scaffold` | Resource: index / new / create + form |

## Scaffold a site

```bash
# from packages/
./ori-web-app/bin/new myapp ./myapp
cd myapp
ori get .
ori run main.orl
# http://127.0.0.1:3000/
```

Add a resource:

```bash
cd myapp
../ori-web-app/bin/generate-controller posts   # index only
../ori-web-app/bin/generate-scaffold notes     # index + new + create
ori check main.orl
```

## Convention tree

```text
myapp/
  ori.proj
  main.orl                 -- boot
  config/
    app.orl                -- port, roots
    routes.orl             -- explicit draw()
  app/
    application.orl        -- shared helpers
    controllers/
      home.orl
  views/
    layouts/app.orix
    home/index.orix
  public/                  -- served at /assets/*
```

## Import

```ori
import web_app = wa
import web = web

main()
    var a: web.App = wa.standard_app()
    wa.mount_assets(a)
    -- register routes…
    match wa.run(a)
    case ok(_):
    case err(msg):
    end
end
```

## Env

| Variable | Meaning |
|----------|---------|
| `ORI_ENV` / `ORI_WEB_ENV` | `development` (default) or `production` / `prod` |
| `ORI_WEB_SECRET` | Required in production (min 16 chars) |

## Example

`packages/blog_app` — `bin/new` + `generate-controller posts` + `generate-scaffold notes`.
