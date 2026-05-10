# Decision 033 - Language Philosophy And Manifesto

- Status: accepted
- Date: 2026-04-17
- Type: language / philosophy / product
- Scope: language identity, accessibility, readability, neurodivergent-friendly design, syntax principles

## Summary

Zenith is a reading-first, systems-capable language focused on explicit intent, predictable behavior and compositional design.

Zenith is designed with TDAH and dyslexia-friendly principles as a core requirement, not as an afterthought.

The language should reduce cognitive noise, preserve visual stability and make technical intent easier to read, review, debug and teach.

## Canonical Statement

English:

> Zenith is a reading-first systems-capable language focused on explicit intent, predictable behavior, and compositional design.

Portuguese:

> Zenith e uma linguagem reading-first com capacidade de sistemas, focada em intencao explicita, comportamento previsivel e design composicional.

## Personal Mission And Inclusion

Zenith was born from the need to study compilers and tooling deeply without turning every step into a fight against the tool itself.

As a project designed by and for people with TDAH and dyslexia, Zenith commits to the idea that clarity, predictability and welcome are fundamental to the programming experience.

Accessibility is not a detail. It is the foundation.

Zenith should prove that neurodivergent minds can build ambitious technical systems when the environment reduces noise and respects human processing rhythm.

## Central Thesis: Readability As Infrastructure

Most modern languages frequently sacrifice clarity in favor of extreme concision.

Zenith inverts that logic through three principles:

- Low Neural Friction
- Visual Stability
- Explicit Over Implicit

These principles are not cosmetic. They are part of the language architecture.

## Low Neural Friction

The programmer should spend energy solving the problem, not decoding syntax.

Zenith prefers explicit keywords and predictable vertical flow:

```zt
match result
case success(value):
    return value
case error(message):
    return 0
end
```

Keywords such as `end`, `match`, `where`, `namespace`, `trait` and `apply` are intentional.

They make block structure and semantic roles visible without requiring the reader to count braces or mentally infer layout from punctuation density.

## Visual Stability

Mathematical symbols and operators should appear only when semantically necessary.

Zenith remains predominantly textual and readable to avoid the visual noise and letter-dance effect that can make dense symbolic syntax harder for dyslexic readers.

Accepted examples:

```zt
func calculate_total(price: int, quantity: int) -> int
    return price * quantity
end
```

```zt
hp: int where it >= 0
```

Rejected direction:

```zt
fn c<T>(x:T)=>x?.m()?.v??0
```

The goal is not to ban symbols. The goal is to prevent symbol density from becoming the dominant language texture.

## Explicit Over Implicit

Zenith avoids magical behavior.

The reader should not need to know hidden conversions, overload ranking, macro expansion rules or implicit runtime hooks to understand ordinary code.

Preferred direction:

```zt
const user: User = load_user(id)?
```

because `?` has one explicit semantic meaning: propagation.

Non-preferred direction:

```zt
const message: text = "hp: " + player.hp
```

if it depends on implicit conversion from `int` to `text`.

Text conversion should be explicit through language-defined or stdlib-defined APIs.

## Reading-First

Zenith optimizes for the reader over the typist.

A few extra words are acceptable when they prevent ambiguity.

Preferred:

```zt
const total: int = price * quantity
```

instead of relying on broad local type inference.

Function signatures should make intent visible:

```zt
func transfer(
    amount: int where it > 0,
    from_balance: int where from_balance >= amount
) -> int
    return from_balance - amount
end
```

The formatter may help maintain this style, especially for long signatures and `where` clauses.

## Explicit But Not Ceremonial

Zenith should be explicit without becoming bureaucratic.

The language should not require boilerplate when a compact form has one clear meaning.

Accepted example:

```zt
const raw: text = read_file("user.txt")?
```

This is preferable to an overly ceremonial equivalent:

```zt
const raw: text = Result.unwrap_or_return(read_file("user.txt"))
```

The guiding rule is:

