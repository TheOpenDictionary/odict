use clap::{arg, command, Args};
use odict::tokenize::TokenizeOptions;

use crate::{enums::PrintFormat, get_lookup_entries, load_dictionary, print_entries, CLIContext};

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
        default_value_t = 0,
        help = "Number of redirects to follow via \"see also\" attributes. Use a high number like 999999 for infinite following (old behavior)."
    )]
    follow: u32,

    #[arg(
        short = 'i',
        long,
        default_value_t = false,
        help = "Perform case-insensitive lookups when matching tokens"
    )]
    insensitive: bool,
}

pub async fn tokenize<'a>(ctx: &mut CLIContext<'a>, args: &TokenizeArgs) -> anyhow::Result<()> {
    let TokenizeArgs {
        dictionary_path: path,
        text,
        format,
        follow,
        insensitive,
    } = args;

    let file = load_dictionary(path).await?;

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
