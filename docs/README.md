# Ori Language Documentation

This directory contains all documentation for the Ori programming language.

## Structure

`
docs/
 spec/           # Formal language specification (chapters 0113)
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
`

## Spec Status

The spec/ directory is the **source of truth** for the Ori language.
All compiler implementation decisions must be consistent with these documents.

Public documentation (tutorials, cookbook, language reference for users)
will be written after the compiler reaches a working state.
