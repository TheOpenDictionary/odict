use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

pub type ProgressCallback<'a> = Arc<dyn Fn(u64, Option<u64>, f64) + Send + Sync + 'a>;

#[derive(Clone)]
pub struct DownloadOptions<'a> {
    pub caching: bool,
    pub(crate) config_dir: Option<PathBuf>,
    pub out_dir: Option<PathBuf>,
    pub on_progress: Option<ProgressCallback<'a>>,
    pub retries: u32,
}

impl fmt::Debug for DownloadOptions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DownloadOptions")
            .field("caching", &self.caching)
            .field("out_dir", &self.out_dir)
            .field("retries", &self.retries)
            .field(
                "on_progress",
                &self.on_progress.as_ref().map(|_| "Some(callback)"),
            )
            .finish()
    }
}

impl Default for DownloadOptions<'_> {
    fn default() -> Self {
        Self {
            caching: true,
            config_dir: None,
            out_dir: None,
            on_progress: None,
            retries: 3,
        }
    }
}

impl<'a> DownloadOptions<'a> {
    pub fn with_caching(mut self, value: bool) -> Self {
        self.caching = value;
        self
    }

    pub fn with_config_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.config_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn with_out_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(u64, Option<u64>, f64) + Send + Sync + 'a,
    {
        self.on_progress = Some(Arc::new(callback));
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}

impl<'a> AsRef<DownloadOptions<'a>> for DownloadOptions<'a> {
    fn as_ref(&self) -> &DownloadOptions<'a> {
        self
    }
}
