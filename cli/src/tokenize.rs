use clap::{arg, command, Args};
use odict::{
    download::DictionaryDownloader, tokenize::TokenizeOptions, LoadOptions, OpenDictionary,
};

use crate::{enums::PrintFormat, get_lookup_entries, print_entries, CLIContext};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct TokenizeArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,

    #[arg(required = true, help = "Text to tokenize")]
    text: String,

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
        short = 'i',
        long,
        default_value_t = false,
        help = "Perform case-insensitive lookups when matching tokens"
    )]
    insensitive: bool,

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn tokenize<'a>(ctx: &mut CLIContext<'a>, args: &TokenizeArgs) -> anyhow::Result<()> {
    let TokenizeArgs {
        dictionary_path: path,
        text,
        format,
        follow,
        insensitive,
        retries,
    } = args;

    let file = OpenDictionary::load_with_options(
        path,
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(*retries)),
    )
    .await?;

    let opts = TokenizeOptions::default()
        .follow(*follow)
        .insensitive(*insensitive);

    let dict = file.contents()?;
    let result = dict.tokenize(text, opts);

    match result {
        Ok(tokens) => {
            // Flatten all entries from all tokens
            let all_entries = tokens
                .iter()
                .flat_map(|token| token.entries.clone())
                .collect::<Vec<_>>();

            if all_entries.is_empty() {
                ctx.println(format!("No entries found for the text: \"{text}\""));
            } else {
                print_entries(ctx, get_lookup_entries(all_entries), format)?;
            }
            Ok(())
        }
        Err(err) => Err(anyhow::Error::from(err)),
    }
}
