use clap::Parser;
use cli::{compile, lookup, merge, t, CLIContext, Commands, CLI};

fn main() {
    let cli = CLI::parse();
    let mut ctx = CLIContext::default(&cli);

    let result = t(
        |c| match cli.command {
            Commands::Compile(ref args) => compile(c, &args),
            Commands::Lookup(ref args) => lookup(c, &args),
            Commands::Merge(ref args) => merge(c, &args),
        },
        &mut ctx,
    );

    if let Err(e) = result {
        ctx.stderr
            .write_all(format!("Error: {}", e).as_bytes())
            .unwrap();
    }
}
