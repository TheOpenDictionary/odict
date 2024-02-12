use dirs::home_dir;
use std::collections::HashMap;
use std::env::var;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, create_dir, read_to_string};
use std::path::{Path, PathBuf};

pub(super) struct ODictConfig {
    pub(super) path: PathBuf,
    pub(super) aliases: HashMap<String, String>,
}

pub(super) fn get_config_dir(custom: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = custom.map(|c| c.to_string()).unwrap_or_else(|| {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(home_dir);

        config_dir.push(".odict");

        return config_dir.to_string_lossy().to_string();
    });

    let path = PathBuf::from(&config_dir);

    if !path.exists() {
        create_dir(&path)?;
    }

    Ok(path)
}

pub(super) fn get_default_config_path(custom_dir: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(&get_config_dir(custom_dir)?).join("aliases.json"))
}

pub(super) fn get_config<S: AsRef<OsStr> + ?Sized>(
    config_path: Option<&S>,
) -> Result<ODictConfig, Box<dyn Error>> {
    let config_path = config_path
        .map(PathBuf::from)
        .unwrap_or(get_default_config_path(
            var("ODICT_CONFIG_DIR").ok().as_deref(),
        )?);

    if !config_path.exists() {
        fs::write(&config_path, "{}")?;
    }

    let config = read_to_string(&config_path)?;
    let config: HashMap<String, String> = serde_json::from_str(&config)?;

    Ok(ODictConfig {
        path: config_path,
        aliases: config,
    })
}

#[cfg(test)]
mod test {
    use dirs::home_dir;
    use std::path::PathBuf;

    use super::get_config_dir;

    #[test]
    fn test_get_config_dir_custom() {
        let config_dir = get_config_dir(Some(".odict")).unwrap();
        let actual = config_dir.to_str().unwrap();

        assert_eq!(actual, ".odict");
    }

    #[test]
    fn test_get_config_dir() {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(home_dir);

        config_dir.push(".odict");

        let expected = config_dir.to_string_lossy().to_string();
        let config_dir = get_config_dir(None).unwrap();
        let actual = config_dir.to_str().unwrap();

        assert_eq!(actual, expected);
    }
}
