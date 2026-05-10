use std::collections::HashSet;
use std::path::{Path, PathBuf};
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label, SourceCache};
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

struct LoadedSource {
    path:    PathBuf,
    file_id: FileId,
    ast:     SourceFile,
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
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(source_path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    if !sink.has_errors() {
        let hir = lower_loaded_sources(&loaded, &resolved, &mut sink);
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
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(path, &mut cache, &mut sink)?;

    // Type checking â€” only if no fatal parse errors so far
    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
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
    let mut cache = SourceCache::default();
    let mut sink  = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    let c_source = if !sink.has_errors() {
        let hir = lower_loaded_sources(&loaded, &resolved, &mut sink);
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

fn load_and_resolve(
    path: &Path,
    cache: &mut SourceCache,
    sink: &mut DiagnosticSink,
) -> Result<(Vec<LoadedSource>, ResolvedModule), String> {
    let mut loaded = Vec::new();
    let mut seen = HashSet::new();
    load_source_recursive(path, cache, sink, &mut seen, &mut loaded)?;
    let entry_namespace = loaded.first()
        .map(|s| namespace_of(&s.ast))
        .unwrap_or_default();
    let files: Vec<_> = loaded.iter().map(|s| (&s.ast, s.file_id)).collect();
    let resolved = ori_types::resolve::resolve_many(&files, entry_namespace, sink);
    Ok((loaded, resolved))
}

fn load_source_recursive(
    path: &Path,
    cache: &mut SourceCache,
    sink: &mut DiagnosticSink,
    seen: &mut HashSet<PathBuf>,
    loaded: &mut Vec<LoadedSource>,
) -> Result<(), String> {
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_owned());
    if !seen.insert(path.clone()) {
        return Ok(());
    }
    let source = read_file(&path)?;
    let file_id = cache.add(&path, source.clone());
    let tokens = ori_lexer::lex(&source, file_id, sink);
    let ast = ori_parser::parse(&tokens, &source, file_id, sink);
    let imports: Vec<_> = ast.imports.iter()
        .map(|i| (i.path.to_string(), i.span))
        .collect();
    loaded.push(LoadedSource { path: path.clone(), file_id, ast });
    for (import, span) in imports {
        if is_stdlib_import(&import) {
            continue;
        }
        if let Some(import_path) = find_import_path(&path, &import) {
            load_source_recursive(&import_path, cache, sink, seen, loaded)?;
            if let Some(imported) = loaded.iter().find(|s| s.path == import_path) {
                let declared = namespace_of(&imported.ast);
                if declared != import {
                    sink.emit(
                        Diagnostic::error(
                            "bind.import_namespace_mismatch",
                            format!("import `{}` resolved to file declaring `{}`", import, declared),
                        )
                            .with_label(Label::primary(file_id, span, "imported here"))
                            .with_action("make the imported file namespace match the import path"),
                    );
                }
            }
        } else {
            sink.emit(
                Diagnostic::error("bind.import_not_found", format!("import `{}` not found", import))
                    .with_label(Label::primary(file_id, span, "imported here"))
                    .with_action("place the imported namespace in a matching `.orl` file"),
            );
        }
    }
    Ok(())
}

fn namespace_of(file: &SourceFile) -> String {
    file.namespace.name.to_string()
}

fn is_stdlib_import(import: &str) -> bool {
    import == "ori" || import.starts_with("ori.")
}

fn find_import_path(importer: &Path, import: &str) -> Option<PathBuf> {
    let dir = importer.parent()?;
    for base in dir.ancestors() {
        for candidate in import_candidates(base, import) {
            if candidate.is_file() {
                return Some(std::fs::canonicalize(&candidate).unwrap_or(candidate));
            }
        }
    }
    None
}

fn import_candidates(base: &Path, import: &str) -> Vec<PathBuf> {
    let parts: Vec<_> = import.split('.').filter(|p| !p.is_empty()).collect();
    let mut candidates = Vec::new();
    if !parts.is_empty() {
        let mut nested = base.to_path_buf();
        for part in &parts {
            nested.push(part);
        }
        nested.set_extension("orl");
        candidates.push(nested);
        if let Some(last) = parts.last() {
            candidates.push(base.join(format!("{last}.orl")));
        }
    }
    candidates
}

fn check_loaded_sources(
    loaded: &[LoadedSource],
    resolved: &ResolvedModule,
    sink: &mut DiagnosticSink,
) {
    for source in loaded {
        let namespace = namespace_of(&source.ast);
        let mut checker = ori_types::check::Checker::new(
            &resolved.def_map,
            &resolved.func_sigs,
            &resolved.value_sigs,
            &namespace,
            source.file_id,
            sink,
        );
        checker.check_file(&source.ast);
    }
}

fn lower_loaded_sources(
    loaded: &[LoadedSource],
    resolved: &ResolvedModule,
    sink: &mut DiagnosticSink,
) -> ori_hir::HirModule {
    let (first, rest) = loaded.split_first().expect("entry source is loaded");
    let first_namespace = namespace_of(&first.ast);
    let mut merged = ori_hir::lower(
        &first.ast,
        &resolved.def_map,
        &first_namespace,
        first.file_id,
        sink,
    );
    for source in rest {
        let namespace = namespace_of(&source.ast);
        let mut hir = ori_hir::lower(
            &source.ast,
            &resolved.def_map,
            &namespace,
            source.file_id,
            sink,
        );
        merged.structs.append(&mut hir.structs);
        merged.enums.append(&mut hir.enums);
        merged.funcs.append(&mut hir.funcs);
        merged.consts.append(&mut hir.consts);
    }
    merged
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

/* ---------- optional<int> helpers (value = i64) ---------- */
typedef struct { char has_value; long long value; } ori_opt_i64_t;

ori_opt_i64_t ori_some_i64(long long v) {
    ori_opt_i64_t r; r.has_value = 1; r.value = v; return r;
}
ori_opt_i64_t ori_none_i64(void) {
    ori_opt_i64_t r; r.has_value = 0; r.value = 0; return r;
}

/* ---------- result<int, *char> helpers ---------- */
typedef struct { char is_ok; long long ok; const char* err; } ori_result_i64_str_t;

ori_result_i64_str_t ori_success_i64(long long v) {
    ori_result_i64_str_t r; r.is_ok = 1; r.ok = v; r.err = 0; return r;
}
ori_result_i64_str_t ori_error_str(const char* e) {
    ori_result_i64_str_t r; r.is_ok = 0; r.ok = 0; r.err = e; return r;
}

/* ---------- list<T> (dynamic array of i64) ---------- */
/* All list operations work on i64 elements for now (covers int, bool, pointer) */
typedef struct { long long* data; long long len; long long cap; } ori_list_t;

ori_list_t* ori_list_new(void) {
    ori_list_t* l = (ori_list_t*)malloc(sizeof(ori_list_t));
    l->data = (long long*)malloc(8 * sizeof(long long));
    l->len  = 0;
    l->cap  = 8;
    return l;
}
void ori_list_push(ori_list_t* l, long long v) {
    if (l->len >= l->cap) {
        l->cap *= 2;
        l->data = (long long*)realloc(l->data, (size_t)l->cap * sizeof(long long));
    }
    l->data[l->len++] = v;
}
long long ori_list_get(ori_list_t* l, long long i) {
    if (i < 0 || i >= l->len) return 0;
    return l->data[i];
}
void ori_list_set(ori_list_t* l, long long i, long long v) {
    if (i >= 0 && i < l->len) l->data[i] = v;
}
long long ori_list_len(ori_list_t* l) { return l ? l->len : 0; }
void ori_list_free(ori_list_t* l) {
    if (l) { free(l->data); free(l); }
}

/* ---------- generic optional/result helpers ---------- */
/* These operate on pointer-sized optional (has_value + ptr) */
typedef struct { char has_value; void* ptr; } ori_opt_ptr_t;
ori_opt_ptr_t ori_some_ptr(void* p) {
    ori_opt_ptr_t r; r.has_value = 1; r.ptr = p; return r;
}
ori_opt_ptr_t ori_none_ptr(void) {
    ori_opt_ptr_t r; r.has_value = 0; r.ptr = 0; return r;
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
