use std::collections::HashMap;
use std::path::PathBuf;
use tower_lsp::lsp_types::Url;

use super::semantic::SemanticIndex;
use crate::utils::uri;

/// Manages the workspace: open documents, parse cache, and project root.
pub struct ProjectManager {
    /// Currently open documents (buffer in memory).
    open_documents: HashMap<Url, DocumentState>,
    /// Discovered workspace root.
    workspace_root: Option<PathBuf>,
}

struct DocumentState {
    uri: Url,
    content: String,
    version: i32,
    /// Semantic index built from the parsed AST.
    index: Option<SemanticIndex>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            open_documents: HashMap::new(),
            workspace_root: None,
        }
    }

    /// Set the workspace root (discovered during initialization).
    pub fn set_workspace_root(&mut self, root: Option<PathBuf>) {
        self.workspace_root = root;
    }

    /// Get the workspace root, if known.
    pub fn workspace_root(&self) -> Option<&PathBuf> {
        self.workspace_root.as_ref()
    }

    /// Register or update a document in memory.
    pub fn upsert_document(&mut self, uri: Url, content: String, version: i32) {
        let index = Some(SemanticIndex::build(&content));
        self.open_documents.insert(
            uri.clone(),
            DocumentState {
                uri,
                content,
                version,
                index,
            },
        );
    }

    /// Get the content of a document (from buffer or disk).
    pub fn document_content(&self, uri: &Url) -> Option<String> {
        if let Some(state) = self.open_documents.get(uri) {
            return Some(state.content.clone());
        }
        let path = uri::document_path_from_uri(uri)?;
        std::fs::read_to_string(path).ok()
    }

    /// Get the semantic index for a document, building it if needed.
    pub fn document_index(&self, uri: &Url) -> Option<&SemanticIndex> {
        self.open_documents.get(uri).and_then(|s| s.index.as_ref())
    }

    /// Remove a document (when closed).
    pub fn remove_document(&mut self, uri: &Url) {
        self.open_documents.remove(uri);
    }

    /// Check if a document is open.
    pub fn is_open(&self, uri: &Url) -> bool {
        self.open_documents.contains_key(uri)
    }
}
