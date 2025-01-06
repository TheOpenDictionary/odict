use color_eyre::eyre::Result;

use crate::{browser::launch_browser, CLIContext};

pub fn browse(ctx: &CLIContext, path_or_alias: &str) -> Result<()> {
    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&path_or_alias, &ctx.alias_manager)?;

    launch_browser(&file)
}
