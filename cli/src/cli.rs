use clap::{command, crate_version, Parser, Subcommand};

use crate::alias::AliasCommands;
use crate::{CompileArgs, LexiconArgs, LookupArgs, MergeArgs};

#[derive(Debug, Parser)]
#[command(name = "odict", about = "the lighting-fast open-source dictionary compiler", version = crate_version!(), long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Silence any non-important output"
    )]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage dictionary aliases
    #[command(subcommand, arg_required_else_help = true)]
    Alias(AliasCommands),

    /// Compiles a dictionary from ODXML
    #[command(arg_required_else_help = true)]
    Compile(CompileArgs),

    /// Looks up an entry in a compiled dictionary without indexing
    #[command(arg_required_else_help = true)]
    Lookup(LookupArgs),

    /// Merge entries from multiple dictionaries into a destination dictionary
    #[command(arg_required_else_help = true)]
    Merge(MergeArgs),

    /// Lists all words defined in a dictionary
    #[command(arg_required_else_help = true)]
    Lexicon(LexiconArgs),
}
