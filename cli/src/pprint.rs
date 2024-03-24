use std::{borrow::Cow, error::Error, io::Write};

use console::{pad_str, style, Alignment, Style};
use indicatif::TermLike;
use odict::{
    dump::{ToJSON, ToXML},
    Definition, DefinitionType, Entry, Group, MarkdownStrategy,
};

use once_cell::sync::Lazy;
use termimad::{crossterm::style::Stylize, minimad::TextTemplate, MadSkin};

use crate::{enums::PrintFormat, CLIContext};

const STYLE_POS: Lazy<Style> = Lazy::new(|| Style::new().italic());
const STYLE_TITLE: Lazy<Style> = Lazy::new(|| Style::new().bold().underlined());
const STYLE_EXAMPLE_BULLET: Lazy<Style> = Lazy::new(|| Style::new().dim());
const STYLE_EXAMPLE: Lazy<Style> = Lazy::new(|| STYLE_EXAMPLE_BULLET.clone().italic().dim());
const STYLE_EXAMPLE_TARGET: Lazy<Style> = Lazy::new(|| STYLE_EXAMPLE.clone().underlined());

fn divider() -> String {
    "─".repeat(32)
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

pub(super) fn pretty_print(
    ctx: &mut CLIContext,
    entries: Vec<Vec<Entry>>,
) -> Result<(), Box<dyn Error>> {
    let out = &ctx.stdout;

    for entry in entries.into_iter().flat_map(|e| e) {
        let term = entry.term;

        out.write_line("")?;
        out.write_line(&divider())?;
        out.write_line(&format!("{}", style(&term).bold()))?;
        out.write_line(&divider())?;

        let ety_count = entry.etymologies.len();

        for (idx, etymology) in entry.etymologies.into_iter().enumerate() {
            if ety_count > 1 {
                out.write_line(
                    &STYLE_TITLE
                        .apply_to(&format!("Etymology #{}", idx + 1))
                        .to_string(),
                )?;
            }

            if let Some(desc) = etymology.description {
                out.write_line(&format!("\n{}", &desc.parse(MarkdownStrategy::Disabled)))?;
            }

            for (pos, sense) in etymology.senses {
                out.write_line(&format!(
                    "\n{}\n",
                    STYLE_POS.apply_to(pos.to_string()).italic()
                ))?;

                for (idx, dt) in sense.definitions.into_iter().enumerate() {
                    let num = idx + 1;

                    match dt {
                        DefinitionType::Definition(d) => {
                            let definition =
                                &format!("{}. {}", num, d.value.parse(MarkdownStrategy::Disabled));

                            out.write_line(&indent(definition, 2))?;

                            let has_subitems = d.examples.len() > 0 || d.notes.len() > 0;

                            if has_subitems {
                                out.write_line("")?;
                            }

                            for example in d.examples.into_iter() {
                                let text = &format!(
                                    "{} {}",
                                    STYLE_EXAMPLE_BULLET.apply_to("▸").to_string(),
                                    &underline_target(
                                        &example.value.parse(MarkdownStrategy::Disabled),
                                        &term
                                    )
                                );

                                out.write_line(&indent(&text, 5))?;
                            }

                            if d.notes.len() > 0 {
                                let header = &format!("\n{}\n\n", STYLE_TITLE.apply_to("Notes"));

                                out.write_line(&indent(header, 5))?;

                                for (ndx, note) in d.notes.into_iter().enumerate() {
                                    out.write_line(&indent(
                                        &format!(
                                            "{}. {}",
                                            (ndx as u8 + b'a') as char,
                                            note.value.parse(MarkdownStrategy::Disabled)
                                        ),
                                        6,
                                    ))?;

                                    if (note.examples.len() > 0) {
                                        out.write_line("")?;

                                        for example in note.examples.into_iter() {
                                            let text = &format!(
                                                "{} {}",
                                                STYLE_EXAMPLE_BULLET.apply_to("▸").to_string(),
                                                &underline_target(
                                                    &example
                                                        .value
                                                        .parse(MarkdownStrategy::Disabled),
                                                    &term
                                                )
                                            );

                                            out.write_line(&indent(&text, 9))?;
                                        }
                                    }
                                }
                            }

                            if has_subitems {
                                out.write_line("")?;
                            }
                        }
                        DefinitionType::Group(g) => {}
                    }
                }
            }
        }
    }

    out.flush()?;
    // skin.print_text(&output);

    Ok(())
}
