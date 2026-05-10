# Decision 094 — Syntax Coherence Refinements

- Status: accepted
- Date: 2026-04-28
- Scope: surface syntax unification, keyword reduction, ergonomic improvements
- Upstream: coherence audit session (2026-04-28)
- Cross-reference: `docs/internal/planning/roadmap-v7.md` Phase 1G, `docs/internal/planning/checklist-v7.md` L.34–L.55

## Context

A coherence audit of the Zenith language surface identified inconsistencies between documentation, implementation, and philosophy. This decision records all changes agreed upon to unify the language surface with its reading-first, neurodivergent-friendly philosophy.

## Decisions

### T-1: Match case delimiter

**Change:** `->` to `:` as the canonical match case delimiter.

Before:
```zt
case 10 -> handle()
```

After:
```zt
case 10:
    handle()
```

Rationale: `:` is universally recognized. `->` conflicts visually with function return type syntax.

### T-2: Comment syntax

**Change:** Fix all documentation examples using `/* */` to use `--` / `--- ---`.

The lexer only supports `--` and `--- ---`. Documentation inconsistencies are resolved.

### T-3: Beginner example

**Change:** Create a canonical `hello-world` example using `print()` and `func main()`.

```zt
namespace app.main

func main()
    print("Hello, Zenith!")
end
```

### T-4: Optional match syntax

**Change:** Replace `case value name` with `case some(name)` for optional matching.

Before:
```zt
match maybe_user
case value user:
    return user.name
case none:
    return "anonymous"
end
```

After:
```zt
match maybe_user
case some(user):
    return user.name
case none:
    return "anonymous"
end
```

Rationale: `some(name)` is visually consistent with destructuring (`case variant(binding)`), explicit, and self-documenting.

Adds `some` as a keyword (`ZT_TOKEN_SOME`).

### T-6: Generic constraints — inline syntax

**Change:** Add inline generic constraints `<T: Trait>` as the primary form. `given` as optional trailing clause for complex constraints.

Before:
```zt
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable
```

After:
```zt
func contains<Item: Equatable>(items: list<Item>, target: Item) -> bool

-- Complex constraints use given:
func merge<K: Hashable, V>(a: map<K, V>, b: map<K, V>) -> map<K, V>
given K is Equatable
```

`where` remains for value-level runtime contracts only. `given` is a contextual keyword (not reserved).

### T-7: Type aliases

**Change:** Implement `type` aliases.

```zt
public type io_result = result<void, core.Error>
```

### T-8: `func main()` without return type

**Change:** Allow `func main()` with no return type. Implicit exit code 0.

```zt
func main()
    print("Hello!")
end
```

### T-9: If-else as expression

**Change:** `if-else` as expression with `then` keyword.

```zt
-- Inline form (requires else):
const label: text = if active then "on" else "off"

-- Multiline form:
const result: int = if score > 100
    compute_bonus()
else
    default_value()
end
```

`then` is a contextual keyword. `else` is always required in expression form.

Ternary `cond ? a : b` and postfix guard `return x if cond` are rejected.

### T-10: Mutable closure capture

**Change:** `capture` keyword (without `mut`) enables persistent mutable state inside closures.

```zt
func counter() -> func() -> int
    return func() -> int
        capture count: int = 0
        count = count + 1
        return count
    end
end
```

`capture` declares a heap-allocated binding local to the closure. Always mutable. No `mut` prefix needed — `capture` itself implies persistence and mutability.

### T-11: Keyword reduction

**Changes:**

1. **Demote specialized collections:** `grid2d`, `grid3d`, `pqueue`, `circbuf`, `btreemap`, `btreeset` become stdlib identifiers (not reserved keywords).
2. **Replace `default` with `else` in match:** `case else:` replaces `case default:`.
3. **Contextual keywords:** `then` and `given` are contextual identifiers, not reserved keywords.

Keyword count: 54 → 49 reserved + 2 contextual.

### T-12: `dyn` renamed to `any`

**Change:** Rename `dyn` keyword to `any` for dynamic dispatch.

Before:
```zt
const shape: dyn Shape = Circle(radius: 10)
var shapes: list<dyn Shape> = [circle, rect]
```

After:
```zt
const shape: any Shape = Circle(radius: 10)
var shapes: list<any Shape> = [circle, rect]
```

Rationale: `any Shape` reads as natural English ("any shape"). `dyn` is a Rust-specific abbreviation that violates the "words over symbols" principle. Swift uses `any Protocol` for the same concept.

Both `any Trait` and `any<Trait>` syntax forms are accepted.

### T-13: Struct literal shorthand

**Change:** Allow `{ fields }` when expected type is known from context.

```zt
-- Declaration with known type
const pos: Point = { x: 10, y: 20 }

-- Function argument
draw_rect({ x: 0, y: 0, width: 100, height: 50 })

-- Return value
func origin() -> Point
    return { x: 0, y: 0 }
end

-- List literals
const points: list<Point> = [{ x: 0, y: 0 }, { x: 1, y: 1 }]
```

The explicit `Type { fields }` form remains valid.

### T-14: Enum dot shorthand

**Change:** Allow `.variant` when expected type is known from context.

```zt
const dir: Direction = .north
const color: Color = .rgb(255, 0, 0)
```

### T-15: Closure return type inference

**Change:** Closure return type can be omitted when inferable from body.

```zt
const doubled = numbers.map(func(x: int)
    return x * 2
end)
```

The checker collects types from all `return` statements and unifies them.

### T-16: Single-expression closures

**Change:** Closures with a single expression body use implicit return without `end`.

```zt
const doubled = numbers.map(func(x: int) x * 2)
const evens = numbers.filter(func(x: int) x % 2 == 0)
```

This replaces the `=>` lambda syntax (`func(x: int) => x * 2`).

## Deferred (diálogo futuro, pre-1.0)

| Feature | Notes |
|---------|-------|
| Full type inference (`const x = 42`) | Requires inference engine |
| Generic argument inference (`foo(42)` vs `foo<int>(42)`) | Requires call-site unification |

## Superseded documents

The following spec files are superseded by the unified `docs/spec/language/language-reference.md`:

- `docs/spec/language/surface-syntax.md`
- `docs/spec/language/closures.md`
- `docs/spec/language/dyn-dispatch.md`
- `docs/spec/language/callables.md`

## Consequences

- Language surface gains: `any`, `some`, `capture`, `given` (contextual), `then` (contextual), inline constraints `<T: Trait>`, struct literal shorthand, enum dot shorthand, single-expression closures, closure return type inference.
- Language surface loses: `dyn`, `default` (in match), 6 collection keywords.
- Net keyword change: -5 reserved keywords, +2 contextual keywords.
- All documentation consolidated into `docs/spec/language/language-reference.md`.
