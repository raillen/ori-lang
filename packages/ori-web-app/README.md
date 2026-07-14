# web_app (`ori-web-app`)

**Level 3 — App** layer for Ori: Rails-like conventions without magic.

- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) §12 (D21–D27, APP*)
- Builds on: `web` + `templates`

## What you get

| Piece | Role |
|-------|------|
| `standard_app()` | Batteries: rate limit, CSP, Secure cookie in production |
| `page_data` / `render` / `render_partial` | **W5** template integration: CSRF + flash + user in ctx |
| `csrf_field(req)` | HTML hidden input |
| `run(app)` | Listen + secret check when `ORI_ENV=production` |
| `bin/new` | Scaffold app tree |
| `bin/generate-controller` | Controller + index view + route stub |
| `bin/generate-scaffold` | Full REST resource: index / new / create / show / edit / update / destroy + views |
| `bin/generate-model` | Domain stub + optional password helpers |

Template engine is owned here (not in `web`) to avoid Library↔templates cycles.

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
../ori-web-app/bin/generate-scaffold notes     # full REST (CRUD + views)
ori check main.orl
```

### REST routes from `generate-scaffold <res>`

| Method | Path | Action |
|--------|------|--------|
| GET | `/{res}` | index |
| GET | `/{res}/new` | new_form |
| POST | `/{res}` | create |
| GET | `/{res}/:id` | show |
| GET | `/{res}/:id/edit` | edit_form |
| POST | `/{res}/:id` | update |
| POST | `/{res}/:id/delete` | destroy |

In-memory list store (IDs are 0-based string indices). Replace with domain/DB later.

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
