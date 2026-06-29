# Ori Package Registry (v2 backlog)

> Status: planning document (2026-06-29). Not implemented.
> Audience: tooling implementers, release engineers
> CLI stubs: `ori install`, `ori publish` (exit 2 with pointer to this doc)

## Goal

Distribute Ori libraries and applications without requiring consumers to clone
the full `ori-lang` repository. The registry is a separate concern from the
compiler runtime staging (`runtime/<triple>/`).

## Package manifest (`ori.pkg.toml`)

```toml
[package]
name = "example.demo"
version = "0.1.0"
authors = ["Team <team@example.com>"]
license = "MIT"
description = "Short summary"
entry = "src/main.orl"
ori_version = "0.2.0"

[dependencies]
other.lib = "1.0.0"
```

Rules (planned):

- `name` uses dotted namespaces (`author.project`), aligned with Ori `namespace`.
- `entry` is the program or library root `.orl` file.
- `ori_version` is the minimum compiler semver the package requires.
- Dependencies resolve from the registry index (not yet specified).

## Registry index entry

```json
{
  "name": "example.demo",
  "version": "0.1.0",
  "sha256": "< tarball digest >",
  "url": "https://registry.example/ori/example.demo-0.1.0.tar.gz",
  "ori_version": "0.2.0",
  "published_at": "2026-06-29T00:00:00Z"
}
```

## Planned CLI surface

| Command | Status | Purpose |
|---------|--------|---------|
| `ori publish <path>` | stub | Validate manifest, build tarball, upload to registry |
| `ori install <name>` | stub | Resolve version, fetch tarball, extract to local cache |
| `ori add <name>` | future | Add dependency to `ori.pkg.toml` |

Local cache layout (planned):

```
~/.ori/packages/<name>/<version>/
  ori.pkg.toml
  src/...
```

## Relationship to stdlib Layer 2

The in-repo `stdlib/` tree remains the **prelude** shipped with the compiler.
Third-party packages use the same `.orl` module layout but live outside the
compiler install directory. `ORI_STDLIB_ROOT` overrides only the prelude, not
the package cache.

## Security (planned)

- Tarballs verified by `sha256` before extraction.
- Registry HTTPS only; optional signature verification in a later milestone.
- `ori install` never executes package code during fetch — only `ori check` /
  `ori compile` after explicit user action.

## Implementation order (suggested)

1. Manifest parser + `ori.pkg.toml` validation in `ori-driver`.
2. Local path dependencies (`path = "../other"`) for monorepos — no network.
3. Registry index fetch + tarball cache on disk.
4. `ori publish` upload API (requires hosted registry service).

Until step 1 lands, use `ori install` / `ori publish` stubs as the contract
anchor — they fail fast with a link to this document.
