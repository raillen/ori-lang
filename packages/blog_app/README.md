# blog_app

Scaffolded by `ori-web-app` (Rails-like App layer for Ori).

## Run

```bash
cd packages/blog_app
ori get .
ori run main.orl
# http://127.0.0.1:3000/
```

## Layout

| Path | Role |
|------|------|
| `main.orl` | boot |
| `config/app.orl` | port, roots |
| `config/routes.orl` | explicit routes |
| `app/controllers/` | handlers |
| `app/application.orl` | shared helpers |
| `views/` | `.orix` templates |
| `public/` | static (`/assets/*`) |

## Generators

```bash
# from app root
/home/raillen/.grok/worktrees/projetos-ori-lang/analise-e-avaliao/packages/ori-web-app/bin/generate-controller posts
```
