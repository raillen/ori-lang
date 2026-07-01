use std::collections::HashMap;
use std::path::PathBuf;
use tower_lsp::lsp_types::{Position, Range};

use crate::utils::position;
use crate::utils::uri;

/// A symbol extracted from source code for hover and navigation.
#[derive(Clone, Debug)]
pub struct SemanticSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub hover: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Variable,
    Parameter,
    Field,
    Import,
}

impl SymbolKind {
    pub fn display(&self) -> &'static str {
        match self {
            SymbolKind::Function => "function",
            SymbolKind::Method => "method",
            SymbolKind::Struct => "struct",
            SymbolKind::Enum => "enum",
            SymbolKind::Trait => "trait",
            SymbolKind::Variable => "variable",
            SymbolKind::Parameter => "parameter",
            SymbolKind::Field => "field",
            SymbolKind::Import => "import",
        }
    }
}

/// Information about a resolved import for cross-file navigation.
#[derive(Clone, Debug)]
pub struct ResolvedImport {
    pub alias: String,
    pub namespace: String,
    /// File path where the imported symbols are defined.
    pub file_path: Option<PathBuf>,
}

/// AST-based semantic index for a single file.
#[derive(Default, Clone)]
pub struct SemanticIndex {
    symbols: HashMap<String, Vec<SemanticSymbol>>,
    symbols_by_kind: HashMap<SymbolKind, Vec<SemanticSymbol>>,
    /// All import paths discovered in the file (for cross-file resolution).
    imports: Vec<ResolvedImport>,
}

impl SemanticIndex {
    pub fn build(source: &str) -> Self {
        let mut index = Self::default();
        index.index_ast(source);
        index
    }

    pub fn hover(&self, symbol: &str) -> Option<String> {
        let entries = self.symbols.get(symbol)?;
        if entries.len() == 1 {
            return Some(entries[0].hover.clone());
        }
        let summaries: Vec<_> = entries
            .iter()
            .map(|entry| format!("- {}: {}", entry.kind.display(), entry.summary))
            .collect();
        Some(format!(
            "Multiple local symbols named `{symbol}`:\n\n{}",
            summaries.join("\n")
        ))
    }

    pub fn definition(&self, symbol: &str) -> Option<Range> {
        self.symbols
            .get(symbol)
            .and_then(|entries| entries.first())
            .map(|entry| entry.range)
    }

    /// Find all references to a symbol name in the source text.
    /// Uses word-boundary scanning to find identifiers matching the name.
    pub fn find_references(&self, source: &str, symbol: &str) -> Vec<Range> {
        let mut refs = Vec::new();
        let bytes = source.as_bytes();
        let sym_bytes = symbol.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            // Skip non-identifier bytes
            if !is_ident_byte(bytes[i]) {
                i += 1;
                continue;
            }

            // Check if this is a word boundary match
            let start = i;
            while i < bytes.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }

