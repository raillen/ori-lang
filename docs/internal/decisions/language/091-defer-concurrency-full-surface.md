# Decision 091 - Defer Full Concurrency Surface Out Of R3

Status: accepted
Date: 2026-04-24
Scope: R3.M2, R3.M9, R4 backlog

## Decision

R3 ships only the Phase 1 concurrency base.

The rest of the original R3.M2 scope is formally moved out of R3.

Moved scope:

- `task`;
- `channel`;
- `Shared<T>`;
- race runtime tests;
- cancellation runtime tests;
- full checker-level `transferable` predicate.

## Why

These items are not small release-polish tasks.

They affect:

- runtime scheduling;
- ownership boundaries;
- managed-memory safety;
- cancellation behavior;
- test determinism;
- `public var` under concurrent execution.

Shipping them inside the R3 release closeout would create avoidable risk.

## What R3 Still Guarantees

R3 includes:

- the authoritative concurrency spec;
- boundary-copy helpers;
- deterministic copy behavior;
- namespace-state rules for `public var`;
- no implicit global state;
- no implicit cross-thread sharing.

## Where The Work Moves

The moved scope becomes `R4.CF1 - Concurrency Full Surface`.

`R4.CF1` is a planned backlog track, not a hidden R3 blocker.

## Release Impact

R3.M9 is no longer blocked by the missing `task`/`channel`/`Shared<T>` implementation.

R3.M9 is still blocked by the remaining release gates:

- PR gate failures;
- nightly perf gate failures;
- any unresolved P0 without explicit acceptance.

## Non-Goals

This decision does not remove concurrency from the language direction.

It only prevents a rushed concurrency runtime from being bundled into the R3 release.
