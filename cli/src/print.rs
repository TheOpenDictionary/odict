use std::error::Error;

use odict::{DefinitionType, Entry, Etymology, Group, MarkdownStrategy, Note, Sense};
use once_cell::sync::Lazy;
use serde_json::to_string_pretty;

use crate::{enums::PrintFormat, CLIContext};
use console::Style;
use regex::Regex;
use std::io::Write;
use std::vec::Vec;

const INDENT_SIZE: usize = 4;

static PARENTHETICAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\(.*?\)\s*)").unwrap());

struct PrintContext<'a> {
    ctx: &'a mut CLIContext<'a>,
    markdown: &'a MarkdownStrategy,
    entry: &'a Entry,
}

fn print_new_line() {
    println!();
}

fn print_example(p: &mut PrintContext, example: &str, target_word: &str, indent: usize) {
    let start = example.to_lowercase().find(&target_word.to_lowercase());
    let caret = Style::new().dim().apply_to("▸");

    if let Some(start) = start {
        let end = start + target_word.len();
        let before = example[..start].to_string();
        let after = example[end..].to_string();
        let target = Style::new()
            .dim()
            .underlined()
            .italic()
            .apply_to(&example[start..end]);

        p.ctx.println(format!(
            "{:indent$}{caret} {before}{target}{after}",
            "",
            indent = indent,
            caret = caret,
            before = before,
            target = target,
            after = after
        ));
    } else {
        p.ctx.println(format!(
            "{:indent$}{caret} {example}",
            "",
            indent = indent,
            caret = caret,
            example = example
        ));
    }
}

fn print_divider(p: &mut PrintContext) {
    p.ctx.println(format!("{}", "─".repeat(32)));
}

fn print_note(
    p: &mut PrintContext,
    note: &Note,
    target_word: &str,
    numbering: &str,
    indent: usize,
) {
    let fmt_numbering = format!("%{}s.", indent);

    p.ctx.println(format!(
        "{} {} {}",
        fmt_numbering,
        numbering,
        note.value.parse(*p.markdown)
    ));

    for example in &note.examples {
        print_example(p, &example.value, target_word, indent + 2);
    }

    println!();
}

// fn print_definition(p: &PrintContext, definition: &Definition, numbering: &str, indent: usize) {
//     let value = &definition.value;

//     let matches: Vec<_> = PARENTHETICAL_REGEX
//         .find_iter(value.parse(*p.markdown).as_str())
//         .collect();

//     let fmt_numbering = format!("%{}s.", indent);

//     if !matches.is_empty() {
//         let mut j = 0;

//         p.ctx.print(format!(
//             "{} {} {}",
//             fmt_numbering,
//             numbering,
//             &value[..matches[0].start()]
//         ));

//         for i in 0..matches.len() {
//             let start = matches[i].start();
//             let end = matches[i].end();

//             print!(
//                 "{}{}",
//                 &value[j..start],
//                 Style::new().italic().apply_to(&value[start..end])
//             );

//             j = end;
//         }

//         println!("{}", &value[j..]);
//     } else {
//         println!(fmt_numbering + " {}", numbering, value);
//     }

//     for example in &definition.examples {
//         print_example(example, &entry.term, indent + 2);
//     }

//     if !definition.notes.is_empty() {
//         println!(
//             "\n{}{}\n",
//             pad_left("", ' ', indent + 2),
//             Style::new().underline().bold().apply_to("Notes")
//         );
//     }

//     for (j, note) in definition.notes.iter().enumerate() {
//         print_note(
//             note,
//             &entry.term,
//             &(char::from(b'a' + j as u8).to_string()),
//             indent + 4,
//         );
//     }
// }

fn print_group(p: &PrintContext, group: &Group, i: usize) {
    let Group {
        description,
        definitions,
        ..
    } = group;

    println!("{:>4}. {}", i + 1, description.parse(*p.markdown));

    for (j, definition) in definitions.iter().enumerate() {
        // print_definition(p, definition, &(char::from(b'a' + j as u8).to_string()), 7);
    }
}

fn print_sense(p: &PrintContext, sense: &Sense) {
    let Sense { pos, definitions } = sense;

    println!(
        "\n{}",
        Style::new().italic().apply_to(&sense.pos.to_string())
    );

    for (i, definition) in definitions.iter().enumerate() {
        match definition {
            DefinitionType::Definition(definition) => {
                // print_definition(p, definition, &(i + 1).to_string(), 4);
            }
            DefinitionType::Group(group) => {
                print_group(p, group, i);
            }
        }
    }
}

fn print_ety(p: &PrintContext, ety: &Etymology, i: usize, show_title: bool) {
    let PrintContext { ctx, markdown, .. } = *p;

    if show_title {
        let header = Style::new()
            .underlined()
            .bold()
            .apply_to(format!("Etymology #{}", i + 1));

        ctx.println(format!("\n{}", header));
    }

    if let Some(description) = &ety.description {
        ctx.println(format!("{}", description.parse(*markdown)));
    }

    for sense in ety.senses.values() {
        print_sense(p, sense);
    }

    println!();
}

fn print_entry(p: &mut PrintContext) {
    let PrintContext { ctx, entry, .. } = p;

    let term = Style::new().bold().apply_to(&p.entry.term);

    print_divider(p);

    p.ctx.println(format!("{}", term));

    print_divider(p);

    p.ctx.newln();

    for (i, ety) in entry.etymologies.iter().enumerate() {
        print_ety(p, ety, i, entry.etymologies.len() > 1);
    }
}

fn pretty_print<'a>(
    ctx: &'a mut CLIContext<'a>,
    entries: &'a [Vec<Entry>],
    markdown: &'a MarkdownStrategy,
) -> Result<(), Box<dyn Error>> {
    if entries.len() == 0 {
        return Err("No entries found".into());
    }

    for entry in entries {
        for subentry in entry {
            print_entry(&mut PrintContext {
                ctx,
                markdown,
                entry: subentry,
            });
        }
    }

    // ctx.stdout.flush()?;

    Ok(())
}

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
