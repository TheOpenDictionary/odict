use crate::context::CLIContext;
use clap::{arg, command, Args};
use console::Style;
use indicatif::DecimalBytes;
use num_format::{Locale, ToFormattedString};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct InfoArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,
}

pub async fn info<'a>(ctx: &mut CLIContext<'a>, args: &InfoArgs) -> anyhow::Result<()> {
    let InfoArgs {
        dictionary_path: path,
    } = args;

    let file = internal::load_dictionary(path).await?;

    let bold = Style::new().bold();
    let dict = file.contents()?;

    let min_rank = dict.min_rank();
    let max_rank = dict.max_rank();

    if let Option::Some(name) = &dict.name.as_ref() {
        ctx.println(format!(
            "\n{}\n{}\n",
            bold.apply_to(name),
            "─".repeat(name.len())
        ));
    }

    ctx.println(format!(
        "{} {}",
        bold.apply_to("File Version:"),
        file.version()
    ));

    ctx.println(format!(
        "{} {}",
        bold.apply_to("File Size:"),
        DecimalBytes(file.size()?)
    ));

    ctx.println(format!(
        "{} {}",
        bold.apply_to("Entries:"),
        dict.entries.len().to_formatted_string(&Locale::en)
    ));

    if let (Some(min), Some(max)) = (min_rank, max_rank) {
        ctx.println(format!(
            "{} {}",
            bold.apply_to("Min Word Rank:"),
            min.to_formatted_string(&Locale::en)
        ));
        ctx.println(format!(
            "{} {}",
            bold.apply_to("Max Word Rank:"),
            max.to_formatted_string(&Locale::en)
        ));
    }

    ctx.println("");

    Ok(())
}
