# Functions and Control Flow Reference

> Surface: reference
> Status: current

## Function

```zt
func add(a: int, b: int) -> int
    return a + b
end
```

No useful return value:

```zt
func log(message: text)
    -- body
    return
end
```

## Public Function

```zt
public func version() -> text
    return "0.1.0"
end
```

## Default Parameters

```zt
func open(path: text, mode: text = "read") -> int
    return 0
end
```

## If

```zt
if value > 0
    return 1
else
    return 0
end
```

## While

```zt
var i: int = 0
while i < 10
    i = i + 1
end
```

## Repeat

```zt
repeat 3 times
    -- body
end
```

## Match

Use `match` when one value can follow several paths.

```zt
match value
    case 0 ->
        return "zero"
    case else ->
        return "other"
end
```

## Match With Multiple Values

Several literal values can share the same branch.

```zt
match status_code
    case 200, 201, 204 ->
        return "success"
    case 400, 401, 403, 404 ->
        return "client_error"
    case 500, 502, 503 ->
        return "server_error"
    case else ->
        return "unknown"
end
```

This avoids repeated branches with the same body.

## Match Optional

```zt
match find_name(1)
    case value name ->
        return name
    case none ->
        return "anonymous"
end
```

## Match Result

```zt
match parse_port("8080")
    case success(port) ->
        return port
    case error(message) ->
        panic(message)
end
```

## Assignment

Assignment is a statement, not an expression.

```zt
var count: int = 0
count = count + 1
```

This keeps side effects visible and avoids dense expression tricks.
