use ori_ast::item::SourceFile;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label, SourceCache};
use ori_lexer::Token;
use ori_types::resolve::ResolvedModule;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

// â”€â”€ Output types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

pub struct LexOutput {
    pub cache: SourceCache,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct ParseOutput {
    pub cache: SourceCache,
    pub ast: SourceFile,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct CheckOutput {
    pub cache: SourceCache,
    pub resolved: ResolvedModule,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

struct LoadedSource {
    path: PathBuf,
    file_id: FileId,
    ast: SourceFile,
}

struct ProjectConfig {
    entry: PathBuf,
}

// â”€â”€ Pipeline steps â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Read `path` from disk, lex it and return the token stream.
pub fn run_lex(path: &Path) -> Result<LexOutput, String> {
    let source = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let file_id = cache.add(path, source.clone());
    let tokens = ori_lexer::lex(&source, file_id, &mut sink);
    let diags = sink.into_diagnostics();
    Ok(LexOutput {
        cache,
        tokens,
        diagnostics: diags,
    })
}

/// Read + lex + parse. Returns the AST (possibly partial on errors).
pub fn run_parse(path: &Path) -> Result<ParseOutput, String> {
    let source = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let file_id = cache.add(path, source.clone());
    let tokens = ori_lexer::lex(&source, file_id, &mut sink);
    let ast = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let diags = sink.into_diagnostics();
    Ok(ParseOutput {
        cache,
        ast,
        tokens,
        diagnostics: diags,
    })
}

pub struct CompileOutput {
    pub cache: SourceCache,
    pub exe_path: std::path::PathBuf,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

/// Full pipeline â†’ Cranelift object â†’ linker â†’ native binary.
pub fn run_compile(source_path: &Path, output: &Path) -> Result<CompileOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(source_path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    if !sink.has_errors() {
        let hir = lower_loaded_sources(&loaded, &resolved, &mut sink);
        let obj_path = output.with_extension("o");
        let rt_lib = build_runtime_lib()?;
        ori_codegen::emit_native(&hir, &obj_path)?;
        let extra: Vec<_> = rt_lib.into_iter().collect();
        ori_codegen::link(&obj_path, output, &extra)?;
        let _ = std::fs::remove_file(&obj_path);
        for e in &extra {
            let _ = std::fs::remove_file(e);
        }
    }

    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(CompileOutput {
        cache,
        exe_path: output.to_owned(),
        diagnostics,
        has_errors,
    })
}

/// Full pipeline: lex â†’ parse â†’ resolve names â†’ type-check.
pub fn run_check(path: &Path) -> Result<CheckOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(path, &mut cache, &mut sink)?;

    // Type checking â€” only if no fatal parse errors so far
    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(CheckOutput {
        cache,
        resolved,
        diagnostics,
        has_errors,
    })
}

pub struct BuildOutput {
    pub cache: SourceCache,
    pub c_source: String,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

/// Full pipeline + HIR lowering + C code generation.
pub fn run_build(path: &Path) -> Result<BuildOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
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

    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(BuildOutput {
        cache,
        c_source,
        diagnostics,
        has_errors,
    })
}

// â”€â”€ Utilities â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn read_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("cannot read `{}`: {}", path.display(), e))
}

fn resolve_entry_path(path: &Path) -> Result<PathBuf, String> {
    if path.is_dir() {
        let manifest = path.join("ori.proj");
        if !manifest.is_file() {
            return Err(format!(
                "project manifest `{}` not found",
                manifest.display()
            ));
        }
        return read_project_config(&manifest).map(|config| config.entry);
    }

    if path.file_name().and_then(|name| name.to_str()) == Some("ori.proj") {
        return read_project_config(path).map(|config| config.entry);
    }

    Ok(path.to_owned())
}

fn read_project_config(manifest: &Path) -> Result<ProjectConfig, String> {
    let source = read_file(manifest)?;
    let root = manifest.parent().unwrap_or_else(|| Path::new("."));
    let mut entry = None;

    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("--") || line.starts_with('[') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() == "entry" {
            let value = value.trim().trim_matches('"');
            entry = Some(root.join(value));
        }
    }

    let Some(entry) = entry else {
        return Err(format!(
            "project manifest `{}` is missing `entry`",
            manifest.display()
        ));
    };
    if !entry.is_file() {
        return Err(format!(
            "project entry `{}` does not exist",
            entry.display()
        ));
    }
    Ok(ProjectConfig { entry })
}

fn load_and_resolve(
    path: &Path,
    cache: &mut SourceCache,
    sink: &mut DiagnosticSink,
) -> Result<(Vec<LoadedSource>, ResolvedModule), String> {
    let entry = resolve_entry_path(path)?;
    let mut loaded = Vec::new();
    let mut seen = HashSet::new();
    let mut active = Vec::new();
    load_source_recursive(&entry, cache, sink, &mut seen, &mut active, &mut loaded)?;
    let entry_namespace = loaded
        .first()
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
    active: &mut Vec<PathBuf>,
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
    let imports: Vec<_> = ast
        .imports
        .iter()
        .map(|i| (i.path.to_string(), i.span))
        .collect();
    loaded.push(LoadedSource {
        path: path.clone(),
        file_id,
        ast,
    });
    active.push(path.clone());
    for (import, span) in imports {
        if is_stdlib_import(&import) {
            continue;
        }
        match resolve_import_path(&path, &import) {
            ImportResolution::Found(import_path) => {
                if active.contains(&import_path) {
                    let cycle = import_cycle_description(active, loaded, &import_path, &import);
                    sink.emit(
                        Diagnostic::error(
                            "bind.import_cycle",
                            format!("import cycle detected: {}", cycle),
                        )
                        .with_label(Label::primary(file_id, span, "cyclic import here"))
                        .with_action(
                            "remove one import or move shared definitions into an acyclic module",
                        ),
                    );
                    validate_import_namespace(loaded, &import_path, &import, file_id, span, sink);
                    continue;
                }
                load_source_recursive(&import_path, cache, sink, seen, active, loaded)?;
                validate_import_namespace(loaded, &import_path, &import, file_id, span, sink);
            }
            ImportResolution::Ambiguous(paths) => {
                let mut diagnostic = Diagnostic::error(
                    "bind.import_ambiguous",
                    format!("import `{}` matches more than one file", import),
                )
                    .with_label(Label::primary(file_id, span, "ambiguous import here"))
                    .with_why("the current import search policy found multiple matching `.orl` files")
                    .with_action("keep only one matching file or import through a path that resolves to a single file");
                for path in paths {
                    diagnostic = diagnostic.with_note(format!("candidate: {}", path.display()));
                }
                sink.emit(diagnostic);
            }
            ImportResolution::Missing => {
                sink.emit(
                    Diagnostic::error(
                        "bind.import_not_found",
                        format!("import `{}` not found", import),
                    )
                    .with_label(Label::primary(file_id, span, "imported here"))
                    .with_action("place the imported namespace in a matching `.orl` file"),
                );
            }
        }
    }
    active.pop();
    Ok(())
}

fn validate_import_namespace(
    loaded: &[LoadedSource],
    import_path: &Path,
    import: &str,
    file_id: FileId,
    span: ori_diagnostics::Span,
    sink: &mut DiagnosticSink,
) {
    if let Some(imported) = loaded.iter().find(|s| s.path == import_path) {
        let declared = namespace_of(&imported.ast);
        if declared != import {
            sink.emit(
                Diagnostic::error(
                    "bind.import_namespace_mismatch",
                    format!(
                        "import `{}` resolved to file declaring `{}`",
                        import, declared
                    ),
                )
                .with_label(Label::primary(file_id, span, "imported here"))
                .with_action("make the imported file namespace match the import path"),
            );
        }
    }
}

fn import_cycle_description(
    active: &[PathBuf],
    loaded: &[LoadedSource],
    import_path: &Path,
    import: &str,
) -> String {
    let start = active.iter().position(|p| p == import_path).unwrap_or(0);
    let mut parts: Vec<String> = active[start..]
        .iter()
        .map(|path| {
            loaded
                .iter()
                .find(|s| s.path == *path)
                .map(|s| namespace_of(&s.ast))
                .unwrap_or_else(|| path.display().to_string())
        })
        .collect();
    parts.push(import.to_string());
    parts.join(" -> ")
}

fn namespace_of(file: &SourceFile) -> String {
    file.namespace.name.to_string()
}

fn is_stdlib_import(import: &str) -> bool {
    import == "ori" || import.starts_with("ori.")
}

enum ImportResolution {
    Found(PathBuf),
    Ambiguous(Vec<PathBuf>),
    Missing,
}

fn resolve_import_path(importer: &Path, import: &str) -> ImportResolution {
    // Directory of the file performing the import
    let Some(dir) = importer.parent() else {
        return ImportResolution::Missing;
    };

    // Determine the project root (if any). All candidate paths must reside in or below it.
    let project_root = find_project_root(dir).and_then(|p| std::fs::canonicalize(p).ok());

    let mut matches = Vec::new();

    // Walk ancestors from the importer's directory upwards.
    for base in dir.ancestors() {
        // Stop once we've climbed above the project root (if found)
        if let Some(ref root) = project_root {
            if let Ok(base_real) = std::fs::canonicalize(base) {
                // Always include the root itself in the search, then break.
                let reached_root = base_real == *root;
                for candidate in import_candidates(base, import) {
                    if candidate.is_file() {
                        let path = std::fs::canonicalize(&candidate).unwrap_or(candidate);
                        if !matches.contains(&path) {
                            matches.push(path);
                        }
                    }
                }
                if reached_root {
                    break;
                } else {
                    continue;
                }
            }
        }
        // If no project root was found, or we haven't reached it yet, continue searching.
        for candidate in import_candidates(base, import) {
            if candidate.is_file() {
                let path = std::fs::canonicalize(&candidate).unwrap_or(candidate);
                if !matches.contains(&path) {
                    matches.push(path);
                }
            }
        }
    }

    match matches.len() {
        0 => ImportResolution::Missing,
        1 => ImportResolution::Found(matches.remove(0)),
        _ => ImportResolution::Ambiguous(matches),
    }
}

fn import_candidates(base: &Path, import: &str) -> Vec<PathBuf> {
    let parts: Vec<_> = import.split('.').filter(|p| !p.is_empty()).collect();
    let mut candidates = Vec::new();
    if !parts.is_empty() {
        let mut nested_dir = base.to_path_buf();
        for part in &parts {
            nested_dir.push(part);
        }
        let mut nested = nested_dir.clone();
        nested.set_extension("orl");
        candidates.push(nested.clone());
        candidates.push(nested_dir.join("mod.orl"));
        candidates.push(nested_dir.join("index.orl"));

        if let Some(last) = parts.last() {
            candidates.push(base.join(format!("{last}.orl")));
            candidates.push(base.join(last).join("mod.orl"));
            candidates.push(base.join(last).join("index.orl"));
        }
    }
    candidates
}

/// Walk ancestors upwards from `start` until an `ori.proj` file is found. The directory
/// that contains the manifest is considered the project root.
fn find_project_root(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        let proj = ancestor.join("ori.proj");
        if proj.is_file() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
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
            &resolved.struct_sigs,
            &resolved.enum_sigs,
            &resolved.trait_sigs,
            &resolved.impl_sigs,
            &resolved.type_alias_sigs,
            &resolved.reexports,
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
        &resolved.func_sigs,
        &resolved.trait_sigs,
        &resolved.impl_sigs,
        &resolved.type_alias_sigs,
        &resolved.reexports,
        &first_namespace,
        first.file_id,
        sink,
    );
    for source in rest {
        let namespace = namespace_of(&source.ast);
        let mut hir = ori_hir::lower(
            &source.ast,
            &resolved.def_map,
            &resolved.func_sigs,
            &resolved.trait_sigs,
            &resolved.impl_sigs,
            &resolved.type_alias_sigs,
            &resolved.reexports,
            &namespace,
            source.file_id,
            sink,
        );
        merged.structs.append(&mut hir.structs);
        merged.enums.append(&mut hir.enums);
        merged.traits.append(&mut hir.traits);
        merged.trait_impls.append(&mut hir.trait_impls);
        merged.funcs.append(&mut hir.funcs);
        merged.consts.append(&mut hir.consts);
    }
    ori_hir::insert_default_arguments(&mut merged);
    ori_hir::monomorphize_generics(&mut merged);
    merged
}

