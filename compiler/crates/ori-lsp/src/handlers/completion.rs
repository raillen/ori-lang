use std::collections::BTreeSet;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

/// Generate completion items for the Ori standard library.
pub fn stdlib_completion_items() -> Vec<CompletionItem> {
    let mut modules = BTreeSet::new();
    let mut items = Vec::new();

    for entry in ori_types::stdlib::stdlib_runtime_functions() {
        if let Some((module, _)) = entry.canonical_path.rsplit_once('.') {
            modules.insert(module.to_string());
        }
        items.push(CompletionItem {
            label: entry.canonical_path.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Ori standard library function".to_string()),
            ..CompletionItem::default()
        });
    }

    for module in modules {
        items.push(CompletionItem {
            label: module,
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Ori standard library module".to_string()),
            ..CompletionItem::default()
        });
    }

    items.sort_by(|a, b| a.label.cmp(&b.label));
    items
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
