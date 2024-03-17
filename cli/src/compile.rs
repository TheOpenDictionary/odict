use std::{error::Error, fs::read_to_string, path::PathBuf};

use clap::{arg, command, Args};
use odict::Dictionary;

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

pub fn compile(ctx: &CLIContext, args: &CompileArgs) -> Result<(), Box<dyn Error>> {
    let CompileArgs { input, output } = args;

    let mut out = output.to_owned();

    if out.is_none() {
        let name = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let directory = input.parent().and_then(|s| s.to_str()).unwrap_or_default();

        out = Some(
            PathBuf::new()
                .join(directory)
                .join(format!("{}.odict", name)),
        );
    }

    let contents = read_to_string(input).unwrap();
    let xml = contents.as_str();

    match Dictionary::from(xml) {
        Ok(dict) => {
            ctx.writer.write_to_path(&dict, &out.unwrap())?;
        }
        Err(e) => {
            return Err(format!("\nAn error occurred parsing your XML: \n\n{}", e).into());
        }
    }

    Ok(())
}