            let word = &bytes[start..i];
            if word == sym_bytes {
                let range = Range::new(
                    position::position_for_byte_offset(source, start),
                    position::position_for_byte_offset(source, i),
                );
                refs.push(range);
            }
        }
        refs
    }

    /// Returns import information for cross-file navigation.
    pub fn imports(&self) -> &[ResolvedImport] {
        &self.imports
    }

    /// Find a symbol by its position in the source (for context-aware operations).
    pub fn symbol_at(&self, source: &str, pos: Position) -> Option<&SemanticSymbol> {
        let word = uri::word_at_position(source, pos)?;
        self.symbols
            .get(&word)?
            .iter()
            .find(|entry| position_in_range(pos, &entry.range))
    }

    /// Determine completion context based on cursor position.
    pub fn completion_context(&self, source: &str, pos: Position) -> CompletionContext {
        let offset = position::byte_offset_for_position(source, pos);
        let before = &source[..offset.min(source.len())];

        // Check if we're after a dot (field/method access)
        if let Some(dot_pos) = before.rfind('.') {
            // Check that the dot is part of an identifier chain, not a number
            let after_dot = &before[dot_pos + 1..];
            if !after_dot.contains(|c: char| c.is_whitespace() || c == '\n')
                && !after_dot.contains(')')
            {
                // Find the receiver name before the dot
                let before_dot = &before[..dot_pos];
                if let Some(receiver) = before_dot
                    .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                {
                    if !receiver.is_empty() {
                        return CompletionContext::AfterDot {
                            receiver: receiver.to_string(),
                        };
                    }
                }
            }
        }

        // Check if we're in an import path
        if let Some(import_pos) = before.rfind("import ") {
            let after_import = &before[import_pos + 7..];
            if !after_import.contains('\n') {
                return CompletionContext::Import;
            }
        }

        CompletionContext::Default
    }

    /// All symbols in the index.
    pub fn all_symbols(&self) -> impl Iterator<Item = &SemanticSymbol> {
        self.symbols.values().flat_map(|v| v.iter())
    }

    fn add(&mut self, symbol: SemanticSymbol) {
        self.symbols
            .entry(symbol.name.clone())
            .or_default()
            .push(symbol.clone());
        self.symbols_by_kind
            .entry(symbol.kind.clone())
            .or_default()
            .push(symbol);
    }

    fn index_ast(&mut self, source: &str) {
        let file_id = ori_diagnostics::FileId(0);
        let mut sink = ori_diagnostics::DiagnosticSink::default();
        let tokens = ori_lexer::lex(source, file_id, &mut sink);
        let source_file = ori_parser::parse(&tokens, source, file_id, &mut sink);

        for item_with_attrs in &source_file.items {
            self.index_item(&item_with_attrs.item, source);
        }

        for import in &source_file.imports {
            let namespace = import.path.to_string();
            let file_path = ori_driver::pipeline::stdlib_source_path(&namespace);
            if import.selected.is_empty() {
                let alias = import
                    .alias
                    .as_ref()
                    .map(|n| n.text.to_string())
                    .unwrap_or_else(|| {
                        namespace
                            .rsplit('.')
                            .next()
                            .unwrap_or(&namespace)
                            .to_string()
                    });
                let alias_span = import
                    .alias
                    .as_ref()
                    .map(|name| name.span)
                    .unwrap_or(import.path.span);
                self.imports.push(ResolvedImport {
                    alias: alias.clone(),
                    namespace: namespace.clone(),
                    file_path: file_path.clone(),
                });
                self.add(SemanticSymbol {
                    name: alias,
                    kind: SymbolKind::Import,
                    range: span_to_range(source, alias_span),
                    hover: format!("```ori\nimport {namespace}\n```\n\nModule import."),
                    summary: format!("import {namespace}"),
                });
            } else {
                for item in &import.selected {
                    let alias = item
                        .alias
                        .as_ref()
                        .map(|n| n.text.to_string())
                        .unwrap_or_else(|| item.name.text.to_string());
                    let selected_namespace = format!("{}.{}", namespace, item.name.text);
                    self.imports.push(ResolvedImport {
                        alias: alias.clone(),
                        namespace: selected_namespace.clone(),
                        file_path: file_path.clone(),
                    });
                    self.add(SemanticSymbol {
                        name: alias,
                        kind: SymbolKind::Import,
                        range: span_to_range(source, item.span),
                        hover: format!(
                            "```ori\nimport {namespace} only ({})\n```\n\nSelective import.",
                            item.name.text
                        ),
                        summary: format!("import {selected_namespace}"),
                    });
                }
            }
        }
    }

    fn index_item(&mut self, item: &ori_ast::item::Item, source: &str) {
        match item {
            ori_ast::item::Item::Func(func) => {
                let range = span_to_range(source, func.span);
                let signature = func_signature(func);
                let hover = format!("```ori\n{signature}\n```\n\nUser-defined function.");
                self.add(SemanticSymbol {
                    name: func.name.text.to_string(),
                    kind: SymbolKind::Function,
                    range,
                    hover,
                    summary: format!("function {}", func.name.text),
                });
                for param in &func.params {
                    let p_range = span_to_range(source, param.span);
                    self.add(SemanticSymbol {
                        name: param.name.text.to_string(),
                        kind: SymbolKind::Parameter,
                        range: p_range,
                        hover: format!(
                            "```ori\n{}: {}\n```\n\nFunction parameter.",
                            param.name.text,
                            type_to_string(&param.ty)
                        ),
                        summary: format!("parameter {}", param.name.text),
                    });
                }
            }
            ori_ast::item::Item::Struct(s) => {
                let range = span_to_range(source, s.span);
                let field_list: Vec<_> = s
                    .fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name.text, type_to_string(&f.ty)))
                    .collect();
                let hover = format!(
                    "```ori\nstruct {}\n```\n\nFields:\n{}",
                    s.name.text,
                    field_list
                        .iter()
                        .map(|f| format!("- `{f}`"))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                self.add(SemanticSymbol {
                    name: s.name.text.to_string(),
                    kind: SymbolKind::Struct,
                    range,
                    hover,
                    summary: format!("struct {}", s.name.text),
                });
                for field in &s.fields {
                    let f_range = span_to_range(source, field.span);
                    self.add(SemanticSymbol {
                        name: field.name.text.to_string(),
                        kind: SymbolKind::Field,
                        range: f_range,
                        hover: format!(
                            "```ori\n{}: {}\n```\n\nField of `struct {}`.",
                            field.name.text,
                            type_to_string(&field.ty),
                            s.name.text
                        ),
                        summary: format!("field {}.{}", s.name.text, field.name.text),
                    });
                }
            }
            ori_ast::item::Item::Enum(e) => {
                let range = span_to_range(source, e.span);
                let variant_list: Vec<_> =
                    e.variants.iter().map(|v| v.name.text.to_string()).collect();
                let hover = format!(
                    "```ori\nenum {}\n```\n\nVariants:\n{}",
                    e.name.text,
                    variant_list
                        .iter()
                        .map(|v| format!("- `{v}`"))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                self.add(SemanticSymbol {
                    name: e.name.text.to_string(),
                    kind: SymbolKind::Enum,
                    range,
                    hover,
                    summary: format!("enum {}", e.name.text),
                });
            }
            ori_ast::item::Item::Trait(t) => {
                let range = span_to_range(source, t.span);
                self.add(SemanticSymbol {
                    name: t.name.text.to_string(),
                    kind: SymbolKind::Trait,
                    range,
                    hover: format!("```ori\ntrait {}\n```\n\nUser-defined trait.", t.name.text),
                    summary: format!("trait {}", t.name.text),
                });
            }
            ori_ast::item::Item::Const(c) => {
                let range = span_to_range(source, c.span);
                self.add(SemanticSymbol {
                    name: c.name.text.to_string(),
                    kind: SymbolKind::Variable,
                    range,
                    hover: format!(
                        "```ori\nconst {}: {}\n```\n\nLocal constant binding.",
                        c.name.text,
                        type_to_string(&c.ty)
                    ),
                    summary: format!("const {}", c.name.text),
                });
            }
            ori_ast::item::Item::Var(v) => {
                let range = span_to_range(source, v.span);
                self.add(SemanticSymbol {
                    name: v.name.text.to_string(),
                    kind: SymbolKind::Variable,
                    range,
                    hover: format!(
                        "```ori\nvar {}: {}\n```\n\nLocal mutable binding.",
                        v.name.text,
                        type_to_string(&v.ty)
                    ),
                    summary: format!("var {}", v.name.text),
                });
            }
            ori_ast::item::Item::Implement(imp) => {
                for method in &imp.methods {
                    let range = span_to_range(source, method.span);
                    let sig = func_signature(method);
                    let hover = format!("```ori\n{sig}\n```\n\nMethod implementation.");
                    self.add(SemanticSymbol {
                        name: method.name.text.to_string(),
                        kind: SymbolKind::Method,
                        range,
                        hover,
                        summary: format!(
                            "method {}.{}",
                            imp.for_type.last().text,
                            method.name.text
                        ),
                    });
                }
            }
            _ => {}
        }
    }
}

