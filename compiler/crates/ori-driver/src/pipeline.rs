use ori_ast::common::{TypeParams, WhereClause};
use ori_ast::item::{ExternMember, Item, Param, SourceFile, TraitMember};
use ori_ast::ty::Type;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label, SourceCache};
use ori_hir::{HirArg, HirBlock, HirExpr, HirExprKind, HirFunc, HirStmt};
use ori_lexer::{Token, TokenKind};
use ori_types::{resolve::ResolvedModule, DefId, Ty};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const ORI_VERSION: &str = env!("CARGO_PKG_VERSION");
const ORI_DRIVER_ABI_VERSION: &str = ori_runtime::ORI_ABI_VERSION;
const NATIVE_RUNTIME_MISSING: &str = "native.runtime_missing";
const NATIVE_RUNTIME_METADATA_INVALID: &str = "native.runtime_metadata_invalid";
const NATIVE_RUNTIME_METADATA_MISMATCH: &str = "native.runtime_metadata_mismatch";
const NATIVE_ABI_MISMATCH: &str = "native.abi_mismatch";

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

pub struct DocOutput {
    pub cache: SourceCache,
    pub markdown: String,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

pub struct TestOutput {
    pub cache: SourceCache,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
    pub results: Vec<TestResult>,
}

pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
}

pub struct FmtOutput {
    pub cache: SourceCache,
    pub formatted: String,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

struct LoadedSource {
    path: PathBuf,
    file_id: FileId,
    source: String,
    tokens: Vec<Token>,
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

pub fn run_fmt(path: &Path) -> Result<FmtOutput, String> {
    let source = read_file(path)?;
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let file_id = cache.add(path, source.clone());
    let tokens = ori_lexer::lex(&source, file_id, &mut sink);
    let _ast = ori_parser::parse(&tokens, &source, file_id, &mut sink);
    let formatted = if !sink.has_errors() {
        format_source_text(&source)
    } else {
        String::new()
    };
    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(FmtOutput {
        cache,
        formatted,
        diagnostics,
        has_errors,
    })
}

pub struct CompileOutput {
    pub cache: SourceCache,
    pub exe_path: std::path::PathBuf,
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CompileOptions {
    pub native_raw: bool,
}

#[derive(Clone, Debug)]
struct NativeRuntimeLink {
    runtime_lib: PathBuf,
    native_static_libs: Vec<String>,
}

impl NativeRuntimeLink {
    fn link_args(&self) -> Vec<PathBuf> {
        let mut args = Vec::with_capacity(1 + self.native_static_libs.len());
        args.push(self.runtime_lib.clone());
        args.extend(self.native_static_libs.iter().map(PathBuf::from));
        args
    }
}

/// Full pipeline â†’ Cranelift object â†’ linker â†’ native binary.
pub fn run_compile(source_path: &Path, output: &Path) -> Result<CompileOutput, String> {
    run_compile_with_options(source_path, output, CompileOptions::default())
}

pub fn run_compile_with_options(
    source_path: &Path,
    output: &Path,
    options: CompileOptions,
) -> Result<CompileOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(source_path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    if !sink.has_errors() {
        let hir = lower_loaded_sources(&loaded, &resolved, &mut sink);
        let obj_path = output.with_extension("o");
        let runtime_link = find_native_runtime_link()?;
        ori_codegen::emit_native(&hir, &obj_path)?;
        let extra = runtime_link.link_args();
        ori_codegen::link_with_options(
            &obj_path,
            output,
            &extra,
            ori_codegen::NativeLinkOptions {
                raw_diagnostics: options.native_raw,
            },
        )?;
        let _ = std::fs::remove_file(&obj_path);
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
fn find_native_runtime_link() -> Result<NativeRuntimeLink, String> {
    static CACHED: std::sync::OnceLock<Result<NativeRuntimeLink, String>> =
        std::sync::OnceLock::new();
    CACHED
        .get_or_init(find_native_runtime_link_uncached)
        .clone()
}

fn find_native_runtime_link_uncached() -> Result<NativeRuntimeLink, String> {
    if let Ok(path) = std::env::var("ORI_RUNTIME_LIB") {
        let path = PathBuf::from(path);
        return if path.is_file() {
            let target = native_target_triple();
            let artifact = native_runtime_artifact_name(&target);
            native_runtime_link_for(path, &target, artifact)
        } else {
            Err(format!(
                "ORI_RUNTIME_LIB points to `{}`, but that file does not exist",
                path.display()
            ))
        };
    }

    let target = native_target_triple();
    let artifact = native_runtime_artifact_name(&target);
    let mut searched = Vec::new();
    let packaged_candidates = packaged_runtime_candidates(&target, artifact);

    for candidate in &packaged_candidates {
        if candidate.is_file() {
            return native_runtime_link_for(candidate.clone(), &target, artifact);
        }
    }
    searched.extend(packaged_candidates);

    if env_flag("ORI_REQUIRE_PACKAGED_RUNTIME") {
        return Err(missing_native_runtime_message(
            &target, artifact, &searched, true,
        ));
    }

    build_native_runtime_with_cargo()?;

    let cargo_candidates = cargo_runtime_candidates(&target, artifact);
    for candidate in &cargo_candidates {
        if candidate.is_file() {
            return native_runtime_link_for(candidate.clone(), &target, artifact);
        }
    }
    searched.extend(cargo_candidates);

    Err(missing_native_runtime_message(
        &target, artifact, &searched, false,
    ))
}

fn env_flag(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

fn missing_native_runtime_message(
    target: &str,
    artifact: &str,
    searched: &[PathBuf],
    packaged_only: bool,
) -> String {
    let mut message = format!(
        "{NATIVE_RUNTIME_MISSING}: native Ori runtime `{artifact}` for target `{target}` was not found."
    );
    if packaged_only {
        message.push_str(" Packaged runtime mode is enabled by ORI_REQUIRE_PACKAGED_RUNTIME=1.");
    }
    message.push_str(&format!(
        "\nexpected package path: runtime/{target}/{artifact}\nstaging command: .\\tools\\stage_native_runtime.ps1 -Target {target}"
    ));
    if !packaged_only {
        message.push_str("\nworkspace fallback: cargo build -p ori-runtime --lib");
    }
    if !searched.is_empty() {
        message.push_str("\nsearched paths:");
        for path in searched {
            message.push_str(&format!("\n- {}", path.display()));
        }
    }
    message
}

fn native_runtime_link_for(
    runtime_lib: PathBuf,
    target: &str,
    artifact: &str,
) -> Result<NativeRuntimeLink, String> {
    let metadata_path = runtime_lib
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("runtime-link.json");
    let native_static_libs = if metadata_path.is_file() {
        let metadata = read_runtime_link_metadata(&metadata_path)?;
        if metadata.target != target {
            return Err(format!(
                "{NATIVE_RUNTIME_METADATA_MISMATCH}: runtime metadata `{}` targets `{}`, but the current target is `{target}`",
                metadata_path.display(),
                metadata.target
            ));
        }
        if metadata.runtime != artifact {
            return Err(format!(
                "{NATIVE_RUNTIME_METADATA_MISMATCH}: runtime metadata `{}` names runtime `{}`, but `{artifact}` was expected",
                metadata_path.display(),
                metadata.runtime
            ));
        }
        if metadata.ori_version != ORI_VERSION {
            return Err(format!(
                "{NATIVE_RUNTIME_METADATA_MISMATCH}: runtime metadata `{}` was staged for Ori {}, but the driver is Ori {}",
                metadata_path.display(),
                metadata.ori_version,
                ORI_VERSION
            ));
        }
        if metadata.abi_version != ORI_DRIVER_ABI_VERSION {
            return Err(format!(
                "{NATIVE_ABI_MISMATCH}: runtime metadata `{}` uses ABI {}, but the driver expects ABI {}",
                metadata_path.display(),
                metadata.abi_version,
                ORI_DRIVER_ABI_VERSION
            ));
        }
        metadata.native_static_libs
    } else {
        native_static_libs_for_target(target)
            .iter()
            .map(|lib| (*lib).to_string())
            .collect()
    };

    Ok(NativeRuntimeLink {
        runtime_lib,
        native_static_libs,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeLinkMetadata {
    target: String,
    runtime: String,
    ori_version: String,
    abi_version: String,
    native_static_libs: Vec<String>,
}

fn read_runtime_link_metadata(path: &Path) -> Result<RuntimeLinkMetadata, String> {
    let source = std::fs::read_to_string(path).map_err(|e| {
        format!(
            "{NATIVE_RUNTIME_METADATA_INVALID}: cannot read runtime metadata `{}`: {e}",
            path.display()
        )
    })?;
    let target = json_string_field(&source, "target").ok_or_else(|| {
        format!(
            "{NATIVE_RUNTIME_METADATA_INVALID}: runtime metadata `{}` is missing string field `target`",
            path.display()
        )
    })?;
    let runtime = json_string_field(&source, "runtime").ok_or_else(|| {
        format!(
            "{NATIVE_RUNTIME_METADATA_INVALID}: runtime metadata `{}` is missing string field `runtime`",
            path.display()
        )
    })?;
    let ori_version = json_string_field(&source, "ori_version").ok_or_else(|| {
        format!(
            "{NATIVE_RUNTIME_METADATA_INVALID}: runtime metadata `{}` is missing string field `ori_version`",
            path.display()
        )
    })?;
    let abi_version = json_string_field(&source, "abi_version").ok_or_else(|| {
        format!(
            "{NATIVE_RUNTIME_METADATA_INVALID}: runtime metadata `{}` is missing string field `abi_version`",
            path.display()
        )
    })?;
    let native_static_libs =
        json_string_array_field(&source, "native_static_libs").ok_or_else(|| {
            format!(
                "{NATIVE_RUNTIME_METADATA_INVALID}: runtime metadata `{}` is missing string array field `native_static_libs`",
                path.display()
            )
        })?;
    Ok(RuntimeLinkMetadata {
        target,
        runtime,
        ori_version,
        abi_version,
        native_static_libs,
    })
}

fn json_string_field(source: &str, field: &str) -> Option<String> {
    let rest = json_field_value(source, field)?;
    let (value, _) = parse_json_string(rest.trim_start())?;
    Some(value)
}

fn json_string_array_field(source: &str, field: &str) -> Option<Vec<String>> {
    let mut rest = json_field_value(source, field)?.trim_start();
    rest = rest.strip_prefix('[')?.trim_start();
    let mut values = Vec::new();
    loop {
        if rest.starts_with(']') {
            return Some(values);
        }
        let (value, consumed) = parse_json_string(rest)?;
        values.push(value);
        rest = rest[consumed..].trim_start();
        if let Some(next) = rest.strip_prefix(',') {
            rest = next.trim_start();
            continue;
        }
        rest.strip_prefix(']')?;
        return Some(values);
    }
}

fn json_field_value<'a>(source: &'a str, field: &str) -> Option<&'a str> {
    let key = format!("\"{field}\"");
    let after_key = source.split_once(&key)?.1;
    let after_colon = after_key.split_once(':')?.1;
    Some(after_colon)
}

fn parse_json_string(source: &str) -> Option<(String, usize)> {
    let mut chars = source.char_indices();
    let (_, first) = chars.next()?;
    if first != '"' {
        return None;
    }
    let mut out = String::new();
    let mut escaped = false;
    for (index, ch) in chars {
        if escaped {
            let value = match ch {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{0008}',
                'f' => '\u{000c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => ch,
            };
            out.push(value);
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return Some((out, index + ch.len_utf8())),
            _ => out.push(ch),
        }
    }
    None
}

fn native_target_triple() -> String {
    std::env::var("ORI_TARGET_TRIPLE")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(default_native_target_triple)
}

fn default_native_target_triple() -> String {
    if cfg!(all(windows, target_env = "msvc")) {
        "x86_64-pc-windows-msvc".to_string()
    } else if cfg!(all(windows, target_env = "gnu")) {
        "x86_64-pc-windows-gnu".to_string()
    } else if cfg!(target_os = "linux") {
        "x86_64-unknown-linux-gnu".to_string()
    } else if cfg!(target_os = "macos") {
        "x86_64-apple-darwin".to_string()
    } else {
        format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
    }
}

fn native_runtime_artifact_name(target: &str) -> &'static str {
    if target.contains("windows-msvc") {
        "ori_runtime.lib"
    } else {
        "libori_runtime.a"
    }
}

fn packaged_runtime_candidates(target: &str, artifact: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("runtime").join(target).join(artifact));
            if let Some(parent) = dir.parent() {
                candidates.push(parent.join("runtime").join(target).join(artifact));
            }
        }
    }
    candidates.push(workspace_root().join("runtime").join(target).join(artifact));
    candidates
}

fn cargo_runtime_candidates(target: &str, artifact: &str) -> Vec<PathBuf> {
    let target_dir = cargo_target_dir();
    let preferred = if cfg!(debug_assertions) {
        ["debug", "release"]
    } else {
        ["release", "debug"]
    };
    let mut candidates = Vec::new();
    for profile in preferred {
        candidates.push(target_dir.join(profile).join(artifact));
        candidates.push(target_dir.join(target).join(profile).join(artifact));
    }
    candidates
}

fn cargo_target_dir() -> PathBuf {
    std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root().join("target"))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

fn build_native_runtime_with_cargo() -> Result<(), String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut cmd = std::process::Command::new(&cargo);
    cmd.current_dir(workspace_root())
        .arg("build")
        .arg("-p")
        .arg("ori-runtime")
        .arg("--lib");
    if !cfg!(debug_assertions) {
        cmd.arg("--release");
    }

