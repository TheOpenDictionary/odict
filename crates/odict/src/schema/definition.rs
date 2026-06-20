use crate::intern::Intern;

use crate::serializable;

use super::{example::Example, note::Note};

serializable! {
  #[derive(Default)]
  pub struct Definition {
    #[serde(rename = "@id")]
    #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    #[rkyv(with = Intern)]
    pub value: String,

    #[serde(default, rename="example")]
    pub examples: Vec<Example>,

    #[serde(default, rename="note")]
    pub notes: Vec<Note>,
  }
}
