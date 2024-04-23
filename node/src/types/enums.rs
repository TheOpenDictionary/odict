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
