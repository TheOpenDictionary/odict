use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct SplitOptions {
  #[merge(strategy = merge::option::overwrite_none)]
  pub min_length: Option<u32>,
}

impl Default for SplitOptions {
  fn default() -> Self {
    SplitOptions { min_length: None }
  }
}
