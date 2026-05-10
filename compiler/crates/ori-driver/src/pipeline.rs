use std::path::Path;
use ori_diagnostics::{Diagnostic, DiagnosticSink, SourceCache};
use ori_lexer::Token;
use ori_ast::item::SourceFile;
use ori_types::resolve::ResolvedModule;

// ── Output types ──────────────────────────────────────────────────────────────

pub struct LexOutput {
    pub cache:       SourceCache,
    pub tokens:      Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct ParseOutput {
    pub cache:       SourceCache,
    pub ast:         SourceFile,
    pub tokens:      Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct CheckOutput {
    pub cache:       SourceCache,
    pub resolved:    ResolvedModule,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors:  bool,
}

// ── Pipeline steps ────────────────────────────────────────────────────────────

/// Read `path` from disk, lex it and return the token stream.
pub fn run_lex(path: &Path) -> Result<LexOutput, String> {
    let source = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(path, source.clone());
    let tokens    = ori_lexer::lex(&source, file_id, &mut sink);
    let diags     = sink.into_diagnostics();
    Ok(LexOutput { cache, tokens, diagnostics: diags })
}

/// Read + lex + parse. Returns the AST (possibly partial on errors).
pub fn run_parse(path: &Path) -> Result<ParseOutput, String> {
    let source  = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(path, source.clone());
    let tokens    = ori_lexer::lex(&source, file_id, &mut sink);
    let ast       = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let diags     = sink.into_diagnostics();
    Ok(ParseOutput { cache, ast, tokens, diagnostics: diags })
}

pub struct CompileOutput {
    pub cache:       SourceCache,
    pub exe_path:    std::path::PathBuf,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors:  bool,
}

/// Full pipeline → Cranelift object → linker → native binary.
pub fn run_compile(source_path: &Path, output: &Path) -> Result<CompileOutput, String> {
    let source  = read_file(source_path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(source_path, source.clone());

    let tokens   = ori_lexer::lex(&source, file_id, &mut sink);
    let ast      = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let resolved = ori_types::resolve::resolve(&ast, file_id, &mut sink);

    if !sink.has_errors() {
        let mut checker = ori_types::check::Checker::new(
            &resolved.def_map, &resolved.namespace, file_id, &mut sink,
        );
        checker.check_file(&ast);
    }

    if !sink.has_errors() {
        let hir = ori_hir::lower(&ast, &resolved.def_map, &resolved.namespace, file_id, &mut sink);
        let obj_path = output.with_extension("o");
        ori_codegen::emit_native(&hir, &obj_path)?;
        ori_codegen::link(&obj_path, output)?;
        let _ = std::fs::remove_file(&obj_path); // clean up intermediate .o
    }

    let has_errors  = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(CompileOutput { cache, exe_path: output.to_owned(), diagnostics, has_errors })
}

/// Full pipeline: lex → parse → resolve names → type-check.
pub fn run_check(path: &Path) -> Result<CheckOutput, String> {
    let source  = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(path, source.clone());

    // Lex
    let tokens = ori_lexer::lex(&source, file_id, &mut sink);

    // Parse — continue even with lex errors
    let ast = ori_parser::parse(&tokens, &source, file_id, &mut sink);

    // Name resolution
    let resolved = ori_types::resolve::resolve(&ast, file_id, &mut sink);

    // Type checking — only if no fatal parse errors so far
    if !sink.has_errors() {
        let mut checker = ori_types::check::Checker::new(
            &resolved.def_map,
            &resolved.namespace,
            file_id,
            &mut sink,
        );
        checker.check_file(&ast);
    }

    let has_errors  = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(CheckOutput { cache, resolved, diagnostics, has_errors })
}

pub struct BuildOutput {
    pub cache:       SourceCache,
    pub c_source:    String,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors:  bool,
}

/// Full pipeline + HIR lowering + C code generation.
pub fn run_build(path: &Path) -> Result<BuildOutput, String> {
    let source  = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(path, source.clone());

    let tokens   = ori_lexer::lex(&source, file_id, &mut sink);
    let ast      = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let resolved = ori_types::resolve::resolve(&ast, file_id, &mut sink);

    if !sink.has_errors() {
        let mut checker = ori_types::check::Checker::new(
            &resolved.def_map, &resolved.namespace, file_id, &mut sink,
        );
        checker.check_file(&ast);
    }

    let c_source = if !sink.has_errors() {
        let hir = ori_hir::lower(&ast, &resolved.def_map, &resolved.namespace, file_id, &mut sink);
        ori_codegen::emit_c(&hir)
    } else {
        String::new()
    };

    let has_errors  = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(BuildOutput { cache, c_source, diagnostics, has_errors })
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn read_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read `{}`: {}", path.display(), e))
}
