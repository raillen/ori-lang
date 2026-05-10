use std::path::Path;
use ori_diagnostics::{Diagnostic, DiagnosticSink, SourceCache};
use ori_lexer::Token;
use ori_ast::item::SourceFile;
use ori_types::resolve::ResolvedModule;

// â”€â”€ Output types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

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

// â”€â”€ Pipeline steps â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

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

/// Full pipeline â†’ Cranelift object â†’ linker â†’ native binary.
pub fn run_compile(source_path: &Path, output: &Path) -> Result<CompileOutput, String> {
    let source  = read_file(source_path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(source_path, source.clone());

    let tokens   = ori_lexer::lex(&source, file_id, &mut sink);
    let ast      = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let resolved = ori_types::resolve::resolve(&ast, file_id, &mut sink);

    if !sink.has_errors() {
        let mut checker = ori_types::check::Checker::new(&resolved.def_map, &resolved.func_sigs, &resolved.namespace, file_id, &mut sink);
        checker.check_file(&ast);
    }

    if !sink.has_errors() {
        let hir = ori_hir::lower(&ast, &resolved.def_map, &resolved.namespace, file_id, &mut sink);
        let obj_path  = output.with_extension("o");
        let rt_lib    = build_runtime_lib()?;
        ori_codegen::emit_native(&hir, &obj_path)?;
        let extra: Vec<_> = rt_lib.into_iter().collect();
        ori_codegen::link(&obj_path, output, &extra)?;
        let _ = std::fs::remove_file(&obj_path);
        for e in &extra { let _ = std::fs::remove_file(e); }
    }

    let has_errors  = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(CompileOutput { cache, exe_path: output.to_owned(), diagnostics, has_errors })
}

/// Full pipeline: lex â†’ parse â†’ resolve names â†’ type-check.
pub fn run_check(path: &Path) -> Result<CheckOutput, String> {
    let source  = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let file_id   = cache.add(path, source.clone());

    // Lex
    let tokens = ori_lexer::lex(&source, file_id, &mut sink);

    // Parse â€” continue even with lex errors
    let ast = ori_parser::parse(&tokens, &source, file_id, &mut sink);

    // Name resolution
    let resolved = ori_types::resolve::resolve(&ast, file_id, &mut sink);

    // Type checking â€” only if no fatal parse errors so far
    if !sink.has_errors() {
        let mut checker = ori_types::check::Checker::new(&resolved.def_map, &resolved.func_sigs, &resolved.namespace, file_id, &mut sink);
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
        let mut checker = ori_types::check::Checker::new(&resolved.def_map, &resolved.func_sigs, &resolved.namespace, file_id, &mut sink);
        checker.check_file(&ast);
    }

    let c_source = if !sink.has_errors() {
        let hir = ori_hir::lower(
            &ast, &resolved.def_map, &resolved.namespace, file_id, &mut sink
        );
        ori_codegen::emit_c(&hir)
    } else {
        String::new()
    };

    let has_errors  = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(BuildOutput { cache, c_source, diagnostics, has_errors })
}

// â”€â”€ Utilities â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn read_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read `{}`: {}", path.display(), e))
}

/// The Ori runtime as embedded C source â€” compiled on demand with `cc -c`.
/// This avoids linking issues from Rust staticlibs pulling in Rust std.
const ORI_RUNTIME_C: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ori.io.print(ptr: *u8, len: i64) */
void ori_io_print(const char* ptr, long long len) {
    if (!ptr || len <= 0) { printf("\n"); return; }
    fwrite(ptr, 1, (size_t)len, stdout);
    printf("\n");
    fflush(stdout);
}

/* ori.io.eprint(ptr: *u8, len: i64) */
void ori_io_eprint(const char* ptr, long long len) {
    if (!ptr || len <= 0) { fprintf(stderr, "\n"); return; }
    fwrite(ptr, 1, (size_t)len, stderr);
    fprintf(stderr, "\n");
    fflush(stderr);
}

/* ori_int_to_cstr(n: i64) -> *u8  (malloc'd, caller must free) */
char* ori_int_to_cstr(long long n) {
    char* buf = (char*)malloc(32);
    if (buf) snprintf(buf, 32, "%lld", (long long)n);
    return buf;
}

/* ori_to_string(n: i64) -> *u8  (same as ori_int_to_cstr) */
char* ori_to_string(long long n) {
    return ori_int_to_cstr(n);
}

/* ori_len(ptr: *u8) -> i64  (strlen) */
long long ori_len(const char* ptr) {
    if (!ptr) return 0;
    return (long long)strlen(ptr);
}

/* ori_math_abs(n: i64) -> i64 */
long long ori_math_abs(long long n) {
    return n < 0 ? -n : n;
}

/* ori_math_min(a: i64, b: i64) -> i64 */
long long ori_math_min(long long a, long long b) {
    return a < b ? a : b;
}

/* ori_math_max(a: i64, b: i64) -> i64 */
long long ori_math_max(long long a, long long b) {
    return a > b ? a : b;
}
"#;

/// Compile the embedded C runtime to an object file and return its path.
/// The object file is placed alongside `output` and cleaned up after linking.
fn build_runtime_lib() -> Result<Option<std::path::PathBuf>, String> {
    let tmp_c   = std::env::temp_dir().join("ori_rt.c");
    let tmp_obj = std::env::temp_dir().join("ori_rt.o");

    std::fs::write(&tmp_c, ORI_RUNTIME_C)
        .map_err(|e| format!("write ori_rt.c: {e}"))?;

    let status = std::process::Command::new("cc")
        .arg("-c")
        .arg(&tmp_c)
        .arg("-o")
        .arg(&tmp_obj)
        .status()
        .map_err(|e| format!("cc -c ori_rt.c: {e}"))?;

    let _ = std::fs::remove_file(&tmp_c);

    if status.success() {
        Ok(Some(tmp_obj))
    } else {
        Ok(None) // cc not available; functions will be unresolved at runtime
    }
}
