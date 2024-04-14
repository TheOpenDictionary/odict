use clap::{command, crate_version, Parser, Subcommand};

use crate::alias::AliasCommands;
use crate::{
    CompileArgs, DumpArgs, IndexArgs, LexiconArgs, LookupArgs, MergeArgs, NewArgs, SearchArgs,
    ServeArgs,
};

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

    /// Outputs a dictionary in a human-readable format
    #[command(arg_required_else_help = true)]
    Dump(DumpArgs),

    /// Creates a full-text index of a compiled dictionary
    #[command(arg_required_else_help = true)]
    Index(IndexArgs),

    /// Lists all words defined in a dictionary
    #[command(arg_required_else_help = true)]
    Lexicon(LexiconArgs),

    /// Looks up an entry in a compiled dictionary without indexing
    #[command(arg_required_else_help = true)]
    Lookup(LookupArgs),

    /// Merge entries from multiple dictionaries into a destination dictionary
    #[command(arg_required_else_help = true)]
    Merge(MergeArgs),

    /// Scaffolds a new ODict XML dictionary
    #[command(arg_required_else_help = true)]
    New(NewArgs),

    /// Run a full-text query on a compiled dictionary
    #[command(arg_required_else_help = true)]
    Search(SearchArgs),

    /// Start a local web server to serve one or several dictionaries
    #[command(arg_required_else_help = true)]
    Serve(ServeArgs),
}
