use clap::{command, crate_version, Parser, Subcommand};

mod compile;
mod context;
mod enums;
mod lookup;
mod utils;

use compile::*;
use context::*;
use lookup::*;
use utils::*;

#[derive(Debug, Parser)]
#[command(name = "odict", about = "the lighting-fast open-source dictionary compiler", version = crate_version!(), long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Silence any non-important output"
    )]
    quiet: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compiles a dictionary from ODXML
    #[command(arg_required_else_help = true)]
    Compile(CompileArgs),

    /// Looks up an entry in a compiled dictionary without indexing
    #[command(arg_required_else_help = true)]
    Lookup(LookupArgs),
}

fn main() {
    let cli = CLI::parse();
    let ctx = CLIContext::default();

    let result = t(
        || match cli.command {
            Commands::Compile(ref args) => compile(&ctx, &args),
            Commands::Lookup(ref args) => lookup(&ctx, &args),
        },
        cli.quiet,
    );

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
