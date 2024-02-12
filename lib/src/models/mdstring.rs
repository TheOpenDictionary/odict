use serde::{Deserialize, Serialize, Serializer};

use crate::{
    md::{to_html, to_text},
    serializable_custom,
};

#[derive(Debug, Copy, Clone)]
pub enum MarkdownStrategy {
    HTML,
    Text,
    Disabled,
}

serializable_custom! {
  pub struct MDString(String);
}

impl<'de> Deserialize<'de> for MDString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(MDString(value))
    }
}

impl Serialize for MDString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl From<&str> for MDString {
    fn from(value: &str) -> MDString {
        MDString(value.to_string())
    }
}

impl MDString {
    pub fn parse(&self, strategy: MarkdownStrategy) -> String {
        match strategy {
            MarkdownStrategy::HTML => to_html(&self.0),
            MarkdownStrategy::Text => to_text(&self.0),
            MarkdownStrategy::Disabled => self.0.to_owned(),
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

        assert_eq!(md.parse(MarkdownStrategy::Disabled), md.0);
    }
}
