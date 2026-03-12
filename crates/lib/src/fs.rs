use std::{
    ffi::OsStr,
    fs::{canonicalize, File},
    io::Read,
    path::{Path, PathBuf},
};

pub fn infer_path<P: Into<PathBuf> + AsRef<OsStr>>(path: P) -> PathBuf {
    let pb: PathBuf = path.into();
    let name = pb.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
    let directory = pb.parent().and_then(|s| s.to_str()).unwrap_or_default();

    PathBuf::new()
        .join(directory)
        .join(format!("{name}.odict"))
}

fn open_file<P: AsRef<Path>>(path: P) -> crate::Result<File> {
    let file = File::open(canonicalize(path.as_ref())?)?;
    Ok(file)
}

pub(crate) fn read_to_bytes<P: AsRef<Path>>(path: P) -> crate::Result<Vec<u8>> {
    let mut file = open_file(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub(crate) fn read_to_string<P: AsRef<Path>>(path: P) -> crate::Result<String> {
    let mut file = open_file(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
