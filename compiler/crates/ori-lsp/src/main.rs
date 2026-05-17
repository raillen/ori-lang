mod handlers;
mod index;
mod utils;

use index::project::ProjectManager;
use index::semantic::SemanticIndex;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionOptions, CompletionParams, CompletionResponse, DiagnosticSeverity,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams,
    InitializeParams, InitializeResult, InitializedParams, Location, MessageType, OneOf,
    SaveOptions, ServerCapabilities, ServerInfo, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// The LSP backend, holding the project manager and client handle.
struct Backend {
    client: Client,
    project: RwLock<ProjectManager>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            project: RwLock::new(ProjectManager::new()),
        }
    }

    /// Run type-check diagnostics and publish them for the given URI.
    async fn validate_uri(&self, uri: Url) {
        let source = {
            let project = self.project.read().await;
            project.document_content(&uri)
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
                        &output.cache,
                        &output.diagnostics,
                        target,
                    )
                } else {
                    Vec::new()
                }
            }
            Err(message) => vec![handlers::diagnostics::file_error_diagnostic(message)],
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Store workspace root
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
        let content = params.text_document.text;
        let version = params.text_document.version;

        self.project
            .write()
            .await
            .upsert_document(uri.clone(), content, version);
        self.validate_uri(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            // Invalidate the index — it will be rebuilt on next access
            self.project
                .write()
                .await
                .upsert_document(uri.clone(), change.text, 0);
        }
        self.validate_uri(uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.validate_uri(params.text_document.uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.project.write().await.remove_document(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let source = {
            let project = self.project.read().await;
            project.document_content(&uri)
        };
        let Some(source) = source else {
            return Ok(None);
        };

        let Some(symbol) = utils::uri::word_at_position(&source, position) else {
            return Ok(None);
        };

        // Check built-in types first
        if let Some(hover_text) = handlers::hover::builtin_type_hover(&symbol) {
            return Ok(Some(handlers::hover::markdown_hover(hover_text)));
        }

        // Special case for `it` in contracts
        if symbol == "it" && source.contains(" if it") {
            return Ok(Some(handlers::hover::markdown_hover(
                "`it`\n\nCurrent value checked by a contract on a field or parameter."
                    .to_string(),
            )));
        }

        // Check semantic index (AST-based)
        let index = {
            let project = self.project.read().await;
            project
                .document_index(&uri)
                .cloned()
                .unwrap_or_else(|| SemanticIndex::build(&source))
        };

        if let Some(hover_text) = index.hover(&symbol) {
            return Ok(Some(handlers::hover::markdown_hover(hover_text)));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let source = {
            let project = self.project.read().await;
            project.document_content(&uri)
        };
        let Some(source) = source else {
            return Ok(None);
        };

        let Some(symbol) = utils::uri::word_at_position(&source, position) else {
            return Ok(None);
        };

        // Build or retrieve the semantic index
        let index = {
            let project = self.project.read().await;
            project
                .document_index(&uri)
                .cloned()
                .unwrap_or_else(|| SemanticIndex::build(&source))
        };

        if let Some(range) = index.definition(&symbol) {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location::new(uri, range))));
        }

        Ok(None)
    }

    async fn completion(
        &self,
        _params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let mut items = handlers::completion::stdlib_completion_items();
        items.extend(handlers::completion::keyword_completion_items());
        items.extend(handlers::completion::snippet_completion_items());
        items.sort_by(|a, b| a.label.cmp(&b.label));
        Ok(Some(CompletionResponse::Array(items)))
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

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
