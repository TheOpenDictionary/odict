use std::fmt::{Display, Formatter};

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum PrintFormat {
    Print,
    JSON,
    XML,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum MarkdownStrategy {
    HTML,
    Text,
    Disabled,
}

impl Display for MarkdownStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownStrategy::HTML => write!(f, "HTML"),
            MarkdownStrategy::Text => write!(f, "Text"),
            MarkdownStrategy::Disabled => write!(f, "Disabled"),
        }
    }
}
