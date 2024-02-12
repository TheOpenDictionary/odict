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

pub(super) fn get_config_dir<'a>() -> Result<&'a Path, Box<dyn Error>> {
    let config_dir = var("ODICT_CONFIG_DIR").unwrap_or_else(|_| {
        let home_dir = home_dir().expect("Failed to get home directory");
        let mut config_dir = PathBuf::from(home_dir);

        config_dir.push(".odict");

        return config_dir.to_string_lossy().to_string();
    });

    let path = Path::new(&config_dir);

    if !path.exists() {
        create_dir(&path)?;
    }

    Ok(path)
}

pub(super) fn get_default_config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(&get_config_dir()?).join("config.json"))
}

pub(super) fn get_config<S: AsRef<OsStr> + ?Sized>(
    config_path: Option<&S>,
) -> Result<ODictConfig, Box<dyn Error>> {
    let config_path = config_path
        .map(PathBuf::from)
        .unwrap_or(get_default_config_path()?);

    if !config_path.exists() {
        fs::write(&config_path, "")?;
    }

    let config = read_to_string(&config_path)?;
    let config: HashMap<String, String> = serde_json::from_str(&config)?;

    Ok(ODictConfig {
        path: config_path,
        aliases: config,
    })
}