/// Describes what kind of completion the user expects at the cursor position.
#[derive(Debug, Clone)]
pub enum CompletionContext {
    /// After a dot: `receiver.` — suggest fields or methods.
    AfterDot { receiver: String },
    /// Inside an import statement: `import ` — suggest modules.
    Import,
    /// Default context — suggest everything.
    Default,
}

fn func_signature(func: &ori_ast::item::FuncDecl) -> String {
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
    let modifiers = if func.is_mut { "mut " } else { "" };
    format!(
        "{}func {}({}){}",
        modifiers,
        func.name.text,
        params.join(", "),
        ret
    )
}

fn type_to_string(ty: &ori_ast::ty::Type) -> String {
    match ty {
        ori_ast::ty::Type::Bool(_) => "bool".to_string(),
        ori_ast::ty::Type::Int(_) => "int".to_string(),
        ori_ast::ty::Type::Int8(_) => "int8".to_string(),
        ori_ast::ty::Type::Int16(_) => "int16".to_string(),
        ori_ast::ty::Type::Int32(_) => "int32".to_string(),
        ori_ast::ty::Type::Int64(_) => "int64".to_string(),
        ori_ast::ty::Type::U8(_) => "u8".to_string(),
        ori_ast::ty::Type::U16(_) => "u16".to_string(),
        ori_ast::ty::Type::U32(_) => "u32".to_string(),
        ori_ast::ty::Type::U64(_) => "u64".to_string(),
        ori_ast::ty::Type::Float(_) => "float".to_string(),
        ori_ast::ty::Type::Float32(_) => "float32".to_string(),
        ori_ast::ty::Type::Float64(_) => "float64".to_string(),
        ori_ast::ty::Type::String(_) => "string".to_string(),
        ori_ast::ty::Type::Bytes(_) => "bytes".to_string(),
        ori_ast::ty::Type::Void(_) => "void".to_string(),
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
        ori_ast::ty::Type::Range(t, _) => format!("range<{}>", type_to_string(t)),
        ori_ast::ty::Type::Tuple(types, _) => {
            let inner: Vec<_> = types.iter().map(type_to_string).collect();
            format!("({})", inner.join(", "))
        }
        ori_ast::ty::Type::Func {
            params, return_ty, ..
        } => {
            let p: Vec<_> = params.iter().map(type_to_string).collect();
            let ret = return_ty
                .as_ref()
                .map(|t| format!(" -> {}", type_to_string(t)))
                .unwrap_or_default();
            format!("func({}){}", p.join(", "), ret)
        }
        ori_ast::ty::Type::Generic { name, args, .. } => {
            let a: Vec<_> = args.iter().map(type_to_string).collect();
            if a.is_empty() {
                name.to_string()
            } else {
                format!("{}<{}>", name.to_string(), a.join(", "))
            }
        }
        _ => "?".to_string(),
    }
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn span_to_range(source: &str, span: ori_diagnostics::Span) -> Range {
    let start = position::position_for_byte_offset(source, span.start as usize);
    let end = position::position_for_byte_offset(source, span.end as usize);
    Range::new(start, end)
}

fn position_in_range(pos: Position, range: &Range) -> bool {
    !position_is_before(pos, range.start) && position_is_before(pos, range.end)
}

fn position_is_before(left: Position, right: Position) -> bool {
    left.line < right.line || (left.line == right.line && left.character < right.character)
}
