# ori-web — Phase C helpers

Design: roadmap §5.5 phase C.

| ID | Feature | API |
|----|---------|-----|
| **C1 CSP** | Optional policy | `set_csp` (B) — demos set HTML-first policies |
| **C2 Re-auth** | Recent password / step-up | `mark_reauth`, `reauth_ok`, `require_reauth(max_age_ms, next)` |
| **C3 2FA** | TOTP + recovery | package `ori-web-auth` (`generate_secret`, `verify`, `otpauth_url`, session flags) |
| **C4 Lockout** | Fail counter + cooldown | `login_allowed`, `login_fail(key, max, lock_ms)`, `login_success` |
| **C5 Audit** | Append-only file | `set_audit_log(path)`, `audit(event, detail)` |
| **C6 CSRF rotate** | New token after mutation | `set_csrf_rotate(app, true)`, `rotate_csrf` |
| **C7 `__Host-` cookie** | With Secure | `set_host_cookie(app, true)` + `set_cookie_secure(true)` |
| **C8 Upload** | Multipart parse + safe save | `parse_multipart`, `form_file`, `save_upload(dir, part, max, "txt,png")` |
| **C9 Supply chain** | Process | monorepo hygiene |
| **C10 Password hash** | **Done** | `ori.crypto.password_hash` / `password_verify` (argon2id PHC) |

## Example (auth)

```ori
web.set_audit_log("audit.log")
a = web.set_csrf_rotate(a, true)

if not web.login_allowed(lock_key)
    return ok(web.redirect(303, "/login"))
end
-- on fail:
web.login_fail(lock_key, 5, 60000)
-- on success:
web.login_success(lock_key)
web.mark_reauth(req)
web.audit("login.ok", user)
```
