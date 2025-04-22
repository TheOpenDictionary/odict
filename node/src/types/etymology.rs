use std::collections::HashMap;

use napi::bindgen_prelude::*;

use super::pronunciation::Pronunciation;
use super::sense::Sense;

#[napi(object)]
pub struct Etymology {
  pub id: Option<String>,
  #[napi(ts_type = "Pronunciation[]")]
  pub pronunciations: Vec<Pronunciation>,
  pub description: Option<String>,
  pub senses: HashMap<String, Sense>,
}

impl Etymology {
  pub fn from(etymology: odict::Etymology) -> Result<Self> {
    let odict::Etymology {
      id,
      pronunciations,
      description,
      senses,
    } = etymology;

    Ok(Self {
      id,
      pronunciations: pronunciations
        .into_iter()
        .map(|p| Pronunciation::from(p))
        .collect::<Result<Vec<Pronunciation>, _>>()?,
      description,
      senses: senses
        .into_iter()
        .map(|(k, v)| -> Result<(String, Sense), _> {
          let sense = Sense::from(v)?;
          Ok((k.to_string(), sense))
        })
        .collect::<Result<HashMap<String, Sense>, _>>()?,
    })
  }
}
