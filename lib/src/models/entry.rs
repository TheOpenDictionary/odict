use crate::serializable;

use super::Etymology;

serializable! {
  pub struct Entry {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@see")]
    pub see_also: Option<String>,

    #[serde(default, rename = "ety")]
    pub etymologies: Vec<Etymology>,
  }
}
