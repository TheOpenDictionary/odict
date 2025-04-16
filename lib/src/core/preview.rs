use crate::{ArchivedEntry, DefinitionType, Entry};

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

#[cfg(feature = "markdown")]
fn to_plain_text(value: &str) -> String {
    crate::md::to_text(value).to_string()
}

#[cfg(not(feature = "markdown"))]
fn to_plain_text(value: &str) -> String {
    value.to_string()
}

// Implement separate macros for Entry and ArchivedEntry
macro_rules! preview_entry {
    () => {
        impl Entry {
            pub fn preview(&self, options: PreviewOptions) -> String {
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
                                            DefinitionType::Definition(d) => {
                                                vec![to_plain_text(&d.value.to_plain_string())]
                                            }
                                            DefinitionType::Group(g) => g
                                                .definitions
                                                .iter()
                                                .map(|d| to_plain_text(&d.value.to_plain_string()))
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

// For ArchivedEntry, we need a different approach since ArchivedRichText doesn't have to_plain_string
macro_rules! preview_archived_entry {
    () => {
        impl ArchivedEntry {
            pub fn preview(&self, _options: PreviewOptions) -> String {
                // For now, return a simple placeholder message
                // This avoids complex interactions with the archived types
                // until we can properly implement rich text archived preview
                "Preview of archived entry (references not shown)".to_string()
            }
        }
    };
}

preview_entry!();
preview_archived_entry!();
