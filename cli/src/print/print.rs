use odict::{
    format::{html::ToHTML, json::ToJSON, md::ToMarkdown, xml::ToXML},
    Entry, Error,
};

use super::pprint::pretty_print;
use crate::{enums::PrintFormat, CLIContext};

fn print_json(ctx: &mut CLIContext, entries: Vec<Entry>) -> Result<(), Error> {
    ctx.println(entries.to_json(true)?);
    Ok(())
}

fn print_xml(ctx: &mut CLIContext, entries: Vec<Entry>) -> Result<(), Error> {
    let xml: Vec<String> = entries.iter().map(|v| v.to_xml(true).unwrap()).collect();

    ctx.println(xml.join("\n"));

    Ok(())
}

fn print_markdown(ctx: &mut CLIContext, entries: Vec<Entry>) -> Result<(), Error> {
    let md: Vec<String> = entries.iter().map(|v| v.to_markdown().unwrap()).collect();

    ctx.println(md.join("\n\n---\n\n"));

    Ok(())
}

fn print_html(ctx: &mut CLIContext, entries: Vec<Entry>) -> Result<(), Error> {
    let html: Vec<String> = entries.iter().map(|v| v.to_html().unwrap()).collect();

    ctx.println(html.join("\n\n"));

    Ok(())
}

pub fn print_entries(
    ctx: &mut CLIContext,
    entries: Vec<Entry>,
    format: &PrintFormat,
) -> Result<(), Error> {
    match format {
        PrintFormat::Print => pretty_print(ctx, entries)?,
        PrintFormat::JSON => print_json(ctx, entries)?,
        PrintFormat::XML => print_xml(ctx, entries)?,
        PrintFormat::Markdown => print_markdown(ctx, entries)?,
        PrintFormat::HTML => print_html(ctx, entries)?,
    }
    Ok(())
}
