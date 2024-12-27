use std::path::PathBuf;

use clap::{arg, command, Args};
use odict::fs::infer_path;

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CompileArgs {
    #[arg(required = true, help = "Path to ODXML file")]
    input: PathBuf,

    #[arg(short, help = "Output path of compiled dictionary")]
    output: Option<PathBuf>,
}

pub fn compile(ctx: &CLIContext, args: &CompileArgs) -> color_eyre::Result<()> {
    let CompileArgs { input, output } = args;
    let out = output.to_owned().unwrap_or_else(|| infer_path(&input));
    let _ = ctx.writer.compile_xml(&input, &out).map(|_| ())?;

    Ok(())
}
