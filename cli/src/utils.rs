use odict::{lookup::LookupResult, ArchivedEntry, Entry};

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

pub fn get_lookup_entries(results: Vec<LookupResult<&ArchivedEntry>>) -> Vec<Entry> {
    results
        .iter()
        .map(|result| result.entry.to_entry().unwrap())
        .collect()
}
