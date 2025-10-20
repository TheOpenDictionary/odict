use crate::serializable;

serializable! {
  #[derive(Default)]
  #[serde(rename = "translation")]
  pub struct Translation {
    #[serde(rename = "@lang")]
    #[rkyv(with = crate::intern::Intern)]
    pub lang: String,

    #[serde(rename = "@value")]
    #[rkyv(with = crate::intern::Intern)]
    pub value: String,
  }
}
