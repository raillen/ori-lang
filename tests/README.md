# Ori Tests

The active regression suite lives in the Rust workspace.

Run the full suite:

```bash
cargo test --workspace
```

Useful focused checks:

```bash
cargo test -p ori-driver
cargo test -p ori-codegen
cargo test -p ori-runtime
```

The driver suite also checks the official examples in `examples/*.orl` so the
public samples do not drift away from the implemented language.
