use serde::{Deserialize, Serialize, Serializer};

use crate::{
    md::{to_html, to_text},
    serializable_custom,
};

pub enum MarkdownStrategy {
    HTML,
    Text,
    Disabled,
}

impl Default for MarkdownStrategy {
    fn default() -> Self {
        MarkdownStrategy::HTML
    }
}

impl AsRef<MarkdownStrategy> for MarkdownStrategy {
    fn as_ref(&self) -> &MarkdownStrategy {
        self
    }
}

serializable_custom! {
  pub struct MDString {
    value: String,
  }
}

impl<'de> Deserialize<'de> for MDString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(MDString { value })
    }
}

impl Serialize for MDString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl From<&str> for MDString {
    fn from(value: &str) -> MDString {
        MDString {
            value: value.to_string(),
        }
    }
}

impl MDString {
    pub fn parse<S: AsRef<MarkdownStrategy>>(&self, strategy: S) -> String {
        match strategy.as_ref() {
            MarkdownStrategy::HTML => to_html(&self.value),
            // MarkdownStrategy::HTML => to_html(&self.value),
            MarkdownStrategy::Text => to_text(&self.value),
            MarkdownStrategy::Disabled => self.value.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let md = MDString::from("**This** is a <sup>test</sup> of the _parser_!");

        assert_eq!(
            md.parse(MarkdownStrategy::HTML),
            "<strong>This</strong> is a <sup>test</sup> of the <em>parser</em>!"
        );

        assert_eq!(
            md.parse(MarkdownStrategy::Text),
            "This is a test of the parser!"
        );

        assert_eq!(md.parse(MarkdownStrategy::Disabled), md.value);
    }
}
