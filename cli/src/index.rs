use std::path::PathBuf;

use clap::{arg, command, Args};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use odict::search::{get_default_index_dir, IndexOptions};

use crate::CLIContext;

pub(super) static DEFAULT_INDEX_MEMORY: usize = 15000000;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct IndexArgs {
    /// Path to a compiled dictionary or an alias
    #[arg(required = true)]
    pub(super) dictionary: String,

    /// Custom directory to store the index
    #[arg(short)]
    pub(super) directory: Option<PathBuf>,

    /// Whether to overwrite the index if it already exists
    #[arg(short = 'f', default_value_t = false)]
    pub(super) overwrite: bool,

    /// Memory arena per thread in bytes. Must be above 15MB.
    #[arg(short, default_value_t = DEFAULT_INDEX_MEMORY)]
    pub(super) memory: usize,
}

pub async fn index<'a>(ctx: &mut CLIContext<'a>, args: &IndexArgs) -> anyhow::Result<()> {
    let file = ctx
        .loader
        .load(&args.dictionary)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?;

    ctx.println("".to_string());

    let dict = file.to_dictionary()?;

    let progress1 = ProgressBar::new(dict.entries.len() as u64).with_style(
        ProgressStyle::with_template("[{eta_precise}] {bar} {pos}/{len} entries indexed").unwrap(),
    );

    progress1.set_draw_target(ProgressDrawTarget::term(ctx.stdout.clone(), 20));

    let progress2 = progress1.clone();

    let options = IndexOptions::default()
        .overwrite(args.overwrite)
        .memory(args.memory)
        .dir(args.directory.as_ref().unwrap_or(&get_default_index_dir()))
        .on_item(move |_, _| {
            progress2.inc(1);
        });

    dict.index(&options)?;

    progress1.finish();

    ctx.println(format!(
        "\n\nSuccessfully wrote index to {}",
        options.dir.as_path().to_str().unwrap()
    ));

    Ok(())
}
