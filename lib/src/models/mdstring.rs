use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};

use crate::md::{to_html, to_text};

use crate::serializable_custom;

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

const PARENTHETICAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\(.+?\))").unwrap());

macro_rules! parse {
    ($t:ident) => {
        impl $t {
            pub fn parse<S: AsRef<MarkdownStrategy>>(&self, strategy: S) -> String {
                let repl = PARENTHETICAL_REGEX
                    .replace_all(&self.value, "*${1}*")
                    .to_string();

                match strategy.as_ref() {
                    MarkdownStrategy::HTML => to_html(&repl),
                    MarkdownStrategy::Text => to_text(&repl),
                    MarkdownStrategy::Disabled => repl.to_owned(),
                }
            }
        }
    };
}

parse!(MDString);
parse!(ArchivedMDString);

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

    #[test]
    fn test_parenthetical() {
        let md = MDString::from("(This) is a (test)");

        assert_eq!(
            md.parse(MarkdownStrategy::HTML),
            "<em>(This)</em> is a (test)"
        );

        assert_eq!(md.parse(MarkdownStrategy::Text), "(This) is a (test)");

        assert_eq!(md.parse(MarkdownStrategy::Disabled), "*(This)* is a (test)");
    }
}
