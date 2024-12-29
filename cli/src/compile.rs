use std::path::PathBuf;

use clap::{arg, command, Args};
use odict::{fs::infer_path, write::DictionaryWriterOptions, CompressOptions};

use crate::CLIContext;

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

pub fn compile(ctx: &CLIContext, args: &CompileArgs) -> color_eyre::Result<()> {
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

    let _ = ctx.writer.compile_xml_with_opts(
        &input,
        &out,
        DictionaryWriterOptions::default().compression(compress_opts),
    )?;

    Ok(())
}