    let output = cmd
        .output()
        .map_err(|e| {
            format!(
                "{NATIVE_RUNTIME_MISSING}: failed to start Cargo while building the native Ori runtime: {e}"
            )
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{NATIVE_RUNTIME_MISSING}: failed to build native Ori runtime with `{cargo} build -p ori-runtime --lib`\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

#[cfg(test)]
fn runtime_link_metadata_json(target: &str, artifact: &str) -> String {
    let native_static_libs = native_static_libs_for_target(target);
    format!(
        "{{\n  \"target\": \"{target}\",\n  \"runtime\": \"{artifact}\",\n  \"ori_version\": \"{ORI_VERSION}\",\n  \"abi_version\": \"{ORI_DRIVER_ABI_VERSION}\",\n  \"native_static_libs\": [{}]\n}}\n",
        native_static_libs
            .iter()
            .map(|lib| format!("\"{lib}\""))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn native_static_libs_for_target(target: &str) -> &'static [&'static str] {
    if target.contains("windows-msvc") {
        &[
            "legacy_stdio_definitions.lib",
            "kernel32.lib",
            "ntdll.lib",
            "userenv.lib",
            "ws2_32.lib",
            "dbghelp.lib",
            "/defaultlib:msvcrt",
        ]
    } else if target.contains("linux") {
        &["pthread", "dl", "m"]
    } else {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use super::{
        missing_native_runtime_message, native_runtime_artifact_name, native_runtime_link_for,
        native_static_libs_for_target, read_runtime_link_metadata, runtime_link_metadata_json,
        ORI_DRIVER_ABI_VERSION, ORI_VERSION,
    };

    fn source_section<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
        let start_index = source
            .find(start)
            .unwrap_or_else(|| panic!("source marker `{start}` not found"));
        let tail = &source[start_index..];
        let end_index = tail
            .find(end)
            .unwrap_or_else(|| panic!("source marker `{end}` not found after `{start}`"));
        &tail[..end_index]
    }

    #[test]
    fn native_compile_and_test_pipeline_do_not_use_legacy_c_runtime_hooks() {
        let source = include_str!("pipeline.rs");
        for forbidden in [
            concat!("ensure_", "cc_available"),
            concat!("build_", "runtime_lib"),
            concat!("ORI_", "RUNTIME_C"),
        ] {
            assert!(
                !source.contains(forbidden),
                "native pipeline must not contain legacy C runtime hook `{forbidden}`"
            );
        }

        let run_test = source_section(
            source,
            concat!("pub fn ", "run_test"),
            concat!("pub struct ", "BuildOutput"),
        );
        assert!(run_test.contains("run_native_tests"), "{run_test}");
        assert!(!run_test.contains("emit_c"), "{run_test}");

        let native_tests = source_section(
            source,
            concat!("fn ", "run_native_tests"),
            concat!("fn ", "inject_test_harness"),
        );
        assert!(
            native_tests.contains("find_native_runtime_link"),
            "{native_tests}"
        );
        assert!(
            native_tests.contains("ori_codegen::emit_native"),
            "{native_tests}"
        );
        assert!(native_tests.contains("ori_codegen::link"), "{native_tests}");
        assert!(!native_tests.contains("emit_c"), "{native_tests}");
    }

    #[test]
    fn native_pipeline_text_does_not_require_a_c_compiler() {
        let source = include_str!("pipeline.rs");
        for forbidden in [
            concat!("C ", "compiler"),
            concat!("C ", "toolchain"),
            concat!("requires `", "cc`"),
        ] {
            assert!(
                !source.contains(forbidden),
                "native pipeline text must not expose `{forbidden}` as a requirement"
            );
        }
    }

    #[test]
    fn native_runtime_artifact_names_are_platform_specific() {
        assert_eq!(
            native_runtime_artifact_name("x86_64-pc-windows-msvc"),
            "ori_runtime.lib"
        );
        assert_eq!(
            native_runtime_artifact_name("x86_64-pc-windows-gnu"),
            "libori_runtime.a"
        );
        assert_eq!(
            native_runtime_artifact_name("x86_64-unknown-linux-gnu"),
            "libori_runtime.a"
        );
    }

    #[test]
    fn runtime_link_metadata_names_rust_runtime_artifact() {
        let json = runtime_link_metadata_json(
            "x86_64-pc-windows-msvc",
            native_runtime_artifact_name("x86_64-pc-windows-msvc"),
        );

        assert!(json.contains("\"target\": \"x86_64-pc-windows-msvc\""));
        assert!(json.contains("\"runtime\": \"ori_runtime.lib\""));
        assert!(json.contains(&format!("\"ori_version\": \"{ORI_VERSION}\"")));
        assert!(json.contains(&format!("\"abi_version\": \"{ORI_DRIVER_ABI_VERSION}\"")));
        assert!(json.contains("legacy_stdio_definitions.lib"));
    }

    #[test]
    fn runtime_link_metadata_parser_reads_native_static_libs() {
        let dir = std::env::temp_dir().join(format!(
            "ori_runtime_link_metadata_parser_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let metadata_path = dir.join("runtime-link.json");
        std::fs::write(
            &metadata_path,
            runtime_link_metadata_json("x86_64-pc-windows-msvc", "ori_runtime.lib"),
        )
        .unwrap();

        let metadata = read_runtime_link_metadata(&metadata_path).unwrap();
        assert_eq!(metadata.target, "x86_64-pc-windows-msvc");
        assert_eq!(metadata.runtime, "ori_runtime.lib");
        assert_eq!(metadata.ori_version, ORI_VERSION);
        assert_eq!(metadata.abi_version, ORI_DRIVER_ABI_VERSION);
        assert!(metadata
            .native_static_libs
            .contains(&"kernel32.lib".to_string()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn packaged_runtime_link_reads_sibling_metadata() {
        let dir =
            std::env::temp_dir().join(format!("ori_packaged_runtime_link_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let runtime = dir.join("ori_runtime.lib");
        std::fs::write(&runtime, b"fake runtime").unwrap();
        std::fs::write(
            dir.join("runtime-link.json"),
            runtime_link_metadata_json("x86_64-pc-windows-msvc", "ori_runtime.lib"),
        )
        .unwrap();

        let link =
            native_runtime_link_for(runtime.clone(), "x86_64-pc-windows-msvc", "ori_runtime.lib")
                .unwrap();
        let args = link.link_args();

        assert_eq!(args.first(), Some(&runtime));
        assert!(link
            .native_static_libs
            .contains(&"kernel32.lib".to_string()));
        assert!(args
            .iter()
            .any(|arg| arg == std::path::Path::new("kernel32.lib")));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn runtime_target_mismatch_error_names_expected_and_actual_targets() {
        let dir = std::env::temp_dir().join(format!(
            "ori_runtime_target_mismatch_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let runtime = dir.join("ori_runtime.lib");
        std::fs::write(&runtime, b"fake runtime").unwrap();
        std::fs::write(
            dir.join("runtime-link.json"),
            runtime_link_metadata_json("x86_64-unknown-linux-gnu", "ori_runtime.lib"),
        )
        .unwrap();

        let err = native_runtime_link_for(runtime, "x86_64-pc-windows-msvc", "ori_runtime.lib")
            .expect_err("target mismatch should fail");

        assert!(err.contains("native.runtime_metadata_mismatch"), "{err}");
        assert!(err.contains("x86_64-unknown-linux-gnu"), "{err}");
        assert!(err.contains("x86_64-pc-windows-msvc"), "{err}");
        assert!(err.contains("runtime metadata"), "{err}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn runtime_abi_version_is_shared_between_runtime_and_driver() {
        assert_eq!(ORI_DRIVER_ABI_VERSION, ori_runtime::ORI_ABI_VERSION);
        assert!(!ORI_DRIVER_ABI_VERSION.trim().is_empty());
    }

    #[test]
    fn runtime_abi_mismatch_error_has_stable_code() {
        let dir =
            std::env::temp_dir().join(format!("ori_runtime_abi_mismatch_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let runtime = dir.join("ori_runtime.lib");
        std::fs::write(&runtime, b"fake runtime").unwrap();
        let json = runtime_link_metadata_json("x86_64-pc-windows-msvc", "ori_runtime.lib")
            .replace(ORI_DRIVER_ABI_VERSION, "ori-native-abi-test-mismatch");
        std::fs::write(dir.join("runtime-link.json"), json).unwrap();

        let err = native_runtime_link_for(runtime, "x86_64-pc-windows-msvc", "ori_runtime.lib")
            .expect_err("ABI mismatch should fail");

        assert!(err.contains("native.abi_mismatch"), "{err}");
        assert!(err.contains(ORI_DRIVER_ABI_VERSION), "{err}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_runtime_error_names_target_path_and_staging_command() {
        let searched = [
            std::path::PathBuf::from("package/runtime/x86_64-pc-windows-msvc/ori_runtime.lib"),
            std::path::PathBuf::from("target/debug/ori_runtime.lib"),
        ];
        let message = missing_native_runtime_message(
            "x86_64-pc-windows-msvc",
            "ori_runtime.lib",
            &searched,
            true,
        );

        assert!(message.contains("native.runtime_missing"), "{message}");
        assert!(message.contains("ori_runtime.lib"), "{message}");
        assert!(message.contains("x86_64-pc-windows-msvc"), "{message}");
        assert!(
            message.contains("runtime/x86_64-pc-windows-msvc/ori_runtime.lib"),
            "{message}"
        );
        assert!(message.contains("stage_native_runtime.ps1"), "{message}");
        assert!(
            message.contains("ORI_REQUIRE_PACKAGED_RUNTIME=1"),
            "{message}"
        );
        assert!(message.contains("package/runtime"), "{message}");
    }

    #[test]
    fn native_static_libs_are_known_for_msvc() {
        let libs = native_static_libs_for_target("x86_64-pc-windows-msvc");
        assert!(libs.contains(&"kernel32.lib"));
        assert!(libs.contains(&"/defaultlib:msvcrt"));
    }

    #[test]
    fn native_static_libs_are_known_for_linux() {
        let libs = native_static_libs_for_target("x86_64-unknown-linux-gnu");
        assert!(libs.contains(&"pthread"));
        assert!(libs.contains(&"dl"));
        assert!(libs.contains(&"m"));
    }
}

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

pub fn run_check_source(path: &Path, source: String) -> Result<CheckOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) =
        load_and_resolve_with_entry_source(path, source, &mut cache, &mut sink)?;

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

pub fn run_doc(path: &Path) -> Result<DocOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    let markdown = if !sink.has_errors() {
        render_documentation_markdown(&loaded)
    } else {
        String::new()
    };
    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(DocOutput {
        cache,
        markdown,
        diagnostics,
        has_errors,
    })
}

pub fn run_test(path: &Path) -> Result<TestOutput, String> {
    let mut cache = SourceCache::default();
    let mut sink = DiagnosticSink::default();
    let (loaded, resolved) = load_and_resolve(path, &mut cache, &mut sink)?;

    if !sink.has_errors() {
        check_loaded_sources(&loaded, &resolved, &mut sink);
    }

    let tests = if !sink.has_errors() {
        collect_test_cases(&loaded, &resolved, &mut sink)
    } else {
        Vec::new()
    };

    let results = if !sink.has_errors() && !tests.is_empty() {
        let hir = lower_loaded_sources(&loaded, &resolved, &mut sink);
        if sink.has_errors() {
            Vec::new()
        } else {
            run_native_tests(&hir, &tests)?
        }
    } else {
        Vec::new()
    };

    let has_errors = sink.has_errors();
    let diagnostics = sink.into_diagnostics();
    Ok(TestOutput {
        cache,
        diagnostics,
        has_errors,
        results,
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
        match ori_codegen::emit_c(&hir) {
            Ok(source) => source,
            Err(error) => {
                sink.emit(
                    Diagnostic::error(
                        "backend.c_unsupported",
                        "the C debug backend cannot generate this program correctly",
                    )
                    .with_why(
                        "the C backend is a secondary debug/transpile route with partial feature parity",
                    )
                    .with_action(
                        "use `ori compile` for the native backend, or remove the unsupported feature before generating C",
                    )
                    .with_note(error),
                );
                String::new()
            }
        }
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
    load_and_resolve_entry(&entry, None, cache, sink)
}

fn load_and_resolve_with_entry_source(
    path: &Path,
    source: String,
    cache: &mut SourceCache,
    sink: &mut DiagnosticSink,
) -> Result<(Vec<LoadedSource>, ResolvedModule), String> {
    let entry = resolve_entry_path(path)?;
    let entry = std::fs::canonicalize(entry).unwrap_or_else(|_| path.to_owned());
    load_and_resolve_entry(&entry, Some((&entry, &source)), cache, sink)
}

fn load_and_resolve_entry(
    entry: &Path,
    entry_source: Option<(&Path, &str)>,
    cache: &mut SourceCache,
    sink: &mut DiagnosticSink,
) -> Result<(Vec<LoadedSource>, ResolvedModule), String> {
    let mut loaded = Vec::new();
    let mut seen = HashSet::new();
    let mut active = Vec::new();
    load_source_recursive(
        entry,
        cache,
        sink,
        &mut seen,
        &mut active,
        &mut loaded,
        entry_source,
    )?;
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
    entry_source: Option<(&Path, &str)>,
) -> Result<(), String> {
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_owned());
    if !seen.insert(path.clone()) {
        return Ok(());
    }
    let source = match entry_source {
        Some((entry_path, source)) if entry_path == path.as_path() => source.to_string(),
        _ => read_file(&path)?,
    };
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
        source,
        tokens,
        ast,
    });
    active.push(path.clone());
    for (import, span) in imports {
        match classify_stdlib_import(&import) {
            StdlibImportStatus::Implemented => continue,
            StdlibImportStatus::Unknown => {
                sink.emit(
                    Diagnostic::error(
                        "bind.stdlib_module_unknown",
                        format!("standard library module `{}` is not known", import),
                    )
                    .with_label(Label::primary(file_id, span, "stdlib import here"))
                    .with_action("check the module name or use an implemented `ori.*` module"),
                );
                continue;
            }
            StdlibImportStatus::NotStdlib => {}
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
                load_source_recursive(
                    &import_path,
                    cache,
                    sink,
                    seen,
                    active,
                    loaded,
                    entry_source,
                )?;
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

enum StdlibImportStatus {
    Implemented,
    Unknown,
    NotStdlib,
}

fn classify_stdlib_import(import: &str) -> StdlibImportStatus {
    if import != "ori" && !import.starts_with("ori.") {
        return StdlibImportStatus::NotStdlib;
    }

    match import {
        "ori"
        | "ori.core"
        | "ori.io"
        | "ori.fs"
        | "ori.files"
        | "ori.string"
        | "ori.bytes"
        | "ori.list"
        | "ori.map"
        | "ori.set"
        | "ori.deque"
        | "ori.queue"
        | "ori.stack"
        | "ori.linked_list"
        | "ori.doubly_linked_list"
        | "ori.tree"
        | "ori.hash_table"
        | "ori.graph"
        | "ori.heap"
        | "ori.math"
        | "ori.convert"
        | "ori.mem"
        | "ori.time"
        | "ori.format"
        | "ori.os"
        | "ori.random"
        | "ori.iter"
        | "ori.lazy"
        | "ori.concurrent"
        | "ori.task"
        | "ori.channel"
        | "ori.atomic"
        | "ori.test"
        | "ori.json"
        | "ori.Error" => StdlibImportStatus::Implemented,
        _ => StdlibImportStatus::Unknown,
    }
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
        validate_doc_param_tags(source, sink);
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
            &resolved.deprecated_sigs,
            &resolved.reexports,
            &namespace,
            source.file_id,
            sink,
        );
        checker.check_file(&source.ast);
    }
}

#[derive(Clone)]
struct TestCase {
    name: String,
    span: ori_diagnostics::Span,
    is_async: bool,
}

fn collect_test_cases(
    loaded: &[LoadedSource],
    resolved: &ResolvedModule,
    sink: &mut DiagnosticSink,
) -> Vec<TestCase> {
    let mut tests = Vec::new();
    for source in loaded {
        let namespace = namespace_of(&source.ast);
        for item in &source.ast.items {
            if !item.attrs.iter().any(|attr| attr.name.text == "test") {
                continue;
            }
            let Item::Func(func) = &item.item else {
                continue;
            };
            let name = format!("{}.{}", namespace, func.name.text);
            let Some(def_id) = resolved.def_map.lookup(&name) else {
                continue;
            };
            let Some(sig) = resolved.func_sigs.iter().find(|sig| sig.def_id == def_id) else {
                continue;
            };
            let valid_return = if func.is_async {
                sig.return_ty == Ty::Future(Box::new(Ty::Void))
            } else {
                sig.return_ty == Ty::Void
            };
            if !func.type_params.is_empty() || !sig.params.is_empty() || !valid_return {
                sink.emit(
                    Diagnostic::error(
                        "attr.invalid_test_signature",
                        format!("test function `{}` has an invalid signature", func.name.text),
                    )
                    .with_label(Label::primary(
                        source.file_id,
                        func.span,
                        "test functions must be concrete functions with no parameters and no return value",
                    ))
                    .with_action(
                        "use `@test` on a function shaped like `func test_name() ... end` or `async func test_name() ... end`",
                    ),
                );
                continue;
            }
            tests.push(TestCase {
                name,
                span: func.span,
                is_async: func.is_async,
            });
        }
    }
    tests
}

fn run_native_tests(
    hir: &ori_hir::HirModule,
    tests: &[TestCase],
) -> Result<Vec<TestResult>, String> {
    let runtime_link = find_native_runtime_link()?;
    let mut results = Vec::new();

    for test in tests {
        let (obj_path, exe_path) = temp_test_paths();
        let mut test_hir = hir.clone();
        inject_test_harness(&mut test_hir, test);

        let run_result = (|| {
            ori_codegen::emit_native(&test_hir, &obj_path)?;
            let extra = runtime_link.link_args();
            ori_codegen::link(&obj_path, &exe_path, &extra)?;
            let output = std::process::Command::new(&exe_path)
                .output()
                .map_err(|e| format!("failed to run test `{}`: {e}", test.name))?;
            Ok::<TestResult, String>(TestResult {
                name: test.name.clone(),
                passed: output.status.success(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status.code(),
            })
        })();

        let _ = std::fs::remove_file(&obj_path);
        let _ = std::fs::remove_file(&exe_path);

        match run_result {
            Ok(result) => results.push(result),
            Err(error) => {
                results.push(TestResult {
                    name: test.name.clone(),
                    passed: false,
                    stdout: String::new(),
                    stderr: error,
                    status: Some(1),
                });
            }
        }
    }

    Ok(results)
}

fn inject_test_harness(module: &mut ori_hir::HirModule, test: &TestCase) {
    let span = test.span;
    let test_ret_ty = if test.is_async {
        Ty::Future(Box::new(Ty::Void))
    } else {
        Ty::Void
    };
    let callee_ty = Ty::Func {
        params: Vec::new(),
        ret: Box::new(test_ret_ty.clone()),
    };
    let call = HirExpr {
        kind: HirExprKind::Call {
            callee: Box::new(HirExpr {
                kind: HirExprKind::Var(test.name.as_str().into()),
                ty: callee_ty,
                span,
            }),
            args: Vec::new(),
        },
        ty: test_ret_ty.clone(),
        span,
    };
    let test_expr = if test.is_async {
        HirExpr {
            kind: HirExprKind::Call {
                callee: Box::new(HirExpr {
                    kind: HirExprKind::Var("ori_task_block_on".into()),
                    ty: Ty::Func {
                        params: vec![test_ret_ty.clone()],
                        ret: Box::new(Ty::Void),
                    },
                    span,
                }),
                args: vec![HirArg {
                    label: None,
                    spread: false,
                    value: call,
                }],
            },
            ty: Ty::Void,
            span,
        }
    } else {
        call
    };
    let harness_name = if module.namespace.is_empty() {
        "main".to_string()
    } else {
        format!("{}.main", module.namespace)
    };
    let harness = HirFunc {
        def_id: DefId(u32::MAX - 1),
        name: harness_name.into(),
        params: Vec::new(),
        return_ty: Ty::Void,
        body: HirBlock {
            stmts: vec![HirStmt::Expr(test_expr)],
            span,
        },
        closure_captures: Vec::new(),
        is_public: false,
        is_async: false,
        is_mut: false,
        span,
    };
    module.funcs.insert(0, harness);
}

fn temp_test_paths() -> (PathBuf, PathBuf) {
    static NEXT_TEST_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let id = NEXT_TEST_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let stem = format!("ori_test_{}_{}", std::process::id(), id);
    let tmp_dir = std::env::temp_dir();
    let obj_path = tmp_dir.join(format!("{stem}.o"));
    let exe_name = if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem
    };
    (obj_path, tmp_dir.join(exe_name))
}

fn format_source_text(source: &str) -> String {
    let mut indent = 0usize;
    let mut out = String::new();

    for raw_line in source.replace("\r\n", "\n").replace('\r', "\n").lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            out.push('\n');
            continue;
        }

        if should_dedent_before(line) {
            indent = indent.saturating_sub(1);
        }

        out.push_str(&"    ".repeat(indent));
        out.push_str(line);
        out.push('\n');

        if opens_block_after(line) {
            indent += 1;
        }
    }

    out
}

fn should_dedent_before(line: &str) -> bool {
    line == "end" || line == "else" || line.starts_with("else if ") || line.starts_with("case ")
}

fn opens_block_after(line: &str) -> bool {
    if is_comment_line(line) || line == "end" || line.starts_with("@") {
        return false;
    }

    line == "else"
        || line.starts_with("else if ")
        || line.starts_with("case ")
        || line == "loop"
        || line.starts_with("if ")
        || line.starts_with("while ")
        || line.starts_with("for ")
        || line.starts_with("repeat ")
        || line.starts_with("match ")
        || declaration_opens_block(line)
}

fn declaration_opens_block(line: &str) -> bool {
    let mut line = line;
    loop {
        let next = line
            .strip_prefix("public ")
            .or_else(|| line.strip_prefix("async "))
            .or_else(|| line.strip_prefix("mut "));
        let Some(next) = next else {
            break;
        };
        line = next;
    }
    line.starts_with("func ")
        || line.starts_with("struct ")
        || line.starts_with("enum ")
        || line.starts_with("trait ")
        || line.starts_with("implement ")
        || line.starts_with("extern ")
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("--") || line.starts_with("|--")
}

fn validate_doc_param_tags(source: &LoadedSource, sink: &mut DiagnosticSink) {
    for item in &source.ast.items {
        let leading_start = item
            .attrs
            .first()
            .map(|attr| attr.span.start)
            .unwrap_or_else(|| item.item.span().start);
        match &item.item {
            Item::Func(func) => validate_func_doc_params(
                source,
                leading_start,
                func.name.as_str(),
                &func.params,
                sink,
            ),
            Item::Struct(decl) => {
                for method in &decl.methods {
                    validate_func_doc_params(
                        source,
                        method.span.start,
                        method.name.as_str(),
                        &method.params,
                        sink,
                    );
                }
            }
            Item::Trait(decl) => {
                for member in &decl.members {
                    match member {
                        TraitMember::Required(sig) => validate_signature_doc_params(
                            source,
                            sig.span.start,
                            sig.name.as_str(),
                            &sig.params,
                            sink,
                        ),
                        TraitMember::Default(func) => validate_func_doc_params(
                            source,
                            func.span.start,
                            func.name.as_str(),
                            &func.params,
                            sink,
                        ),
                    }
                }
            }
            Item::Implement(decl) => {
                for method in &decl.methods {
                    validate_func_doc_params(
                        source,
                        method.span.start,
                        method.name.as_str(),
                        &method.params,
                        sink,
                    );
                }
            }
            Item::Extern(decl) => {
                for member in &decl.members {
                    if let ExternMember::Func {
                        name, params, span, ..
                    } = member
                    {
                        validate_signature_doc_params(
                            source,
                            span.start,
                            name.as_str(),
                            params,
                            sink,
                        );
                    }
                }
            }
            Item::Enum(_) | Item::Alias(_) | Item::Const(_) | Item::Var(_) => {}
        }
    }
}

fn validate_func_doc_params(
    source: &LoadedSource,
    leading_start: u32,
    func_name: &str,
    params: &[Param],
    sink: &mut DiagnosticSink,
) {
    validate_doc_params_for_signature(source, leading_start, func_name, params, sink);
}

fn validate_signature_doc_params(
    source: &LoadedSource,
    leading_start: u32,
    func_name: &str,
    params: &[Param],
    sink: &mut DiagnosticSink,
) {
    validate_doc_params_for_signature(source, leading_start, func_name, params, sink);
}

fn validate_doc_params_for_signature(
    source: &LoadedSource,
    leading_start: u32,
    func_name: &str,
    params: &[Param],
    sink: &mut DiagnosticSink,
) {
    let Some(doc_span) = leading_block_comment_before(&source.tokens, leading_start) else {
        return;
    };
    let param_names: HashSet<&str> = params.iter().map(|param| param.name.as_str()).collect();
    for tag in doc_param_tags(&source.source[doc_span.as_range()]) {
        if tag.name.is_empty() || !param_names.contains(tag.name) {
            let name = if tag.name.is_empty() {
                "missing parameter name"
            } else {
                tag.name
            };
            sink.emit(
                Diagnostic::warning(
                    "doc.param_name_mismatch",
                    format!("documentation tag `@param {name}` does not match `{func_name}`"),
                )
                .with_label(Label::primary(
                    source.file_id,
                    doc_span,
                    "documentation comment here",
                ))
                .with_action("rename the @param tag or remove it"),
            );
        }
    }
}

fn leading_block_comment_before(
    tokens: &[Token],
    leading_start: u32,
) -> Option<ori_diagnostics::Span> {
    let item_index = tokens
        .iter()
        .position(|token| token.span.start >= leading_start)?;
    let mut index = item_index;
    while let Some(previous) = index.checked_sub(1) {
        let token = &tokens[previous];
        if token.kind == TokenKind::Public {
            index = previous;
            continue;
        }
        return (token.kind == TokenKind::BlockComment).then_some(token.span);
    }
    None
}

struct DocParamTag<'a> {
    name: &'a str,
}

fn doc_param_tags(comment: &str) -> Vec<DocParamTag<'_>> {
    let body = comment
        .strip_prefix("--|")
        .unwrap_or(comment)
        .strip_suffix("|--")
        .unwrap_or(comment);
    body.lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("@param")?;
            let rest = rest.trim_start();
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(|ch: char| ch == ':' || ch == '-');
            Some(DocParamTag { name })
        })
        .collect()
}

#[derive(Default)]
struct ParsedDocComment {
    body: Vec<String>,
    params: Vec<(String, String)>,
    returns: Option<String>,
}

struct StdlibDocSignature {
    module: &'static str,
    signature: &'static str,
}

const COLLECTION_STDLIB_DOC_SIGNATURES: &[StdlibDocSignature] = &[
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.new<T>() -> deque.Deque<T>",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.push_front<T>(d: deque.Deque<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.push_back<T>(d: deque.Deque<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.pop_front<T>(d: deque.Deque<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.pop_back<T>(d: deque.Deque<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.front<T>(d: deque.Deque<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.back<T>(d: deque.Deque<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.len<T>(d: deque.Deque<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.is_empty<T>(d: deque.Deque<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.clear<T>(d: deque.Deque<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.deque",
        signature: "deque.to_list<T>(d: deque.Deque<T>) -> list<T>",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.new<T>() -> queue.Queue<T>",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.enqueue<T>(q: queue.Queue<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.dequeue<T>(q: queue.Queue<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.peek<T>(q: queue.Queue<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.len<T>(q: queue.Queue<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.is_empty<T>(q: queue.Queue<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.clear<T>(q: queue.Queue<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.queue",
        signature: "queue.to_list<T>(q: queue.Queue<T>) -> list<T>",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.new<T>() -> stack.Stack<T>",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.push<T>(s: stack.Stack<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.pop<T>(s: stack.Stack<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.peek<T>(s: stack.Stack<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.len<T>(s: stack.Stack<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.is_empty<T>(s: stack.Stack<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.clear<T>(s: stack.Stack<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.stack",
        signature: "stack.to_list<T>(s: stack.Stack<T>) -> list<T>",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.new<T>() -> linked_list.LinkedList<T>",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.push_front<T>(list: linked_list.LinkedList<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.push_back<T>(list: linked_list.LinkedList<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.pop_front<T>(list: linked_list.LinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.front<T>(list: linked_list.LinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.len<T>(list: linked_list.LinkedList<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.is_empty<T>(list: linked_list.LinkedList<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.clear<T>(list: linked_list.LinkedList<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.linked_list",
        signature: "linked_list.to_list<T>(list: linked_list.LinkedList<T>) -> list<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.new<T>() -> doubly_linked_list.DoublyLinkedList<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.push_front<T>(list: doubly_linked_list.DoublyLinkedList<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.push_back<T>(list: doubly_linked_list.DoublyLinkedList<T>, value: T) -> void",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.pop_front<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.pop_back<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.front<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.back<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> optional<T>",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.len<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.is_empty<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.clear<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.doubly_linked_list",
        signature: "doubly_linked_list.to_list<T>(list: doubly_linked_list.DoublyLinkedList<T>) -> list<T>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.new<T>(root: T) -> tree.Tree<T>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.root<T>(t: tree.Tree<T>) -> tree.NodeId",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.value<T>(t: tree.Tree<T>, node: tree.NodeId) -> T",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.add_child<T>(t: tree.Tree<T>, parent: tree.NodeId, value: T) -> tree.NodeId",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.children<T>(t: tree.Tree<T>, node: tree.NodeId) -> list<tree.NodeId>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.parent<T>(t: tree.Tree<T>, node: tree.NodeId) -> optional<tree.NodeId>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.remove_subtree<T>(t: tree.Tree<T>, node: tree.NodeId) -> void",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.len<T>(t: tree.Tree<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.depth<T>(t: tree.Tree<T>, node: tree.NodeId) -> int",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.pre_order<T>(t: tree.Tree<T>) -> list<tree.NodeId>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.post_order<T>(t: tree.Tree<T>) -> list<tree.NodeId>",
    },
    StdlibDocSignature {
        module: "ori.tree",
        signature: "tree.breadth_first<T>(t: tree.Tree<T>) -> list<tree.NodeId>",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.new<K, V>() -> hash_table.HashTable<K, V> where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.with_capacity<K, V>(capacity: int) -> hash_table.HashTable<K, V> where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.set<K, V>(table: hash_table.HashTable<K, V>, key: K, value: V) -> void where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.get<K, V>(table: hash_table.HashTable<K, V>, key: K) -> optional<V> where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.remove<K, V>(table: hash_table.HashTable<K, V>, key: K) -> optional<V> where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.contains<K, V>(table: hash_table.HashTable<K, V>, key: K) -> bool where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.len<K, V>(table: hash_table.HashTable<K, V>) -> int",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.capacity<K, V>(table: hash_table.HashTable<K, V>) -> int",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.reserve<K, V>(table: hash_table.HashTable<K, V>, capacity: int) -> void",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.clear<K, V>(table: hash_table.HashTable<K, V>) -> void",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.keys<K, V>(table: hash_table.HashTable<K, V>) -> list<K>",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.values<K, V>(table: hash_table.HashTable<K, V>) -> list<V>",
    },
    StdlibDocSignature {
        module: "ori.hash_table",
        signature: "hash_table.entries<K, V>(table: hash_table.HashTable<K, V>) -> list<tuple<K, V>>",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.new<N>(directed: bool) -> graph.Graph<N> where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.add_node<N>(g: graph.Graph<N>, node: N) -> void where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.remove_node<N>(g: graph.Graph<N>, node: N) -> void where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.add_edge<N>(g: graph.Graph<N>, from: N, to: N) -> void where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.remove_edge<N>(g: graph.Graph<N>, from: N, to: N) -> void where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.has_node<N>(g: graph.Graph<N>, node: N) -> bool where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.has_edge<N>(g: graph.Graph<N>, from: N, to: N) -> bool where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.neighbors<N>(g: graph.Graph<N>, node: N) -> list<N> where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.nodes<N>(g: graph.Graph<N>) -> list<N>",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.edges<N>(g: graph.Graph<N>) -> list<tuple<N, N>>",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.bfs<N>(g: graph.Graph<N>, start: N) -> list<N> where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.dfs<N>(g: graph.Graph<N>, start: N) -> list<N> where N is Hashable and N is Equatable",
    },
    StdlibDocSignature {
        module: "ori.graph",
        signature: "graph.topological_sort<N>(g: graph.Graph<N>) -> list<N>",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.new<T>() -> heap.Heap<T> where T is Comparable",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.push<T>(h: heap.Heap<T>, value: T) -> void where T is Comparable",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.pop<T>(h: heap.Heap<T>) -> optional<T> where T is Comparable",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.peek<T>(h: heap.Heap<T>) -> optional<T> where T is Comparable",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.len<T>(h: heap.Heap<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.heap",
        signature: "heap.is_empty<T>(h: heap.Heap<T>) -> bool",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.new<K, V>() -> map<K, V> where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.set<K, V>(m: map<K, V>, key: K, value: V) -> void where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.get<K, V>(m: map<K, V>, key: K) -> V where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.contains<K, V>(m: map<K, V>, key: K) -> bool where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.remove<K, V>(m: map<K, V>, key: K) -> void where K is Hashable and K is Equatable",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.len<K, V>(m: map<K, V>) -> int",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.capacity<K, V>(m: map<K, V>) -> int",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.reserve<K, V>(m: map<K, V>, capacity: int) -> void",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.clear<K, V>(m: map<K, V>) -> void",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.keys<K, V>(m: map<K, V>) -> list<K>",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.values<K, V>(m: map<K, V>) -> list<V>",
    },
    StdlibDocSignature {
        module: "ori.map",
        signature: "maps.entries<K, V>(m: map<K, V>) -> list<tuple<K, V>>",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.new<T>() -> set<T> where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.add<T>(s: set<T>, value: T) -> void where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.contains<T>(s: set<T>, value: T) -> bool where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.remove<T>(s: set<T>, value: T) -> void where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.len<T>(s: set<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.capacity<T>(s: set<T>) -> int",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.reserve<T>(s: set<T>, capacity: int) -> void",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.clear<T>(s: set<T>) -> void",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.union<T>(a: set<T>, b: set<T>) -> set<T> where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.intersection<T>(a: set<T>, b: set<T>) -> set<T> where T is Hashable and T is Equatable",
    },
    StdlibDocSignature {
        module: "ori.set",
        signature: "sets.difference<T>(a: set<T>, b: set<T>) -> set<T> where T is Hashable and T is Equatable",
    },
];

fn render_documentation_markdown(loaded: &[LoadedSource]) -> String {
    let mut out = String::from("# Ori API Documentation\n\n");
    let mut entry_count = 0usize;

    for source in loaded {
        entry_count += render_source_documentation(source, &mut out);
    }

    if entry_count == 0 {
        out.push_str("No documentation comments found.\n\n");
    }

    append_stdlib_documentation(&mut out);

    out
}

fn append_stdlib_documentation(out: &mut String) {
    let mut modules = BTreeSet::new();
    for entry in ori_types::stdlib::stdlib_runtime_functions() {
        if let Some((module, _)) = entry.canonical_path.rsplit_once('.') {
            modules.insert(module);
        }
    }

    out.push_str("## Standard Library\n\n");
    out.push_str("### Modules\n\n");
    for module in modules {
        let _ = writeln!(out, "- `{module}`");
    }
    out.push('\n');

    let mut by_module: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for entry in COLLECTION_STDLIB_DOC_SIGNATURES {
        by_module
            .entry(entry.module)
            .or_default()
            .push(entry.signature);
    }

    out.push_str("### Collection Signatures\n\n");
    for (module, signatures) in by_module {
        let _ = writeln!(out, "#### `{module}`\n");
        out.push_str("```ori\n");
        for signature in signatures {
            let _ = writeln!(out, "{signature}");
        }
        out.push_str("```\n\n");
    }
}

fn render_source_documentation(source: &LoadedSource, out: &mut String) -> usize {
    let mut entry_count = 0usize;
    let namespace = namespace_of(&source.ast);

    for item in &source.ast.items {
        let leading_start = item
            .attrs
            .first()
            .map(|attr| attr.span.start)
            .unwrap_or_else(|| item.item.span().start);

        match &item.item {
            Item::Func(func) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, func.name),
                        "function",
                        &func_signature_text(source, func),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
            }
            Item::Struct(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "struct",
                        &format!(
                            "{}struct {}{}{}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_params_text(&decl.type_params),
                            where_text(source, decl.where_clause.as_ref())
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
                for method in &decl.methods {
                    if let Some(doc) = doc_comment_for(source, method.span.start) {
                        append_doc_entry(
                            out,
                            &format!("{}.{}.{}", namespace, decl.name, method.name),
                            "method",
                            &func_signature_text(source, method),
                            &doc,
                            source,
                        );
                        entry_count += 1;
                    }
                }
            }
            Item::Enum(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "enum",
                        &format!(
                            "{}enum {}{}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_params_text(&decl.type_params)
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
            }
            Item::Trait(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "trait",
                        &format!(
                            "{}trait {}{}{}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_params_text(&decl.type_params),
                            where_text(source, decl.where_clause.as_ref())
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
                for member in &decl.members {
                    match member {
                        TraitMember::Required(sig) => {
                            if let Some(doc) = doc_comment_for(source, sig.span.start) {
                                append_doc_entry(
                                    out,
                                    &format!("{}.{}.{}", namespace, decl.name, sig.name),
                                    "trait method",
                                    &func_signature_decl_text(source, sig),
                                    &doc,
                                    source,
                                );
                                entry_count += 1;
                            }
                        }
                        TraitMember::Default(func) => {
                            if let Some(doc) = doc_comment_for(source, func.span.start) {
                                append_doc_entry(
                                    out,
                                    &format!("{}.{}.{}", namespace, decl.name, func.name),
                                    "trait method",
                                    &func_signature_text(source, func),
                                    &doc,
                                    source,
                                );
                                entry_count += 1;
                            }
                        }
                    }
                }
            }
            Item::Implement(decl) => {
                for method in &decl.methods {
                    if let Some(doc) = doc_comment_for(source, method.span.start) {
                        append_doc_entry(
                            out,
                            &format!(
                                "{}.implement {} for {}.{}",
                                namespace, decl.trait_name, decl.for_type, method.name
                            ),
                            "implementation method",
                            &func_signature_text(source, method),
                            &doc,
                            source,
                        );
                        entry_count += 1;
                    }
                }
            }
            Item::Alias(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "alias",
                        &format!(
                            "{}alias {}{} = {}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_params_text(&decl.type_params),
                            type_text(source, &decl.ty)
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
            }
            Item::Const(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "constant",
                        &format!(
                            "{}const {}: {}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_text(source, &decl.ty)
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
            }
            Item::Var(decl) => {
                if let Some(doc) = doc_comment_for(source, leading_start) {
                    append_doc_entry(
                        out,
                        &format!("{}.{}", namespace, decl.name),
                        "variable",
                        &format!(
                            "{}var {}: {}",
                            visibility_prefix(decl.visibility),
                            decl.name,
                            type_text(source, &decl.ty)
                        ),
                        &doc,
                        source,
                    );
                    entry_count += 1;
                }
            }
            Item::Extern(decl) => {
                for member in &decl.members {
                    match member {
                        ExternMember::Func {
                            visibility,
                            name,
                            params,
                            return_ty,
                            span,
                        } => {
                            if let Some(doc) = doc_comment_for(source, span.start) {
                                append_doc_entry(
                                    out,
                                    &format!("{}.{}", namespace, name),
                                    "extern function",
                                    &func_signature_parts_text(
                                        source,
                                        *visibility,
                                        name.as_str(),
                                        params,
                                        return_ty.as_ref(),
                                        None,
                                    ),
                                    &doc,
                                    source,
                                );
                                entry_count += 1;
                            }
                        }
                        ExternMember::Var {
                            visibility,
                            name,
                            ty,
                            span,
                        } => {
                            if let Some(doc) = doc_comment_for(source, span.start) {
                                append_doc_entry(
                                    out,
                                    &format!("{}.{}", namespace, name),
                                    "extern variable",
                                    &format!(
                                        "{}var {}: {}",
                                        visibility_prefix(*visibility),
                                        name,
                                        type_text(source, ty)
                                    ),
                                    &doc,
                                    source,
                                );
                                entry_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    entry_count
}

fn doc_comment_for(source: &LoadedSource, leading_start: u32) -> Option<ParsedDocComment> {
    let span = leading_block_comment_before(&source.tokens, leading_start)?;
    Some(parse_doc_comment(&source.source[span.as_range()]))
}

fn parse_doc_comment(comment: &str) -> ParsedDocComment {
    let mut doc = ParsedDocComment::default();
    for line in cleaned_doc_lines(comment) {
        if let Some(rest) = line.strip_prefix("@param") {
            let rest = rest.trim_start();
            let mut parts = rest.splitn(2, char::is_whitespace);
            let name = parts
                .next()
                .unwrap_or("")
                .trim_matches(|ch| ch == ':' || ch == '-');
            let description = parts.next().unwrap_or("").trim();
            doc.params.push((name.to_string(), description.to_string()));
        } else if let Some(rest) = line
            .strip_prefix("@returns")
            .or_else(|| line.strip_prefix("@return"))
        {
            let text = rest.trim();
            if !text.is_empty() {
                doc.returns = Some(text.to_string());
            }
        } else {
            doc.body.push(line);
        }
    }
    trim_empty_doc_lines(&mut doc.body);
    doc
}

fn cleaned_doc_lines(comment: &str) -> Vec<String> {
    let body = comment
        .strip_prefix("--|")
        .unwrap_or(comment)
        .strip_suffix("|--")
        .unwrap_or(comment);
    let mut lines: Vec<String> = body
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix('*')
                .unwrap_or(trimmed)
                .trim()
                .to_string()
        })
        .collect();
    trim_empty_doc_lines(&mut lines);
    lines
}

fn trim_empty_doc_lines(lines: &mut Vec<String>) {
    while lines.first().is_some_and(|line| line.is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
}

fn append_doc_entry(
    out: &mut String,
    name: &str,
    kind: &str,
    signature: &str,
    doc: &ParsedDocComment,
    source: &LoadedSource,
) {
    let _ = writeln!(out, "## {name}");
    let _ = writeln!(out);
    let _ = writeln!(out, "- Kind: {kind}");
    let _ = writeln!(out, "- Source: {}", source.path.display());
    let _ = writeln!(out);
    let _ = writeln!(out, "```ori");
    let _ = writeln!(out, "{signature}");
    let _ = writeln!(out, "```");
    let _ = writeln!(out);

    if !doc.body.is_empty() {
        for line in &doc.body {
            let _ = writeln!(out, "{line}");
        }
        let _ = writeln!(out);
    }

    if !doc.params.is_empty() {
        let _ = writeln!(out, "Parameters:");
        for (name, description) in &doc.params {
            if description.is_empty() {
                let _ = writeln!(out, "- `{name}`");
            } else {
                let _ = writeln!(out, "- `{name}`: {description}");
            }
        }
        let _ = writeln!(out);
    }

    if let Some(returns) = &doc.returns {
        let _ = writeln!(out, "Returns: {returns}");
        let _ = writeln!(out);
    }
}

fn func_signature_text(source: &LoadedSource, func: &ori_ast::item::FuncDecl) -> String {
    func_signature_parts_text(
        source,
        func.visibility,
        func.name.as_str(),
        &func.params,
        func.return_ty.as_ref(),
        func.where_clause.as_ref(),
    )
}

fn func_signature_decl_text(source: &LoadedSource, sig: &ori_ast::item::FuncSignature) -> String {
    func_signature_parts_text(
        source,
        sig.visibility,
        sig.name.as_str(),
        &sig.params,
        sig.return_ty.as_ref(),
        sig.where_clause.as_ref(),
    )
}

fn func_signature_parts_text(
    source: &LoadedSource,
    visibility: ori_ast::Visibility,
    name: &str,
    params: &[Param],
    return_ty: Option<&Type>,
    where_clause: Option<&WhereClause>,
) -> String {
    let params = params
        .iter()
        .map(|param| param_signature_text(source, param))
        .collect::<Vec<_>>()
        .join(", ");
    let mut signature = format!("{}func {}({})", visibility_prefix(visibility), name, params);
    if let Some(return_ty) = return_ty {
        signature.push_str(" -> ");
        signature.push_str(&type_text(source, return_ty));
    }
    signature.push_str(&where_text(source, where_clause));
    signature
}

fn param_signature_text(source: &LoadedSource, param: &Param) -> String {
    let mut text = format!("{}: {}", param.name, type_text(source, &param.ty));
    if matches!(param.kind, ori_ast::ParamKind::Variadic) {
        text.push_str("...");
    }
    text
}

fn type_text(source: &LoadedSource, ty: &Type) -> String {
    clean_source_fragment(&source.source[ty.span().as_range()])
}

fn where_text(source: &LoadedSource, where_clause: Option<&WhereClause>) -> String {
    where_clause
        .map(|clause| {
            format!(
                " {}",
                clean_source_fragment(&source.source[clause.span.as_range()])
            )
        })
        .unwrap_or_default()
}

fn type_params_text(params: &TypeParams) -> String {
    if params.is_empty() {
        String::new()
    } else {
        format!(
            "<{}>",
            params
                .iter()
                .map(|param| param.name.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn clean_source_fragment(fragment: &str) -> String {
    fragment.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn visibility_prefix(visibility: ori_ast::Visibility) -> &'static str {
    if visibility.is_public() {
        "public "
    } else {
        ""
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
        &resolved.value_sigs,
        &resolved.struct_sigs,
        &resolved.enum_sigs,
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

// The native route now links against the Rust ori-runtime static library. ori build remains the C debug backend.
