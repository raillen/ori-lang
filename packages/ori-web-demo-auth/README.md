# ori-web-demo-auth

Login demo with **argon2id**, **TOTP 2FA**, lockout, audit log, and **SQLite sessions** (fallback file).

## Run

```bash
# ensure packages/ori-sqlite symlink + native libs
ln -sfn ~/Documentos/Projetos/ori-sqlite ../ori-sqlite   # if missing
( cd ../ori-sqlite && ./tools/build_linux.sh )

cd packages/ori-web-demo-auth
ORI_USE_AOT=1 ori run main.orl
# http://127.0.0.1:3459/
```

Credentials: **`demo` / `demo`**, then the 6-digit TOTP code.

On boot the server prints the base32 secret and `otpauth://` URI for authenticator enrollment. In development the `/2fa` page also shows a live code hint.

## Env

| Variable | Meaning |
|----------|---------|
| `ORI_WEB_SESSION` | `sqlite` (default) · `file` · `memory` |
| `ORI_ENV` | `production` hides TOTP hint |

SQLite DB: `var/sessions.db` (created next to cwd).
