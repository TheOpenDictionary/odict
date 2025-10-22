mod helpers;

#[cfg(test)]
mod alias_tests {
    use odict::alias::AliasManager;
    use tempfile::TempDir;

    use crate::helpers::get_example_dict;

    /// Test that custom config directory is properly passed to alias manager by creating an alias
    #[test]
    fn test_custom_config_dir_creates_alias_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().to_path_buf();

        // Create alias manager with custom config directory
        let mut alias_manager = AliasManager::new(&config_path);

        // Create a test dictionary and set an alias
        let dict = get_example_dict("example1").unwrap();

        // Set an alias - this should write to our custom config directory
        alias_manager.set("test_alias", &dict).unwrap();

        // Verify the alias file was created in our custom config directory
        let alias_file = config_path.join("aliases.json");

        assert!(
            alias_file.exists(),
            "Alias file should be created in custom config directory"
        );
    }

    /// Test alias manager with nested custom directory structure
    #[test]
    fn test_nested_custom_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("custom").join("nested").join("config");

        // AliasManager should create the directory structure if it doesn't exist
        let mut alias_manager = AliasManager::new(&nested_path);
        let dict = get_example_dict("example1").unwrap();

        // This should create the nested directory structure
        alias_manager.set("nested_test", &dict).unwrap();

        // Verify the nested directory was created and file exists
        let alias_file = nested_path.join("aliases.json");

        assert!(
            alias_file.exists(),
            "Should create nested directory structure and alias file"
        );
    }
}
