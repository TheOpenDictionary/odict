use serde::{Deserialize, Deserializer};
use std::fmt;

use crate::serializable;

use super::EntryRef;

serializable! {
  #[serde(rename_all = "lowercase")]
  pub enum FormType {
      Conjugation,
      Inflection,
      Plural,
      Singular,
      Comparative,
      Superlative,
      PastTense,
      PresentParticiple,
      #[serde(other)]
      Other,
  }
}

impl fmt::Display for FormType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormType::Conjugation => write!(f, "conjugation"),
            FormType::Inflection => write!(f, "inflection"),
            FormType::Plural => write!(f, "plural"),
            FormType::Singular => write!(f, "singular"),
            FormType::Comparative => write!(f, "comparative"),
            FormType::Superlative => write!(f, "superlative"),
            FormType::PastTense => write!(f, "past-tense"),
            FormType::PresentParticiple => write!(f, "present-participle"),
            FormType::Other => write!(f, "other"),
        }
    }
}

serializable! {
  pub struct Form {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<FormType>,

    #[serde(rename = "$text")]
    pub value: EntryRef,
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