/// The Ori runtime as embedded C source â€” compiled on demand with `cc -c`.
/// This avoids linking issues from Rust staticlibs pulling in Rust std.
const ORI_RUNTIME_C: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

/* ARC hooks are ABI placeholders for managed values emitted by the native backend. */

typedef struct {
    long long refcount;
    void (*destructor)(void*);
} ori_heap_header_t;

void* ori_alloc(long long size, void (*destructor)(void*)) {
    long long total = size + sizeof(ori_heap_header_t);
    void* ptr = malloc((size_t)total);
    if (!ptr) return 0;
    ori_heap_header_t* header = (ori_heap_header_t*)ptr;
    header->refcount = 1;
    header->destructor = destructor;
    return (char*)ptr + sizeof(ori_heap_header_t);
}

void ori_arc_retain(void* ptr) {
    if (!ptr) return;
    ori_heap_header_t* header = (ori_heap_header_t*)((char*)ptr - sizeof(ori_heap_header_t));
    /* TODO: replace with proper atomics if multi-threaded */
    header->refcount += 1;
}

void ori_arc_release(void* ptr) {
    if (!ptr) return;
    ori_heap_header_t* header = (ori_heap_header_t*)((char*)ptr - sizeof(ori_heap_header_t));
    header->refcount -= 1;
    if (header->refcount <= 0) {
        if (header->destructor) {
            header->destructor(ptr);
        }
        free(header);
    }
}

