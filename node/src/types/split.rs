use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct SplitOptions {
  pub min_length: Option<u32>,
}

impl Default for SplitOptions {
  fn default() -> Self {
    SplitOptions { min_length: None }
  }
}

impl From<SplitOptions> for odict::split::SplitOptions {
  fn from(opts: SplitOptions) -> Self {
    let mut s = odict::split::SplitOptions::default();

    if let Some(min_length) = opts.min_length {
      s = s.min_length(min_length.try_into().unwrap());
    }

    s
  }
}
