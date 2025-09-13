use clap::Parser;
use console::style;
use odict_cli::{
    alias, compile, download, dump, index, info, lexicon, lookup, merge, new, search, serve,
    tokenize, CLIContext, Commands, CLI,
};

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let mut ctx = CLIContext::default(&cli);

    let result = match cli.command {
        Commands::Alias(ref args) => alias(args).await,
        Commands::Compile(ref args) => compile(args),
        Commands::Download(ref args) => download(&mut ctx, args).await,
        Commands::Dump(ref args) => dump(&mut ctx, args).await,
        Commands::Index(ref args) => index(&mut ctx, args).await,
        Commands::Lexicon(ref args) => lexicon(&mut ctx, args).await,
        Commands::Lookup(ref args) => lookup(&mut ctx, args).await,
        Commands::Merge(ref args) => merge(args).await,
        Commands::New(ref args) => new(&mut ctx, args),
        Commands::Search(ref args) => search(&mut ctx, args).await,
        Commands::Serve(ref args) => serve(&mut ctx, args).await,
        Commands::Info(ref args) => info(&mut ctx, args).await,
        Commands::Tokenize(ref args) => tokenize(&mut ctx, args).await,
    };

    if let Err(e) = result {
        ctx.stderr
            .write_line(format!("{}", style(format!("Error: {e}")).red()).as_str())
            .unwrap();
    }
}
