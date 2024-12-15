use dirs::home_dir;
use std::{env::var, error::Error, fs::create_dir, path::PathBuf};

pub fn get_config_dir() -> crate::Result<PathBuf> {
    let dir_name = var("ODICT_CONFIG_DIR").ok().unwrap_or_else(|| {
        PathBuf::from(home_dir().expect("Failed to get home directory"))
            .join(".odict")
            .to_string_lossy()
            .to_string()
    });

    let path = PathBuf::from(&dir_name.as_str());

    if !path.exists() {
        create_dir(&path)?;
    }

    Ok(path)
}

#[cfg(test)]
mod test {
    use dirs::home_dir;

    use super::get_config_dir;

    #[test]
    fn test_get_config_dir() {
        let home_dir = home_dir().expect("Failed to get home directory");
        let actual = get_config_dir().unwrap();
        let expected = home_dir.join(".odict");

        assert_eq!(actual.to_str().unwrap(), expected.to_str().unwrap());
    }
}
