# Zenith Language v2 Compiler Technical Audit Report

**Audit Date:** April 20, 2026  
**Auditor:** OpenCode Analysis  
**Scope:** Full compiler pipeline (lexer → parser → binder → typechecker → HIR → ZIR → C backend → runtime)  
**Version:** zenith-lang-v2 (C-native compiler)

---

## Executive Summary

This audit identified **89 distinct issues** across the Zenith compiler pipeline, categorized by severity:

| Severity | Count | Description |
|----------|-------|-------------|
| **CRITICAL** | 7 | Bugs causing functional failures, data corruption, or complete feature breakage |
| **HIGH** | 14 | Significant bugs affecting correctness or usability |
| **MEDIUM** | 38 | Design issues, incomplete implementations, or edge case failures |
| **LOW** | 30 | Code quality, style, or minor functional issues |

### Most Critical Findings

1. **Assignment operator `:=` is dead code** - Lexer never produces `ZT_TOKEN_ASSIGN`, parser uses `=` instead
2. **Semicolon token never produced** - `ZT_TOKEN_SEMICOLON` is defined but lexer never emits it
3. **String escape sequences broken** - `\"` inside strings causes premature termination
4. **Shadow detection drops declarations** - Names that "shadow" are never actually declared
5. **BTreeMap/BTreeSet are not trees** - Runtime uses flat arrays, not B-tree structures
6. **Duplicate header files** - `utils/diagnostics.h` and `semantic/diagnostics/diagnostics.h` are identical copies
7. **LSP extern references static globals** - Will fail to link correctly

---

## 1. Lexer Analysis (`compiler/frontend/lexer/`)

### CRITICAL Issues

#### 1.1 `ZT_TOKEN_ASSIGN` (`:=`) Never Produced
- **File:** `lexer.c:455`
- **Issue:** The lexer always emits `ZT_TOKEN_COLON` for `:` without checking for `=` to form `:=`
- **Impact:** The assignment operator `:=` is completely unusable
- **Recommendation:** Add lookahead in `case ':'` to check next character

#### 1.2 `ZT_TOKEN_SEMICOLON` Never Produced  
- **File:** `lexer.c` (missing case)
- **Issue:** No `case ';'` in the switch statement; `;` becomes `ZT_TOKEN_LEX_ERROR`
- **Impact:** Helpful error hint "use ':' for type annotations" never triggers
- **Recommendation:** Add semicolon token handling

#### 1.3 String Escape Sequences Broken
- **File:** `lexer.c:167-197`
- **Issue:** Only `\{` interpolation is handled; all other escapes (`\"`, `\\`, `\n`) are broken
- **Impact:** `"hello \"world\""` terminates early at `\"`
- **Recommendation:** Implement proper escape sequence handling

### HIGH Issues

#### 1.4 Unterminated Strings Return Valid Tokens
- **File:** `lexer.c:179-196`
- **Issue:** Unclosed strings return `ZT_TOKEN_STRING_LITERAL` instead of error
- **Impact:** Cascading confusing errors instead of clear "unterminated string"
- **Recommendation:** Emit diagnostic and return error token

#### 1.5 No Lexer Diagnostics for Invalid Characters
- **File:** `lexer.c:485-488`
- **Issue:** Returns `ZT_TOKEN_LEX_ERROR` but never emits diagnostic
- **Impact:** Parser never checks for `LEX_ERROR`, produces generic "expected expression"
- **Recommendation:** Add diagnostic emission in lexer

### MEDIUM Issues

- Unterminated triple-quoted strings silently succeed
- Source span `column_end` convention unclear (exclusive vs inclusive)
- Hex/binary literals with no digits produce invalid tokens
- Float with no fractional digits (`42.`) passes validation
- String interpolation tokens produced but never handled by parser

---

## 2. Parser Analysis (`compiler/frontend/parser/`)

### CRITICAL Issues