long long ori_arc_collect_cycles(void) {
    return 0;
}

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

char* ori_io_read_line(void) {
    size_t cap = 128;
    size_t len = 0;
    char* buf = (char*)ori_alloc(cap, 0);
    if (!buf) return 0;
    int ch;
    while ((ch = getchar()) != EOF && ch != '\n') {
        if (len + 1 >= cap) {
            cap *= 2;
            char* next = (char*)realloc(((char*)buf - sizeof(ori_heap_header_t)), cap + sizeof(ori_heap_header_t));
            if (!next) { free((char*)buf - sizeof(ori_heap_header_t)); return 0; }
            buf = next + sizeof(ori_heap_header_t);
        }
        buf[len++] = (char)ch;
    }
    buf[len] = '\0';
    return buf;
}

/* ... */
/* skipped intermediate for space, I should replace exactly the functions */

/* ori_to_string_parts(n, out_ptr, out_len) returns a malloc'd C string and its byte length. */
void ori_to_string_parts(long long n, char** out_ptr, long long* out_len) {
    if (out_ptr) *out_ptr = NULL;
    if (out_len) *out_len = 0;
    char* buf = (char*)ori_alloc(32, 0);
    if (!buf) return;
    int len = snprintf(buf, 32, "%lld", (long long)n);
    if (len < 0) len = 0;
    if (out_ptr) *out_ptr = buf;
    if (out_len) *out_len = (long long)len;
}

