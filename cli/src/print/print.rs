use odict::{error::Error, json::ToJSON, xml::ToXML, Entry};

use super::pprint::pretty_print;
use crate::{enums::PrintFormat, CLIContext};

fn print_json(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> Result<(), Error> {
    ctx.println(entries.to_json(true)?);
    Ok(())
}

fn print_xml(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> Result<(), Error> {
    let xml: Vec<String> = entries
        .iter()
        .flatten()
        .map(|v| v.to_xml(true).unwrap())
        .collect();

    ctx.println(xml.join("\n"));

    Ok(())
}

pub fn print_entries(
    ctx: &mut CLIContext,
    entries: Vec<Vec<Entry>>,
    format: &PrintFormat,
) -> Result<(), Error> {
    match format {
        PrintFormat::Print => pretty_print(ctx, entries)?,
        PrintFormat::JSON => print_json(ctx, entries)?,
        PrintFormat::XML => print_xml(ctx, entries)?,
    }
    Ok(())
}