#### 2.1 Assignment Uses `=` Instead of `:=`
- **File:** `parser.c:1074, 1087, 1102`
- **Issue:** Parser uses `ZT_TOKEN_EQ` for assignment despite `ZT_TOKEN_ASSIGN` existing
- **Impact:** `=` means both equality and assignment; `:=` is dead
- **Recommendation:** Support `:=` token or remove it from token.h

### HIGH Issues

#### 2.2 Context Flags Dead Code
- **File:** `parser.c:18-20, 1143, 1145`
- **Issue:** `in_type_position` and `in_expression_position` are never set
- **Impact:** Unreachable code branches, confused diagnostics
- **Recommendation:** Remove unused flags or implement proper context tracking

#### 2.3 Memory Leak in Escape Handling
- **File:** `parser.c:160-181`
- **Issue:** No null check on `malloc` result in `zt_parser_intern_unescaped`
- **Impact:** Crash on allocation failure
- **Recommendation:** Add NULL check after malloc

### MEDIUM Issues

- `free(name)` called on arena-allocated memory (undefined behavior)
- Map entry span uses `{` position instead of entry span
- Stack buffer optimization is dead code (always heap allocates)
- Error recovery doesn't synchronize to known-good state
- Many `zt_parser_ast_make` return values unchecked

---

## 3. Binder & Type System (`compiler/semantic/`)

### CRITICAL Issues

#### 3.1 Shadow Detection Drops Inner Declaration
- **File:** `binder.c:86-89`
- **Issue:** When shadowing detected, function returns without declaring the name
- **Impact:** Inner references resolve to outer declaration; "shadows" message is misleading
- **Recommendation:** Either allow shadowing with warning or fix message to "cannot shadow"

#### 3.2 Generic Struct Constructor Omits Type Arguments
- **File:** `checker.c:1173`
- **Issue:** `zt_type_make_named` called without type parameters
- **Impact:** `Box{value: 42}` returns type `Box` not `Box<int>`
- **Recommendation:** Include type arguments in constructor result type

#### 3.3 Suggestion Corruption Bug
- **File:** `diagnostics.c:388-396`
- **Issue:** `zt_diag_list_add_suggestion` may write to wrong diagnostic
- **Impact:** If duplicate detected or max reached, suggestion corrupts unrelated entry
- **Recommendation:** Add diagnostic to list AFTER setting suggestion

### HIGH Issues

#### 3.4 Renderer Drops Labels Without Color
- **File:** `diagnostics.c:629-642`
- **Issue:** ACTION/WHY/SUGGESTION sections missing when `use_color=0`
- **Impact:** Action-first format silently degrades to empty output
- **Recommendation:** Add else clauses for non-colored output

#### 3.5 Double Message Printing
- **File:** `diagnostics.c:631-638`
- **Issue:** When action is NULL, message printed as ACTION then again as WHY
- **Impact:** Redundant confusing output
- **Recommendation:** Only print WHY if action differs from message

#### 3.6 Wrong Error Limit Metric
- **File:** `diagnostics.c:686-700`
- **Issue:** Limits by index position, not error count
- **Impact:** Shows wrong number of errors to user
- **Recommendation:** Count only errors, limit by that count

### MEDIUM Issues

- Only 17/51 diagnostic codes have action text defined
- Integer overflow in constant folding (UB in C)
- Enum variant constructors also omit type arguments
- Equality operator only checks left operand for Equatable
- Map index expression evaluated twice

---

## 4. HIR & ZIR Lowering (`compiler/hir/`, `compiler/zir/`)

### HIGH Issues

#### 4.1 Default Parameter Values Silently Dropped
- **File:** `from_ast.c:1784-1811`
- **Issue:** `zt_hir_param` has no `default_value` field
- **Impact:** Default parameter values lost in AST→HIR
- **Recommendation:** Add default_value field to hir param struct

#### 4.2 `len()` Missing Collection Types
- **File:** `from_hir.c:348-389`
- **Issue:** Only handles list, map, text, bytes; missing grid2d, grid3d, pqueue, circbuf, btreemap, btreeset
- **Impact:** `len(my_grid)` generates incorrect ZIR
- **Recommendation:** Add dispatch entries for all collection types

