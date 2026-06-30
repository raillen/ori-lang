//! Standard library catalog for LSP completion, hover, and go-to-definition.
//!
//! Merges Layer 1 (runtime manifest in `ori-types`) with Layer 2 (`.orl` sources
//! under `stdlib/`).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use ori_ast::item::{FuncDecl, Item};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Range};

use crate::utils::position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdlibLayer {
    Runtime,
    Orl,
}

#[derive(Debug, Clone)]
pub struct StdlibEntry {
    pub qualified: String,
    pub module: String,
    pub name: String,
    pub signature: String,
    pub layer: StdlibLayer,
    pub source_path: Option<PathBuf>,
    pub name_range: Option<Range>,
}

#[derive(Debug, Default)]
pub struct StdlibCatalog {
    by_qualified: HashMap<String, StdlibEntry>,
    by_module: BTreeMap<String, Vec<String>>,
    modules: BTreeSet<String>,
}

static CATALOG: OnceLock<StdlibCatalog> = OnceLock::new();

pub fn stdlib_catalog() -> &'static StdlibCatalog {
    CATALOG.get_or_init(build_catalog)
}

fn build_catalog() -> StdlibCatalog {
    let mut catalog = StdlibCatalog::default();

    for entry in ori_types::stdlib::stdlib_runtime_functions() {
        let canonical = entry.canonical_path.to_string();
        let (module, name) = canonical
            .rsplit_once('.')
            .map(|(m, n)| (m.to_string(), n.to_string()))
            .unwrap_or_else(|| (canonical.clone(), canonical.clone()));

        let signature = ori_driver::pipeline::stdlib_doc_signature(&canonical)
            .map(str::to_string)
            .unwrap_or_else(|| format!("{name}(...)"));

        catalog.insert(StdlibEntry {
            qualified: canonical.clone(),
            module: module.clone(),
            name,
            signature,
            layer: StdlibLayer::Runtime,
            source_path: None,
            name_range: None,
        });

        for alias in entry.aliases {
            catalog.insert_alias(alias, &canonical);
        }
    }

    if let Some(root) = ori_driver::pipeline::find_stdlib_root() {
        scan_stdlib_dir(&root, &root, &mut catalog);
    }

    catalog
}

impl StdlibCatalog {
    fn insert(&mut self, entry: StdlibEntry) {
        let key = entry.qualified.clone();
        self.modules.insert(entry.module.clone());
        self.by_module
            .entry(entry.module.clone())
            .or_default()
            .push(key.clone());
        self.by_qualified.insert(key, entry);
    }

    fn insert_alias(&mut self, alias: &str, canonical: &str) {
        if self.by_qualified.contains_key(alias) {
            return;
        }
        if let Some(base) = self.by_qualified.get(canonical).cloned() {
            self.insert(StdlibEntry {
                qualified: alias.to_string(),
                module: base.module,
                name: alias.rsplit('.').next().unwrap_or(alias).to_string(),
                signature: base.signature,
                layer: base.layer,
                source_path: base.source_path,
                name_range: base.name_range,
            });
        }
    }

    pub fn lookup(&self, path: &str) -> Option<&StdlibEntry> {
        self.by_qualified.get(path)
    }

    pub fn modules(&self) -> impl Iterator<Item = &String> {
        self.modules.iter()
    }

    pub fn entries_for_module(&self, module: &str) -> Vec<&StdlibEntry> {
        self.by_module
            .get(module)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.by_qualified.get(k))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn members_for_receiver(&self, receiver: &str, import_map: &HashMap<String, String>) -> Vec<&StdlibEntry> {
        if let Some(module) = import_map.get(receiver) {
            return self.entries_for_module(module);
        }
        if receiver.starts_with("ori.") {
            return self.entries_for_module(receiver);
        }
        self.entries_for_module(&format!("ori.{receiver}"))
    }

    pub fn completion_items(&self) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let mut seen = BTreeSet::new();

        for module in &self.modules {
            if seen.insert(module.clone()) {
                items.push(CompletionItem {
                    label: module.clone(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: Some("Ori stdlib module".into()),
                    ..CompletionItem::default()
                });
            }
        }

        for entry in self.by_qualified.values() {
            if !seen.insert(entry.qualified.clone()) {
                continue;
            }
            let layer = match entry.layer {
                StdlibLayer::Runtime => "Layer 1 runtime",
                StdlibLayer::Orl => "Layer 2 .orl",
            };
            items.push(CompletionItem {
                label: entry.qualified.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!("{layer}: {}", entry.signature)),
                ..CompletionItem::default()
            });
        }

