# Web stack — feature freeze (v1)

**Date:** 2026-07-14  
**Scope:** packages under `packages/ori-*` for HTML-first web (not the Ori compiler FREEZE-1).

## Frozen surface (do not expand casually)

| Layer | Package | Includes |
|-------|---------|----------|
| Templates | `ori-templates` | `@{ }`, if/for/layout/include, S8 `-` trim, escape + raw |
| HTTP | `ori-web` | router, static jail, session, CSRF, middleware, JSON flat+nested, upload C8, keep-alive, B7 timeouts, custom session hooks |
| App | `ori-web-app` | `standard_app`, render, generators REST |
| 2FA | `ori-web-auth` | TOTP + recovery helpers |
| Sessions SQLite | `ori-web-session-sqlite` | adapter over `ori-sqlite` |
| Demos | `ori-web-demo*` , `blog_app` | teaching / smoke |

## Runtime contracts used by web (language release)

These live in **ori-runtime / stdlib**, not in packages:

- `ori.crypto` password (argon2id) + TOTP  
- `ori.net` listen/accept/read/write + `set_read_timeout_ms` / `set_write_timeout_ms`  

A language release that “supports the web stack” must stage a runtime that exports those symbols.

## Allowed under freeze

- Bugfixes, security fixes  
- Docs, examples, QA scripts  
- Small additive APIs only with explicit decision (prefer patch versions)

## Not in v1 (deferred C)

Redis sessions · ORM · magic routes · WebAuthn/SMS · template `match` (S10) · in-process TLS server · language multi-OS package CI residual  

## QA gate before calling freeze “green”

```bash
./tools/qa/web_sec8.sh
./tools/qa/web_auth_smoke.sh
# optional if ori-sqlite linked:
./tools/qa/web_session_sqlite_smoke.sh
```

## Unfreeze

Document why in CHANGELOG + this file; bump package `version` fields when published.
