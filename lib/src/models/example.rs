use crate::serializable;

serializable! {
  pub struct Example {
    #[serde(rename = "$text")]
    value: String,
  }
}
