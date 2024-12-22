use odict::{ArchivedEntry, Entry};

use crate::CLIContext;

pub fn t<F>(cb: F, ctx: &mut CLIContext) -> anyhow::Result<()>
where
    F: FnOnce(&mut CLIContext) -> anyhow::Result<()>,
{
    let err = cb(ctx);

    if let Err(msg) = err {
        ctx.println(format!("{}", msg));
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
