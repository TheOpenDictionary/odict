use std::error::Error;

use crate::CLIContext;

pub fn t<F>(cb: F, ctx: &mut CLIContext) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut CLIContext) -> Result<(), Box<dyn Error>>,
{
    let start = std::time::Instant::now();
    let err = cb(ctx);

    if let Err(msg) = err {
        ctx.print(format!("{}", msg));
    } else if !ctx.cli.quiet {
        ctx.print(format!("✨ Completed in {:?}", start.elapsed()));
    }

    Ok(())
}
