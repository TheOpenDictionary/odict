use std::{ffi::OsStr, path::PathBuf};

pub fn infer_path<P: Into<PathBuf> + AsRef<OsStr>>(path: P) -> PathBuf {
    let pb: PathBuf = path.into();
    let name = pb.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
    let directory = pb.parent().and_then(|s| s.to_str()).unwrap_or_default();

    PathBuf::new()
        .join(directory)
        .join(format!("{}.odict", name))
}
