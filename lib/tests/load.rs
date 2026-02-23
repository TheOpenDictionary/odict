mod helpers;

#[cfg(test)]
mod load_tests {
    use tempfile::TempDir;

    use crate::helpers::get_example_dict;
    use odict::{alias::AliasManager, download::DictionaryDownloader, LoadOptions, OpenDictionary};

    /// Test that LoadOptions with custom config dir properly configures alias manager
    #[tokio::test]
    async fn test_load_options_with_custom_config_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().to_path_buf();

        let mut alias_manager = AliasManager::new(&config_path);
        let dict = get_example_dict("example1").unwrap();

        alias_manager.set("test_dict", &dict).unwrap();

        let alias_file = config_path.join("aliases.json");

        assert!(
            alias_file.exists(),
            "Alias file should exist in custom config directory"
        );

        let load_options = LoadOptions::default().with_config_dir(&config_path);

        let result = OpenDictionary::load_with_options("test_dict", load_options).await;

        assert!(
            result.is_ok(),
            "Should successfully load from alias in custom config directory"
        );
    }

    #[tokio::test]
    async fn test_combined_custom_paths() {
        let config_temp_dir = TempDir::new().unwrap();
        let download_temp_dir = TempDir::new().unwrap();

        let config_path = config_temp_dir.path().to_path_buf();
        let download_path = download_temp_dir.path().to_path_buf();

        let mut alias_manager = AliasManager::new(&config_path);
        let dict = get_example_dict("example1").unwrap();

        alias_manager.set("combined_test", &dict).unwrap();

        let alias_file = config_path.join("aliases.json");

        assert!(
            alias_file.exists(),
            "Alias should be in custom config directory"
        );

        let downloader = DictionaryDownloader::default().with_out_dir(&download_path);

        let load_options = LoadOptions::default()
            .with_config_dir(&config_path)
            .with_alias_manager(AliasManager::new(&config_path))
            .with_downloader(downloader);

        let result = OpenDictionary::load_with_options("combined_test", load_options).await;

        assert!(
            result.is_ok(),
            "Should load from alias with combined custom paths"
        );
    }
}
