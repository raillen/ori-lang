# Zenith Lockfile Schema (Initial)

- Status: initial M20 schema
- Date: 2026-04-18
- Scope: local reproducible dependency resolution for bootstrap tooling

## File Name

The canonical lockfile name is `zenith.lock`.

## Goals

- persist exact dependency resolution for reproducible builds
- decouple requested constraints in `zenith.ztproj` from concrete resolved sources
- provide a stable base for future `zpm update` and `zpm publish`

## Initial Format

The initial format is TOML-like, versioned by schema.

```toml
schema = 1
generated_by = "zt-next"

[[package]]
name = "json"
source = "git"
git = "https://github.com/zenith-lang/json.git"
rev = "2f6f2d41d8e0f6a0f6b8f6d67f4f8fca6f0e3fd2"
tag = "v1.2.0"

[[package]]
name = "test_helpers"
source = "path"
path = "../local_libs/test_helpers"
```

## Rules

- `schema` is required and must be `1` in this cut
- each `[[package]]` entry represents one resolved dependency
- `name` is required and must match the key from `[dependencies]` or `[dev_dependencies]`
- `source` is required and must be one of: `git`, `path`, `version`
- for `source = "git"`, `git` and `rev` are required
- for `source = "path"`, `path` is required
- for `source = "version"`, `version` is required
- `tag` and `branch` are optional metadata fields for git sources
- duplicate `name` entries are invalid

## Current Implementation Boundary

- `zenith.ztproj` parser already reads `[dependencies]` and `[dev_dependencies]` entries as initial raw specs
- `zpm install` writes `zenith.lock`
- `zpm install --locked` reads `zenith.lock` and fails if direct dependencies are missing or stale
- full registry resolution, downloads for version dependencies, and complete transitive resolution remain post-bootstrap tasks
