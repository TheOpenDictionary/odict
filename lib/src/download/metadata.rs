use std::path::Path;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Metadata for tracking etags and file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryMetadata {
    /// The etag value from the server
    pub etag: String,
    /// Last modified timestamp
    pub last_modified: SystemTime,
    /// URL that was downloaded
    pub url: String,
}

const METADATA_EXTENSION: &str = "meta.json";

pub fn get_metadata<P: AsRef<Path>>(local_path: P) -> Result<Option<DictionaryMetadata>> {
    let metadata = local_path.as_ref().with_extension(METADATA_EXTENSION);

    if !metadata.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&metadata)
        .map_err(|e| Error::Other(format!("Failed to read metadata json: {e}")))?;

    serde_json::from_str(&content)
        .map_err(|e| Error::Other(format!("Failed to parse metadata json: {e}")))
}

/// Save etag cache to disk
pub fn set_metadata<P: AsRef<Path>>(local_path: P, metadata: DictionaryMetadata) -> Result<()> {
    let metadata_path = local_path.as_ref().with_extension(METADATA_EXTENSION);

    let content = serde_json::to_string_pretty(&metadata)
        .map_err(|e| Error::Other(format!("Failed to serialize metadata json: {e}")))?;

    std::fs::write(&metadata_path, content)
        .map_err(|e| Error::Other(format!("Failed to write metadata json: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_metadata_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.odict");

        let result = get_metadata(&file_path).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_set_and_get_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.odict");

        let metadata = DictionaryMetadata {
            etag: "test-etag".to_string(),
            last_modified: SystemTime::now(),
            url: "https://example.com/test".to_string(),
        };

        set_metadata(&file_path, metadata.clone()).unwrap();

        // Verify metadata file was created
        let metadata_path = file_path.with_extension(METADATA_EXTENSION);
        assert!(metadata_path.exists());

        // Verify we can read back the metadata
        let loaded_metadata = get_metadata(&file_path).unwrap().unwrap();

        assert_eq!(loaded_metadata.etag, metadata.etag);
        assert_eq!(loaded_metadata.url, metadata.url);
    }

    #[test]
    fn test_get_metadata_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.odict");
        let metadata_path = file_path.with_extension(METADATA_EXTENSION);

        let metadata = DictionaryMetadata {
            etag: "existing-etag".to_string(),
            last_modified: SystemTime::now(),
            url: "https://example.com/existing".to_string(),
        };

        let content = serde_json::to_string_pretty(&metadata).unwrap();
        fs::write(&metadata_path, content).unwrap();

        let loaded_metadata = get_metadata(&file_path).unwrap().unwrap();
        assert_eq!(loaded_metadata.etag, "existing-etag");
        assert_eq!(loaded_metadata.url, "https://example.com/existing");
    }

    #[test]
    fn test_get_metadata_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.odict");
        let metadata_path = file_path.with_extension(METADATA_EXTENSION);

        fs::write(&metadata_path, "invalid json").unwrap();

        let result = get_metadata(&file_path);
        assert!(result.is_err());
    }
}
