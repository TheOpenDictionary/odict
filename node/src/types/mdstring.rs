use napi::bindgen_prelude::*;

#[napi]
pub enum MarkdownStrategy {
  Disabled,
  HTML,
  Text,
}

impl From<MarkdownStrategy> for odict::MarkdownStrategy {
  fn from(strategy: MarkdownStrategy) -> Self {
    match strategy {
      MarkdownStrategy::Disabled => odict::MarkdownStrategy::Disabled,
      MarkdownStrategy::HTML => odict::MarkdownStrategy::HTML,
      MarkdownStrategy::Text => odict::MarkdownStrategy::Text,
    }
  }
}

impl From<&str> for MarkdownStrategy {
  fn from(strategy: &str) -> Self {
    match strategy {
      "html" => MarkdownStrategy::HTML,
      "text" => MarkdownStrategy::Text,
      _ => MarkdownStrategy::Disabled,
    }
  }
}

#[napi]
pub struct String {
  mds: odict::String,
}

#[napi]
impl String {
  #[napi(constructor)]
  pub fn new(value: String) -> Result<Self> {
    Ok(Self {
      mds: odict::String::from(value.as_str()),
    })
  }

  #[napi(getter)]
  pub fn value(&self) -> String {
    self.mds.value().to_string()
  }

  #[napi]
  pub fn parse(&self, strategy: MarkdownStrategy) -> String {
    let s: odict::MarkdownStrategy = strategy.into();
    self.mds.parse(s)
  }
}

impl From<odict::String> for String {
  fn from(mds: odict::String) -> Self {
    Self { mds }
  }
}
