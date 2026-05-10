# Data Parser Example

This example parses a JSON object into `map<text, text>`.

It shows:

- `std.json.parse`;
- safe map lookup with `.get`;
- `optional` matching;
- `result` for missing required fields.

## Run

```powershell
.\zt.exe run examples\data-parser\zenith.ztproj --ci
```

Expected output:

```text
parsed
```