/* ori_int_to_cstr(n: i64) -> *u8  (malloc'd, caller must free) */
char* ori_int_to_cstr(long long n) {
    char* buf = NULL;
    long long len = 0;
    ori_to_string_parts(n, &buf, &len);
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

long long ori_string_len(const char* ptr) {
    if (!ptr) return 0;
    return (long long)strlen(ptr);
}

static char* ori_string_copy_range(const char* start, size_t len) {
    char* out = (char*)ori_alloc(len + 1, 0);
    if (!out) return 0;
    if (len > 0) memcpy(out, start, len);
    out[len] = '\0';
    return out;
}

char* ori_string_concat_parts(const char* a, long long a_len, const char* b, long long b_len) {
    size_t la = (!a || a_len <= 0) ? 0 : (size_t)a_len;
    size_t lb = (!b || b_len <= 0) ? 0 : (size_t)b_len;
    char* out = (char*)ori_alloc(la + lb + 1, 0);
    if (!out) return 0;
    if (a && la) memcpy(out, a, la);
    if (b && lb) memcpy(out + la, b, lb);
    out[la + lb] = '\0';
    return out;
}

char* ori_string_concat(const char* a, const char* b) {
    return ori_string_concat_parts(
        a,
        a ? (long long)strlen(a) : 0,
        b,
        b ? (long long)strlen(b) : 0
    );
}

char* ori_string_slice(const char* s, long long start, long long end) {
    if (!s) return 0;
    long long len = (long long)strlen(s);
    if (start < 0) start = 0;
    if (end < start) end = start;
    if (end > len) end = len;
    size_t n = (size_t)(end - start);
    char* out = (char*)ori_alloc(n + 1, 0);
    if (!out) return 0;
    memcpy(out, s + start, n);
    out[n] = '\0';
    return out;
}

char ori_string_contains(const char* s, const char* sub) {
    if (!s || !sub) return 0;
    return strstr(s, sub) ? 1 : 0;
}

char ori_string_starts_with(const char* s, const char* prefix) {
    if (!s || !prefix) return 0;
    size_t lp = strlen(prefix);
    return strncmp(s, prefix, lp) == 0 ? 1 : 0;
}

char ori_string_ends_with(const char* s, const char* suffix) {
    if (!s || !suffix) return 0;
    size_t ls = strlen(s);
    size_t lf = strlen(suffix);
    if (lf > ls) return 0;
    return strcmp(s + (ls - lf), suffix) == 0 ? 1 : 0;
}

char* ori_string_trim(const char* s) {
    if (!s) return 0;
    const char* start = s;
    while (*start && isspace((unsigned char)*start)) start++;
    const char* end = s + strlen(s);
    while (end > start && isspace((unsigned char)*(end - 1))) end--;
    return ori_string_copy_range(start, (size_t)(end - start));
}

char* ori_string_to_upper(const char* s) {
    if (!s) return 0;
    size_t len = strlen(s);
    char* out = ori_string_copy_range(s, len);
    if (!out) return 0;
    for (size_t i = 0; i < len; i++) out[i] = (char)toupper((unsigned char)out[i]);
    return out;
}

char* ori_string_to_lower(const char* s) {
    if (!s) return 0;
    size_t len = strlen(s);
    char* out = ori_string_copy_range(s, len);
    if (!out) return 0;
    for (size_t i = 0; i < len; i++) out[i] = (char)tolower((unsigned char)out[i]);
    return out;
}

char* ori_string_replace(const char* s, const char* from, const char* to) {
    if (!s) return 0;
    if (!from || from[0] == '\0') return ori_string_copy_range(s, strlen(s));
    if (!to) to = "";
    size_t ls = strlen(s);
    size_t lf = strlen(from);
    size_t lt = strlen(to);
    size_t count = 0;
    const char* p = s;
    while ((p = strstr(p, from)) != 0) {
        count++;
        p += lf;
    }
    size_t out_len = ls + count * lt - count * lf;
    char* out = (char*)ori_alloc(out_len + 1, 0);
    if (!out) return 0;
    char* dst = out;
    p = s;
    const char* next;
    while ((next = strstr(p, from)) != 0) {
        size_t chunk = (size_t)(next - p);
        memcpy(dst, p, chunk);
        dst += chunk;
        memcpy(dst, to, lt);
        dst += lt;
        p = next + lf;
    }
    strcpy(dst, p);
    return out;
}

long long ori_string_index_of(const char* s, const char* sub) {
    if (!s || !sub) return -1;
    const char* found = strstr(s, sub);
    return found ? (long long)(found - s) : -1;
}

char* ori_string_repeat(const char* s, long long count) {
    if (!s || count <= 0) return ori_string_copy_range("", 0);
    size_t len = strlen(s);
    size_t out_len = len * (size_t)count;
    char* out = (char*)ori_alloc(out_len + 1, 0);
    if (!out) return 0;
    char* dst = out;
    for (long long i = 0; i < count; i++) {
        memcpy(dst, s, len);
        dst += len;
    }
    out[out_len] = '\0';
    return out;
}

static char* ori_string_pad(const char* s, long long target_len, const char* fill, int left) {
    if (!s) s = "";
    if (!fill || fill[0] == '\0') fill = " ";
    size_t len = strlen(s);
    size_t target = target_len <= 0 ? 0 : (size_t)target_len;
    if (len >= target) return ori_string_copy_range(s, len);
    size_t fill_len = strlen(fill);
    size_t pad_len = target - len;
    char* out = (char*)ori_alloc(target + 1, 0);
    if (!out) return 0;
    char* pad_start = left ? out : out + len;
    char* text_start = left ? out + pad_len : out;
    memcpy(text_start, s, len);
    for (size_t i = 0; i < pad_len; i++) pad_start[i] = fill[i % fill_len];
    out[target] = '\0';
    return out;
}

char* ori_string_pad_left(const char* s, long long target_len, const char* fill) {
    return ori_string_pad(s, target_len, fill, 1);
}

char* ori_string_pad_right(const char* s, long long target_len, const char* fill) {
    return ori_string_pad(s, target_len, fill, 0);
}

/* ori_math_abs(n: i64) -> i64 */
double ori_math_sqrt(double n) {
    if (n <= 0.0) return 0.0;
    double x = n;
    for (int i = 0; i < 32; i++) x = 0.5 * (x + n / x);
    return x;
}

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

static double ori_abs_double(double n) {
    return n < 0.0 ? -n : n;
}

long long ori_math_floor(double n) {
    long long i = (long long)n;
    return ((double)i > n) ? i - 1 : i;
}

long long ori_math_ceil(double n) {
    long long i = (long long)n;
    return ((double)i < n) ? i + 1 : i;
}

long long ori_math_round(double n) {
    return n >= 0.0 ? ori_math_floor(n + 0.5) : ori_math_ceil(n - 0.5);
}

double ori_math_log(double n) {
    if (n <= 0.0) return 0.0;
    int scale = 0;
    while (n > 2.0) { n *= 0.5; scale++; }
    while (n < 0.5) { n *= 2.0; scale--; }
    double y = (n - 1.0) / (n + 1.0);
    double y2 = y * y;
    double term = y;
    double sum = 0.0;
    for (int k = 1; k < 80; k += 2) {
        sum += term / (double)k;
        term *= y2;
    }
    return 2.0 * sum + (double)scale * 0.6931471805599453;
}

static double ori_math_exp_approx(double n) {
    int scale = 0;
    while (n > 1.0) { n *= 0.5; scale++; }
    while (n < -1.0) { n *= 0.5; scale++; }
    double term = 1.0;
    double sum = 1.0;
    for (int i = 1; i < 40; i++) {
        term *= n / (double)i;
        sum += term;
    }
    while (scale-- > 0) sum *= sum;
    return sum;
}

double ori_math_pow(double base, double exp) {
    long long whole = (long long)exp;
    if ((double)whole == exp) {
        double result = 1.0;
        long long n = whole < 0 ? -whole : whole;
        while (n > 0) {
            if (n & 1) result *= base;
            base *= base;
            n >>= 1;
        }
        return whole < 0 ? 1.0 / result : result;
    }
    if (base <= 0.0) return 0.0;
    return ori_math_exp_approx(ori_math_log(base) * exp);
}

static double ori_reduce_angle(double n) {
    const double two_pi = 6.283185307179586;
    while (n > 3.141592653589793) n -= two_pi;
    while (n < -3.141592653589793) n += two_pi;
    return n;
}

double ori_math_sin(double n) {
    n = ori_reduce_angle(n);
    double term = n;
    double sum = n;
    for (int i = 3; i <= 17; i += 2) {
        term *= -n * n / ((double)(i - 1) * (double)i);
        sum += term;
    }
    return sum;
}

double ori_math_cos(double n) {
    n = ori_reduce_angle(n);
    double term = 1.0;
    double sum = 1.0;
    for (int i = 2; i <= 18; i += 2) {
        term *= -n * n / ((double)(i - 1) * (double)i);
        sum += term;
    }
    return sum;
}

double ori_math_tan(double n) {
    double c = ori_math_cos(n);
    return ori_abs_double(c) < 0.000000000001 ? 0.0 : ori_math_sin(n) / c;
}

/* ---------- optional<int> helpers (value = i64) ---------- */
typedef struct { char has_value; long long value; } ori_opt_i64_t;
typedef struct { char has_value; double value; } ori_opt_f64_t;

ori_opt_i64_t ori_some_i64(long long v) {
    ori_opt_i64_t r; r.has_value = 1; r.value = v; return r;
}
ori_opt_i64_t ori_none_i64(void) {
    ori_opt_i64_t r; r.has_value = 0; r.value = 0; return r;
}

char* ori_float_to_string(double n) {
    char* buf = (char*)malloc(64);
    if (!buf) return 0;
    snprintf(buf, 64, "%.15g", n);
    return buf;
}

char* ori_bool_to_string(char value) {
    return ori_string_copy_range(value ? "true" : "false", value ? 4 : 5);
}

ori_opt_i64_t* ori_string_to_int(const char* s) {
    ori_opt_i64_t* r = (ori_opt_i64_t*)malloc(sizeof(ori_opt_i64_t));
    if (!r) return 0;
    r->has_value = 0;
    r->value = 0;
    if (!s) return r;
    char* end = 0;
    long long value = strtoll(s, &end, 10);
    while (end && *end && isspace((unsigned char)*end)) end++;
    if (end && end != s && *end == '\0') {
        r->has_value = 1;
        r->value = value;
    }
    return r;
}

ori_opt_f64_t* ori_string_to_float(const char* s) {
    ori_opt_f64_t* r = (ori_opt_f64_t*)malloc(sizeof(ori_opt_f64_t));
    if (!r) return 0;
    r->has_value = 0;
    r->value = 0.0;
    if (!s) return r;
    char* end = 0;
    double value = strtod(s, &end);
    while (end && *end && isspace((unsigned char)*end)) end++;
    if (end && end != s && *end == '\0') {
        r->has_value = 1;
        r->value = value;
    }
    return r;
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

void ori_list_dtor(void* ptr) {
    ori_list_t* list = (ori_list_t*)ptr;
    if (list->data) {
        free(list->data);
    }
}

ori_list_t* ori_list_new(void) {
    long long cap = 8;
    long long* data = (long long*)malloc((size_t)cap * sizeof(long long));
    ori_list_t* l = (ori_list_t*)ori_alloc(sizeof(ori_list_t), ori_list_dtor);
    if (l) {
        l->data = data;
        l->len  = 0;
        l->cap  = cap;
    }
    return l;
}
void ori_list_push(ori_list_t* l, long long v) {
    if (!l) return;
    if (l->len >= l->cap) {
        long long next_cap = (l->cap * 2) > 1 ? (l->cap * 2) : 1;
        l->data = (long long*)realloc(l->data, (size_t)next_cap * sizeof(long long));
        l->cap = next_cap;
    }
    l->data[l->len++] = v;
}
long long ori_list_get(ori_list_t* l, long long i) {
    if (!l || i < 0 || i >= l->len) return 0;
    return l->data[i];
}
void ori_list_set(ori_list_t* l, long long i, long long v) {
    if (l && i >= 0 && i < l->len) l->data[i] = v;
}
long long ori_list_len(ori_list_t* l) { return l ? l->len : 0; }
void ori_list_free(ori_list_t* l) {
    ori_arc_release((void*)l);
}
long long ori_list_pop(ori_list_t* l) {
    if (!l || l->len <= 0) return 0;
    return l->data[--l->len];
}
void ori_list_remove(ori_list_t* l, long long index) {
    if (!l || index < 0 || index >= l->len) return;
    for (long long i = index; i + 1 < l->len; i++) l->data[i] = l->data[i + 1];
    l->len--;
}
void ori_list_insert(ori_list_t* l, long long index, long long value) {
    if (!l) return;
    if (l->len >= l->cap) {
        long long next_cap = (l->cap * 2) > 1 ? (l->cap * 2) : 1;
        l->data = (long long*)realloc(l->data, (size_t)next_cap * sizeof(long long));
        l->cap = next_cap;
    }
    if (index < 0) index = 0;
    if (index > l->len) index = l->len;
    for (long long i = l->len; i > index; i--) l->data[i] = l->data[i - 1];
    l->data[index] = value;
    l->len++;
}
long long ori_list_index_of(ori_list_t* l, long long value) {
    if (!l) return -1;
    for (long long i = 0; i < l->len; i++) {
        if (l->data[i] == value) return i;
    }
    return -1;
}
char ori_list_contains(ori_list_t* l, long long value) {
    return ori_list_index_of(l, value) >= 0 ? 1 : 0;
}
void ori_list_sort(ori_list_t* l) {
    if (!l || l->len <= 1) return;
    for (long long i = 1; i < l->len; i++) {
        long long value = l->data[i];
        long long j = i - 1;
        while (j >= 0 && l->data[j] > value) {
            l->data[j + 1] = l->data[j];
            j--;
        }
        l->data[j + 1] = value;
    }
}
void ori_list_reverse(ori_list_t* l) {
    if (!l) return;
    for (long long i = 0, j = l->len - 1; i < j; i++, j--) {
        long long tmp = l->data[i];
        l->data[i] = l->data[j];
        l->data[j] = tmp;
    }
}
ori_list_t* ori_list_slice(ori_list_t* l, long long start, long long end) {
    ori_list_t* out = ori_list_new();
    if (!l) return out;
    if (start < 0) start = 0;
    if (end < start) end = start;
    if (end > l->len) end = l->len;
    for (long long i = start; i < end; i++) ori_list_push(out, l->data[i]);
    return out;
}

char* ori_string_join(ori_list_t* list, const char* sep) {
    if (!list) return ori_string_copy_range("", 0);
    if (!sep) sep = "";
    size_t sep_len = strlen(sep);
    size_t total = 0;
    for (long long i = 0; i < list->len; i++) {
        const char* item = (const char*)list->data[i];
        total += item ? strlen(item) : 0;
        if (i > 0) total += sep_len;
    }
    char* out = (char*)malloc(total + 1);
    if (!out) return 0;
    char* dst = out;
    for (long long i = 0; i < list->len; i++) {
        if (i > 0 && sep_len > 0) {
            memcpy(dst, sep, sep_len);
            dst += sep_len;
        }
        const char* item = (const char*)list->data[i];
        size_t len = item ? strlen(item) : 0;
        if (len > 0) {
            memcpy(dst, item, len);
            dst += len;
        }
    }
    *dst = '\0';
    return out;
}

ori_list_t* ori_string_split(const char* s, const char* sep) {
    ori_list_t* out = ori_list_new();
    if (!s) return out;
    if (!sep || sep[0] == '\0') {
        for (const char* p = s; *p; p++) {
            char* item = ori_string_copy_range(p, 1);
            ori_list_push(out, (long long)item);
        }
        return out;
    }
    size_t sep_len = strlen(sep);
    const char* start = s;
    const char* next;
    while ((next = strstr(start, sep)) != 0) {
        char* item = ori_string_copy_range(start, (size_t)(next - start));
        ori_list_push(out, (long long)item);
        start = next + sep_len;
    }
    ori_list_push(out, (long long)ori_string_copy_range(start, strlen(start)));
    return out;
}

ori_list_t* ori_string_chars(const char* s) {
    ori_list_t* out = ori_list_new();
    if (!s) return out;
    for (const char* p = s; *p; p++) {
        char* item = ori_string_copy_range(p, 1);
        ori_list_push(out, (long long)item);
    }
    return out;
}

ori_list_t* ori_set_new(void) {
    return ori_list_new();
}
void ori_set_add(ori_list_t* s, long long v) {
    if (!s) return;
    for (long long i = 0; i < s->len; i++) {
        if (s->data[i] == v) return;
    }
    ori_list_push(s, v);
}
char ori_set_contains(ori_list_t* s, long long v) {
    if (!s) return 0;
    for (long long i = 0; i < s->len; i++) {
        if (s->data[i] == v) return 1;
    }
    return 0;
}
void ori_set_remove(ori_list_t* s, long long v) {
    if (!s) return;
    long long index = ori_list_index_of(s, v);
    if (index >= 0) ori_list_remove(s, index);
}
long long ori_set_len(ori_list_t* s) {
    return ori_list_len(s);
}
void ori_set_free(ori_list_t* s) {
    ori_list_free(s);
}

typedef struct { long long* keys; long long* values; long long len; long long cap; } ori_map_t;

void ori_map_dtor(void* ptr) {
    ori_map_t* map = (ori_map_t*)ptr;
    if (map->keys) free(map->keys);
    if (map->values) free(map->values);
}

ori_map_t* ori_map_new(void) {
    long long cap = 8;
    ori_map_t* m = (ori_map_t*)ori_alloc(sizeof(ori_map_t), ori_map_dtor);
    if (m) {
        m->keys = (long long*)malloc((size_t)cap * sizeof(long long));
        m->values = (long long*)malloc((size_t)cap * sizeof(long long));
        m->len = 0;
        m->cap = cap;
    }
    return m;
}
void ori_map_set(ori_map_t* m, long long key, long long value) {
    if (!m) return;
    for (long long i = 0; i < m->len; i++) {
        if (m->keys[i] == key) {
            m->values[i] = value;
            return;
        }
    }
    if (m->len >= m->cap) {
        long long next_cap = (m->cap * 2) > 1 ? (m->cap * 2) : 1;
        m->keys = (long long*)realloc(m->keys, (size_t)next_cap * sizeof(long long));
        m->values = (long long*)realloc(m->values, (size_t)next_cap * sizeof(long long));
        m->cap = next_cap;
    }
    m->keys[m->len] = key;
    m->values[m->len] = value;
    m->len++;
}
long long ori_map_get(ori_map_t* m, long long key) {
    if (!m) return 0;
    for (long long i = 0; i < m->len; i++) {
        if (m->keys[i] == key) return m->values[i];
    }
    return 0;
}
char ori_map_contains(ori_map_t* m, long long key) {
    if (!m) return 0;
    for (long long i = 0; i < m->len; i++) {
        if (m->keys[i] == key) return 1;
    }
    return 0;
}
void ori_map_remove(ori_map_t* m, long long key) {
    if (!m) return;
    for (long long i = 0; i < m->len; i++) {
        if (m->keys[i] == key) {
            for (long long j = i; j + 1 < m->len; j++) {
                m->keys[j] = m->keys[j + 1];
                m->values[j] = m->values[j + 1];
            }
            m->len--;
            return;
        }
    }
}
ori_list_t* ori_map_keys(ori_map_t* m) {
    ori_list_t* out = ori_list_new();
    if (!m) return out;
    for (long long i = 0; i < m->len; i++) ori_list_push(out, m->keys[i]);
    return out;
}
ori_list_t* ori_map_values(ori_map_t* m) {
    ori_list_t* out = ori_list_new();
    if (!m) return out;
    for (long long i = 0; i < m->len; i++) ori_list_push(out, m->values[i]);
    return out;
}
ori_list_t* ori_map_entries(ori_map_t* m) {
    ori_list_t* out = ori_list_new();
    if (!m) return out;
    for (long long i = 0; i < m->len; i++) {
        long long* tuple = (long long*)calloc(1, 16);
        tuple[0] = m->keys[i];
        tuple[1] = m->values[i];
        ori_list_push(out, (long long)tuple);
    }
    return out;
}
long long ori_map_len(ori_map_t* m) {
    return m ? m->len : 0;
}
long long ori_map_key_at(ori_map_t* m, long long index) {
    if (!m || index < 0 || index >= m->len) return 0;
    return m->keys[index];
}
long long ori_map_value_at(ori_map_t* m, long long index) {
    if (!m || index < 0 || index >= m->len) return 0;
    return m->values[index];
}
void ori_map_free(ori_map_t* m) {
    ori_arc_release((void*)m);
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
    static NEXT_RUNTIME_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let id = NEXT_RUNTIME_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let stem = format!("ori_rt_{}_{}", std::process::id(), id);
    let tmp_dir = std::env::temp_dir();
    let tmp_c = tmp_dir.join(format!("{stem}.c"));
    let tmp_obj = tmp_dir.join(format!("{stem}.o"));

    std::fs::write(&tmp_c, ORI_RUNTIME_C).map_err(|e| format!("write {}: {e}", tmp_c.display()))?;

    let status = std::process::Command::new("cc")
        .arg("-c")
        .arg(&tmp_c)
        .arg("-o")
        .arg(&tmp_obj)
        .status()
        .map_err(|e| format!("cc -c {}: {e}", tmp_c.display()))?;

    let _ = std::fs::remove_file(&tmp_c);

    if status.success() {
        Ok(Some(tmp_obj))
    } else {
        let _ = std::fs::remove_file(&tmp_obj);
        Ok(None) // cc not available; functions will be unresolved at runtime
    }
}
