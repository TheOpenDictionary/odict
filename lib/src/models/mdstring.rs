use serde::{Deserialize, Serialize, Serializer};

use crate::{serializable, serializable_custom};

pub enum MarkdownStrategy {
    HTML,
    Text,
    Disabled,
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
    pub fn parse(&self, strategy: MarkdownStrategy) -> String {
        // output := []byte(mds)

        // switch markdownStrategy {
        // case MarkdownStrategyHTML:
        //   output = utils.MarkdownToHTML(output)
        // case MarkdownStrategyText:
        //   output = utils.MarkdownToText(output)
        // }

        // TODO:implement

        self.value.clone()
    }
}
