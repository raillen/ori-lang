# ori-web — middleware

Middleware wraps the **matched route handler only** (after static, CSRF, rate limit).

## API

| Function | Role |
|----------|------|
| `use_middleware(app, mw)` | Push onto stack (last = outermost) |
| `clear_middleware()` | Clear stack (`app()` also clears) |
| `mw_set_header(name, value)` | Set response header after handler |
| `mw_timing()` | `X-Response-Time-Ms` |
| `mw_request_id()` | `X-Request-Id` (honors inbound header) |

`use` is a reserved keyword in Ori — use **`use_middleware`**.

## Custom middleware

```ori
public alias Middleware = func(Handler) -> Handler

mw_tag(next: web.Handler) -> web.Handler
    return (req: web.Request) => run_tag(next, req)
end

run_tag(next: web.Handler, req: web.Request) -> web.ActionResult
    match next(req)
    case ok(res):
        return ok(web.text(res.status, str.concat("mw:", res.body)))
    case err(e):
        return err(e)
    end
    return err("mw")
end

a = web.use_middleware(a, mw_tag)
a = web.use_middleware(a, web.mw_timing())
```

## Order

1. Static mounts  
2. Rate limit (mutations)  
3. CSRF (mutations)  
4. User middleware onion (outermost last registered)  
5. Route handler  
