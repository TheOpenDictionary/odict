use std::error::Error;

use odict::{ArchivedEntry, Entry};

use crate::CLIContext;

pub fn t<F>(cb: F, ctx: &mut CLIContext) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut CLIContext) -> Result<(), Box<dyn Error>>,
{
    let start = std::time::Instant::now();
    let err = cb(ctx);

    if let Err(msg) = err {
        ctx.println(format!("{}", msg));
    } else if !ctx.cli.quiet {
        ctx.println(format!("\n✨ Completed in {:?}", start.elapsed()));
    }

    Ok(())
}

pub fn deserialize_nested_entries(entries: Vec<Vec<&ArchivedEntry>>) -> Vec<Vec<Entry>> {
    entries
        .iter()
        .map(|entries_inner| -> Vec<Entry> {
            entries_inner
                .iter()
                .map(|entry| entry.to_entry().unwrap())
                .collect()
        })
        .collect()
}
