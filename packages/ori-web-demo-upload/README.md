# ori-web-demo-upload

C8 multipart upload demo (path jail, size + extension allowlist).

```bash
cd packages/ori-web-demo-upload
ori run main.orl
# http://127.0.0.1:3460/
```

Files land in `var/uploads/` (never under a static mount).
