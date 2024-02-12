use std::error::Error;

use crate::enums::PrintFormat;
use crate::{context::CLIContext, print_entries};
use crate::{deserialize_nested_entries, MarkdownStrategy};
use clap::{arg, command, Args};
use odict::LookupOptions;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LookupArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,

    #[arg(required = true, help = "Words to look up")]
    queries: Vec<String>,

    // Strategy for rendering Markdown strings
    #[arg(default_value_t = MarkdownStrategy::HTML, help = "Words to look up")]
    markdown: MarkdownStrategy,

    #[arg(
      short,
      long,
      value_enum,
      default_value_t = PrintFormat::Print,
      help = "Output format of the entries"
  )]
    format: PrintFormat,

    #[arg(
        short = 'F',
        long,
        default_value_t = false,
        help = "Follows all \"see also\" attributes (\"see\") until it finds a root term."
    )]
    follow: bool,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "If a definition cannot be found, attempt to split the query into words of at least length S and look up each word separately. Can be relatively slow."
    )]
    split: usize,
}

pub fn lookup(ctx: &mut CLIContext, args: &LookupArgs) -> Result<(), Box<dyn Error>> {
    let LookupArgs {
        dictionary_path: path,
        queries,
        format,
        follow,
        split,
        markdown,
    } = args;

    let file = ctx.reader.read_from_path(&path)?;

    let result = file.to_archive()?.lookup(
        queries,
        &LookupOptions::default().follow(*follow).split(*split),
    );

    match result {
        Ok(entries) => {
            print_entries(ctx, &deserialize_nested_entries(entries), format)?;
            Ok(())
        }
        Err(err) => {
            return Err(err);
        }
    }
}
