use crate::{DefinitionType, Entry, Etymology, MarkdownStrategy, Sense};

fn flatten_definition_type(value: &DefinitionType, strategy: MarkdownStrategy) -> Vec<String> {
    match value {
        DefinitionType::Definition(d) => vec![d.value.parse(strategy)],
        DefinitionType::Group(g) => g
            .definitions
            .iter()
            .map(|d| d.value.parse(strategy))
            .collect(),
    }
}

fn flatten_sense(value: &Sense, strategy: MarkdownStrategy) -> Vec<String> {
    value
        .definitions
        .iter()
        .flat_map(|d| flatten_definition_type(d, strategy))
        .collect()
}

fn flatten_etymology(value: &Etymology, strategy: MarkdownStrategy) -> Vec<String> {
    value
        .senses
        .values()
        .flat_map(|s| flatten_sense(s, strategy))
        .collect()
}

fn flatten_entry(entry: &Entry, strategy: MarkdownStrategy) -> Vec<String> {
    entry
        .etymologies
        .iter()
        .flat_map(|e| flatten_etymology(e, strategy))
        .collect()
}

pub struct PreviewOptions {
    strategy: MarkdownStrategy,
    delimiter: String,
}

impl Default for PreviewOptions {
    fn default() -> Self {
        Self {
            strategy: MarkdownStrategy::default(),
            delimiter: "; ".to_string(),
        }
    }
}

impl PreviewOptions {
    pub fn strategy(mut self, strategy: MarkdownStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn delimiter(mut self, delimiter: String) -> Self {
        self.delimiter = delimiter;
        self
    }
}

impl Entry {
    pub fn preview(&self, options: PreviewOptions) -> String {
        let definitions = flatten_entry(self, options.strategy);
        definitions.join(&options.delimiter)
    }
}
