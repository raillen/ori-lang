# Zenith Backend Scalability Risk Model Spec

- Status: canonical closure spec
- Date: 2026-04-18
- Scope: backend scalability risks for the C target

## Purpose

This spec records scalability risks that must be handled before Zenith can be called stable for larger programs.

The language surface should stay simple, but the compiler must make backend costs visible and controlled.

## Risk Areas

Required tracked risks:

- RC cycles
- weak references or handle policy
- monomorphization code bloat
- stack-vs-heap representation
- cache locality for common value wrappers
- cleanup on early exit
- exhaustive match coverage
- runtime contract lowering

## RC Cycles

The MVP may use reference counting without cycle collection.

This is a known limitation, not an invisible non-issue.

Before official support for rich callbacks, UI graphs, game object graphs or stored references, the compiler/runtime must choose one of:

- `weak<T>`
- runtime handles or arenas
- cycle collection
- constrained ownership graph APIs

Until then, APIs likely to create cycles must be considered unstable or post-MVP.

## Monomorphization

C target generics use monomorphization first.

Required controls:

- canonical type keys
- instance cache
- duplicate instantiation prevention
- recursive instantiation guard
- build report or diagnostic for excessive instantiation count
- tests with nested generic containers

A stable compiler must never generate unbounded duplicate code for the same canonical instantiation.

## Stack-First Value Wrappers

`optional<T>` and `result<T,E>` should be stack/in-place values whenever practical.

Rules:

- scalar wrappers should not allocate
- `result<void,E>` success should not allocate
- managed payloads may contain pointers/RC-managed internals
- wrappers may use heap only when payload size, escape or bootstrap constraints require it
- heap-first implementation is performance debt and must be tracked

## Runtime Contracts

Value-level `where` lowers to runtime checks.

Direct constructors remain direct constructors.

Contracts do not secretly turn constructors into `result`.

Expected validation uses explicit result-returning APIs such as `try_create_*`.

## Exhaustive Match

Enum match without `case else` must be checked for exhaustiveness when the enum definition is known.

Missing enum cases are compile-time diagnostics.

`case else` is allowed, but it intentionally gives up compile-time future-proofing.

## Definition Of Done

This risk model is closed only when:

- roadmap/checklist items refer to these risks
- runtime model includes RC cycle and stack/heap policy
- compiler model includes monomorphization controls
- enum match conformance includes missing-case diagnostics
- where contract conformance proves constructors do not change return type
