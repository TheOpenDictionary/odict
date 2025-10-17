use std::time::Duration;

use crate::enums::PrintFormat;
use crate::get_lookup_entries;
use crate::{context::CLIContext, print_entries};
use clap::{arg, command, Args};
use odict::lookup::{LookupOptions, LookupStrategy};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LookupArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,

    #[arg(required = true, help = "Words to look up")]
    queries: Vec<String>,

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
        help = "Follow see_also redirects until finding an entry with etymologies"
    )]
    follow: bool,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "If a definition cannot be found, attempt to split the query into words of at least length S and look up each word separately. Can be relatively slow."
    )]
    split: usize,

    #[arg(
        short = 'i',
        long,
        default_value_t = false,
        help = "Perform case-insensitive lookups"
    )]
    insensitive: bool,
}

pub async fn lookup<'a>(ctx: &mut CLIContext<'a>, args: &LookupArgs) -> anyhow::Result<()> {
    let LookupArgs {
        dictionary_path: path,
        queries,
        format,
        follow,
        split,
        insensitive,
    } = args;

    let spinner = indicatif::ProgressBar::new_spinner();

    spinner.enable_steady_tick(Duration::from_millis(100));

    let file = internal::load_dictionary(path).await?;

    let mut opts: LookupOptions = LookupOptions::default()
        .follow(*follow)
        .insensitive(*insensitive);

    if *split > 0 {
        opts = opts.strategy(LookupStrategy::Split(*split));
    }

    let result = file.contents()?.lookup(queries, opts);

    spinner.finish_and_clear();

    match result {
        Ok(entries) => {
            print_entries(ctx, get_lookup_entries(entries), format)?;
            Ok(())
        }
        Err(err) => Err(anyhow::Error::from(err)),
    }
}
