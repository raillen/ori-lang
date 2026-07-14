# Ori packages (web stack)

External to the language FREEZE-1 core. Path-depend from apps with `ori.proj`.

| Package | Role | Entry |
|---------|------|--------|
| **ori-templates** | HTML templates (`@{ }`, `.orix`) | Library |
| **ori-web** | HTTP, session, CSRF, phase B/C | Library |
| **ori-web-app** | Rails-like App layer + generators | App |
| **ori-web-auth** | Optional 2FA (TOTP + recovery codes) | Library |
| **ori-web-session-sqlite** | SQLite session store (B3 adapter) | Library (+ `ori-sqlite`) |
| **ori-web-demo** | HTML-first notes (htmx) | :3457 |
| **ori-web-demo-api** | JSON API | :3458 |
| **ori-web-demo-auth** | Login + argon2id + lockout | :3459 `demo`/`demo` |
| **blog_app** | Scaffolded App example | :3000 |

## Generators (`ori-web-app/bin`)

```bash
./ori-web-app/bin/new myapp ./myapp
cd myapp
../ori-web-app/bin/generate-controller posts
../ori-web-app/bin/generate-scaffold notes
../ori-web-app/bin/generate-model User
ori get . && ori run main.orl
```

## Docs

- Templates/web design: `docs/planning/web-templates-discussion-roadmap.md`
- Phase B/C/D: `ori-web/docs/phase-*.md`
- Password hashing: `ori.crypto` (argon2id) — needs Ori build with staged runtime
- 2FA: `ori-web-auth` + `ori.crypto.totp_*` — smoke: `packages/ori-web-auth/examples/smoke`
