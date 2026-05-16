# Ori Examples

This directory contains small examples that are expected to type-check with the
current compiler.

## Examples

- [hello_world.orl](hello_world.orl): Basic output and variable usage.
- [calculator.orl](calculator.orl): Function declarations and arithmetic operations.
- [bytes_usage.orl](bytes_usage.orl): Bytes conversion, concatenation, slicing, and UTF-8 decoding.
- [collections_demo.orl](collections_demo.orl): Lists, maps, and collection helpers.
- [queue_demo.orl](queue_demo.orl): FIFO queue operations and empty handling.
- [stack_demo.orl](stack_demo.orl): LIFO stack operations and empty handling.
- [deque_demo.orl](deque_demo.orl): Double-ended queue operations.
- [tree_demo.orl](tree_demo.orl): Tree arena operations and traversal.
- [graph_demo.orl](graph_demo.orl): Directed graph traversal and topological sort.
- [heap_demo.orl](heap_demo.orl): Min-heap push, peek, pop, and empty handling.
- [async_demo.orl](async_demo.orl): `async func`, `await`, and `ori.task.sleep`.
- [logic_and_matching.orl](logic_and_matching.orl): Control flow, `optional`, and range loops.
- [struct_demo.orl](struct_demo.orl): Custom struct declarations and instantiation.
- [error_handling.orl](error_handling.orl): Result type, `success/error`, and the `?` operator.
- [release_smoke.orl](release_smoke.orl): Small release-package compile smoke test.

## How to run

Type-check an example:

```bash
cargo run -p ori-driver -- check examples/hello_world.orl
```

Compile an example:

```bash
cargo run -p ori-driver -- compile examples/hello_world.orl --out examples/hello_world.exe
```

Run a release-style compile from a package directory:

```powershell
$env:ORI_REQUIRE_PACKAGED_RUNTIME = "1"
.\ori.exe compile .\examples\release_smoke.orl --out .\release_smoke.exe
.\release_smoke.exe
```

That command expects the package to contain `ori.exe` plus `runtime/{target-triple}`.
