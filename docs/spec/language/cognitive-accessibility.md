# Zenith Cognitive Accessibility Design (Neurodiversity)

- Status: Proposed / Under Discussion
- Scope: Language design, tooling, and diagnostics guidelines to support neurodivergent developers.
- Updated: 2026-04-19

This document defines accessibility-first language and tool behavior for Zenith.
The goal is to reduce cognitive load without reducing technical rigor.

## 0. Delivery Levels (Default vs Optional vs Experimental)

- **Default**: Enabled out of the box. Must be low-friction and stable.
- **Optional**: Disabled by default. Can be enabled per project or user profile.
- **Experimental**: Behind explicit opt-in. Requires measurement before promotion.

Recommended project config:

```toml
[accessibility]
profile = "balanced"      # beginner | balanced | full
emoji = true               # keep icon + emoji cues in diagnostics
plain_output = false       # if true, render pure text output
locale = "en"             # can be overridden by CLI --lang
```

## 1. Working Memory and Context Resumption

This section addresses cold starts, task switching, and context loss.

### 1.1. Context Anchors and `zt summary` / `zt resume`

- **Challenge:** Losing track of the current objective after interruption.
- **Feature (Default):** Native comment anchors.
- `-- FOCUS: <message>` means current objective.
- `-- NEXT: <message>` means immediate next action.
- **Tooling (Default):** `zt summary` prints a short project briefing.
- **Tooling (Optional):** `zt resume` reconstructs context from recent edits and diagnostics.

Example:

```text
📌 Project: my-game (app)
🕒 Last modified: src/physics.zt (2h ago)
🎯 FOCUS: Implementing circular collision (physics.zt:45)
➡️ NEXT: Test with larger sprites (render.zt:120)
```

### 1.2. Progressive Disclosure of Diagnostics

- **Challenge:** A wall of errors causes paralysis.
- **Feature (Default):** Limit errors shown in one pass by profile.

Profiles:

- `beginner`: show max 3 errors.
- `balanced`: show max 5 errors.
- `full`: show all errors.

Behavior example:

```text
... and 12 more errors. Run `zt check --all` to show everything.
```

CLI and config:

```bash
zt check --profile beginner
zt check --all
```

```toml
[diagnostics]
profile = "balanced"
```

### 1.3. Scoped Checks to Reduce Context Drift

- **Challenge:** Large outputs break focus when working in one module.
- **Feature (Optional):** `zt check --focus <path-or-module>`.
- **Feature (Optional):** `zt check --since <git-ref>` for recent changes only.

## 2. Visual Distinction and Reading Predictability

This section addresses transposition, crowding, and symbol confusion.

### 2.1. Confusable Names Linter and Active Help

- **Challenge:** Similar names increase error probability.
- **Feature (Default):** Warn when names are too similar in same scope.
- Similarity combines:
- Edit distance.
- Visual confusables (`l`, `I`, `1`, `O`, `0`).
- Keyboard-neighbor typos.
- **Active Fix (Default):** `name.unresolved` suggests top candidates.

Example:

```text
Unknown name `pice`.
Did you mean `price`?
```

### 2.2. Numeric Legibility (Dyscalculia Support)

- **Challenge:** Long digit runs increase mental effort.
- **Feature (Default warning):** Recommend `_` separators for long literals.
- **Quick Fix (Optional):** Auto-format numeric separators.

Examples:

- Bad: `1000000`, `0xFFCC00`
- Good: `1_000_000`, `0xFF_CC_00`

### 2.3. Native `zt spellcheck`

- **Challenge:** Identifier/comment typos create extra debugging loops.
- **Feature (Optional):** Built-in spell check for comments and split identifiers.
- Language follows `--lang` or project config.

### 2.4. Typography and Layout Guidance (Tooling)

- **Challenge:** Readability depends on spacing, contrast, and layout stability.
- **Feature (Optional):** `zt doctor accessibility` checks editor and terminal hints.
- Focus on user-adjustable readability settings instead of forced fonts.
- Useful knobs:
- Font size.
- Line height.
- Line length.
- Contrast.

## 3. Sensory Load and Emotional Friction

This section addresses overstimulation, ambiguity, and emotional cost.

### 3.1. Low-Sensory Diagnostic Themes

- **Challenge:** Visual noise slows parsing and increases fatigue.
- **Feature (Default):** Respect `NO_COLOR` and `--no-color`.
- **Feature (Optional):** `--plain` removes decoration and keeps concise text.
- **Policy (Default):** Color is never the only carrier of meaning.

### 3.2. Diagnostic Effort Hints + Action-First Format

- **Challenge:** Unclear effort level increases frustration.
- **Feature (Optional):** Effort hints in diagnostics.
- `⚡ quick fix` for local typo or import issues.
- `🔧 moderate` for local type or API mismatch.
- `🧩 requires thinking` for architecture-level constraints.
- **Feature (Default):** First line always states next action.

Example:

```text
📌 ACTION: Rename `pice` to `price`.
ℹ️ WHY: `pice` is unresolved in this scope.
🧭 NEXT: Run `zt check --focus src/pricing.zt`.
```

### 3.3. Accidental Complexity Guardrails (Soft Limits)

- **Challenge:** Deep nesting and long methods overload working memory.
- **Feature (Default warnings, suppressible):**
- Function length threshold.
- Nesting depth threshold.
- Chained operator threshold.
- Cognitive complexity threshold.

