use std::{path::PathBuf, time::Duration};

use anyhow::Context;
use clap::{arg, command, Args};
use indicatif::{HumanDuration, ProgressBar};
use odict::{fs::infer_path, CompressOptions};

use crate::{t, CLIContext};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CompileArgs {
    #[arg(required = true, help = "Path to ODXML file")]
    input: PathBuf,

    #[arg(short, help = "Output path of compiled dictionary")]
    output: Option<PathBuf>,

    #[arg(short, value_parser = clap::value_parser!(u32).range(0..=11), help = "Brotli compression level (between 0 and 11)", default_value_t = 11)]
    quality: u32,

    #[arg(short, value_parser = clap::value_parser!(u32).range(0..=22), help = "Brotli large window size (between 0 and 22)", default_value_t = 22)]
    window_size: u32,
}

pub fn compile(ctx: &CLIContext, args: &CompileArgs) -> anyhow::Result<()> {
    let CompileArgs {
        input,
        output,
        quality,
        window_size,
    } = args;

    let out = output.to_owned().unwrap_or_else(|| infer_path(&input));

    let compress_opts = CompressOptions::default()
        .quality(*quality)
        .window_size(*window_size);

    let spinner = ProgressBar::new_spinner();

    spinner.set_message(format!(
        "Compiling {}...",
        input.file_stem().unwrap().to_str().unwrap()
    ));

    spinner.enable_steady_tick(Duration::from_millis(100));

    let duration = t(|| {
        ctx.writer
            .compile_xml_with_opts(
                &input,
                &out,
                DictionaryWriterOptions::default().compression(compress_opts),
            )
            .map(|_| ())
            .with_context(|| "An error occurred compiling your XML")
    })?;

    spinner.finish_with_message(format!("Compiled in {}!", HumanDuration(duration)));

    Ok(())
}
