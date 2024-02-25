use std::error::Error;

use odict::{
    dump::{ToJSON, ToXML},
    Entry,
};

use crate::{enums::PrintFormat, CLIContext};

fn print_json(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> Result<(), Box<dyn Error>> {
    ctx.println(entries.to_json(false)?);
    Ok(())
}

fn print_xml(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> Result<(), Box<dyn Error>> {
    let xml: Vec<String> = entries
        .iter()
        .flatten()
        .map(|v| v.to_xml().unwrap())
        .collect();

    ctx.println(xml.join("\n"));

    Ok(())
}

pub fn print_entries(
    ctx: &mut CLIContext,
    entries: Vec<Vec<Entry>>,
    format: &PrintFormat,
) -> Result<(), Box<dyn Error>> {
    match format {
        PrintFormat::Print => {}
        PrintFormat::JSON => print_json(ctx, entries)?,
        PrintFormat::XML => print_xml(ctx, entries)?,
    }
    Ok(())
}
