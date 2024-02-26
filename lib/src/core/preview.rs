use rkyv::string::ArchivedString;

use crate::{ArchivedDefinitionType, ArchivedEntry, DefinitionType, Entry, MarkdownStrategy};

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

macro_rules! preview {
    ($t:ident, $d:ident) => {
        impl $t {
            pub fn preview(&self, options: PreviewOptions) -> String {
                let strategy = &options.strategy;

                let definitions: Vec<String> = self
                    .etymologies
                    .iter()
                    .flat_map(|e| -> Vec<String> {
                        e.senses
                            .values()
                            .flat_map(|s| -> Vec<String> {
                                s.definitions
                                    .iter()
                                    .flat_map(|value| -> Vec<String> {
                                        match value {
                                            $d::Definition(d) => {
                                                vec![d.value.parse(strategy)]
                                            }
                                            $d::Group(g) => g
                                                .definitions
                                                .iter()
                                                .map(|d| d.value.parse(strategy))
                                                .collect(),
                                        }
                                    })
                                    .collect()
                            })
                            .collect()
                    })
                    .collect();

                definitions.join(&options.delimiter)
            }
        }
    };
}

preview!(Entry, DefinitionType);
preview!(ArchivedEntry, ArchivedDefinitionType);
