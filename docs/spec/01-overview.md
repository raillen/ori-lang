# Ori Language Specification — Chapter 01: Overview

> Status: normative
> Audience: language designers, compiler implementers, contributors

---

## What Ori Is

Ori is a statically typed, reading-first programming language compiled to native code.

*ori* (אוֹרִי) — Hebrew for "my light."

Ori exists to make programming accessible to people who find mainstream languages
hostile — in particular people with ADHD, autism, and dyslexia. It achieves this
not by being simpler, but by being more honest: every piece of information the reader
needs is visible at the point where it is needed.

---

## What Ori Optimizes

Ori optimizes for **reading**, not writing.

A program is read many more times than it is written. Ori makes each read cheaper:

- **Where a file belongs** — `namespace` is the first declaration in every file.
- **What each value is** — types are explicit at every binding.
- **Where absence can happen** — `optional<T>` is the only representation of absence.
- **Where failure can happen** — `result<T, E>` is the only representation of recoverable failure.
- **When resources are released** — `using` makes cleanup visible and deterministic.
- **When behavior comes from a trait** — `implement` blocks are explicit and named.

---

## Core Design Goals

1. **Explicit over implicit.** If something happens, you can see it in the source.
2. **No surprises.** The reader should be able to predict runtime behavior from syntax alone.
3. **No null.** Absence is modeled as `optional<T>`.
4. **No exceptions as control flow.** Failure is modeled as `result<T, E>`.
5. **Composition over inheritance.** Types are composed with structs, enums, and traits.
6. **Readable diagnostics.** Every error message names what happened, where, and what to do.
7. **Accessible documentation.** Examples are short, syntactically valid, and always up to date.

---

## What Ori Is Not

- Ori is not a scripting language. Programs have explicit structure.
- Ori is not a functional language. It supports functional patterns but is not pure.
- Ori is not an object-oriented language. There are no classes or inheritance.
- Ori is not a systems language in the sense of manual memory management.
  Memory is managed automatically through value semantics and automatic reference counting.

---

## Mental Model

An Ori program is a set of **namespaces**.

Each namespace is a source file. The namespace name is declared first.

```ori
namespace app.inventory

import ori.io as io

public func item_count() -> int
    return 42
end
```

The common path through Ori code:

1. Define data shapes with `struct` and `enum`.
2. Define behavior contracts with `trait`.
3. Attach behavior with `implement Trait for Type`.
4. Return `optional<T>` when a value may be absent.
5. Return `result<T, E>` when an operation may fail.
6. Use `using` to bind resources that need deterministic cleanup.
7. Use `check` for programmer assertions that must hold.

---

## Complete Introductory Example

```ori
namespace app.main

import ori.io as io

struct User
    name: string
    age: int where it >= 0
end

trait Displayable
    func display() -> string
end

implement Displayable for User
    func display() -> string
        return f"{self.name} ({self.age})"
    end
end

func load_user(id: int) -> result<User, string>
    if id < 0
        return error("invalid id")
    end
    return success(User(name: "Ada", age: 36))
end

func main() -> result<void, string>
    const user: User = load_user(1)?
    io.print(user.display())?
    return success()
end
```

---

## Relationship to Zenith

Ori is a new language. It was designed with the lessons of Zenith as its foundation,
but it is not Zenith and is not compatible with Zenith source code.

Key differences from Zenith:

| Zenith | Ori |
|---|---|
| `text` | `string` |
| `apply Trait to Type` | `implement Trait for Type` |
| `func f(mut self)` | `mut func f()` |
| `while true` | `loop` |
| `type Alias = T` | `alias Alias = T` |
| `to_text()` | `to_string()` |
| `TextRepresentable` | `Displayable` |
| Ranges are exclusive (`0..9` = 0–8) | Ranges are inclusive (`0..9` = 0–9) |
| Anonymous functions use `func` | Anonymous functions use `do` |
| `std.*` stdlib namespace | `ori.*` stdlib namespace |

---

## Spec Structure

| Chapter | Title |
|---|---|
| 01 | Overview (this chapter) |
| 02 | Lexical Structure |
| 03 | Grammar (EBNF) |
| 04 | Type System |
| 05 | Expressions |
| 06 | Statements and Control Flow |
| 07 | Functions and Closures |
| 08 | Traits and Implement |
| 09 | Errors and Propagation |
| 10 | Memory and Cleanup |
| 11 | Generics and Constraints |
| 12 | Standard Library Contracts |
| 13 | Diagnostic Error Catalog |
