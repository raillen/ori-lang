use clap::{Parser, Subcommand};
use ori_driver::{emit, pipeline};
use std::path::PathBuf;
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
    /// Extract documentation comments as Markdown.
    Doc {
        /// Path to the `.orl` source file or project manifest.
        file: PathBuf,
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

        Commands::Doc { file } => match pipeline::run_doc(file) {
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
                    print!("{}", out.markdown);
                }
                process::exit(if out.has_errors { 1 } else { 0 });
            }
        },

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
                    let passed = out.results.iter().filter(|result| result.passed).count();
                    let failed = out.results.len() - passed;
                    for result in &out.results {
                        if result.passed {
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
                    eprintln!("tests: {passed} passed, {failed} failed");
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
