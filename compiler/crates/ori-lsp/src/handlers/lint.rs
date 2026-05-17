/// Lint checks that augment compiler diagnostics.
///
/// The compiler already reports parse errors, type errors, etc.
/// This module adds *lint* warnings: style and correctness hints
/// that are not compilation errors but are good practice in Ori.

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// Configuration for which lints are enabled.
#[derive(Debug, Clone)]
pub struct LintConfig {
    pub unused_variable: bool,
    pub shadowed_variable: bool,
    pub prefer_const: bool,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            unused_variable: true,
            shadowed_variable: true,
            prefer_const: true,
        }
    }
}

/// Run all enabled lint checks on source code and return LSP diagnostics.
pub fn lint(source: &str, _config: &LintConfig) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // ── unused-variable ─────────────────────────────────────────────────────
    // Walk through lines, look for simple patterns like:
    //   const x = ...  (but 'x' is never referenced again)
    //
    // This is a best-effort pattern — the full check belongs in the compiler
    // type-checker (ori-types). Here we use a simple heuristic.

    let lines: Vec<&str> = source.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();

        // Detect `const <name> =` or `<name> :=` bindings
        if let Some(name) = extract_binding_name(trimmed) {
            // Check if the name appears elsewhere in the file
            let mut ref_count = 0;
            for (j, other) in lines.iter().enumerate() {
                if i == j {
                    continue;
                }
                ref_count += other.matches(&name).count();
            }
            if ref_count == 0 && !name.starts_with('_') {
                let range = range_for_word_on_line(line, &name, i);
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(tower_lsp::lsp_types::NumberOrString::String(
                        "lint.unused_variable".into(),
                    )),
                    source: Some("ori-lint".into()),
                    message: format!(
                        "Variable `{name}` is never used. Prefix with `_` to suppress.",
                    ),
                    ..Default::default()
                });
            }
        }

        // Detect `var <name>` (mutable, could be `const`)
        if let Some(name) = extract_var_binding(trimmed) {
            if !name.starts_with('_') {
                let is_mutated = lines
                    .iter()
                    .any(|l| l.contains(&format!("{name} :=")) || l.contains(&format!("{name} =")));
                if !is_mutated {
                    let range = range_for_word_on_line(line, &name, i);
                    diagnostics.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::HINT),
                        code: Some(tower_lsp::lsp_types::NumberOrString::String(
                            "lint.prefer_const".into(),
                        )),
                        source: Some("ori-lint".into()),
                        message: format!(
                            "`{name}` is never mutated; consider using `const`.",
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }

    diagnostics
}

fn extract_binding_name(line: &str) -> Option<String> {
    let line = line.trim_start();
    // const x = ...
    if let Some(rest) = line.strip_prefix("const ") {
        let name = rest.split(|c: char| c.is_whitespace() || c == ':' || c == '=')
            .next()?
            .to_string();
        if !name.is_empty() && name.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
            return Some(name);
        }
    }
    // x := ...  (single name, assignment)
    if let Some((name, _)) = line.split_once(" :=") {
        let name = name.trim().to_string();
        if !name.is_empty()
            && !name.contains(' ')
            && name.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false)
        {
            return Some(name);
        }
    }
    None
}

fn extract_var_binding(line: &str) -> Option<String> {
    let line = line.trim_start();
    if let Some(rest) = line.strip_prefix("var ") {
        let name = rest
            .split(|c: char| c.is_whitespace() || c == ':' || c == '=')
            .next()?
            .to_string();
        if !name.is_empty() && name.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
            return Some(name);
        }
    }
    None
}

fn range_for_word_on_line(line: &str, word: &str, line_idx: usize) -> Range {
    let col = line.find(word).unwrap_or(0);
    let line = line_idx as u32;
    Range {
        start: Position { line, character: col as u32 },
        end: Position { line, character: (col + word.len()) as u32 },
    }
}
