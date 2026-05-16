use ori_diagnostics::{
    Diagnostic as OriDiagnostic, Label as OriLabel, Severity as OriSeverity, SourceCache,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    Diagnostic as LspDiagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    InitializeParams, InitializeResult, InitializedParams, Location, MarkupContent, MarkupKind,
    MessageType, NumberOrString, OneOf, Position, Range, SaveOptions, ServerCapabilities,
    ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    documents: RwLock<HashMap<Url, String>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    async fn validate_uri(&self, uri: Url) {
        let Some(path) = document_path_from_uri(&uri) else {
            return;
        };
        let source = {
            let documents = self.documents.read().await;
            documents.get(&uri).cloned()
        };

        let result = match source {
            Some(source) => ori_driver::pipeline::run_check_source(&path, source),
            None => ori_driver::pipeline::run_check(&path),
        };

        let diagnostics = match result {
            Ok(output) => diagnostics_for_path(&output.cache, &output.diagnostics, &path),
            Err(message) => vec![file_error_diagnostic(message)],
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn document_text(&self, uri: &Url) -> Option<String> {
        if let Some(text) = self.documents.read().await.get(uri).cloned() {
            return Some(text);
        }
        let path = document_path_from_uri(uri)?;
        std::fs::read_to_string(path).ok()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: "ori-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ori-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents
            .write()
            .await
            .insert(uri.clone(), params.text_document.text);
        self.validate_uri(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents
                .write()
                .await
                .insert(uri.clone(), change.text);
        }
        self.validate_uri(uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.validate_uri(params.text_document.uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let Some(source) = self.document_text(&uri).await else {
            return Ok(None);
        };
        let Some(symbol) = word_at_position(&source, position) else {
            return Ok(None);
        };
        let Some(value) = semantic_hover(&source, &symbol) else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let Some(source) = self.document_text(&uri).await else {
            return Ok(None);
        };
        let Some(symbol) = word_at_position(&source, position) else {
            return Ok(None);
        };
        let Some(range) = find_local_definition(&source, &symbol) else {
            return Ok(None);
        };

        Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
            uri, range,
        ))))
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(stdlib_completion_items())))
    }
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: Some(false),
                will_save_wait_until: Some(false),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(false),
                })),
            },
        )),
        hover_provider: Some(tower_lsp::lsp_types::HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            ..CompletionOptions::default()
        }),
        ..ServerCapabilities::default()
    }
}

