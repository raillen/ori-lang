# ori-web-demo-auth

Login demo: session regenerate (A8), protected dashboard, flash + PRG, rate limit (B4), file sessions (B3).

**Demo credentials:** `demo` / `demo` (not for production).

## Run

```bash
cd packages/ori-web-demo-auth
ori get .
ori run main.orl
# http://127.0.0.1:3459/
```

Sessions persist under `./.sessions/` (file store).

## Smoke

```bash
# login form
curl -s -c /tmp/au -b /tmp/au http://127.0.0.1:3459/login
# dashboard without login → redirect
curl -s -o /dev/null -w "%{http_code} %{redirect_url}\n" -c /tmp/au -b /tmp/au http://127.0.0.1:3459/dashboard
```
