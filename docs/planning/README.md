# Ori Language Documentation

This directory contains all documentation for the Ori programming language.

## Structure

`
docs/
 IMPLEMENTATION_CHECKLIST.md    # Current implementation tracker
 IMPLEMENTATION_CHECKLIST_2.md  # Native route 100% backlog
 native-route.md                # Native route and C debug backend contract
 native-abi.md                  # Native backend/runtime ABI and ownership contract
 spec/           # Formal language specification (chapters 01-13)
     01-overview.md
     02-lexical.md
     03-grammar.ebnf
     04-types.md
     05-expressions.md
     06-statements.md
     07-functions.md
     08-traits.md
     09-errors.md
     10-memory.md
     11-generics.md
     12-stdlib.md
     13-error-catalog.md
 planning/       # Technical implementation plans before they become spec
     native-runtime-route-correction-plan.md
     async-implementation-plan.md
`

## Spec Status

The spec/ directory is the **source of truth** for the Ori language.
All compiler implementation decisions must be consistent with these documents.

The planning/ directory is not normative spec. It records implementation routes,
tradeoffs, and migration order before a decision is promoted into spec chapters.

Public documentation (tutorials, cookbook, language reference for users)
will be written after the compiler reaches a working state.
