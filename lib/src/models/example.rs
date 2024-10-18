use crate::{serializable, MDString};

serializable! {
  pub struct Example {
    #[serde(rename = "$text")]
    pub value: MDString,
  }
}
