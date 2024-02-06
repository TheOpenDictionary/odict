use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum PrintFormat {
    Print,
    JSON,
    XML,
}
