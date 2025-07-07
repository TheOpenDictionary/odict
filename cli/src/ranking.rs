use crate::context::CLIContext;
use clap::{arg, command, Args};
use console::Style;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct RankingArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,
}

pub fn ranking(ctx: &mut CLIContext, args: &RankingArgs) -> anyhow::Result<()> {
    let RankingArgs {
        dictionary_path: path,
    } = args;

    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&path, &ctx.alias_manager)?;

    let bold = Style::new().bold();
    let muted = Style::new().dim();
    let danger = Style::new().red().bold();
    let warn = Style::new().yellow().bold();
    let dict = file.to_archive()?;

    let min_rank = dict.min_rank();
    let max_rank = dict.max_rank();

    if let Option::Some(name) = &dict.name.as_ref() {
        ctx.println(format!(
            "\n{}\n{}\n",
            bold.apply_to(format!("Rankings for {}", name)),
            "─".repeat(format!("Rankings for {}", name).len())
        ));
    } else {
        ctx.println(format!(
            "\n{}\n{}\n",
            bold.apply_to("Rankings"),
            "─".repeat("Rankings".len())
        ));
    }

    match (min_rank, max_rank) {
        (Some(min), Some(max)) => {
            ctx.println(format!("{} {}", bold.apply_to("Minimum Rank:"), min));
            ctx.println(format!("{} {}", bold.apply_to("Maximum Rank:"), max));

            if min == max {
                ctx.println(format!(
                    "\n{}",
                    warn.apply_to("All ranked entries have the same rank value.")
                ));
            } else {
                ctx.println(format!(
                    "\n{}",
                    muted.apply_to(format!("Rank range spans {} values.", max - min + 1))
                ));
            }
        }

        (None, None) => {
            ctx.println(format!(
                "{}",
                warn.apply_to("No entries in this dictionary have rank values.")
            ));
        }

        _ => {
            // This shouldn't happen in practice since min_rank and max_rank
            // should both be Some or both be None
            ctx.println(format!(
                "{}",
                danger.apply_to("Inconsistent rank data detected.")
            ));
        }
    }

    ctx.println("");

    Ok(())
}
