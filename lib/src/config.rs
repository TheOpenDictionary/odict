use dirs::home_dir;
use std::{env::var, path::PathBuf, sync::LazyLock};

pub const DEFAULT_CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    var("ODICT_CONFIG_DIR")
        .map(|d| PathBuf::from(d))
        .ok()
        .unwrap_or_else(|| {
            home_dir()
                .expect("Failed to get home directory")
                .join(".odict")
        })
});

#[cfg(test)]
mod test {
    use dirs::home_dir;

    use crate::config::DEFAULT_CONFIG_DIR;

    #[test]
    fn test_get_config_dir() {
        let home_dir = home_dir().expect("Failed to get home directory");
        let expected = home_dir.join(".odict");

        assert_eq!(*DEFAULT_CONFIG_DIR, expected);
    }
}
