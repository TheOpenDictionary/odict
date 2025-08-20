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
        default_value_t = 0,
        help = "Number of redirects to follow via \"see also\" attributes. Use a high number like 999999 for infinite following (old behavior)."
    )]
    follow: u32,

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

    let file = ctx
        .loader
        .load(&path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?;

    let mut opts: LookupOptions = LookupOptions::default()
        .follow(*follow)
        .insensitive(*insensitive);

    if *split > 0 {
        opts = opts.strategy(LookupStrategy::Split(*split));
    }

    let result = file.to_archive()?.lookup(queries, opts);

    match result {
        Ok(entries) => {
            print_entries(ctx, get_lookup_entries(entries), format)?;
            Ok(())
        }
        Err(err) => {
            return Err(anyhow::Error::from(err));
        }
    }
}