Suggested defaults:

- Function length: 50 lines.
- Nesting depth: 4.
- Chained operators: 3.
- Cognitive complexity: 25.

### 3.4. Explicit over Implicit (One Canonical Way)

- Zenith favors explicit structure and deterministic formatting.
- Canonical formatting (`zt fmt`) reduces decision fatigue.
- The language should avoid equivalent syntax forms with different style burden.

## 4. Linguistic Simplicity

This section addresses comprehension barriers in learning and debugging.

### 4.1. `zt explain <code-id>` with Quick and Deep Modes

- **Challenge:** Jargon blocks fast understanding.
- **Feature (Default):** `zt explain <code-id> --quick` for short, action-oriented help.
- **Feature (Optional):** `zt explain <code-id> --deep` for expanded explanation and examples.
- Explanations use stable diagnostic IDs and localization.

### 4.2. Plain Language Documentation Protocol

For every core feature and error code:

- Use short sentences.
- Use active voice.
- Use one idea per paragraph.
- Use concrete examples before edge cases.
- Use simple ASCII diagrams when needed.

### 4.3. Documentation Page Template

- Start with a 1-sentence summary.
- Add "When to use".
- Add "Common mistakes".
- Add "Quick fix steps".
- Add "Related codes".

## 5. Measurement and Validation Plan

The project should validate impact with data, not only opinions.

### 5.1. Core Metrics

- Time to first successful fix per diagnostic code.
- Repeated error rate per diagnostic code.
- Session abandonment rate after first error burst.
- Number of errors resolved per focused pass.
- Self-reported friction after debug session (short scale).

### 5.2. Evaluation Method

- Use opt-in telemetry where possible.
- Run A/B tests for new diagnostic formats.
- Validate across beginner and experienced developers.
- Include neurodivergent participants in testing panels.

## 6. Evidence Notes (Confidence Levels)

### 6.1. Strong Evidence

- Task interruption and context switching harm software productivity.
- Lower perceptual load can improve IDE performance.
- Error identification plus correction suggestions improve accessibility.

### 6.2. Medium Evidence

- Naming quality strongly affects comprehension.
- Identifier style and spacing can change reading effort.
- Neurodivergent developers report specific friction points in SE workflows.

### 6.3. Mixed Evidence

- Enhanced error wording helps some contexts but not all outcomes.
- Specialized dyslexia fonts show inconsistent gains across studies.

## 7. References and Research Sources

### Standards and Guidelines

1. W3C COGA Design Guide: https://www.w3.org/TR/2020/WD-coga-usable-20201211/design_guide.html
2. WCAG 2.1 Understanding SC 3.3.3 Error Suggestion: https://www.w3.org/WAI/WCAG21/Understanding/error-suggestion.html
3. WCAG 2.2 Understanding SC 1.4.1 Use of Color: https://www.w3.org/WAI/WCAG22/Understanding/use-of-color
4. NO_COLOR standard: https://no-color.org/

### Software Engineering and Neurodiversity

5. Task Interruption in Software Development Projects (arXiv:1805.05508): https://arxiv.org/abs/1805.05508
6. Two Sides of the Same Coin: Task Switching and Interruption (arXiv:1805.05504): https://arxiv.org/abs/1805.05504
7. Challenges, Strengths, and Strategies of Software Engineers with ADHD (arXiv:2312.05029): https://arxiv.org/abs/2312.05029
8. The Effect of Perceptual Load on Performance within IDE in People with ADHD Symptoms (arXiv:2302.06376): https://arxiv.org/abs/2302.06376
9. Differences between Neurodivergent and Neurotypical Software Engineers (arXiv:2506.03840): https://arxiv.org/abs/2506.03840
10. An Empirical Investigation of the Experiences of Dyslexic Software Engineers (arXiv:2511.00706): https://arxiv.org/abs/2511.00706
11. Get Me In The Groove (mixed methods, PDF): https://kaianew.github.io/GetMeInTheGroove.pdf

### Code Readability and Identifier Studies

12. An Eye Tracking Study on camelCase and under_score Identifier Styles (ICPC 2010, PDF): https://www.cs.kent.edu/~jmaletic/papers/ICPC2010-CamelCaseUnderScoreClouds.pdf
13. Eye tracking analysis of computer program comprehension in programmers with dyslexia (Empirical Software Engineering): https://link.springer.com/article/10.1007/s10664-018-9649-y

### Dyslexia Font Evidence (Mixed Results)

14. The effect of a specialized dyslexia font, OpenDyslexic, on reading rate and accuracy (Annals of Dyslexia): https://link.springer.com/article/10.1007/s11881-016-0127-1
15. Dyslexie font does not benefit reading in children with or without dyslexia (Annals of Dyslexia): https://link.springer.com/article/10.1007/s11881-017-0154-6

### Diagnostics and Complexity Tooling Context

16. Clang `-ferror-limit` behavior: https://clang.llvm.org/docs/UsersManual.html
17. GCC `-fmax-errors` documentation: https://gcc.gnu.org/onlinedocs/gcc.pdf
18. clang-tidy cognitive complexity check: https://clang.llvm.org/extra/clang-tidy/checks/readability/function-cognitive-complexity.html

---
**Goal:** Build a language toolchain where cognitive accessibility is a first-class engineering constraint.
