use std::{
    error::Error,
    fs::{canonicalize, File},
    io::Write,
    path::PathBuf,
};

use clap::{arg, command, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct NewArgs {
    #[arg(required = true, help = "Name of your new dictionary file")]
    file_name: String,

    #[arg(short, help = "Name attribute of the dictionary element")]
    name: Option<String>,
}

pub fn new(ctx: &mut CLIContext, args: &NewArgs) -> Result<(), Box<dyn Error>> {
    let mut template = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<dictionary",
    );

    if let Some(name) = &args.name {
        template.push_str(&format!(" name=\"{}\"", name));
    }

    template.push_str(
        ">
</dictionary>",
    );

    let output = canonicalize(PathBuf::from(format!("{}.xml", args.file_name)))?;

    if output.exists() {
        return Err("\n🚫️ A file already exists with this name! Please choose another one.".into());
    }

    let mut file = File::create(&output)?;

    file.write_all(template.as_bytes())?;

    ctx.println(format!(
        "\n✨ Created a new dictionary at {}!",
        output.display()
    ));

    Ok(())
}