- explicit semantic intent is good
- repeated ceremony without added meaning is bad

## Composition Over Classical OOP

Zenith does not use classical object-oriented inheritance as its central model.

Zenith uses:

- `struct` for data
- `enum` for states and variants
- `trait` for behavior contracts
- `apply` for attaching behavior
- functions and namespaces for organization

Canonical direction:

```zt
struct Player
    name: text
    hp: int
end

trait Healable
    mut func heal(amount: int)
end

apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Non-goal direction:

```zt
class Player extends Character implements Healable
    ...
end
```

Zenith should not grow classes, inheritance chains or hidden virtual dispatch as its default design model.

## Predictability Over Cleverness

Zenith should prefer behavior that can be predicted locally.

This affects accepted decisions:

- variables use explicit types
- `const` means observable immutability
- mutating methods use `mut func`
- `?` has one meaning: propagation
- `where` is a contract, not automatic `result`
- `optional` replaces null
- map indexing is strict
- safe lookup is explicit
- generics may infer calls only when unambiguous
- overloads and macros stay out of the MVP

## Error Semantics

Zenith keeps failure categories separate:

- `result<T, E>` means expected recoverable failure
- `optional<T>` means expected absence
- contract panic means invariant violation
- compile-time diagnostic means statically invalid program

These categories should not be silently converted into each other.

Example:

```zt
const user: User = find_user(id).or_error("user not found")?
```

This future form is preferred over implicit optional-to-result conversion.

## Mutability Visibility

Mutation should be visible.

Canonical forms:

```zt
const name: text = "Julia"
var hp: int = 100
```

```zt
player.heal(10)
```

Mutation intent is explicit in method declarations through `mut func`, making write-capable behavior visible without punctuation markers.

`const` collections are observably immutable. A `const list<Player>` does not allow changing a player through the list.

## Syntax Sugar Policy

Zenith can have syntax sugar, but only when it is:

- limited
- documented
- semantically transparent
- readable
- aligned with the core philosophy

Accepted direction:

```zt
update players[0] as player
    player.hp = 80
end
```

because it is transparent sugar for read-modify-write.

Non-goal direction:

```zt
players[0].hp++
```

because it hides too much mutation behind dense symbolic syntax and requires deeper aliasing semantics.

## What Zenith Is

Zenith is:

- explicit
- readable
- predictable
- accessible
- compositional
- strongly typed
- compiled
- target-oriented
- friendly to neurodivergent readers
- suitable for learning compilers and building real tooling
- pragmatic about systems-capable implementation through C and later targets

## What Zenith Is Not

Zenith is not:

- a code-golf language
- a JavaScript/Python/Rust/Go clone
- a classical OOP language
- a macro-heavy language
- a language of aggressive implicit inference
- a language where stdlib APIs hide critical semantics
- a language that treats accessibility as optional polish
- a language that optimizes for shortest possible code at the cost of readability

## Design Consequences

Future design decisions should be checked against these questions:

- Does this reduce or increase cognitive noise?
- Is the behavior visible at the call site or declaration site?
- Does this help a dyslexic or TDAH reader keep context?
- Does this preserve predictable vertical reading flow?
- Does this introduce multiple meanings for the same symbol?
- Does this require hidden inference or magic?
- Can diagnostics explain this clearly?
- Can the formatter make this easier to scan?
- Does this fit composition over inheritance?

If a feature is powerful but violates several of these questions, it should be deferred or redesigned.

## Rationale

A language's philosophy must be explicit because otherwise implementation convenience will silently become language design.

Zenith's technical ambition depends on controlling complexity, not pretending it does not exist.

By treating readability and accessibility as infrastructure, Zenith can support deeper technical work while keeping the source language approachable and stable.

## Out of Scope

This decision does not yet define:

- concrete formatter rules
- complete naming convention tables
- final stdlib API names
- IDE visual design rules
- documentation template rules
- teaching material structure
- package ecosystem policy


