use serde::{Deserialize, Deserializer};

use crate::serializable;

use super::EntryRef;

serializable! {
  pub struct Form {
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
