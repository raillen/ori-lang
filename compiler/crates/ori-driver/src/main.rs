use std::path::PathBuf;
use std::process;
use clap::{Parser, Subcommand};
use ori_driver::{emit, pipeline};

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
    /// Compile to a native binary via Cranelift (no C compiler needed).
    Compile {
        /// Path to the `.orl` source file.
        file: PathBuf,
        /// Output executable path (default: same name as source, no extension).
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Compile to C source (debug backend).
    Build {
        /// Path to the `.orl` source file.
        file: PathBuf,
        /// Write generated C to this file instead of stdout.
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() {
    let cli  = Cli::parse();
    let color = !cli.no_color && std::env::var("NO_COLOR").is_err();

    match &cli.command {
        Commands::Check { file } => {
            match pipeline::run_check(file) {
                Err(e) => {
                    eprintln!("ori: {}", e);
                    process::exit(2);
                }
                Ok(out) => {
                    let errors   = out.diagnostics.iter().filter(|d| d.is_error()).count();
                    let warnings = out.diagnostics.len() - errors;
                    emit::render_all(&out.cache, &out.diagnostics, color);
                    emit::print_summary(errors, warnings, color);
                    process::exit(if out.has_errors { 1 } else { 0 });
                }
            }
        }

        Commands::Lex { file } => {
            match pipeline::run_lex(file) {
                Err(e) => { eprintln!("ori: {}", e); process::exit(2); }
                Ok(out) => {
                    for tok in &out.tokens {
                        println!("{:?}  {:?}", tok.kind, tok.span);
                    }
                    let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                    emit::render_all(&out.cache, &out.diagnostics, color);
                    process::exit(if errors > 0 { 1 } else { 0 });
                }
            }
        }

        Commands::Compile { file, out } => {
            let default_out = file.with_extension(if cfg!(windows) { "exe" } else { "" });
            let exe = out.as_deref().unwrap_or(&default_out);
            match pipeline::run_compile(file, exe) {
                Err(e) => { eprintln!("ori: {}", e); process::exit(2); }
                Ok(out) => {
                    let errors   = out.diagnostics.iter().filter(|d| d.is_error()).count();
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

        Commands::Build { file, out } => {
            match pipeline::run_build(file) {
                Err(e) => { eprintln!("ori: {}", e); process::exit(2); }
                Ok(build) => {
                    let errors   = build.diagnostics.iter().filter(|d| d.is_error()).count();
                    let warnings = build.diagnostics.len() - errors;
                    emit::render_all(&build.cache, &build.diagnostics, color);
                    emit::print_summary(errors, warnings, color);
                    if !build.has_errors {
                        match out {
                            Some(p) => {
                                std::fs::write(p, &build.c_source)
                                    .unwrap_or_else(|e| { eprintln!("ori: {}", e); process::exit(2); });
                            }
                            None => print!("{}", build.c_source),
                        }
                    }
                    process::exit(if build.has_errors { 1 } else { 0 });
                }
            }
        }

        Commands::Parse { file } => {
            match pipeline::run_parse(file) {
                Err(e) => { eprintln!("ori: {}", e); process::exit(2); }
                Ok(out) => {
                    println!("{:#?}", out.ast);
                    let errors = out.diagnostics.iter().filter(|d| d.is_error()).count();
                    emit::render_all(&out.cache, &out.diagnostics, color);
                    process::exit(if errors > 0 { 1 } else { 0 });
                }
            }
        }
    }
}
