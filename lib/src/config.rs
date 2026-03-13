use std::path::PathBuf;
use crate::get_storage_path;

pub fn get_config_dir() -> PathBuf {
    get_storage_path().join(".odict")
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;
    use crate::init;
    use super::get_config_dir;

    #[test]
    fn test_get_config_dir() {
        let tmp = tempdir().unwrap();
        let storage_path = tmp.path().to_path_buf();
        init(storage_path.clone());
        let expected = storage_path.join(".odict");

        assert_eq!(get_config_dir(), expected);
    }
}
