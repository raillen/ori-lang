# Expression Readability Guide

> Audience: user, package-author
> Status: current
> Surface: reference

## Purpose

Keep symbol-heavy Zenith expressions readable without hiding intent.

Use this guide when code stacks multiple symbols such as `?`, `|>`, `@field`, generic types, and named arguments.

## Core Rules

- Prefer one intent per line.
- Break before cognitive overload, not only at hard width.
- Use intermediate names when a chain mixes validation, conversion, and side effects.
- Keep panic/contract boundaries visible.

## Symbol-Heavy Patterns

### 1) Pipeline + Error Propagation

Prefer:

```zt
const loaded: result<bytes, core.Error> = fs.read_bytes(path)
const raw: bytes = loaded.or_wrap("read input")?
const parsed: result<Model, core.Error> = parse_model(raw)
const model: Model = parsed.or_wrap("parse model")?
```

Avoid:

```zt
const model: Model = parse_model(fs.read_bytes(path).or_wrap("read input")?).or_wrap("parse model")?
```

### 2) `match` With Explicit Branch Intent

Prefer:

```zt
match result
    case success(value):
        return value
    case error(err):
        return fallback(err)
end
```

Avoid stacking guard logic and long calls in the `case` line.

### 3) Named Calls

Use multiline calls when named arguments are dense:

```zt
const report: text = format.render(
    title: "Build Report",
    subtitle: "Nightly",
    status: "ok",
    duration_ms: elapsed
)
```

## Intermediate Variables vs Chaining

### Choose intermediate variables when

- the chain mixes domain transformations and error conversion;
- the same subexpression is reused;
- diagnostics should point to a specific stage;
- reviewers need to reason about each step independently.

```zt
const raw: text = io.read_line()
const cleaned: text = text.trim(raw)
const valid: bool = validate.not_empty(cleaned)
if not valid
    return error(AppError.EmptyInput)
end
return success(cleaned)
```

### Keep chaining when

- each step is short and same-domain;
- no branch, side effect, or error remap is hidden;
- the final line stays readable in one scan.

```zt
const slug: text = text.trim(name)
    |> text.to_lower
    |> text.replace(" ", "-")
```

## Team Checklist

Before committing, ask:

- Can someone explain this expression without reading it twice?
- Are failure boundaries (`?`, `.or_return`, `panic`) obvious?
- Would a named intermediate improve diagnostics and maintenance?

If any answer is "no", split the expression.
