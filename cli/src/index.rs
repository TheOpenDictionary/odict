use std::{error::Error, path::PathBuf};

use clap::{arg, command, Args};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use odict::search::{get_default_index_dir, IndexOptions};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct IndexArgs {
    /// Path to a compiled dictionary
    #[arg(required = true)]
    dictionary: String,

    /// Custom directory to store the index
    #[arg(short)]
    directory: Option<PathBuf>,

    /// Whether to overwrite the index if it already exists
    #[arg(short = 'f', default_value_t = false)]
    overwrite: bool,

    /// Memory arena per thread in bytes. Must be above 15MB.
    #[arg(short, default_value_t = 15000000)]
    memory: usize,
}

pub fn index(ctx: &mut CLIContext, args: &IndexArgs) -> Result<(), Box<dyn Error>> {
    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&args.dictionary, &ctx.alias_manager)?;

    ctx.println("".to_string());

    let archive = file.to_archive()?;

    let progress1 = ProgressBar::new(archive.entries.len() as u64).with_style(
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

    archive.index(&options)?;

    progress1.finish();

    ctx.println(format!(
        "\n\nSuccessfully wrote index to {}",
        options.dir.as_path().to_str().unwrap()
    ));

    Ok(())
}
