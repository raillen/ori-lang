# File Processor Example

This example reads text, trims it, validates it, and writes a summary file.

It uses:

- `std.fs.create_dir_all`;
- `std.fs.write_text`;
- `std.fs.read_text`;
- `std.text.trim`;
- `result` and `?` for recoverable errors.

## Run

```powershell
.\zt.exe run examples\file-processor\zenith.ztproj --ci
```

Expected output:

```text
processed
```
