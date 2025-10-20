use crate::serializable;

use super::example::Example;

serializable! {
  #[derive(Default)]
  #[serde(rename = "note")]
  pub struct Note {
    #[serde(default, rename = "example")]
    pub examples: Vec<Example>,

    #[serde(rename = "@id")]
    #[rkyv(with = rkyv::with::Map<rkyv_intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    #[rkyv(with = rkyv_intern::Intern)]
    pub value: String,
  }
}
