use std::collections::BTreeSet;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::stdlib_catalog::{import_alias_map, stdlib_catalog};

/// Generate completion items for the Ori standard library (Layer 1 + Layer 2).
pub fn stdlib_completion_items() -> Vec<CompletionItem> {
    stdlib_catalog().completion_items()
}

/// Completion items when the cursor is inside an `import` statement.
pub fn stdlib_import_completion_items(prefix: &str) -> Vec<CompletionItem> {
    stdlib_catalog().module_completion_items(prefix)
}

/// Dot-completion items for a stdlib import alias or module prefix.
pub fn stdlib_dot_completion_items(receiver: &str, source: &str) -> Vec<CompletionItem> {
    let import_map = import_alias_map(source);
    stdlib_catalog().dot_completion_items(receiver, &import_map)
}

/// Keyword completions for the Ori language.
pub fn keyword_completion_items() -> Vec<CompletionItem> {
    let keywords = [
        "namespace", "import", "as", "public",
        "func", "return", "end", "const", "var",
        "if", "else", "while", "for", "in", "repeat", "loop",
        "break", "continue", "match", "case",
        "struct", "trait", "implement", "enum",
        "where", "is", "alias", "do",
        "and", "or", "not", "true", "false",
        "none", "success", "error", "some",
        "mut", "self", "extern", "any",
        "optional", "result", "list", "map", "set", "range",
        "void", "using", "check", "with", "then", "tuple", "lazy",
    ];

    keywords
        .iter()
        .map(|kw| CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Ori keyword".to_string()),
            ..CompletionItem::default()
        })
        .collect()
}

/// Snippet completions for common Ori constructs.
pub fn snippet_completion_items() -> Vec<CompletionItem> {
    vec![
        snippet("func", "func ${1:name}(${2:params}) -> ${3:ret}\n    ${0}\nend"),
        snippet("struct", "struct ${1:Name}\n    ${0}\nend"),
        snippet("enum", "enum ${1:Name}\n    ${0}\nend"),
        snippet("trait", "trait ${1:Name}\n    func ${2:method}() -> ${3:ret}\nend"),
        snippet("implement", "implement ${1:Trait} for ${2:Type}\n    func ${3:method}() -> ${4:ret}\n        ${0}\n    end\nend"),
        snippet("if", "if ${1:condition}\n    ${0}\nend"),
        snippet("ifelse", "if ${1:condition}\n    ${2}\nelse\n    ${0}\nend"),
        snippet("while", "while ${1:condition}\n    ${0}\nend"),
        snippet("for", "for ${1:item} in ${2:collection}\n    ${0}\nend"),
        snippet("loop", "loop\n    ${0}\nend"),
        snippet("match", "match ${1:value}\ncase ${2:pattern}:\n    ${0}\nend"),
        snippet("using", "using ${1:name}: ${2:Type} = ${3:expr}\n    ${0}\nend"),
        snippet("check", "check ${1:condition}, \"${2:message}\""),
        snippet("import", "import ${1:ori.module} as ${2:alias}"),
    ]
}

fn snippet(label: &str, body: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Ori snippet".to_string()),
        insert_text: Some(body.to_string()),
        insert_text_format: Some(tower_lsp::lsp_types::InsertTextFormat::SNIPPET),
        ..CompletionItem::default()
    }
}

/// Deduplicate completion items by label while preserving order.
pub fn dedupe_completion_items(items: &mut Vec<CompletionItem>) {
    let mut seen = BTreeSet::new();
    items.retain(|item| seen.insert(item.label.clone()));
}
