use std::{borrow::Cow, sync::LazyLock};

use console::{style, Style};
use odict::{Definition, DefinitionType, Entry, Error, Etymology, Example, Group, Note, Sense};

use crate::CLIContext;

use super::md::print_md;

const STYLE_POS: LazyLock<Style> = LazyLock::new(|| Style::new().italic());
const STYLE_TITLE: LazyLock<Style> = LazyLock::new(|| Style::new().bold().underlined());
const STYLE_EXAMPLE_BULLET: LazyLock<Style> = LazyLock::new(|| Style::new().dim());
const STYLE_EXAMPLE: LazyLock<Style> =
    LazyLock::new(|| STYLE_EXAMPLE_BULLET.clone().italic().dim());
const STYLE_EXAMPLE_TARGET: LazyLock<Style> = LazyLock::new(|| STYLE_EXAMPLE.clone().underlined());

fn divider() -> String {
    "─".repeat(32)
}

fn index_to_alpha(i: usize) -> char {
    (i as u8 + b'a') as char
}

fn underline_target(example: &str, target: &str) -> String {
    let mut parts = Vec::new();
    let mut last_index = 0;

    for (index, _) in example.match_indices(target) {
        parts.push(
            STYLE_EXAMPLE
                .apply_to(&example[last_index..index])
                .to_string(),
        );

        parts.push(STYLE_EXAMPLE_TARGET.apply_to(target).to_string());

        last_index = index + target.len();
    }

    if last_index < example.len() {
        parts.push(STYLE_EXAMPLE.apply_to(&example[last_index..]).to_string());
    }

    let modified_string = parts.concat();

    modified_string
}

fn indent<'a>(s: &'a str, width: usize) -> Cow<'a, str> {
    let padding = " ".repeat(width);

    s.lines()
        .into_iter()
        .map(|l| format!("{}{}", padding, l))
        .collect::<Vec<String>>()
        .join("\n")
        .into()
}

fn print_note(ctx: &CLIContext, index: usize, note: &Note, entry: &Entry) -> Result<(), Error> {
    let out = &ctx.stdout;

    out.write_line(&indent(
        &format!("{}. {}", index_to_alpha(index), note.value),
        6,
    ))?;

    if note.examples.len() > 0 {
        out.write_line("")?;

        for example in (&note).examples.iter() {
            print_example(ctx, 9, example, entry)?;
        }
    }

    Ok(())
}

fn print_example(
    ctx: &CLIContext,
    indent_width: usize,
    example: &Example,
    entry: &Entry,
) -> Result<(), Error> {
    let out = &ctx.stdout;

    let text = &format!(
        "{} {}",
        STYLE_EXAMPLE_BULLET.apply_to("▸").to_string(),
        &underline_target(&example.value, &entry.term)
    );

    out.write_line(&indent(&text, indent_width))?;

    Ok(())
}

fn print_definition(
    ctx: &CLIContext,
    index: usize,
    indent_width: usize,
    use_alpha: bool,
    definition: &Definition,
    entry: &Entry,
) -> Result<(), Error> {
    let out = &ctx.stdout;

    let numbering = match use_alpha {
        false => (index + 1).to_string(),
        true => index_to_alpha(index).to_string(),
    };

    let text = &format!("{}. {}", numbering, print_md(&definition.value));

    out.write_line(&indent(text, indent_width))?;

    for example in (&definition).examples.iter() {
        print_example(ctx, indent_width + 3, example, entry)?;
    }

    if definition.notes.len() > 0 {
        let header = &format!("\n{}\n\n", STYLE_TITLE.apply_to("Notes"));

        out.write_line(&indent(header, indent_width + 3))?;

        for (ndx, note) in (&definition).notes.iter().enumerate() {
            print_note(ctx, ndx, note, entry)?;
        }

        out.write_line("")?;
    }

    Ok(())
}

fn print_group(ctx: &CLIContext, index: usize, group: &Group, entry: &Entry) -> Result<(), Error> {
    let out = &ctx.stdout;

    let text = &format!("{}. {}", index + 1, group.description);

    out.write_line(&indent(text, 2))?;

    for (idx, definition) in group.definitions.iter().enumerate() {
        print_definition(ctx, idx, 5, true, definition, entry)?;
    }

    Ok(())
}
fn print_sense(ctx: &CLIContext, sense: &Sense, entry: &Entry) -> Result<(), Error> {
    let out = &ctx.stdout;

    out.write_line(&format!(
        "\n{}\n",
        STYLE_POS.apply_to(sense.pos.description()).italic()
    ))?;

    for (idx, dt) in sense.definitions.iter().enumerate() {
        match dt {
            DefinitionType::Definition(d) => print_definition(ctx, idx, 2, false, d, entry)?,
            DefinitionType::Group(g) => print_group(ctx, idx, g, entry)?,
        }
    }

    Ok(())
}

fn print_ety(ctx: &CLIContext, etymology: &Etymology, entry: &Entry) -> Result<(), Error> {
    let out = &ctx.stdout;

    if let Some(desc) = &etymology.description {
        out.write_line(&format!("\n{}", &desc))?;
    }

    for sense in etymology.senses.values() {
        print_sense(ctx, sense, entry)?;
    }

    Ok(())
}

pub(super) fn pretty_print(ctx: &CLIContext, entries: Vec<Vec<Entry>>) -> Result<(), Error> {
    let out = &ctx.stdout;

    for entry in entries.iter().flat_map(|e| e) {
        out.write_line("")?;
        out.write_line(&divider())?;
        out.write_line(&format!("{}", style(&entry.term).bold()))?;
        out.write_line(&divider())?;

        let ety_count = entry.etymologies.len();

        for (idx, etymology) in entry.etymologies.iter().enumerate() {
            if ety_count > 1 {
                out.write_line(
                    &STYLE_TITLE
                        .apply_to(&format!("\nEtymology #{}", idx + 1))
                        .to_string(),
                )?;
            }

            print_ety(ctx, etymology, &entry)?;
        }
    }

    out.flush()?;

    Ok(())
}
