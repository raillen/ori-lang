# web_auth (`ori-web-auth`)

Optional **2FA** helpers for Ori web apps (phase **C3**).

- TOTP: RFC 6238 via `ori.crypto` (HMAC-SHA1, 30s, 6 digits)
- Recovery codes (plain list helpers — hash at rest in real apps)
- Session flags for pending / completed 2FA

## Depend

```toml
[dependencies]
web = { path = "../ori-web", version = "0.1.0" }
web_auth = { path = "../ori-web-auth", version = "0.1.0" }
```

Requires an Ori runtime that exports `ori_totp_*` (0.3.5+ with staged runtime).

## API

| Function | Role |
|----------|------|
| `generate_secret()` | New base32 secret |
| `current_code(secret)` / `code_at(secret, unix)` | 6-digit code |
| `verify(secret, code)` / `verify_window(..., window)` | Check code (±window steps) |
| `otpauth_url(issuer, account, secret)` | Enrollment URI for authenticator apps |
| `mark_pending_2fa` / `mark_2fa_ok` / `is_2fa_ok` | Session flow after password |
| `generate_recovery_codes(n)` / `consume_recovery_code` | One-time backup codes |
| `require_2fa(next)` | Handler guard → `/2fa` or `/login` |

## Flow sketch

```ori
-- after password ok:
web_auth.mark_pending_2fa(req, user_id)
return ok(web.redirect(303, "/2fa"))

-- on /2fa POST:
if web_auth.verify(stored_secret, code)
    web_auth.mark_2fa_ok(req)
    web.session_set(req, "user_id", user_id)
    return ok(web.redirect(303, "/dashboard"))
end
```

## Smoke

```bash
cd packages/ori-web-auth/examples/smoke
ori run main.orl
```

## Not included

QR image generation, SMS, WebAuthn, or user database persistence.
