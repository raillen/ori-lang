//! Human-readable explanations for diagnostic codes (`ori explain <code>`).

#[derive(Debug, Clone, Copy)]
pub struct ExplainEntry {
    pub code: &'static str,
    pub severity: &'static str,
    pub summary: &'static str,
    pub cause: &'static str,
    pub fix: &'static str,
}

const ENTRIES: &[ExplainEntry] = &[
    ExplainEntry {
        code: "name.undefined",
        severity: "error",
        summary: "A name was used but is not defined in the current scope.",
        cause: "The identifier is missing a binding: typo, wrong import, or symbol defined in another module without import.",
        fix: "Check spelling, add `import module as alias`, or qualify with the module alias (e.g. `lib.Point`).",
    },
    ExplainEntry {
        code: "name.private",
        severity: "error",
        summary: "A non-public item was accessed from outside its module.",
        cause: "The symbol exists but is not marked `public` in the defining file.",
        fix: "Mark the declaration `public` in its home module, or use a public wrapper/API.",
    },
    ExplainEntry {
        code: "name.duplicate",
        severity: "error",
        summary: "The same name is defined twice in one namespace.",
        cause: "Two top-level declarations share an identifier.",
        fix: "Rename one of the definitions or merge them into a single declaration.",
    },
    ExplainEntry {
        code: "type.type_mismatch",
        severity: "error",
        summary: "An expression type does not match what the context requires.",
        cause: "Assignment, return, argument, or branch expects a different type than inferred.",
        fix: "Adjust the expression, add an explicit conversion, or change the expected type annotation.",
    },
    ExplainEntry {
        code: "type.arg_type_mismatch",
        severity: "error",
        summary: "A function argument has the wrong type.",
        cause: "The callee parameter type differs from the value passed at this position.",
        fix: "Convert the argument or pick an overload/method that accepts this type.",
    },
    ExplainEntry {
        code: "type.unused_result",
        severity: "warning",
        summary: "A `result` value is computed but discarded.",
        cause: "Calling a function returning `result<T, E>` without `?`, `match`, or explicit handling.",
        fix: "Use `const _ = expr`, propagate with `?`, or handle `success`/`error` explicitly.",
    },
    ExplainEntry {
        code: "project.circular_import",
        severity: "error",
        summary: "Local imports form a cycle.",
        cause: "File A imports B (transitively) and B imports A before definitions are complete.",
        fix: "Extract shared types to a third module or invert the dependency direction.",
    },
    ExplainEntry {
        code: "project.namespace_file_mismatch",
        severity: "error",
        summary: "File namespace does not match its import path.",
        cause: "The `namespace` declaration in the file differs from the path implied by the import.",
        fix: "Align `namespace` with the directory layout or fix the import path.",
    },
    ExplainEntry {
        code: "project.entry_not_found",
        severity: "error",
        summary: "Project entry file is missing.",
        cause: "`ori.proj` points to a non-existent file, or no entry could be resolved.",
        fix: "Create the entry `.orl` file or fix the `entry` key in `ori.proj`.",
    },
    ExplainEntry {
        code: "project.no_proj_file",
        severity: "error",
        summary: "No project manifest was found.",
        cause: "The driver searched upward from the entry path and found no `ori.proj`.",
        fix: "Add `ori.proj` with an `entry = \"path/to/main.orl\"` line at the project root.",
    },
    ExplainEntry {
        code: "bind.import_not_found",
        severity: "error",
        summary: "Import path does not resolve to any file.",
        cause: "No matching `.orl` exists for the import path under the project or stdlib roots.",
        fix: "Fix the import path, add the missing file, or set `ORI_STDLIB_ROOT` if importing stdlib.",
    },
    ExplainEntry {
        code: "bind.stdlib_module_unknown",
        severity: "error",
        summary: "Unknown stdlib module.",
        cause: "The import looks like `ori.*` but is not in the runtime manifest and has no `.orl` source.",
        fix: "Check the module name in `docs/spec/12-stdlib.md` or `stdlib/README.md`.",
    },
    ExplainEntry {
        code: "parse.expected_type",
        severity: "error",
        summary: "Parser expected a type annotation.",
        cause: "Ori requires explicit types on bindings and parameters; a type was omitted or malformed.",
        fix: "Add `: Type` after the binding name (e.g. `var x: int = 0`).",
    },
    ExplainEntry {
        code: "parse.unexpected_token",
        severity: "error",
        summary: "Unexpected token at this position.",
        cause: "Syntax does not match Ori grammar (often a missing `end`, comma, or keyword).",
        fix: "Read the surrounding block structure; ensure each `if`/`func`/`struct` has a matching `end`.",
    },
    ExplainEntry {
        code: "match.non_exhaustive",
        severity: "error",
        summary: "`match` does not cover all cases.",
        cause: "Some enum variants or patterns are not handled.",
        fix: "Add missing `case` arms or a catch-all pattern.",
    },
    ExplainEntry {
        code: "generic.constraint_not_satisfied",
        severity: "error",
        summary: "Generic constraint is not satisfied.",
        cause: "A type argument does not implement a required trait (`where T is Trait`).",
        fix: "Implement the trait for your type or use a type that already satisfies the constraint.",
    },
];

/// Lookup explanation for a diagnostic code (exact match).
pub fn explain_code(code: &str) -> Option<&ExplainEntry> {
    ENTRIES.iter().find(|e| e.code == code)
}

/// All codes with curated explanations.
pub fn explained_codes() -> impl Iterator<Item = &'static ExplainEntry> {
    ENTRIES.iter()
}

/// Format a full explanation for terminal or tooling output.
pub fn format_explanation(entry: &ExplainEntry) -> String {
    format!(
        "code:     {}\nseverity: {}\n\n{}\n\ncause:\n  {}\n\nfix:\n  {}",
        entry.code, entry.severity, entry.summary, entry.cause, entry.fix
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explain_known_codes() {
        for code in [
            "name.undefined",
            "project.circular_import",
            "type.type_mismatch",
        ] {
            assert!(
                explain_code(code).is_some(),
                "missing explain entry for {code}"
            );
        }
    }
}
