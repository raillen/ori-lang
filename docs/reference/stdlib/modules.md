# Stdlib Modules

> Surface: reference
> Status: current

| Module | Responsibility |
| --- | --- |
| `std.io` | input/output handles, read/write/print |
| `std.console` | interactive terminal helpers over `std.io`: prompts, line output, terminal size, ANSI colors/styles, clear, optional key read |
| `std.text` | text helpers and UTF-8 conversion |
| `std.bytes` | bytes helpers |
| `std.format` | number/date/bytes formatting helpers |
| `std.fs` | filesystem operations |
| `std.fs.path` | path operations |
| `std.os` | process args, env, platform and cwd |
| `std.os.process` | run external processes |
| `std.time` | instant and duration helpers |
| `std.collections` | queues, stacks, grids, heaps, btree wrappers |
| `std.list` | small compiler-known helpers for built-in lists |
| `std.map` | compiler-known helpers for built-in maps, including value-style helpers for generated `int`/`text` key maps and safe structural-key maps |
| `std.math` | numeric functions and constants |
| `std.regex` | simple pattern validation and text search |
| `std.random` | simple random state and stats |
| `std.validate` | validation predicates |
| `std.concurrent` | explicit copy helpers for boundaries |
| `std.lazy` | explicit one-shot lazy value helpers |
| `std.test` | test fail/skip helpers and simple readable assertions |
| `std.net` | alpha network connection facade |

Underscore-prefixed declarations are compatibility/internal helpers and should not be used in public examples.
