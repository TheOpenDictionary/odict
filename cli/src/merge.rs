use clap::{arg, command, Args};
use odict::{download::DictionaryDownloader, LoadOptions, OpenDictionary};

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

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn merge<'a>(args: &MergeArgs) -> anyhow::Result<()> {
    let downloader = DictionaryDownloader::default().with_retries(args.retries);
    let load_opts = LoadOptions::default().with_downloader(downloader);
    let mut dict = OpenDictionary::load_with_options(&args.destination, load_opts.clone())
        .await?
        .contents()?
        .deserialize()?;

    for source in &args.sources {
        let source_dict = OpenDictionary::load_with_options(source, load_opts.clone())
            .await?
            .contents()?
            .deserialize()?;

        dict.merge(&source_dict);
    }

    dict.build()?
        .to_disk(args.output.as_ref().unwrap_or(&args.destination))?;

    Ok(())
}
