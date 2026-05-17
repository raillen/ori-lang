mod handlers;
mod index;
mod utils;

use index::project::ProjectManager;
use index::semantic::{CompletionContext, SemanticIndex};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams,
    CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentSymbol,
    DocumentSymbolParams, DocumentSymbolResponse, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverParams, InitializeParams, InitializeResult,
    InitializedParams, Location, MessageType, OneOf, Range, ReferenceParams,
    SaveOptions, ServerCapabilities, ServerInfo, SymbolKind, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
    TextEdit, Url, WorkspaceEdit,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    project: Arc<RwLock<ProjectManager>>,
    last_change: Arc<RwLock<HashMap<Url, Instant>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            project: Arc::new(RwLock::new(ProjectManager::new())),
            last_change: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn validate_uri(&self, uri: Url) {
        let source = {
            let project = self.project.read().await;
            project.document_content(&uri)
        };
        let path = utils::uri::document_path_from_uri(&uri);
        let result = match (path.as_deref(), source.clone()) {
            (Some(path), Some(source)) => ori_driver::pipeline::run_check_source(path, source),
            (Some(path), None) => ori_driver::pipeline::run_check(path),
            _ => return,
        };
        let mut diagnostics = match result {
            Ok(output) => {
                if let Some(target) = &path {
                    handlers::diagnostics::diagnostics_for_path(
                        &output.cache, &output.diagnostics, target,
                    )
                } else {
                    Vec::new()
                }
            }
            Err(message) => vec![handlers::diagnostics::file_error_diagnostic(message)],
        };
        // Add lint diagnostics
        if let Some(ref src) = source {
            let config = handlers::lint::LintConfig::default();
            let lint_diags = handlers::lint::lint(src, &config);
            diagnostics.extend(lint_diags);
        }
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    /// Schedule a debounced validation for the given URI (300ms cooldown).
    async fn schedule_debounced_validate(&self, uri: Url) {
        let now = Instant::now();
        {
            let mut last = self.last_change.write().await;
            last.insert(uri.clone(), now);
        }

        let client = self.client.clone();
        let project = Arc::clone(&self.project);
        let last_change = Arc::clone(&self.last_change);

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;

            let should_run = {
                let last = last_change.read().await;
                last.get(&uri).map(|t| *t <= now).unwrap_or(false)
            };
            if !should_run {
                return;
            }

            let source = {
                let proj = project.read().await;
                proj.document_content(&uri)
            };
            let path = utils::uri::document_path_from_uri(&uri);
            let result = match (path.as_deref(), source) {
                (Some(path), Some(source)) => {
                    ori_driver::pipeline::run_check_source(path, source)
                }
                (Some(path), None) => ori_driver::pipeline::run_check(path),
                _ => return,
            };
            let diagnostics = match result {
                Ok(output) => {
                    if let Some(target) = &path {
                        handlers::diagnostics::diagnostics_for_path(
                            &output.cache, &output.diagnostics, target,
                        )
                    } else {
                        Vec::new()
                    }
                }
                Err(message) => {
                    vec![handlers::diagnostics::file_error_diagnostic(message)]
                }
            };
            client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        });
    }

    async fn get_source_and_index(&self, uri: &Url) -> Option<(String, SemanticIndex)> {
        let source = {
            let project = self.project.read().await;
            project.document_content(uri)
        }?;
        let index = {
            let project = self.project.read().await;
            project
                .document_index(uri)
                .cloned()
                .unwrap_or_else(|| SemanticIndex::build(&source))
        };
        Some((source, index))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri {
            let root = root_uri.to_file_path().ok();
            self.project.write().await.set_workspace_root(root);
        }
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
        let uri = params.text_document.uri.clone();
        self.project.write().await.upsert_document(
            uri.clone(),
            params.text_document.text,
            params.text_document.version,
        );
        self.validate_uri(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            self.project
                .write()
                .await
                .upsert_document(uri.clone(), change.text, 0);
        }
        self.schedule_debounced_validate(uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.validate_uri(params.text_document.uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.project.write().await.remove_document(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    // ── Hover ────────────────────────────────────────────────────────────────

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let Some((source, index)) = self.get_source_and_index(&uri).await else {
            return Ok(None);
        };
        let Some(symbol) = utils::uri::word_at_position(&source, position) else {
            return Ok(None);
        };
        if let Some(hover_text) = handlers::hover::builtin_type_hover(&symbol) {
            return Ok(Some(handlers::hover::markdown_hover(hover_text)));
        }
        if symbol == "it" && source.contains(" if it") {
            return Ok(Some(handlers::hover::markdown_hover(
                "`it`\n\nCurrent value checked by a contract on a field or parameter."
                    .into(),
            )));
        }
        if let Some(hover_text) = index.hover(&symbol) {
            return Ok(Some(handlers::hover::markdown_hover(hover_text)));
        }
        Ok(None)
    }

    // ── Go-to-definition ─────────────────────────────────────────────────────

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let Some((source, index)) = self.get_source_and_index(&uri).await else {
            return Ok(None);
        };
        let Some(symbol) = utils::uri::word_at_position(&source, position) else {
            return Ok(None);
        };
        if let Some(range) = index.definition(&symbol) {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location::new(uri, range))));
        }
        if let Some(target_uri) = self.resolve_import_target(&index, &symbol).await {
            if let Some((target_source, _)) = self.get_source_and_index(&target_uri).await {
                let target_index = SemanticIndex::build(&target_source);
                if let Some(range) = target_index.definition(&symbol) {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
                        target_uri, range,
                    ))));
                }
            }
        }
        Ok(None)
    }

    // ── Find references ──────────────────────────────────────────────────────

    async fn references(
        &self,
        params: ReferenceParams,
    ) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let Some((source, index)) = self.get_source_and_index(&uri).await else {
            return Ok(None);
        };
        let Some(symbol) = utils::uri::word_at_position(&source, position) else {
            return Ok(None);
        };
        let refs = index.find_references(&source, &symbol);
        let locations: Vec<Location> = refs
            .into_iter()
            .map(|range| Location::new(uri.clone(), range))
            .collect();
        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }

    // ── Completions (context-aware) ──────────────────────────────────────────

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let mut items: Vec<CompletionItem> = Vec::new();

        let context = if let Some((source, index)) = self.get_source_and_index(&uri).await {
            index.completion_context(&source, position)
        } else {
            CompletionContext::Default
        };

        match context {
            CompletionContext::AfterDot { .. } => {
                items.extend(handlers::completion::stdlib_completion_items());
            }
            CompletionContext::Import => {
                items.extend(handlers::completion::stdlib_completion_items());
                items.extend(handlers::completion::keyword_completion_items());
            }
            CompletionContext::Default => {
                items.extend(handlers::completion::stdlib_completion_items());
                items.extend(handlers::completion::keyword_completion_items());
                items.extend(handlers::completion::snippet_completion_items());
                if let Some((source, index)) = self.get_source_and_index(&uri).await {
                    let partial = utils::uri::word_at_position(&source, position)
                        .unwrap_or_default();
                    for sym in index.all_symbols() {
                        if sym.name.starts_with(&partial) || partial.is_empty() {
                            items.push(CompletionItem {
                                label: sym.name.clone(),
                                kind: Some(symbol_kind_to_lsp(&sym.kind)),
                                detail: Some(sym.summary.clone()),
                                ..CompletionItem::default()
                            });
                        }
                    }
                }
            }
        }

        items.sort_by(|a, b| a.label.cmp(&b.label));
        items.dedup_by(|a, b| a.label == b.label);
        Ok(Some(CompletionResponse::Array(items)))
    }

    // ── Document Symbols ─────────────────────────────────────────────────────

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let Some((_source, index)) = self.get_source_and_index(&uri).await else {
            return Ok(None);
        };

        let mut symbols: Vec<DocumentSymbol> = Vec::new();

        for sym in index.all_symbols() {
            if matches!(
                sym.kind,
                index::semantic::SymbolKind::Function
                    | index::semantic::SymbolKind::Struct
                    | index::semantic::SymbolKind::Enum
                    | index::semantic::SymbolKind::Trait
            ) {
                let children = if sym.kind == index::semantic::SymbolKind::Struct {
                    index
                        .all_symbols()
                        .filter(|s| {
                            s.kind == index::semantic::SymbolKind::Field
                                && s.range.start.line >= sym.range.start.line
                                && s.range.end.line <= sym.range.end.line + 5
                        })
                        .map(|s| DocumentSymbol {
                            name: s.name.clone(),
                            detail: Some(s.summary.clone()),
                            kind: SymbolKind::FIELD,
                            range: s.range,
                            selection_range: s.range,
                            children: None,
                            tags: None,
                            deprecated: None,
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                symbols.push(DocumentSymbol {
                    name: sym.name.clone(),
                    detail: Some(sym.summary.clone()),
                    kind: semantic_kind_to_lsp(&sym.kind),
                    range: sym.range,
                    selection_range: sym.range,
                    children: if children.is_empty() {
                        None
                    } else {
                        Some(children)
                    },
                    tags: None,
                    deprecated: None,
                });
            }
        }

        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    }

    // ── Code Actions ─────────────────────────────────────────────────────────

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>> {
        let mut actions: Vec<CodeActionOrCommand> = Vec::new();

        for diag in &params.context.diagnostics {
            let code = diag
                .code
                .as_ref()
                .and_then(|c| match c {
                    tower_lsp::lsp_types::NumberOrString::String(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("");

            match code {
                "type.unused_result" => {
                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: "Discard result explicitly with `const _ =`".into(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(vec![diag.clone()]),
                        edit: Some(WorkspaceEdit {
                            changes: Some({
                                let mut map = HashMap::new();
                                map.insert(
                                    params.text_document.uri.clone(),
                                    vec![TextEdit {
                                        range: Range::new(diag.range.start, diag.range.start),
                                        new_text: "const _ = ".into(),
                                    }],
                                );
                                map
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }));

                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: "Propagate result with `?`".into(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(vec![diag.clone()]),
                        edit: Some(WorkspaceEdit {
                            changes: Some({
                                let mut map = HashMap::new();
                                map.insert(
                                    params.text_document.uri.clone(),
                                    vec![TextEdit {
                                        range: Range::new(diag.range.end, diag.range.end),
                                        new_text: "?".into(),
                                    }],
                                );
                                map
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }));
                }
                "type.expected_bool" => {
                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: "Wrap in boolean check".into(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(vec![diag.clone()]),
                        ..Default::default()
                    }));
                }
                _ => {}
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }
}

impl Backend {
    async fn resolve_import_target(
        &self,
        index: &SemanticIndex,
        _name: &str,
    ) -> Option<Url> {
        let imports = index.imports();
        for import in imports {
            if let Some(file_path) = &import.file_path {
                if file_path.exists() {
                    if let Ok(url) = Url::from_file_path(file_path) {
                        return Some(url);
                    }
                }
            }
        }
        None
    }
}

fn symbol_kind_to_lsp(kind: &index::semantic::SymbolKind) -> CompletionItemKind {
    match kind {
        index::semantic::SymbolKind::Function => CompletionItemKind::FUNCTION,
        index::semantic::SymbolKind::Method => CompletionItemKind::METHOD,
        index::semantic::SymbolKind::Struct => CompletionItemKind::STRUCT,
        index::semantic::SymbolKind::Enum => CompletionItemKind::ENUM,
        index::semantic::SymbolKind::Trait => CompletionItemKind::INTERFACE,
        index::semantic::SymbolKind::Variable => CompletionItemKind::VARIABLE,
        index::semantic::SymbolKind::Parameter => CompletionItemKind::VARIABLE,
        index::semantic::SymbolKind::Field => CompletionItemKind::FIELD,
        index::semantic::SymbolKind::Import => CompletionItemKind::MODULE,
    }
}

fn semantic_kind_to_lsp(kind: &index::semantic::SymbolKind) -> SymbolKind {
    match kind {
        index::semantic::SymbolKind::Function => SymbolKind::FUNCTION,
        index::semantic::SymbolKind::Method => SymbolKind::METHOD,
        index::semantic::SymbolKind::Struct => SymbolKind::STRUCT,
        index::semantic::SymbolKind::Enum => SymbolKind::ENUM,
        index::semantic::SymbolKind::Trait => SymbolKind::INTERFACE,
        index::semantic::SymbolKind::Variable => SymbolKind::VARIABLE,
        index::semantic::SymbolKind::Field => SymbolKind::FIELD,
        _ => SymbolKind::OBJECT,
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
        references_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            ..CompletionOptions::default()
        }),
        document_symbol_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(tower_lsp::lsp_types::CodeActionProviderCapability::Simple(
            true,
        )),
        ..ServerCapabilities::default()
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
