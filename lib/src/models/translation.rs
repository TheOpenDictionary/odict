use crate::serializable;

serializable! {
  #[serde(rename = "translation")]
  pub struct Translation {
    #[serde(rename = "@lang")]
    pub lang: String,

    #[serde(rename = "@value")]
    pub value: String,
  }
}
