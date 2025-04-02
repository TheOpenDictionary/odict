use clap::{Args, arg, command};
use odict::lookup::TokenizeOptions;

use crate::{CLIContext, enums::PrintFormat, get_lookup_entries, print_entries};

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
        default_value_t = false,
        help = "Follows all \"see also\" attributes (\"see\") until it finds a root term."
    )]
    follow: bool,
}

pub fn tokenize(ctx: &mut CLIContext, args: &TokenizeArgs) -> anyhow::Result<()> {
    let TokenizeArgs {
        dictionary_path: path,
        text,
        format,
        follow,
    } = args;

    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&path, &ctx.alias_manager)?;

    let opts = TokenizeOptions::default().follow(*follow);
    let archive = file.to_archive()?;

    let result = archive.tokenize(text, opts);

    match result {
        Ok(tokens) => {
            // Flatten all entries from all tokens
            let all_entries = tokens
                .iter()
                .flat_map(|token| token.entries.clone())
                .collect::<Vec<_>>();

            if all_entries.is_empty() {
                ctx.println(format!("No entries found for the text: \"{}\"", text));
            } else {
                print_entries(ctx, get_lookup_entries(all_entries), format)?;
            }
            Ok(())
        }
        Err(err) => Err(anyhow::Error::from(err)),
    }
}
