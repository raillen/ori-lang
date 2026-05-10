# C Binding Example: SQLite

This example shows the current safe pattern for C libraries that need raw pointers.

Zenith does not expose raw pointers in the language surface yet. The binding uses a tiny C shim:

- Zenith calls `zt_sqlite3_memory_exec(sql: text) -> int`.
- The C shim opens an in-memory SQLite database.
- The shim runs the SQL and closes the database.
- Zenith receives only an integer status code.

Run the checked workflow from the repository root:

```powershell
python tests/driver/test_cbinding_sqlite3_example.py
```

The script looks for SQLite in common MinGW locations or through these environment variables:

- `SQLITE3_INCLUDE`
- `SQLITE3_LIB_DIR`
- `SQLITE3_DLL_DIR`

If SQLite is not installed, the script exits successfully with a clear skip message.