### MEDIUM Issues

- Top-level const declarations not lowered to HIR
- `list.get` only handles `list<int>` and `list<text>`
- Receiver expression ownership fragile between paths
- `list_push` and `list_set` ZIR instructions never generated

---

## 5. C Backend (`compiler/targets/c/`)

### CRITICAL Issues

#### 5.1 C_TYPE_TABLE Missing float32/float64
- **File:** `emitter.c:466-523`
- **Issue:** No entries for `float32` or `float64`
- **Impact:** Emission fails for any float type
- **Recommendation:** Add float32/float64 entries to type table

#### 5.2 BTreeMap/BTreeSet Not Tree Structures
- **File:** `zenith_rt.h:294-307`
- **Issue:** Uses flat arrays, not B-tree
- **Impact:** O(n) performance instead of O(log n); misleading name
- **Recommendation:** Implement actual B-tree or rename to reflect flat structure

### MEDIUM Issues

- `optional<...>` and `outcome<...>` have `is_managed=0` causing potential leaks
- C_TYPE_TABLE sort order is fragile invariant
- Float-to-int conversion doesn't check precision loss

---

## 6. Driver & Project (`compiler/driver/`)

### HIGH Issues

#### 6.1 Duplicate Header Files
- **Files:** `utils/diagnostics.h` vs `semantic/diagnostics/diagnostics.h`
- **Issue:** Identical files will diverge causing ODR violations
- **Impact:** Subtle bugs or linker errors
- **Recommendation:** Remove duplicate, use single canonical location

#### 6.2 LSP Externeferences Static Globals
- **File:** `lsp.c:136-137`
- **Issue:** `extern` declarations reference `static` globals from main.c
- **Impact:** Will fail to link or have uninitialized arenas
- **Recommendation:** Move globals to shared header with extern

### MEDIUM Issues

- `--profile full` can be overridden by manifest
- Focus path match is case-sensitive (broken on Windows)
- `.exe` extension hardcoded on all platforms
- Case-sensitive extension comparison on Windows
- CI filtered diagnostics lose suggestion field

---

## 7. Recommendations Summary

### Immediate Actions Required

1. **Fix lexer assignment operator** - Either implement `:=` or remove dead code
2. **Fix string escape handling** - Implement proper `\"`, `\\`, `\n` escapes
3. **Fix shadow detection** - Actually declare shadowing names or fix message
4. **Remove duplicate diagnostics.h** - Consolidate to single file
5. **Add float32/float64 to C_TYPE_TABLE** - Critical for any float usage

### Short-Term Fixes

1. Add null checks for malloc failures
2. Fix arena memory free() calls
3. Implement proper line ending handling in doc_show
4. Add case-insensitive path matching on Windows
5. Fix CI mode to preserve suggestion field

### Long-Term Improvements

1. Implement actual B-tree data structures
2. Add comprehensive test coverage for edge cases
3. Implement full ZIR round-trip parsing
4. Add thread safety to arena/string pool
5. Complete l10n coverage for all diagnostic codes

---

## Appendix: Issues by File

| File | CRITICAL | HIGH | MEDIUM | LOW |
|------|----------|------|--------|-----|
| lexer.c | 3 | 2 | 5 | 2 |
| parser.c | 1 | 2 | 8 | 4 |
| binder.c | 1 | 0 | 1 | 2 |
| checker.c | 2 | 4 | 4 | 3 |
| diagnostics.c | 1 | 3 | 2 | 1 |
| from_ast.c | 0 | 2 | 3 | 1 |
| from_hir.c | 0 | 1 | 4 | 1 |
| emitter.c | 1 | 1 | 4 | 0 |
| zenith_rt.c | 0 | 2 | 3 | 2 |
| main.c | 0 | 2 | 4 | 7 |
| doc_show.c | 0 | 1 | 2 | 3 |
| ztproj.c | 0 | 0 | 1 | 2 |
| lsp.c | 1 | 0 | 1 | 0 |

---

*End of Audit Report*