fn stdlib_completion_items() -> Vec<CompletionItem> {
    let mut modules = std::collections::BTreeSet::new();
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

fn document_path_from_uri(uri: &Url) -> Option<PathBuf> {
    uri.to_file_path().ok()
}

fn diagnostics_for_path(
    cache: &SourceCache,
    diagnostics: &[OriDiagnostic],
    target: &Path,
) -> Vec<LspDiagnostic> {
    let target = canonical_path(target);
    diagnostics
        .iter()
        .filter_map(|diagnostic| {
            let label = diagnostic
                .labels
                .iter()
                .find(|label| label_points_to_path(cache, label, &target));

            if !diagnostic.labels.is_empty() && label.is_none() {
                return None;
            }

            Some(LspDiagnostic {
                range: label
                    .map(|label| range_for_label(cache, label))
                    .unwrap_or_else(default_range),
                severity: Some(match diagnostic.severity {
                    OriSeverity::Error => DiagnosticSeverity::ERROR,
                    OriSeverity::Warning => DiagnosticSeverity::WARNING,
                }),
                code: Some(NumberOrString::String(diagnostic.code.to_string())),
                code_description: None,
                source: Some("ori".to_string()),
                message: diagnostic_message(diagnostic, label),
                related_information: None,
                tags: None,
                data: None,
            })
        })
        .collect()
}

fn label_points_to_path(cache: &SourceCache, label: &OriLabel, target: &Path) -> bool {
    cache
        .get(label.file_id)
        .map(|file| canonical_path(&file.path) == target)
        .unwrap_or(false)
}

fn range_for_label(cache: &SourceCache, label: &OriLabel) -> Range {
    let Some(file) = cache.get(label.file_id) else {
        return default_range();
    };
    let content_len = file.content.len() as u32;
    let start = label.span.start.min(content_len);
    let mut end = label.span.end.min(content_len);
    if end <= start && start < content_len {
        end = start + 1;
    }

    let (start_line, start_col) = file.line_col(start);
    let (end_line, end_col) = file.line_col(end);
    Range::new(
        Position::new(start_line.saturating_sub(1), start_col.saturating_sub(1)),
        Position::new(end_line.saturating_sub(1), end_col.saturating_sub(1)),
    )
}

fn diagnostic_message(diagnostic: &OriDiagnostic, label: Option<&OriLabel>) -> String {
    let mut message = diagnostic.message.clone();
    if let Some(label) = label {
        if !label.message.is_empty() {
            message.push_str("\n");
            message.push_str(&label.message);
        }
    }
    if let Some(why) = &diagnostic.why {
        message.push_str("\nwhy: ");
        message.push_str(why);
    }
    if let Some(action) = &diagnostic.action {
        message.push_str("\naction: ");
        message.push_str(action);
    }
    for note in &diagnostic.notes {
        message.push_str("\nnote: ");
        message.push_str(note);
    }
    message
}

fn file_error_diagnostic(message: String) -> LspDiagnostic {
    LspDiagnostic {
        range: default_range(),
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String("lsp.file".to_string())),
        code_description: None,
        source: Some("ori".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn default_range() -> Range {
    Range::new(Position::new(0, 0), Position::new(0, 1))
}

fn canonical_path(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn builtin_type_hover(symbol: &str) -> Option<String> {
    let text = match symbol {
        "int" => "`int`\n\nSigned integer value used for whole numbers.",
        "float" => "`float`\n\nFloating point value used for decimal numbers.",
        "bool" => "`bool`\n\nBoolean value. It is either `true` or `false`.",
        "string" => "`string`\n\nUTF-8 text value managed by the Ori runtime.",
        "bytes" => "`bytes`\n\nByte buffer used for binary data.",
        "void" => "`void`\n\nFunction return type for functions that do not return a value.",
        "list" => "`list<T>`\n\nOrdered runtime collection of values with the same element type.",
        "map" => {
            "`map<K, V>`\n\nHash map. Keys must be `int`, `string`, or implement `Hashable` and `Equatable`."
        }
        "set" => {
            "`set<T>`\n\nHash set. Elements must be `int`, `string`, or implement `Hashable` and `Equatable`."
        }
        "optional" => "`optional<T>`\n\nRepresents either a value of type `T` or `none`.",
        "result" => "`result<T, E>`\n\nRepresents either success `ok(T)` or failure `err(E)`.",
        "future" => "`future<T>`\n\nAsynchronous result that will produce a value of type `T`.",
        _ => return None,
    };
    Some(text.to_string())
}

fn semantic_hover(source: &str, symbol: &str) -> Option<String> {
    if symbol == "it" && source.contains(" if it") {
        return Some(
            "`it`\n\nCurrent value checked by an `if` contract on a field or parameter."
                .to_string(),
        );
    }
    builtin_type_hover(symbol).or_else(|| build_semantic_index(source).hover(symbol))
}

#[derive(Clone, Debug)]
struct SemanticSymbol {
    range: Range,
    hover: String,
    summary: String,
}

#[derive(Default)]
struct SemanticIndex {
    symbols: HashMap<String, Vec<SemanticSymbol>>,
}

impl SemanticIndex {
    fn add(&mut self, name: &str, range: Range, hover: String, summary: String) {
        self.symbols
            .entry(name.to_string())
            .or_default()
            .push(SemanticSymbol {
                range,
                hover,
                summary,
            });
    }

    fn hover(&self, symbol: &str) -> Option<String> {
        let entries = self.symbols.get(symbol)?;
        if entries.len() == 1 {
            return Some(entries[0].hover.clone());
        }

        let summaries = entries
            .iter()
            .map(|entry| format!("- {}", entry.summary))
            .collect::<Vec<_>>()
            .join("\n");
        Some(format!(
            "Multiple local symbols named `{symbol}`:\n\n{summaries}"
        ))
    }

    fn definition(&self, symbol: &str) -> Option<Range> {
        self.symbols
            .get(symbol)
            .and_then(|entries| entries.first())
            .map(|entry| entry.range.clone())
    }
}

fn build_semantic_index(source: &str) -> SemanticIndex {
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = SemanticIndex::default();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = strip_item_prefixes(line.trim_start());

        if let Some(name) = declaration_name(trimmed, "struct") {
            let (next, fields) = index_struct_block(&mut index, &lines, i, &name);
            let summary = format!("struct {name}");
            let field_lines = if fields.is_empty() {
                "Fields: none indexed.".to_string()
            } else {
                format!("Fields:\n{}", fields.join("\n"))
            };
            index.add(
                &name,
                range_for_symbol_in_line(line, i, &name),
                format!("```ori\nstruct {name}\n```\n\n{field_lines}"),
                summary,
            );
            i = next;
            continue;
        }

        if let Some(name) = declaration_name(trimmed, "trait") {
            let (next, methods) = index_trait_block(&mut index, &lines, i, &name);
            let summary = format!("trait {name}");
            let method_lines = if methods.is_empty() {
                "Methods: none indexed.".to_string()
            } else {
                format!("Methods:\n{}", methods.join("\n"))
            };
            index.add(
                &name,
                range_for_symbol_in_line(line, i, &name),
                format!("```ori\ntrait {name}\n```\n\n{method_lines}"),
                summary,
            );
            i = next;
            continue;
        }

        if let Some(name) = declaration_name(trimmed, "enum") {
            index.add(
                &name,
                range_for_symbol_in_line(line, i, &name),
                format!("```ori\nenum {name}\n```\n\nUser-defined enum."),
                format!("enum {name}"),
            );
        }

        index_function_line(&mut index, line, i, trimmed, None);
        index_binding_line(&mut index, line, i, trimmed);
        i += 1;
    }

    index
}

fn index_struct_block(
    index: &mut SemanticIndex,
    lines: &[&str],
    start: usize,
    struct_name: &str,
) -> (usize, Vec<String>) {
    let mut fields = Vec::new();
    let mut i = start + 1;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = strip_item_prefixes(line.trim_start());
        if is_block_end(trimmed) {
            return (i + 1, fields);
        }

        if let Some((name, ty, contract)) = parse_typed_name(trimmed) {
            let summary = format!("field {struct_name}.{name}: {ty}");
            let mut hover =
                format!("```ori\n{name}: {ty}\n```\n\nField of `struct {struct_name}`.");
            if let Some(contract) = &contract {
                hover.push_str("\n\nContract: `");
                hover.push_str(contract);
                hover.push('`');
            }
            fields.push(format!("- `{name}: {ty}`"));
            index.add(
                &name,
                range_for_symbol_in_line(line, i, &name),
                hover,
                summary,
            );
        }

        index_function_line(index, line, i, trimmed, Some(struct_name));
        i += 1;
    }
    (i, fields)
}

fn index_trait_block(
    index: &mut SemanticIndex,
    lines: &[&str],
    start: usize,
    trait_name: &str,
) -> (usize, Vec<String>) {
    let mut methods = Vec::new();
    let mut i = start + 1;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = strip_item_prefixes(line.trim_start());
        if is_block_end(trimmed) {
            return (i + 1, methods);
        }

        if let Some((name, signature)) = parse_function_signature(trimmed) {
            let summary = format!("trait method {trait_name}.{name}");
            methods.push(format!("- `{signature}`"));
            index.add(
                &name,
                range_for_symbol_in_line(line, i, &name),
                format!("```ori\n{signature}\n```\n\nMethod required by `trait {trait_name}`."),
                summary,
            );
            index_params(index, line, i, &signature);
        }
        i += 1;
    }
    (i, methods)
}

fn index_function_line(
    index: &mut SemanticIndex,
    line: &str,
    line_index: usize,
    trimmed: &str,
    owner: Option<&str>,
) {
    let Some((name, signature)) = parse_function_signature(trimmed) else {
        return;
    };

    let summary = owner
        .map(|owner| format!("method {owner}.{name}"))
        .unwrap_or_else(|| format!("function {name}"));
    let mut hover = format!("```ori\n{signature}\n```");
    if let Some(owner) = owner {
        hover.push_str("\n\nMethod on `");
        hover.push_str(owner);
        hover.push_str("`.");
    } else {
        hover.push_str("\n\nUser-defined function.");
    }
    index.add(
        &name,
        range_for_symbol_in_line(line, line_index, &name),
        hover,
        summary,
    );
    index_params(index, line, line_index, &signature);
}

fn index_binding_line(index: &mut SemanticIndex, line: &str, line_index: usize, trimmed: &str) {
    let Some((kind, name, ty, contract)) = parse_binding(trimmed) else {
        return;
    };

    let summary = if ty.is_empty() {
        format!("{kind} {name}")
    } else {
        format!("{kind} {name}: {ty}")
    };
    let mut signature = format!("{kind} {name}");
    if !ty.is_empty() {
        signature.push_str(": ");
        signature.push_str(&ty);
    }
    let mut hover = format!("```ori\n{signature}\n```\n\nLocal `{kind}` binding.");
    if let Some(contract) = contract {
        hover.push_str("\n\nContract: `");
        hover.push_str(&contract);
        hover.push('`');
    }

    index.add(
        &name,
        range_for_symbol_in_line(line, line_index, &name),
        hover,
        summary,
    );
}

fn index_params(index: &mut SemanticIndex, line: &str, line_index: usize, signature: &str) {
    let Some(open) = signature.find('(') else {
        return;
    };
    let Some(close) = matching_paren(signature, open) else {
        return;
    };

    for param in split_top_level(&signature[open + 1..close], ',') {
        let param = param.trim();
        let param = param.strip_prefix("mut ").unwrap_or(param);
        let Some((name, ty, contract)) = parse_typed_name(param) else {
            continue;
        };
        let summary = format!("parameter {name}: {ty}");
        let mut hover = format!("```ori\n{name}: {ty}\n```\n\nFunction parameter.");
        if let Some(contract) = contract {
            hover.push_str("\n\nContract: `");
            hover.push_str(&contract);
            hover.push('`');
        }
        index.add(
            &name,
            range_for_symbol_in_line(line, line_index, &name),
            hover,
            summary,
        );
    }
}

fn strip_item_prefixes(mut line: &str) -> &str {
    loop {
        let next = line
            .strip_prefix("public ")
            .or_else(|| line.strip_prefix("deprecated "));
        let Some(next) = next else {
            return line;
        };
        line = next.trim_start();
    }
}

fn strip_line_comment(line: &str) -> &str {
    line.split_once("--")
        .map(|(before, _)| before)
        .unwrap_or(line)
        .trim()
}

fn declaration_name(line: &str, keyword: &str) -> Option<String> {
    let line = strip_line_comment(line);
    let rest = line.strip_prefix(keyword)?.trim_start();
    let (name, _) = take_identifier(rest)?;
    Some(name.to_string())
}

fn parse_function_signature(line: &str) -> Option<(String, String)> {
    let line = strip_line_comment(line);
    let rest = line
        .strip_prefix("async mut func ")
        .or_else(|| line.strip_prefix("mut async func "))
        .or_else(|| line.strip_prefix("async func "))
        .or_else(|| line.strip_prefix("mut func "))
        .or_else(|| line.strip_prefix("func "))?;
    let (name, _) = take_identifier(rest.trim_start())?;
    Some((name.to_string(), line.to_string()))
}

fn parse_binding(line: &str) -> Option<(&'static str, String, String, Option<String>)> {
    let line = strip_line_comment(line);
    for kind in ["const", "var"] {
        let Some(rest) = line.strip_prefix(kind) else {
            continue;
        };
        let rest = rest.trim_start();
        let (name, tail) = take_identifier(rest)?;
        let tail = tail.trim_start();
        if !tail.starts_with(':') {
            return Some((kind, name.to_string(), String::new(), None));
        }
        let type_text = tail[1..].trim();
        let (ty, contract) = split_type_contract(type_text);
        return Some((kind, name.to_string(), ty, contract));
    }
    None
}

fn parse_typed_name(line: &str) -> Option<(String, String, Option<String>)> {
    let line = strip_line_comment(line);
    let (name, tail) = take_identifier(line)?;
    let tail = tail.trim_start();
    if !tail.starts_with(':') {
        return None;
    }
    let (ty, contract) = split_type_contract(tail[1..].trim());
    Some((name.to_string(), ty, contract))
}

fn split_type_contract(text: &str) -> (String, Option<String>) {
    let (head, contract) = text
        .split_once(" if ")
        .map(|(head, contract)| (head, Some(contract.trim().to_string())))
        .unwrap_or((text, None));
    let ty = head
        .split_once('=')
        .map(|(before, _)| before)
        .unwrap_or(head)
        .trim()
        .to_string();
    (ty, contract)
}

fn take_identifier(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    let mut end = 0;
    for (index, ch) in input.char_indices() {
        if index == 0 && !(ch == '_' || ch.is_ascii_alphabetic()) {
            return None;
        }
        if ch == '_' || ch.is_ascii_alphanumeric() {
            end = index + ch.len_utf8();
            continue;
        }
        break;
    }
    if end == 0 {
        return None;
    }
    Some((&input[..end], &input[end..]))
}

fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in text.char_indices().skip_while(|(index, _)| *index < open) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_top_level(text: &str, delimiter: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut angle_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    for (index, ch) in text.char_indices() {
        match ch {
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ if ch == delimiter && angle_depth == 0 && paren_depth == 0 && bracket_depth == 0 => {
                parts.push(&text[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&text[start..]);
    parts
}

fn is_block_end(line: &str) -> bool {
    strip_line_comment(line) == "end"
}

fn range_for_symbol_in_line(line: &str, line_index: usize, symbol: &str) -> Range {
    let column = line.find(symbol).unwrap_or(0);
    Range::new(
        Position::new(line_index as u32, column as u32),
        Position::new(line_index as u32, (column + symbol.len()) as u32),
    )
}

fn word_at_position(source: &str, position: Position) -> Option<String> {
    let offset = offset_at_position(source, position)?;
    let bytes = source.as_bytes();
    if offset >= bytes.len() || !is_word_byte(bytes[offset]) {
        return None;
    }

    let mut start = offset;
    while start > 0 && is_word_byte(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = offset;
    while end < bytes.len() && is_word_byte(bytes[end]) {
        end += 1;
    }

    Some(source[start..end].to_string())
}

fn offset_at_position(source: &str, position: Position) -> Option<usize> {
    let line = source.lines().nth(position.line as usize)?;
    let line_start = source
        .lines()
        .take(position.line as usize)
        .map(|line| line.len() + 1)
        .sum::<usize>();
    let column = (position.character as usize).min(line.len());
    Some(line_start + column)
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn find_local_definition(source: &str, symbol: &str) -> Option<Range> {
    build_semantic_index(source).definition(symbol)
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ori_diagnostics::{Diagnostic as OriDiagnostic, Label, Span};

    #[test]
    fn capabilities_enable_full_text_sync_and_diagnostics() {
        let capabilities = server_capabilities();
        match capabilities.text_document_sync {
            Some(TextDocumentSyncCapability::Options(options)) => {
                assert_eq!(options.open_close, Some(true));
                assert_eq!(options.change, Some(TextDocumentSyncKind::FULL));
            }
            other => panic!("unexpected text sync capabilities: {other:?}"),
        }
        assert!(capabilities.hover_provider.is_some());
        assert_eq!(capabilities.definition_provider, Some(OneOf::Left(true)));
        assert!(capabilities.completion_provider.is_some());
    }

    #[test]
    fn diagnostic_conversion_preserves_code_range_and_guidance() {
        let mut cache = SourceCache::default();
        let file_id = cache.add(
            "main.orl",
            "namespace app.main\nconst value: int = \"x\"\n".into(),
        );
        let diagnostic = OriDiagnostic::error("type.type_mismatch", "type mismatch")
            .with_label(Label::primary(file_id, Span::new(35, 38), "value here"))
            .with_why("expected int")
            .with_action("use an int value");

        let diagnostics = diagnostics_for_path(&cache, &[diagnostic], Path::new("main.orl"));

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code,
            Some(NumberOrString::String("type.type_mismatch".to_string()))
        );
        assert_eq!(diagnostics[0].range.start.line, 1);
        assert!(diagnostics[0].message.contains("type mismatch"));
        assert!(diagnostics[0].message.contains("why: expected int"));
        assert!(diagnostics[0].message.contains("action: use an int value"));
    }

    #[test]
    fn hover_word_detection_finds_builtin_type() {
        let source = "namespace app.main\n\nfunc main(value: int)\nend\n";
        let symbol = word_at_position(source, Position::new(2, 18));

        assert_eq!(symbol.as_deref(), Some("int"));
        assert!(semantic_hover(source, symbol.as_deref().unwrap())
            .unwrap()
            .contains("Signed integer"));
    }

    #[test]
    fn local_definition_finds_functions_and_types() {
        let source =
            "namespace app.main\n\npublic struct User\nend\n\nfunc save(user: User)\nend\n";

        let user_range = find_local_definition(source, "User").unwrap();
        let save_range = find_local_definition(source, "save").unwrap();

        assert_eq!(user_range.start.line, 2);
        assert_eq!(save_range.start.line, 5);
    }

    #[test]
    fn semantic_hover_finds_user_function_signature() {
        let source = "namespace app.main\n\nfunc save(user: User) -> result<void, string>\nend\n";

        let hover = semantic_hover(source, "save").unwrap();

        assert!(hover.contains("func save(user: User) -> result<void, string>"));
        assert!(hover.contains("User-defined function"));
    }

    #[test]
    fn semantic_hover_finds_struct_field_and_local_binding() {
        let source = "namespace app.main\n\nstruct User\n    name: string\nend\n\nfunc main()\n    const active: bool = true\nend\n";

        let field_hover = semantic_hover(source, "name").unwrap();
        let binding_hover = semantic_hover(source, "active").unwrap();

        assert!(field_hover.contains("name: string"));
        assert!(field_hover.contains("struct User"));
        assert!(binding_hover.contains("const active: bool"));
    }

    #[test]
    fn semantic_hover_finds_params_and_contract_placeholder() {
        let source = "namespace app.main\n\nfunc sqrt(value: float if it >= 0.0) -> float\nend\n";

        let param_hover = semantic_hover(source, "value").unwrap();
        let contract_hover = semantic_hover(source, "it").unwrap();

        assert!(param_hover.contains("value: float"));
        assert!(param_hover.contains("Contract: `it >= 0.0"));
        assert!(contract_hover.contains("Current value checked"));
    }

    #[test]
    fn local_definition_finds_fields_and_bindings() {
        let source = "namespace app.main\n\nstruct User\n    name: string\nend\n\nfunc main()\n    const active: bool = true\nend\n";

        let field_range = find_local_definition(source, "name").unwrap();
        let binding_range = find_local_definition(source, "active").unwrap();

        assert_eq!(field_range.start.line, 3);
        assert_eq!(binding_range.start.line, 7);
    }

    #[test]
    fn completion_includes_stdlib_and_new_collections() {
        let items = stdlib_completion_items();
        let labels: Vec<_> = items.iter().map(|item| item.label.as_str()).collect();

        assert!(labels.contains(&"ori.map"));
        assert!(labels.contains(&"ori.heap.push"));
        assert!(labels.contains(&"ori.graph.topological_sort"));
    }
}
