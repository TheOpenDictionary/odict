use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use crate::serializable;

use super::{
    tags::{unwrap_tags, wrap_tags},
    EntryRef,
};

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
  #[serde(rename = "form")]
  pub struct Form {
    #[serde(rename = "@kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(rename = "@term")]
    pub term: EntryRef,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "unwrap_tags")]
    #[serde(serialize_with = "wrap_tags")]
    pub tags: Vec<String>,
  }
}

/// Deserialize a vector of forms from XML that has the following structure:
/// <forms>
///   <form term="word" kind="plural">...</form>
/// </forms>
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

/// Serialize a vector of forms to XML with the following structure:
/// <forms>
///   <form term="word" kind="plural">...</form>
/// </forms>
pub fn wrap_forms<S>(forms: &Vec<Form>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Forms {
        form: Vec<Form>,
    }

    if forms.is_empty() {
        // Don't serialize anything if there are no forms
        serializer.serialize_none()
    } else {
        let wrapped_forms = Forms {
            form: forms.clone(),
        };

        wrapped_forms.serialize(serializer)
    }
}
