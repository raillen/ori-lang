# `zpm` CLI Reference

> Surface: reference
> Status: current

## Forma geral

```text
zpm <command> [options]
```

## Commands

| Command | Use |
| --- | --- |
| `init [path|.]` | initialize a Zenith project |
| `add <pkg>[@version]` | add a dependency to `zenith.ztproj` |
| `remove <pkg>` | remove a dependency |
| `install` | install dependencies and generate or reuse `zenith.lock` |
| `install --locked` | verify that `zenith.lock` matches `zenith.ztproj` |
| `update [pkg]` | update dependencies |
| `list` | list installed dependencies |
| `find <query>` | search for a package |
| `run <script>` | run a script from `zenith.ztproj` |
| `publish` | validate package publication readiness |
| `help` | show help |

## Examples

```powershell
.\zpm.exe init my_project
cd my_project
..\zpm.exe add borealis@0.1.0
..\zpm.exe install
..\zpm.exe install --locked
..\zpm.exe list
..\zpm.exe publish
```

## Manifest

```toml
[project]
name = "my_app"
kind = "app"
version = "0.1.0"

[dependencies]
borealis = "0.1.0"
json = "^1.2.3"
tools = "~2.0.1"
```

## Version Constraints

Dependency versions are SemVer-shaped strings:

- `1.2.3` means exactly that version.
- `^1.2.3` allows compatible updates inside the same major version.
- `~1.2.3` allows patch updates inside the same minor version.

Use an inline table for Git or local path dependencies:

```toml
[dependencies]
local_helpers = { path = "../helpers" }
net = { git = "https://example.invalid/net.git", rev = "2f6f2d" }
```

## Lockfile

Commit `zenith.lock` when the project needs reproducible dependency versions.

Use `zpm install --locked` in CI.

This mode:

- requires `zenith.lock`;
- checks `[dependencies]` and `[dev_dependencies]`;
- fails when the manifest changed but the lockfile was not refreshed;
- does not rewrite the lockfile.

## Registry Sync

Remote registry sync is not part of the public RC contract yet.

For this RC, version dependencies and local path dependencies are supported by
the lockfile flow. A future release can expose registry sync when the remote
registry is stable.
