use dirs::home_dir;
use std::{collections::HashMap, env::var, error::Error, fs::create_dir, path::PathBuf};

pub(super) struct ODictConfig {
    pub(super) path: PathBuf,
    pub(super) aliases: HashMap<String, String>,
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir_name = var("ODICT_CONFIG_DIR").ok().unwrap_or_else(|| {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(home_dir).join(".odict");

        config_dir.to_string_lossy().to_string()
    });

    get_dir(dir_name.as_str())
}

pub(super) fn get_dir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = PathBuf::from(&name);

    if !path.exists() {
        create_dir(&path)?;
    }

    Ok(path)
}

#[cfg(test)]
mod test {
    use dirs::home_dir;
    use std::path::PathBuf;

    use super::get_config_dir;

    #[test]
    fn test_get_config_dir() {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(home_dir);
        let config_dir = get_config_dir().unwrap().join(".odict");
        let actual = config_dir.to_str().unwrap();
        let expected = config_dir.to_string_lossy().to_string();

        assert_eq!(actual, expected);
    }
}
