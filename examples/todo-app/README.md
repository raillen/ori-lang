# TODO App Example

This example models a tiny TODO list in memory.

It is intentionally small:

- one `struct`;
- three TODO items;
- one small counting helper;
- one validation;
- one printed result.

## Note

The current native backend does not support `list<Todo>` yet.

This example keeps the data as individual `Todo` values so it stays runnable.

## Run

```powershell
.\zt.exe run examples\todo-app\zenith.ztproj --ci
```

Expected output:

```text
2
```
