# Quickstart

> Audience: user
> Status: current
> Surface: public

Use this page to check that the compiler works.

## 1. Check The Compiler

From the repository root:

```powershell
.\zt.exe help
```

On Linux or macOS, use the platform binary name when available:

```bash
./zt help
```

## 2. Check The Hello World Example

```powershell
.\zt.exe check examples/hello-world/zenith.ztproj
```

This validates the project without running it.

## 3. Run The Example

```powershell
.\zt.exe run examples/hello-world/zenith.ztproj
```

Expected result:

```text
Hello, Zenith!
```

## 4. Read Next

- `docs/public/learn/learn-zenith-in-30-minutes.md`
- `docs/public/packages/tooling-guide.md`
- `docs/public/language/language-reference.md`
