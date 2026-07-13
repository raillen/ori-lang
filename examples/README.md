# Ori Examples

Each example is a **mini-project** (M2.layout):

```text
example-name/
  ori.proj      # required
  main.orl      # recommended default entry
```

**Stdlib style:** import `ori.X` (not `ori.X.utils`). See `docs/planning/stdlib-merge-policy.md`.

## Run

```bash
cd compiler
cargo run -p ori-driver -- check ../examples/hello
cargo run -p ori-driver -- run ../examples/hello
```

Or with a packaged `ori`:

```bash
ori check examples/hello
ori run examples/hello
```

## Projects

See directories under `examples/*/`. Canonical minimal app: [`hello/`](hello/).
