use std::error::Error;

use odict::Entry;
use serde_json::to_string_pretty;

use crate::{enums::PrintFormat, CLIContext};

fn print_as_json(ctx: &mut CLIContext, entries: &Vec<Vec<Entry>>) -> Result<(), Box<dyn Error>> {
    ctx.println(to_string_pretty(entries)?);
    Ok(())
}

pub fn print_entries(
    ctx: &mut CLIContext,
    entries: &Vec<Vec<Entry>>,
    format: &PrintFormat,
) -> Result<(), Box<dyn Error>> {
    match format {
        PrintFormat::Print => {}
        PrintFormat::JSON => {
            print_as_json(ctx, entries)?;
        }
        PrintFormat::XML => todo!(),
    }
    Ok(())
}
