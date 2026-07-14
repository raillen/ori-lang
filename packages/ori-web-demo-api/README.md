# ori-web-demo-api

JSON notes API on **`web`** (phase B: rate limit, security headers, CSRF header).

| Method | Path | Notes |
|--------|------|--------|
| `GET` | `/api/health` | liveness |
| `GET` | `/api/notes` | list notes as JSON |
| `POST` | `/api/notes` | create (`text=…` form or body); needs CSRF |
| `GET` | `/api/csrf` | `{ "csrf_token": "…" }` for clients |

## Run

```bash
cd packages/ori-web-demo-api
ori get .
ori run main.orl
# http://127.0.0.1:3458/
```

## Smoke

```bash
curl -s -c /tmp/api -b /tmp/api http://127.0.0.1:3458/api/csrf
TOKEN=$(curl -s -c /tmp/api -b /tmp/api http://127.0.0.1:3458/api/csrf | sed -n 's/.*"csrf_token":"\([^"]*\)".*/\1/p')
curl -s -c /tmp/api -b /tmp/api -H "X-CSRF-Token: $TOKEN" \
  -X POST -d 'text=hello+api' http://127.0.0.1:3458/api/notes
curl -s -c /tmp/api -b /tmp/api http://127.0.0.1:3458/api/notes
```
