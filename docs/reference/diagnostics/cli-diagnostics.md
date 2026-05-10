# Diagnostics Reference

> Surface: reference
> Status: current

## Sources

- Catalog: `docs/spec/language/diagnostic-code-catalog.md`.
- Model: `docs/spec/language/diagnostics-model.md`.
- KB: `docs/reference/zenith-kb/diagnostics.md`.

## CLI Modes

Use normal output for local development:

```powershell
.\zt.exe check zenith.ztproj
.\zt.exe check hello.zt
```

Use CI output for automation:

```powershell
.\zt.exe check zenith.ztproj --ci
```

## Rule

Docs should include exact diagnostic text only when it is pinned by tests or fixtures.

Otherwise, link to the catalog and describe the class of error.
