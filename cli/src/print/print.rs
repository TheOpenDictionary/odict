use odict::{
    format::{html::ToHTML, json::ToJSON, md::ToMarkdown, xml::ToXML},
    Entry,
};

use crate::{enums::PrintFormat, print::md::render_md, CLIContext};

fn print_json(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> color_eyre::Result<()> {
    ctx.println(entries.to_json(true)?);
    Ok(())
}

fn print_xml(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> color_eyre::Result<()> {
    let xml: Vec<String> = entries
        .iter()
        .flatten()
        .map(|v| v.to_xml(true).unwrap())
        .collect();

    ctx.println(xml.join("\n"));

    Ok(())
}

fn print_markdown(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> color_eyre::Result<()> {
    let md: Vec<String> = entries
        .iter()
        .flatten()
        .map(|v| v.to_markdown().unwrap())
        .collect();

    termimad::print_text((&md.join("\n\n---\n\n").as_str()));
    // ctx.println(format!("{}", md.join("\n\n---\n\n").as_str()));

    Ok(())
}

fn print_html(ctx: &mut CLIContext, entries: Vec<Vec<Entry>>) -> color_eyre::Result<()> {
    let html: Vec<String> = entries
        .iter()
        .flatten()
        .map(|v| v.to_html().unwrap())
        .collect();

    ctx.println(html.join("\n\n"));

    Ok(())
}

pub fn print_entries(
    ctx: &mut CLIContext,
    entries: Vec<Vec<Entry>>,
    format: &PrintFormat,
) -> color_eyre::Result<()> {
    match format {
        PrintFormat::Print => todo!(),
        PrintFormat::JSON => print_json(ctx, entries)?,
        PrintFormat::XML => print_xml(ctx, entries)?,
        PrintFormat::Markdown => print_markdown(ctx, entries)?,
        PrintFormat::HTML => print_html(ctx, entries)?,
    }
    Ok(())
}
