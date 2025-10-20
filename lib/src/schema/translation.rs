use crate::serializable;

serializable! {
  #[derive(Default)]
  #[serde(rename = "translation")]
  pub struct Translation {
    #[serde(rename = "@lang")]
    #[rkyv(with = rkyv_intern::Intern)]
    pub lang: String,

    #[serde(rename = "@value")]
    #[rkyv(with = rkyv_intern::Intern)]
    pub value: String,
  }
}
