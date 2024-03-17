use std::io::Write;

use clap::Parser;
use cli::{alias, compile, lexicon, lookup, merge, search, t, CLIContext, Commands, CLI};

fn main() {
    let cli = CLI::parse();
    let mut ctx = CLIContext::default(&cli);

    let result = t(
        |c| match cli.command {
            Commands::Alias(ref args) => alias(c, args),
            Commands::Compile(ref args) => compile(c, args),
            Commands::Dump(ref args) => cli::dump(c, args),
            Commands::Index(ref args) => cli::index(c, args),
            Commands::Lexicon(ref args) => lexicon(c, args),
            Commands::Lookup(ref args) => lookup(c, args),
            Commands::Merge(ref args) => merge(c, args),
            Commands::Search(ref args) => search(c, args),
        },
        &mut ctx,
    );

    if let Err(e) = result {
        ctx.stderr
            .write_all(format!("Error: {}", e).as_bytes())
            .unwrap();
    }
}
