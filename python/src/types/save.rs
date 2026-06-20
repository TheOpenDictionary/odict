use pyo3::prelude::*;

/// Brotli compression options for saving dictionaries.
#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct CompressOptions {
    /// Compression quality level (0–11).
    #[pyo3(get, set)]
    pub quality: Option<u32>,

    /// Compression window size (0–22).
    #[pyo3(get, set)]
    pub window_size: Option<u32>,
}

#[pymethods]
impl CompressOptions {
    #[new]
    #[pyo3(signature = (quality=None, window_size=None))]
    pub fn new(quality: Option<u32>, window_size: Option<u32>) -> Self {
        CompressOptions {
            quality,
            window_size,
        }
    }
}

/// Options for saving a dictionary to disk.
#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct SaveOptions {
    /// Optional Brotli compression settings.
    #[pyo3(get, set)]
    pub compress: Option<CompressOptions>,
}

#[pymethods]
impl SaveOptions {
    #[new]
    #[pyo3(signature = (compress=None))]
    pub fn new(compress: Option<CompressOptions>) -> Self {
        SaveOptions { compress }
    }
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
