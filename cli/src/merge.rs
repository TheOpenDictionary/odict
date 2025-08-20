use clap::{arg, command, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct MergeArgs {
    #[arg(
        required = true,
        help = "Path of the dictionary to merge into (unless --output is specified)"
    )]
    destination: String,

    #[arg(required = true, help = "Paths of dictionaries to merge")]
    sources: Vec<String>,

    #[arg(short, long, help = "Separate output path for the compiled dictionary")]
    output: Option<String>,
}

pub async fn merge<'a>(ctx: &CLIContext<'a>, args: &MergeArgs) -> anyhow::Result<()> {
    let mut dict = ctx
        .loader
        .load(&args.destination)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?
        .to_dictionary()?;

    for source in &args.sources {
        let source_dict = ctx
            .loader
            .load(source)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?
            .to_dictionary()?;

        dict.merge(&source_dict);
    }

    if let Some(output) = &args.output {
        ctx.writer.write_to_path(&dict, &output)?;
    } else {
        ctx.writer.write_to_path(&dict, &args.destination)?;
    }

    Ok(())
}
