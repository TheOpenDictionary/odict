use std::fmt::Display;

use clap::ValueEnum;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, ValueEnum)]
pub enum PrintFormat {
    Print,
    JSON,
    XML,
    Markdown,
    HTML,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, ValueEnum)]
pub enum DumpFormat {
    XML,
    SQLite,
    Postgres,
    MySQL,
}

impl Display for PrintFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintFormat::Print => write!(f, "print"),
            PrintFormat::JSON => write!(f, "json"),
            PrintFormat::XML => write!(f, "xml"),
            PrintFormat::Markdown => write!(f, "markdown"),
            PrintFormat::HTML => write!(f, "html"),
        }
    }
}

impl Display for DumpFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DumpFormat::XML => write!(f, "xml"),
            DumpFormat::SQLite => write!(f, "sqlite"),
            DumpFormat::Postgres => write!(f, "postgres"),
            DumpFormat::MySQL => write!(f, "mysql"),
        }
    }
}
