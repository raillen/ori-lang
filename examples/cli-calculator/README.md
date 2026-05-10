# CLI Calculator Example

This example shows the shape of a small command-line calculator.

The current driver does not forward extra user arguments to `zt run` yet, so the
example uses fixed values. The code is still structured like a real CLI command:

- parse or choose an operation;
- call one small function;
- return `result<void, core.Error>`;
- print the final value.

## Run

```powershell
.\zt.exe run examples\cli-calculator\zenith.ztproj --ci
```

Expected output:

```text
42
```
