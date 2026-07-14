# templates (`ori-templates`)

Server-side HTML template engine for Ori.

- Delimiters: `@{ … }`, comments `@{-- … --}`
- Escape HTML by default; raw only as **last** pipe stage: `@{ html |> raw }`
- Directives: `if` / `elif` / `else` / `for` / `include` / `layout` / `assign` / `end`
- **S8 whitespace:** leading/trailing `-` trims adjacent whitespace  
  `@{ - if x -}` … `@{ - end -}` (Jinja/ERB-style)
- Path jail under a views root; logical names → `.orix` files
- Design: `docs/planning/web-templates-discussion-roadmap.md` (D3–D28)

## Use (path dependency)

```toml
# ori.proj
[dependencies]
templates = { path = "../packages/ori-templates", version = "0.1.0" }
```

```ori
import templates = tpl

match tpl.render_string("Hello @{ name }!", tpl.data_with("name", "World"))
case ok(html):
    io.println(html)
case err(e):
    io.println(tpl.error_message(e))
end
```

## Smoke

```bash
cd packages/ori-templates/examples/smoke
ori get .
ori run .
```

## Status

MVP Library (not full `ori-web` / App layer). Context is string values + named string lists for `for`.
