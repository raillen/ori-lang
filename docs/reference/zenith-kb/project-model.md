# Zenith Project Model

## Manifest File: `zenith.ztproj`

TOML-like format, strict. Unknown sections/keys = errors.

### App Project

```toml
[project]
name = "hello"
kind = "app"
version = "0.1.0"

[source]
root = "src"

[app]
entry = "app.main"

[build]
target = "native"
output = "build"
profile = "debug"
monomorphization_limit = 1024

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Rules:
- `project.name`, `project.kind`, `project.version` = required
- Accepted MVP kinds: `app`, `lib`
- App projects require `[app] entry`
- `app.entry` = namespace containing `func main()`
- `source.root` = required
- Build defaults: `target = "native"`, `output = "build"`, `profile = "debug"`, `monomorphization_limit = 1024`
- Accepted MVP profiles: `debug`, `release`
- `test.root` defaults to `tests`
- `zdoc.root` defaults to `zdoc`

### Lib Project

```toml
[project]
name = "text_utils"
kind = "lib"
version = "1.0.0"

[source]
root = "src"

[lib]
root_namespace = "text_utils"

[build]
target = "native"
output = "build"
profile = "debug"
monomorphization_limit = 1024

[test]
root = "tests"

[zdoc]
root = "zdoc"
```

Rules:
- Lib projects do NOT require `func main()`
- Lib projects require `lib.root_namespace`
- `lib.root_namespace` = public namespace boundary
- Apps are NOT importable dependencies by default

## Source Layout

"Folder is the Namespace" paradigm is absolute.

```text
my_app/
  zenith.ztproj
  src/
    app/
      main/
        main.zt              -- namespace app.main
      users/
        service.zt           -- namespace app.users
        validation.zt        -- namespace app.users
        types/
          administrator.zt   -- namespace app.users.types
  tests/
    app/
      users/
        users_test.zt
  zdoc/
    app/
      users/
        service.zdoc
  build/
```

Rules:
- `src/app/users/service.zt` MUST declare `namespace app.users`
- Mismatch between declared namespace and directory path = **fatal compilation error**
- Multiple files in same directory declare the **identical** namespace
- File name does NOT become part of namespace string
- Imports resolve namespaces, not files
- Generated directories (`build/`, `dist/`) are NOT source roots

## Imports and Package Boundaries

```zt
import text_utils.formatting as formatting
```

Rules:
- Namespace aliases written in source imports
- Dependency aliases in manifest = NOT MVP
- Exported API controlled by `public`, not manifest export list

## Dependencies

Decentralized package architecture. No central registry.

```toml
[dependencies]
json = { git = "https://github.com/zenith-lang/json.git", tag = "v1.2.0" }
net = { git = "https://github.com/zenith-lang/net.git", branch = "main" }

[dev_dependencies]
test_helpers = { path = "../local_libs/test_helpers" }
```

Rules:
- `git` URLs with strict `tag` or `rev` hashes required for reproducible releases
- Local `path` dependencies allowed for monorepos
- Version ranges = post-MVP
- Optional dependencies = post-MVP
- Feature flags = post-MVP

## Versioning

SemVer-shaped strings: `major.minor.patch`

```toml
version = "1.4.2"
```

## Lockfile

- Name: `zenith.lock`
- `zenith.ztproj` declares requested dependencies
- `zenith.lock` records exact resolved dependencies
- Apps should commit lockfile
- Libs may commit lockfile for local dev reproducibility

## ZDoc Layout

```text
src/app/users/service.zt        -- source
zdoc/app/users/service.zdoc     -- paired API doc
zdoc/guides/getting-started.zdoc -- guide
```

Rules:
- Paired docs use `@target`
- Guide pages use `@page`
- Private symbols may be documented
- Public generated docs exclude private docs by default
- Missing public documentation = warning

## ZPM (Package Manager)

```text
zpm add json
zpm update
zpm publish
zpm remove json
```

`zpm build` is NOT canonical. Build is a `zt` responsibility.
