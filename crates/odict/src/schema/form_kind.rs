use crate::serializable_enum;

serializable_enum! {
  pub enum FormKind {
      Conjugation,
      Inflection,
      Plural,
      Singular,
      Comparative,
      Superlative,
      #[strum(to_string = "{0}")]
      #[serde(untagged)]
      Other(String),
  }
}
