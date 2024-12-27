use crate::context::CLIContext;
use clap::{arg, command, Args};
use console::Style;
use humansize::{format_size, DECIMAL};
use num_format::{Locale, ToFormattedString};
use odict::ArchivedOption;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct InfoArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,
}

pub fn info(ctx: &mut CLIContext, args: &InfoArgs) -> color_eyre::Result<()> {
    let InfoArgs {
        dictionary_path: path,
    } = args;

    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&path, &ctx.alias_manager)?;

    let bold = Style::new().bold();
    let dict = file.to_archive()?;

    if let ArchivedOption::Some(name) = &dict.name {
        ctx.println(format!(
            "\n{}\n{}\n",
            bold.apply_to(name),
            "─".repeat(name.len())
        ));
    }

    ctx.println(format!(
        "{} {}",
        bold.apply_to("File Version:"),
        file.version
    ));

    ctx.println(format!(
        "{} {}",
        bold.apply_to("File Size:"),
        format_size(file.total_size, DECIMAL)
    ));

    ctx.println(format!(
        "{} {}",
        bold.apply_to("Entries:"),
        dict.entries.len().to_formatted_string(&Locale::en)
    ));

    ctx.println("");

    Ok(())
}
