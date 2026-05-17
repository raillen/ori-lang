use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, Url};

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

/// AST-based semantic index for a single file.
///
/// Uses the real Ori parser (`ori_parser`) to extract symbols with precise
/// spans, replacing the previous regex-based approach.
#[derive(Default, Clone)]
pub struct SemanticIndex {
    symbols: HashMap<String, Vec<SemanticSymbol>>,
    /// Symbols grouped by kind for completion filtering.
    symbols_by_kind: HashMap<SymbolKind, Vec<SemanticSymbol>>,
}

impl SemanticIndex {
    /// Build a semantic index from Ori source text.
    pub fn build(source: &str) -> Self {
        let mut index = Self::default();
        index.index_ast(source);
        index
    }

    /// Find hover information for a symbol name.
    pub fn hover(&self, symbol: &str) -> Option<String> {
        let entries = self.symbols.get(symbol)?;
        if entries.len() == 1 {
            return Some(entries[0].hover.clone());
        }

        let summaries: Vec<_> = entries
            .iter()
            .map(|entry| format!("- {}", entry.summary))
            .collect();
        Some(format!(
            "Multiple local symbols named `{symbol}`:\n\n{}",
            summaries.join("\n")
        ))
    }

    /// Find the definition location for a symbol name.
    /// Returns the range of the first declaration found.
    pub fn definition(&self, symbol: &str) -> Option<Range> {
        self.symbols
            .get(symbol)
            .and_then(|entries| entries.first())
            .map(|entry| entry.range)
    }

    /// Find all symbols matching a name prefix (for completions).
    pub fn completions_for_prefix(&self, prefix: &str) -> Vec<&SemanticSymbol> {
        self.symbols
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .flat_map(|(_, entries)| entries)
            .collect()
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

    /// Use the real Ori parser to extract symbols from source code.
    fn index_ast(&mut self, source: &str) {
        let file_id = ori_diagnostics::FileId(0);
        let mut sink = ori_diagnostics::DiagnosticSink::default();
        let tokens = ori_lexer::lex(source, file_id, &mut sink);
        let source_file = ori_parser::parse(&tokens, source, file_id, &mut sink);

        // Index items from the parsed AST
        for item_with_attrs in &source_file.items {
            self.index_item(&item_with_attrs.item, source);
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

                // Index parameters
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

                // Index fields
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
                    hover: format!(
                        "```ori\ntrait {}\n```\n\nUser-defined trait.",
                        t.name.text
                    ),
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
                            imp.for_type.last().text, method.name.text
                        ),
                    });
                }
            }
            _ => {}
        }
    }

    /// Fallback to simple keyword/identifier scanning when the parser fails.
    fn index_fallback(&mut self, source: &str) {
        let lines: Vec<&str> = source.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim_start();
            let stripped = strip_item_prefixes(line);

            // Try to detect declarations via simple keyword matching
            for keyword in &["func", "struct", "enum", "trait", "const", "var"] {
                if let Some(rest) = stripped.strip_prefix(&format!("{keyword} ")) {
                    if let Some((name, _)) = uri::take_identifier(rest) {
                        let range = position::range_for_line_and_columns(
                            i,
                            line.find(name).unwrap_or(0),
                            i,
                            line.find(name).unwrap_or(0) + name.len(),
                        );
                        let kind = match *keyword {
                            "func" => SymbolKind::Function,
                            "struct" => SymbolKind::Struct,
                            "enum" => SymbolKind::Enum,
                            "trait" => SymbolKind::Trait,
                            _ => SymbolKind::Variable,
                        };
                        self.add(SemanticSymbol {
                            name: name.to_string(),
                            kind,
                            range,
                            hover: format!("`{name}` — {keyword} declaration."),
                            summary: format!("{keyword} {name}"),
                        });
                    }
                    break;
                }
            }
            i += 1;
        }
    }
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
        ori_ast::ty::Type::Func { params, return_ty, .. } => {
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

fn span_to_range(source: &str, span: ori_diagnostics::Span) -> Range {
    let start = position::position_for_byte_offset(source, span.start as usize);
    let end = position::position_for_byte_offset(source, span.end as usize);
    Range::new(start, end)
}
