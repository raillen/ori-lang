use clap::{Parser, Subcommand};
use ori_driver::{emit, explain, pipeline};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(
    name    = "ori",
    version = env!("CARGO_PKG_VERSION"),
    about   = "The Ori language compiler",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Disable ANSI colour output.
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Type-check an Ori source file and report diagnostics.
    Check {
        /// Path to the `.orl` source file.
        file: PathBuf,
    },
    /// Extract documentation comments as Markdown or static HTML.
    Doc {
        #[command(subcommand)]
        action: DocAction,
    },
    /// Install an Ori package from the registry (not yet available).
    Install {
        /// Package name (e.g. `example.demo`).
        name: String,
    },
    /// Publish an Ori package to the registry (not yet available).
    Publish {
        /// Path to the package manifest or project root.
        path: PathBuf,
    },
    /// Run functions marked with `@test` through the native runtime.
    Test {
        /// Path to the `.orl` source file or project manifest.
        file: PathBuf,
    },
    /// Format an Ori source file and print the result.
    Fmt {
        /// Path to the `.orl` source file.
        file: PathBuf,
    },
    /// Print the raw token stream (debug).
    Lex {
        /// Path to the `.orl` source file.
        file: PathBuf,
    },
    /// Print the AST (debug).
    Parse {
        /// Path to the `.orl` source file.
        file: PathBuf,
    },
    /// Compile to a native binary via Cranelift and the packaged native runtime.
    Compile {
        /// Path to the `.orl` source file.
        file: PathBuf,
        /// Output executable path (default: same name as source, no extension).
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Print full native linker stdout/stderr when link fails.
        #[arg(long)]
        native_raw: bool,
    },
    /// Compile and run an Ori source file through the native runtime.
    Run {
        /// Path to the `.orl` source file.
        file: PathBuf,
        /// Print full native linker stdout/stderr when link fails.
        #[arg(long)]
        native_raw: bool,
    },
    /// Compile to C source (debug backend with partial feature parity).
    Build {
        /// Path to the `.orl` source file.
        file: PathBuf,
        /// Write generated C to this file instead of stdout.
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Report environment, stdlib, and native runtime health.
    Doctor,
    /// Explain a diagnostic code from the error catalog.
    Explain {
        /// Diagnostic code (e.g. `name.undefined`).
        code: String,
    },
    /// Print project overview: entry, namespaces, imports.
    Summary {
        /// Path to the `.orl` entry file or project root.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum DocAction {
    /// Extract documentation from a source file (Markdown or HTML).
    File {
        /// Path to the `.orl` source file or project manifest.
        file: PathBuf,
        /// Output format (`markdown` default, or `html` for a static page).
        #[arg(long, value_enum, default_value_t = DocFormatCli::Markdown)]
        format: DocFormatCli,
        /// Write output to this file instead of stdout.
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Export stdlib + error catalog JSON for the documentation website.
    Export {
        /// Output JSON path (default: stdout).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum DocFormatCli {
    Markdown,
    Html,
}

fn main() {
    let cli = Cli::parse();
    let color = !cli.no_color && std::env::var("NO_COLOR").is_err();

    match &cli.command {
        Commands::Check { file } => match pipeline::run_check(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(out) => {
                let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                let warnings = out.diagnostics.len() - errors;
                emit::render_all(&out.cache, &out.diagnostics, color);
                emit::print_summary(errors, warnings, color);
                process::exit(if out.has_errors { 1 } else { 0 });
            }
        },

        Commands::Doc { action } => match action {
            DocAction::File { file, format, out } => {
                let doc_format = match format {
                    DocFormatCli::Markdown => pipeline::DocFormat::Markdown,
                    DocFormatCli::Html => pipeline::DocFormat::Html,
                };
                match pipeline::run_doc_with_options(
                    file,
                    pipeline::DocOptions { format: doc_format },
                ) {
                    Err(e) => {
                        eprintln!("ori: {}", e);
                        process::exit(2);
                    }
                    Ok(doc) => {
                        let errors = doc.diagnostics.iter().filter(|d| d.is_error()).count();
                        let warnings = doc.diagnostics.len() - errors;
                        emit::render_all(&doc.cache, &doc.diagnostics, color);
                        emit::print_summary(errors, warnings, color);
                        if !doc.has_errors {
                            let content = match doc_format {
                                pipeline::DocFormat::Html => &doc.html,
                                pipeline::DocFormat::Markdown => &doc.markdown,
                            };
                            if let Some(path) = out {
                                std::fs::write(path, content).unwrap_or_else(|e| {
                                    eprintln!("ori: {}", e);
                                    process::exit(2);
                                });
                            } else {
                                print!("{content}");
                            }
                        }
                        process::exit(if doc.has_errors { 1 } else { 0 });
                    }
                }
            }
            DocAction::Export { out } => {
                use ori_driver::doc_export;
                match out {
                    Some(path) => {
                        if let Err(e) = doc_export::write_doc_export(path) {
                            eprintln!("ori: {}", e);
                            process::exit(2);
                        }
                    }
                    None => match doc_export::export_doc_json() {
                        Err(e) => {
                            eprintln!("ori: {}", e);
                            process::exit(2);
                        }
                        Ok(json) => print!("{json}"),
                    },
                }
            }
        },

        Commands::Install { name } => {
            eprintln!(
                "ori: registry install is not available yet (backlog v2)\n\
                 package `{name}` cannot be fetched — see docs/planning/registry-v2.md"
            );
            process::exit(2);
        }

        Commands::Publish { path } => {
            eprintln!(
                "ori: registry publish is not available yet (backlog v2)\n\
                 cannot publish `{}` — see docs/planning/registry-v2.md",
                path.display()
            );
            process::exit(2);
        }

        Commands::Test { file } => match pipeline::run_test(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(out) => {
                let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                let warnings = out.diagnostics.len() - errors;
                emit::render_all(&out.cache, &out.diagnostics, color);
                emit::print_summary(errors, warnings, color);
                if !out.has_errors {
                    let skipped = out.results.iter().filter(|result| result.skipped).count();
                    let passed = out
                        .results
                        .iter()
                        .filter(|result| result.passed && !result.skipped)
                        .count();
                    let failed = out
                        .results
                        .iter()
                        .filter(|result| !result.passed)
                        .count();
                    for result in &out.results {
                        if result.skipped {
                            eprintln!("skip: {}", result.name);
                        } else if result.passed {
                            eprintln!("ok: {}", result.name);
                        } else {
                            eprintln!(
                                "fail: {} (status: {})",
                                result.name,
                                result
                                    .status
                                    .map(|status| status.to_string())
                                    .unwrap_or_else(|| "terminated".to_string())
                            );
                            if !result.stdout.trim().is_empty() {
                                eprintln!("stdout:\n{}", result.stdout.trim_end());
                            }
                            if !result.stderr.trim().is_empty() {
                                eprintln!("stderr:\n{}", result.stderr.trim_end());
                            }
                        }
                    }
                    if skipped > 0 {
                        eprintln!("tests: {passed} passed, {skipped} skipped, {failed} failed");
                    } else {
                        eprintln!("tests: {passed} passed, {failed} failed");
                    }
                }
                let failed = out.results.iter().any(|result| !result.passed);
                process::exit(if out.has_errors || failed { 1 } else { 0 });
            }
        },

        Commands::Fmt { file } => match pipeline::run_fmt(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(out) => {
                let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                let warnings = out.diagnostics.len() - errors;
                emit::render_all(&out.cache, &out.diagnostics, color);
                emit::print_summary(errors, warnings, color);
                if !out.has_errors {
                    print!("{}", out.formatted);
                }
                process::exit(if out.has_errors { 1 } else { 0 });
            }
        },

        Commands::Lex { file } => match pipeline::run_lex(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(out) => {
                for tok in &out.tokens {
                    println!("{:?}  {:?}", tok.kind, tok.span);
                }
                let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                emit::render_all(&out.cache, &out.diagnostics, color);
                process::exit(if errors > 0 { 1 } else { 0 });
            }
        },

        Commands::Compile {
            file,
            out,
            native_raw,
        } => {
            let default_out = file.with_extension(if cfg!(windows) { "exe" } else { "" });
            let exe = out.as_deref().unwrap_or(&default_out);
            match pipeline::run_compile_with_options(
                file,
                exe,
                pipeline::CompileOptions {
                    native_raw: *native_raw,
                },
            ) {
                Err(e) => {
                    eprintln!("ori: {}", e);
                    process::exit(2);
                }
                Ok(out) => {
                    let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                    let warnings = out.diagnostics.len() - errors;
                    emit::render_all(&out.cache, &out.diagnostics, color);
                    emit::print_summary(errors, warnings, color);
                    if !out.has_errors {
                        eprintln!("binary: {}", out.exe_path.display());
                    }
                    process::exit(if out.has_errors { 1 } else { 0 });
                }
            }
        }

        Commands::Run { file, native_raw } => {
            if pipeline::should_use_jit_for_run() {
                match pipeline::run_jit(file) {
                    Err(e) => {
                        eprintln!("ori: {}", e);
                        process::exit(2);
                    }
                    Ok(out) => {
                        let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                        let warnings = out.diagnostics.len() - errors;
                        emit::render_all(&out.cache, &out.diagnostics, color);
                        emit::print_summary(errors, warnings, color);
                        if out.has_errors {
                            process::exit(1);
                        }
                        process::exit(out.exit_code);
                    }
                }
            }
            let exe = temp_run_exe_path(file);
            let compile = pipeline::run_compile_with_options(
                file,
                &exe,
                pipeline::CompileOptions {
                    native_raw: *native_raw,
                },
            );
            match compile {
                Err(e) => {
                    let _ = std::fs::remove_file(&exe);
                    eprintln!("ori: {}", e);
                    process::exit(2);
                }
                Ok(out) => {
                    let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                    let warnings = out.diagnostics.len() - errors;
                    emit::render_all(&out.cache, &out.diagnostics, color);
                    emit::print_summary(errors, warnings, color);
                    if out.has_errors {
                        let _ = std::fs::remove_file(&out.exe_path);
                        process::exit(1);
                    }
                    let status = process::Command::new(&out.exe_path).status();
                    let _ = std::fs::remove_file(&out.exe_path);
                    match status {
                        Err(e) => {
                            eprintln!("ori: {}", e);
                            process::exit(2);
                        }
                        Ok(status) => process::exit(status.code().unwrap_or(1)),
                    }
                }
            }
        }

        Commands::Build { file, out } => match pipeline::run_build(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(build) => {
                let errors = build.diagnostics.iter().filter(|d| d.is_error()).count();
                let warnings = build.diagnostics.len() - errors;
                emit::render_all(&build.cache, &build.diagnostics, color);
                emit::print_summary(errors, warnings, color);
                if !build.has_errors {
                    match out {
                        Some(p) => {
                            std::fs::write(p, &build.c_source).unwrap_or_else(|e| {
                                eprintln!("ori: {}", e);
                                process::exit(2);
                            });
                        }
                        None => print!("{}", build.c_source),
                    }
                }
                process::exit(if build.has_errors { 1 } else { 0 });
            }
        },

        Commands::Doctor => {
            let report = pipeline::run_doctor();
            for check in &report.checks {
                let (icon, label) = match check.status {
                    pipeline::DoctorStatus::Ok => ("ok", "OK"),
                    pipeline::DoctorStatus::Warn => ("warn", "WARN"),
                    pipeline::DoctorStatus::Fail => ("fail", "FAIL"),
                };
                if color {
                    let color_code = match check.status {
                        pipeline::DoctorStatus::Ok => "\x1b[32m",
                        pipeline::DoctorStatus::Warn => "\x1b[33m",
                        pipeline::DoctorStatus::Fail => "\x1b[31m",
                    };
                    eprintln!(
                        "{color_code}[{label}]\x1b[0m {} — {}",
                        check.name, check.detail
                    );
                } else {
                    eprintln!("[{icon}] {} — {}", check.name, check.detail);
                }
            }
            process::exit(if report.has_failures() { 1 } else { 0 });
        }

        Commands::Explain { code } => {
            match explain::explain_code(&code) {
                Some(entry) => {
                    print!("{}", explain::format_explanation(entry));
                }
                None => {
                    eprintln!("ori: unknown diagnostic code `{code}`");
                    eprintln!("ori: see docs/spec/13-error-catalog.md for the full catalog");
                    process::exit(2);
                }
            }
        }

        Commands::Summary { path } => {
            let target = if path.as_os_str().is_empty() || path.as_path() == Path::new(".") {
                std::env::current_dir().unwrap_or_else(|e| {
                    eprintln!("ori: {}", e);
                    process::exit(2);
                })
            } else {
                path.clone()
            };
            let entry = if target.is_dir() {
                target.join("main.orl")
            } else {
                target
            };
            match pipeline::run_summary(&entry) {
                Err(e) => {
                    eprintln!("ori: {}", e);
                    process::exit(2);
                }
                Ok(summary) => {
                    print!("{}", pipeline::format_summary_text(&summary));
                }
            }
        }

        Commands::Parse { file } => match pipeline::run_parse(file) {
            Err(e) => {
                eprintln!("ori: {}", e);
                process::exit(2);
            }
            Ok(out) => {
                println!("{:#?}", out.ast);
                let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                emit::render_all(&out.cache, &out.diagnostics, color);
                process::exit(if errors > 0 { 1 } else { 0 });
            }
        },
    }
}

fn temp_run_exe_path(file: &std::path::Path) -> PathBuf {
    let stem = file
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("app");
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    let mut name = format!("ori-run-{stem}-{}-{millis}", process::id());
    if cfg!(windows) {
        name.push_str(".exe");
    }
    std::env::temp_dir().join(name)
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn compile_help_declares_native_runtime_contract() {
        let mut command = Cli::command();
        let help = command.render_long_help().to_string();
        let compile_help = command
            .find_subcommand_mut("compile")
            .expect("compile subcommand should exist")
            .render_long_help()
            .to_string();
        let test_help = command
            .find_subcommand_mut("test")
            .expect("test subcommand should exist")
            .render_long_help()
            .to_string();
        let build_help = command
            .find_subcommand_mut("build")
            .expect("build subcommand should exist")
            .render_long_help()
            .to_string();
        let run_help = command
            .find_subcommand_mut("run")
            .expect("run subcommand should exist")
            .render_long_help()
            .to_string();

        assert!(help.contains("packaged native runtime"), "{help}");
        assert!(!help.contains("requires `cc`"), "{help}");
        assert!(!help.contains("C compiler"), "{help}");
        assert!(
            compile_help.contains("packaged native runtime"),
            "{compile_help}"
        );
        assert!(compile_help.contains("--native-raw"), "{compile_help}");
        assert!(
            compile_help.contains("full native linker stdout/stderr"),
            "{compile_help}"
        );
        assert!(test_help.contains("native runtime"), "{test_help}");
        assert!(
            build_help.contains("debug backend with partial feature parity"),
            "{build_help}"
        );
        assert!(
            run_help.contains("Compile and run an Ori source file"),
            "{run_help}"
        );
        assert!(run_help.contains("--native-raw"), "{run_help}");
        assert!(
            help.contains("Extract documentation comments as Markdown"),
            "{help}"
        );
        assert!(help.contains("Run functions marked with `@test`"), "{help}");
        assert!(
            help.contains("Format an Ori source file and print the result"),
            "{help}"
        );
    }
}
