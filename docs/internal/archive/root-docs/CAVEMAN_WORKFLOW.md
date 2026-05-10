# Caveman Workflow for Zenith Projects

We have integrated the **Caveman Extension** into our GSD (Get Shit Done) framework. This workflow is designed for high-speed, low-token iteration in Zenith language development.

## 🦴 What is Caveman?
Caveman is a communication style that removes all filler, articles, and politeness from the agent's output. It preserves 100% of the technical substance but cuts output token usage by ~65%.

## 🚀 New Commands

### 1. `/gsd-caveman [lite|full|ultra|off]`
Manually toggle Caveman mode. 
- Use **full** (default) for standard terse output.
- Use **ultra** for extreme compression (no whitespace, minimal punctuation).
- Use **off** to return to normal polite mode.

### 2. `/gsd-caveman-speedrun <phase>`
A high-velocity workflow that:
1. Activates **Caveman Mode**.
2. Executes the specified GSD phase in **YOLO mode**.
3. Summarizes results and deactivates Caveman Mode.

## 🛠 Integration with Zenith
This workflow is particularly effective for Zenith because:
- **Compiler Development**: Terse debugging output saves context space during long build/test cycles.
- **Large Test Suites**: Speedrun mode allows executing many small edge-case tests without constant manual confirmation.
- **Context Preservation**: By saving tokens, you can stay in the same session longer before hitting context limits.

---

### Example Usage:
```bash
/gsd-caveman-speedrun 5
```

> [!TIP]
> Use Caveman mode during heavy refactoring phases in `zenith-lang-v2` to keep the context focused on AST and Binder logic rather than conversational filler.
