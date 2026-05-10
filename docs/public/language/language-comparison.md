# Zenith Language Comparison

> Audience: user
> Status: current
> Surface: public

## Short Position

Zenith aims for code that is explicit, readable, and friendly to long-term maintenance.

It is not trying to be the shortest language.

It is trying to reduce hidden behavior.

## Compared With Python

Python is very fast to write.

Zenith is more explicit:

- local types are written;
- absence uses `optional<T>`;
- failures use `result<T, E>`;
- imports stay qualified.

Choose Zenith when predictable reading matters more than minimal typing.

## Compared With Rust

Rust gives strong ownership control.

Zenith avoids user-facing ownership syntax.

Zenith uses:

- value semantics;
- managed runtime helpers;
- explicit cleanup with `using`;
- explicit transfer boundaries for concurrency.

Choose Zenith when you want safer patterns without making ownership the main teaching surface.

## Compared With Go

Go keeps a small language surface.

Zenith shares that bias, but adds:

- algebraic-style absence and result types;
- traits and `apply`;
- stricter expression readability goals;
- a formatter contract designed around reading.

Choose Zenith when explicit error and absence types are important.

## Compared With TypeScript

TypeScript is strong for web and gradual adoption.

Zenith is not a JavaScript layer.

It is a compiled language with:

- a C backend oracle today;
- explicit project manifests;
- a standard CLI;
- a future path for more backends.

## Compared With C

C gives direct control and portability.

Zenith keeps C as a backend target, but user code gets:

- checked errors;
- checked bounds;
- explicit modules;
- standard formatting;
- safer standard library contracts.

Choose Zenith when you want native output with a more guided source language.

## What Zenith Gives Up

Zenith intentionally rejects or limits some familiar features:

- no null;
- no broad operator overloading;
- no hidden wildcard imports;
- no C-style loop syntax;
- no unstructured exception model;
- no broad macro system in the current public surface.

The tradeoff is lower surprise while reading code.

## Good Fit

Zenith is a good fit for:

- tools;
- data processing;
- package code where examples must stay readable;
- educational compiler/runtime work;
- systems-adjacent apps that benefit from a compact surface.

## Not Yet A Good Fit

Zenith is still young.

Do not choose it yet when you need:

- a large package registry;
- mature IDE polish across every editor;
- production-grade async networking;
- many backend targets today.

Those are tracked as future tooling and ecosystem work.
