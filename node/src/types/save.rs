#[napi(object)]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct CompressOptions {
  pub quality: Option<u32>,
  pub window_size: Option<u32>,
}

#[napi(object)]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct SaveOptions {
  pub compress: Option<CompressOptions>,
}

impl From<SaveOptions> for odict::compile::CompilerOptions {
  fn from(opts: SaveOptions) -> Self {
    let mut compiler_options = odict::compile::CompilerOptions::default();

    if let Some(compress) = opts.compress {
      let mut compress_options = odict::CompressOptions::default();

      if let Some(quality) = compress.quality {
        compress_options = compress_options.quality(quality);
      }

      if let Some(window_size) = compress.window_size {
        compress_options = compress_options.window_size(window_size);
      }

      compiler_options = compiler_options.with_compression(compress_options);
    }

    compiler_options
  }
}
