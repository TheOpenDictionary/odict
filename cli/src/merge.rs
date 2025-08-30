use clap::{arg, command, Args};

use crate::load_dictionary;

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

pub async fn merge<'a>(args: &MergeArgs) -> anyhow::Result<()> {
    let mut dict = internal::load_dictionary(&args.destination)
        .await?
        .contents()?
        .deserialize()?;

    for source in &args.sources {
        let source_dict = internal::load_dictionary(source).await?.contents()?.deserialize()?;

        dict.merge(&source_dict);
    }

    dict.build()?
        .to_disk(args.output.as_ref().unwrap_or(&args.destination))?;

    Ok(())
}
