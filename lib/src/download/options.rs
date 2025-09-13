use std::{
    fmt,
    path::{Path, PathBuf},
};

pub type ProgressCallback<'a> = Box<dyn Fn(u64, Option<u64>, f64) + Send + Sync + 'a>;

pub struct DownloadOptions<'a> {
    pub caching: bool,
    pub out_dir: Option<PathBuf>,
    pub on_progress: Option<ProgressCallback<'a>>,
}

impl fmt::Debug for DownloadOptions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DownloadOptions")
            .field("caching", &self.caching)
            .field("out_dir", &self.out_dir)
            .field(
                "on_progress",
                &self.on_progress.as_ref().map(|_| "Some(callback)"),
            )
            .finish()
    }
}

impl Clone for DownloadOptions<'_> {
    fn clone(&self) -> Self {
        Self {
            caching: self.caching,
            out_dir: self.out_dir.clone(),
            on_progress: None, // Function pointers can't be cloned, so we set to None
        }
    }
}

impl Default for DownloadOptions<'_> {
    fn default() -> Self {
        Self {
            caching: true,
            out_dir: None,
            on_progress: None,
        }
    }
}

impl<'a> DownloadOptions<'a> {
    pub fn caching(mut self, value: bool) -> Self {
        self.caching = value;
        self
    }

    pub fn out_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(u64, Option<u64>, f64) + Send + Sync + 'a,
    {
        self.on_progress = Some(Box::new(callback));
        self
    }
}

impl<'a> AsRef<DownloadOptions<'a>> for DownloadOptions<'a> {
    fn as_ref(&self) -> &DownloadOptions<'a> {
        self
    }
}
