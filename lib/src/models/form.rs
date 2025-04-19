use serde::{Deserialize, Deserializer};
use std::fmt;

use crate::serializable;

use super::EntryRef;

serializable! {
  #[serde(rename_all = "lowercase")]
  pub enum FormKind {
      Conjugation,
      Inflection,
      Plural,
      Singular,
      Comparative,
      Superlative,
      #[serde(other)]
      Other,
  }
}

impl fmt::Display for FormKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormKind::Conjugation => write!(f, "conjugation"),
            FormKind::Inflection => write!(f, "inflection"),
            FormKind::Plural => write!(f, "plural"),
            FormKind::Singular => write!(f, "singular"),
            FormKind::Comparative => write!(f, "comparative"),
            FormKind::Superlative => write!(f, "superlative"),
            FormKind::Other => write!(f, "other"),
        }
    }
}

serializable! {
  pub struct Form {
    #[serde(rename = "@kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(rename = "$text")]
    pub term: EntryRef,
  }
}

pub fn unwrap_forms<'de, D>(deserializer: D) -> Result<Vec<Form>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Forms {
        #[serde(default)]
        form: Vec<Form>,
    }

    let wrapper = Option::<Forms>::deserialize(deserializer)?;

    Ok(wrapper.map(|forms| forms.form).unwrap_or_default())
}