        items
    }

    pub fn module_completion_items(&self, prefix: &str) -> Vec<CompletionItem> {
        self.modules()
            .filter(|m| prefix.is_empty() || m.starts_with(prefix))
            .map(|m| CompletionItem {
                label: m.clone(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Ori stdlib module".into()),
                ..CompletionItem::default()
            })
            .collect()
    }

    pub fn dot_completion_items(&self, receiver: &str, import_map: &HashMap<String, String>) -> Vec<CompletionItem> {
        self.members_for_receiver(receiver, import_map)
            .into_iter()
            .map(|entry| CompletionItem {
                label: entry.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(entry.signature.clone()),
                ..CompletionItem::default()
            })
            .collect()
    }

    pub fn hover_markdown(&self, path: &str) -> Option<String> {
        let entry = self.lookup(path)?;
        let layer = match entry.layer {
            StdlibLayer::Runtime => "Layer 1 — native runtime",
            StdlibLayer::Orl => "Layer 2 — `.orl` stdlib",
        };
        let mut md = format!(
            "```ori\n{}\n```\n\n**{}** · `{}`",
            entry.signature, layer, entry.qualified
        );
        if let Some(src) = &entry.source_path {
            md.push_str(&format!("\n\nSource: `{}`", src.display()));
        }
        Some(md)
    }

    /// Resolve a function reference (`print`, `io.print`, `ori.io.print`) to its signature.
    pub fn signature_for_call(&self, func_ref: &str, import_map: &HashMap<String, String>) -> Option<String> {
        if let Some(entry) = self.lookup(func_ref) {
            return Some(entry.signature.clone());
        }
        if let Some((receiver, method)) = func_ref.rsplit_once('.') {
            let module = import_map.get(receiver).cloned().unwrap_or_else(|| format!("ori.{receiver}"));
            let qualified = format!("{module}.{method}");
            if let Some(entry) = self.lookup(&qualified) {
                return Some(entry.signature.clone());
            }
        }
        None
    }
}

fn scan_stdlib_dir(root: &Path, dir: &Path, catalog: &mut StdlibCatalog) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_stdlib_dir(root, &path, catalog);
        } else if path.extension().is_some_and(|e| e == "orl") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                scan_stdlib_orl(&path, &content, catalog);
            }
        }
    }
}

fn scan_stdlib_orl(path: &Path, content: &str, catalog: &mut StdlibCatalog) {
    let file_id = ori_diagnostics::FileId(0);
    let mut sink = ori_diagnostics::DiagnosticSink::default();
    let tokens = ori_lexer::lex(content, file_id, &mut sink);
    let source_file = ori_parser::parse(&tokens, content, file_id, &mut sink);
    let namespace = source_file.namespace.name.to_string();

    for item in &source_file.items {
        if let Item::Func(func) = &item.item {
            if !func.visibility.is_public() {
                continue;
            }
            let qualified = format!("{}.{}", namespace, func.name.text);
            let signature = func_signature(func);
            let name_range = name_span_to_range(content, func);
            catalog.insert(StdlibEntry {
                qualified: qualified.clone(),
                module: namespace.clone(),
                name: func.name.text.to_string(),
                signature,
                layer: StdlibLayer::Orl,
                source_path: Some(path.to_path_buf()),
                name_range: Some(name_range),
            });
        }
    }
}

fn func_signature(func: &FuncDecl) -> String {
    let params: Vec<String> = func
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name.text, type_to_string(&p.ty)))
        .collect();
    let ret = func
        .return_ty
        .as_ref()
        .map(|t| format!(" -> {}", type_to_string(t)))
        .unwrap_or_default();
    format!("func {}({}){}", func.name.text, params.join(", "), ret)
}

fn type_to_string(ty: &ori_ast::ty::Type) -> String {
    match ty {
        ori_ast::ty::Type::Named(q) => q.to_string(),
        ori_ast::ty::Type::Optional(t, _) => format!("optional<{}>", type_to_string(t)),
        ori_ast::ty::Type::Result(ok, err, _) => {
            format!("result<{}, {}>", type_to_string(ok), type_to_string(err))
        }
        ori_ast::ty::Type::List(t, _) => format!("list<{}>", type_to_string(t)),
        ori_ast::ty::Type::Map(k, v, _) => {
            format!("map<{}, {}>", type_to_string(k), type_to_string(v))
        }
        ori_ast::ty::Type::Set(t, _) => format!("set<{}>", type_to_string(t)),
        ori_ast::ty::Type::Bool(_) => "bool".into(),
        ori_ast::ty::Type::Int(_) => "int".into(),
        ori_ast::ty::Type::Float(_) => "float".into(),
        ori_ast::ty::Type::String(_) => "string".into(),
        ori_ast::ty::Type::Bytes(_) => "bytes".into(),
        ori_ast::ty::Type::Void(_) => "void".into(),
        other => format!("{other:?}"),
    }
}

fn name_span_to_range(source: &str, func: &FuncDecl) -> Range {
    let start = position::position_for_byte_offset(source, func.name.span.start as usize);
    let end = position::position_for_byte_offset(source, func.name.span.end as usize);
    Range::new(start, end)
}

/// Build a map of import alias → stdlib module path from source text.
pub fn import_alias_map(source: &str) -> HashMap<String, String> {
    let file_id = ori_diagnostics::FileId(0);
    let mut sink = ori_diagnostics::DiagnosticSink::default();
    let tokens = ori_lexer::lex(source, file_id, &mut sink);
    let source_file = ori_parser::parse(&tokens, source, file_id, &mut sink);
    let mut map = HashMap::new();
    for import in &source_file.imports {
        let module = import.path.to_string();
        let alias = import
            .alias
            .as_ref()
            .map(|n| n.text.to_string())
            .unwrap_or_else(|| {
                module
                    .rsplit('.')
                    .next()
                    .unwrap_or(&module)
                    .to_string()
            });
        map.insert(alias, module);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_includes_layer1_and_layer2() {
        let catalog = stdlib_catalog();
        assert!(catalog.lookup("ori.io.print").is_some());
        assert!(catalog.lookup("ori.string.utils.is_empty").is_some());
    }

    #[test]
    fn import_alias_map_resolves_io() {
        let source = r#"
namespace app.main
import ori.io as io
func main() -> void
end
"#;
        let map = import_alias_map(source);
        assert_eq!(map.get("io"), Some(&"ori.io".to_string()));
    }
}
