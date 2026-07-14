# ori-web-demo

HTML-first smoke app on top of **`web`** + **`templates`** (path **C** in the web roadmap).

- Full page: `GET /`
- Notes partial (htmx-friendly): `GET /notes/rows`
- Create note: `POST /notes` with CSRF (PRG redirect, or partial when `HX-Request: true`)
- Static assets: `/assets/*` → `public/`
- Design: [`docs/planning/web-templates-discussion-roadmap.md`](../../docs/planning/web-templates-discussion-roadmap.md) D19 / §11

## Run

```bash
cd packages/ori-web-demo
ori run main.orl
# open http://127.0.0.1:3457/
```

Stack index / freeze: [`../README.md`](../README.md) · [`../FREEZE-WEB.md`](../FREEZE-WEB.md).

## Smoke

```bash
# full page
curl -s -c /tmp/wd -b /tmp/wd http://127.0.0.1:3457/ | head

# partial
curl -s http://127.0.0.1:3457/notes/rows

# POST without CSRF → 403
curl -s -o /dev/null -w "%{http_code}\n" -c /tmp/wd -b /tmp/wd \
  -X POST -d 'text=hi' http://127.0.0.1:3457/notes
```

Use the form on `/` (or extract `csrf_token` from HTML) for a successful create.
