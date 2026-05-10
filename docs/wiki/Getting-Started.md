# Getting Started

Requirements:

- Python 3.
- GCC or Clang in `PATH`.
- PowerShell, bash, or equivalent shell.

Build the compiler:

```bash
python build.py
```

Run help:

```bash
./zt.exe help
```

Run a single `.zt` file (no project needed):

```bash
./zt.exe run hello.zt
./zt.exe check hello.zt
./zt.exe build hello.zt
```

Create a project:

```bash
./zt.exe create examples/hello --app
```

Check a project:

```bash
./zt.exe check examples/hello/zenith.ztproj --all
```

Run a project:

```bash
./zt.exe run examples/hello/zenith.ztproj
```

More detail:

- Final language contract: https://github.com/raillen/zenithlang/blob/master/docs/spec/language/final-language-contract.md
- CLI reference: https://github.com/raillen/zenithlang/blob/master/docs/reference/cli/README.md
- Language reference: https://github.com/raillen/zenithlang/blob/master/docs/reference/language/README.md
