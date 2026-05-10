# Decision 035 - Evaluation Order And Temporaries

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / implementation
- Scope: expression evaluation order, temporary lifetime, cleanup, early return, C backend lowering

## Summary

Zenith evaluates expressions left to right.

Operator precedence defines grouping, but it does not permit hidden reordering of observable evaluation.

Temporary values live until the end of the current statement unless they are bound or transferred into a longer-lived value.

The backend C emitter must not depend on C's unspecified/undefined evaluation ordering. It must materialize temporaries when needed to preserve Zenith semantics.

## Decision

Canonical evaluation direction:

```zt
const result: int = a() + b() * c()
```

Precedence groups this as:

```zt
a() + (b() * c())
```

Evaluation order is still:

1. `a()`
2. `b()`
3. `c()`
4. multiply `b * c`
5. add `a + result`

Normative rules for this cut:

- expression evaluation is left-to-right
- precedence controls grouping, not observable evaluation reordering
- function call arguments are evaluated in written order
- method receivers are evaluated before method arguments
- field/index/call chains evaluate from left to right
- named call arguments are evaluated in written/declaration order
- default argument expressions are evaluated at call time
- default argument expressions are evaluated after provided earlier arguments
- temporaries live until the end of the current statement
- values bound to `const` or `var` live until the end of their lexical scope
- block-local values are released when leaving the block
- early return releases live values that must be cleaned up before leaving the function
- `?` propagation releases temporaries and live values according to the same early-return cleanup rules
- contract panic is fatal, but the semantic direction is ordered cleanup when viable
- backend C must emit code that preserves Zenith order explicitly

## Function Calls

Arguments are evaluated in written order:

```zt
save(load_name(), load_age())
```

Order:

1. `load_name()`
2. `load_age()`
3. `save(...)`

For named arguments, Zenith already requires declaration/written order in the MVP:

```zt
open_file(path: get_path(), mode: get_mode())
```

Order:

1. `get_path()`
2. `get_mode()`
3. `open_file(...)`

## Default Arguments

Default expressions are evaluated at call time.

```zt
func open_file(path: text, mode: text = default_mode()) -> File
    ...
end

const file: File = open_file(get_path())
```

Order:

1. `get_path()`
2. `default_mode()`
3. `open_file(...)`

Defaults are not evaluated at declaration time.

## Method Calls And Chains

Receiver first:

```zt
const name: text = user.profile().display_name()
```

Order:

1. evaluate `user`
2. call `profile()`
3. call `display_name()`
4. bind `name`

Future method chaining must preserve the same rule.

## Temporary Lifetime

Temporaries live until the end of the current statement:

```zt
const size: int = len(make_list())
```

The list produced by `make_list()` stays valid through the `len(...)` call and is released at the end of the statement.

Another example:

```zt
save_text(make_text())
```

The text produced by `make_text()` lives until `save_text(...)` completes and the statement ends.

## Bound Values

A temporary bound to a variable becomes the variable's value:

```zt
const name: text = make_name()
```

The produced `text` is now owned by `name` and lives until `name` leaves scope.

It is not released as an independent temporary at the end of the statement.

## Block Cleanup

Block-local values are released when leaving the block:

```zt
if condition
    const temp: text = make_text()
    print(temp)
end
```

`temp` is released after the `if` block finishes.

Loops release values declared inside each iteration according to block scope rules.

## Early Return

Live managed values must be cleaned up before an early return leaves the function.

```zt
func example(condition: bool) -> int
    const name: text = make_name()

    if condition
        return len(name)
    end

    return 0
end
```

If the first `return` runs, `name` is released before the function exits.

## Question Propagation

Statement-level `?` follows early-return cleanup rules.

```zt
const raw: text = read_file(path)?
const user: User = parse_user(raw)?
```

For each statement:

- evaluate left to right
- if the value propagates failure/absence, clean up relevant temporaries and live values
- return early through the current function's `result` or `optional` channel
- otherwise bind the extracted value

Future expression-level `?`, if added, must preserve left-to-right order:

```zt
const total: int = parse(a)? + parse(b)?
```

Potential future order:

1. `parse(a)?`
2. return early if needed
3. `parse(b)?`
4. return early if needed
5. add

Expression-level `?` remains out of scope for the current executable cut.

## Contract Panic

Contract panic is fatal.

Example:

```zt
const player: Player = Player(name: "Julia", hp: -1)
```

If field contract validation fails, the runtime reports a contract panic.

The semantic direction is ordered cleanup where viable. However, because contract panic is fatal, the first C backend implementation may choose direct abort behavior on fatal paths while non-fatal early returns must preserve cleanup.

## Backend C Requirement

C does not guarantee the same evaluation order for all expression forms.

The C backend must not emit code that relies on C evaluation order when Zenith order matters.

For example, this Zenith expression:

```zt
const result: int = a() + b()
```

may need to lower conceptually to:

```c
zt_int tmp_a = a();
zt_int tmp_b = b();
zt_int result = tmp_a + tmp_b;
```

rather than depending on the C compiler's order for evaluating operands.

This rule is especially important for:

- function calls with side effects
- methods declared with `mut func`
- temporaries requiring release
- `?` propagation
- contract checks
- collection updates
- future method chaining

## Rationale

Left-to-right evaluation is easier to read, debug and teach.

It aligns with Zenith's reading-first philosophy: source order should match mental execution order.

Temporary lifetime until statement end gives a simple and safe rule for generated C, managed values and chained calls.

Explicit backend lowering avoids inheriting surprising C behavior.

## Non-Canonical Forms

Assuming C-style unspecified operand order is invalid:

```zt
const result: int = a() + b()
```

`a()` is always evaluated before `b()`.

Assuming temporaries die immediately after their subexpression is invalid:

```zt
const size: int = len(make_list())
```

`make_list()` remains valid until the statement ends.

Assuming defaults are precomputed at declaration time is invalid:

```zt
func open(path: text, mode: text = default_mode())
    ...
end
```

`default_mode()` runs when the call needs the default.

## Diagnostics

Most rules in this decision are semantic/lowering obligations, not ordinary user diagnostics.

Possible diagnostic directions for future invalid features:

```text
expression-level ? is not supported in this implementation cut
move the operation into a const or var declaration
```

```text
default argument expression cannot reference unavailable value
```

## Out of Scope

This decision does not yet define:

- expression-level `?`
- defer/finally blocks
- destructors/custom finalizers
- async/concurrent evaluation ordering
- lazy evaluation
- guaranteed cleanup behavior for fatal abort on all targets
- optimizer rules for reordering pure expressions
