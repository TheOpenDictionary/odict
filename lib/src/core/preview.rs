#[cfg(feature = "markdown")]
use crate::md::to_text;
use crate::schema::{ArchivedDefinitionType, ArchivedEntry, DefinitionType, Entry};

pub struct PreviewOptions {
    delimiter: String,
}

impl Default for PreviewOptions {
    fn default() -> Self {
        Self {
            delimiter: "; ".to_string(),
        }
    }
}

impl PreviewOptions {
    pub fn delimiter(mut self, delimiter: String) -> Self {
        self.delimiter = delimiter;
        self
    }
}

#[cfg(not(feature = "markdown"))]
fn to_text(value: &str) -> &str {
    value
}

macro_rules! preview {
    ($t:ident, $d:ident) => {
        impl $t {
            pub fn preview(&self, options: PreviewOptions) -> String {
                let definitions: Vec<String> = self
                    .etymologies
                    .iter()
                    .flat_map(|e| -> Vec<String> {
                        e.senses
                            .iter()
                            .flat_map(|s| -> Vec<String> {
                                s.definitions
                                    .iter()
                                    .flat_map(|value| -> Vec<String> {
                                        match value {
                                            $d::Definition(d) => {
                                                vec![to_text(d.value.as_str()).to_string()]
                                            }
                                            $d::Group(g) => g
                                                .definitions
                                                .iter()
                                                .map(|d| to_text(d.value.as_str()).to_string())
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
