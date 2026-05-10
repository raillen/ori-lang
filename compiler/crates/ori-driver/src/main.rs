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
        /// Path to the `.ori` source file.
        file: PathBuf,
    },
    /// Print the raw token stream (debug).
    Lex {
        file: PathBuf,
    },
    /// Print the AST (debug).
    Parse {
        file: PathBuf,
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
