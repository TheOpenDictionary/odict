use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{
    config::get_config_dir,
    download::{
        client::get_client,
        metadata::{get_metadata, set_metadata, DictionaryMetadata},
        parse_dictionary_name,
    },
    Error,
};

#[derive(Clone)]
pub struct DownloadOptions {
    caching: bool,
    out_dir: Option<PathBuf>,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            caching: true,
            out_dir: None,
        }
    }
}

impl DownloadOptions {
    pub fn caching(&mut self, value: bool) -> &Self {
        self.caching = value;
        self
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }
}

impl AsRef<DownloadOptions> for DownloadOptions {
    fn as_ref(&self) -> &DownloadOptions {
        self
    }
}

pub struct Downloader {
    base_url: String,
}

impl Default for Downloader {
    fn default() -> Self {
        Self {
            base_url: "https://pub-520e4751d2374bc5bc14265c6e02e06e.r2.dev".to_string(),
        }
    }
}

impl Downloader {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn download(&self, dictionary_name: &str) -> crate::Result<Vec<u8>> {
        self.download_with_options(dictionary_name, &DownloadOptions::default())
            .await
    }

    pub async fn download_with_options<Options: AsRef<DownloadOptions>>(
        &self,
        dictionary_name: &str,
        options: Options,
    ) -> crate::Result<Vec<u8>> {
        let (dictionary, language) = parse_dictionary_name(dictionary_name)?;

        let opts = options.as_ref();

        let out_dir = match opts.out_dir {
            Some(ref dir) => dir.clone(),
            None => get_config_dir()?.join("dictionaries").join(&dictionary),
        };

        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir).map_err(crate::error::Error::Io)?;
        }

        let url = format!("{}/{}/{}.odict", self.base_url, dictionary, language);
        let out_path = out_dir.join(format!("{}.odict", language));

        let etag = if opts.caching {
            get_metadata(&out_path)?.map(|meta| meta.etag)
        } else {
            None
        };

        let (bytes, new_etag) = Self::fetch_with_etag(&url, etag.as_deref()).await?;

        if let Some(etag) = new_etag {
            if opts.caching {
                set_metadata(
                    &out_path,
                    DictionaryMetadata {
                        etag,
                        last_modified: SystemTime::now(),
                        url: url.clone(),
                    },
                )?;
            }
        }

        if !bytes.is_empty() {
            std::fs::write(&out_path, &bytes)?;
            return Ok(bytes.clone());
        } else {
            return Ok(std::fs::read(&out_path)?);
        }
    }

    async fn fetch_with_etag(
        url: &str,
        etag: Option<&str>,
    ) -> crate::Result<(Vec<u8>, Option<String>)> {
        let client = get_client();
        let mut request = client.get(url);

        if let Some(tag) = etag {
            request = request.header("If-None-Match", tag);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch from {}: {}", url, e)))?;

        match response.status().as_u16() {
            304 => {
                // Not Modified - return empty bytes (caller should handle this case)
                Ok((Vec::new(), None))
            }
            200 => {
                // Success - extract etag and bytes
                let etag = super::extract_etag(&response);

                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| Error::Other(format!("Failed to read response body: {}", e)))?
                    .to_vec();

                Ok((bytes, etag))
            }
            status => Err(Error::Other(format!(
                "Unexpected response status from {}: {}",
                url, status
            ))),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    use tempfile::TempDir;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_downloader(base_url: String) -> Downloader {
        Downloader::new(base_url)
    }

    #[tokio::test]
    async fn test_download_success() {
        let mock_server = MockServer::start().await;
        let test_data = b"test dictionary data";

        Mock::given(method("GET"))
            .and(path("/wiktionary/eng.odict"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(test_data))
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let temp_dir = TempDir::new().unwrap();

        let result = downloader
            .download_with_options(
                "wiktionary/eng",
                DownloadOptions::default().out_dir(temp_dir.path()),
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);

        let output_file = temp_dir.path().join("eng.odict");
        assert!(output_file.exists());
        assert_eq!(fs::read(output_file).unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_with_caching_enabled() {
        let mock_server = MockServer::start().await;
        let test_data = b"cached dictionary data";

        Mock::given(method("GET"))
            .and(path("/wiktionary/eng.odict"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(test_data)
                    .insert_header("etag", "\"test-etag-123\""),
            )
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let temp_dir = TempDir::new().unwrap();
        let options = DownloadOptions {
            caching: true,
            out_dir: Some(temp_dir.path().to_path_buf()),
        };

        let result = downloader
            .download_with_options("wiktionary/eng", &options)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_with_caching_disabled() {
        let mock_server = MockServer::start().await;
        let test_data = b"non-cached dictionary data";

        Mock::given(method("GET"))
            .and(path("/wiktionary/de.odict"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(test_data))
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let temp_dir = TempDir::new().unwrap();
        let options = DownloadOptions {
            caching: false,
            out_dir: Some(temp_dir.path().to_path_buf()),
        };

        let result = downloader
            .download_with_options("wiktionary/de", &options)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_with_etag_not_modified() {
        let mock_server = MockServer::start().await;
        let test_data = b"existing data";
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("es.odict");

        // Pre-populate the file
        fs::create_dir_all(output_file.parent().unwrap()).unwrap();
        fs::write(&output_file, test_data).unwrap();

        // Set up metadata with etag
        set_metadata(
            &output_file,
            DictionaryMetadata {
                etag: "\"existing-etag\"".to_string(),
                last_modified: SystemTime::now(),
                url: format!("{}/etag-dict.odict", mock_server.uri()),
            },
        )
        .unwrap();

        Mock::given(method("GET"))
            .and(path("/wiktionary/es.odict"))
            .and(header("If-None-Match", "\"existing-etag\""))
            .respond_with(ResponseTemplate::new(304))
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let options = DownloadOptions {
            caching: true,
            out_dir: Some(temp_dir.path().to_path_buf()),
        };

        let result = downloader
            .download_with_options("wiktionary/es", &options)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/wiktionary/err.odict"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let result = downloader.download("wiktionary/err").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Other(msg) => assert!(msg.contains("Unexpected response status")),
            _ => panic!("Expected Error::Other"),
        }
    }

    #[tokio::test]
    async fn test_download_invalid_dictionary_name() {
        let downloader = Downloader::default();
        let temp_dir = TempDir::new().unwrap();

        let result = downloader
            .download_with_options(
                "invalid-name",
                DownloadOptions::default().out_dir(temp_dir.path()),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_creates_directory() {
        let mock_server = MockServer::start().await;
        let test_data = b"test data for new directory";

        Mock::given(method("GET"))
            .and(path("/wiktionary/ger.odict"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(test_data))
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("path");

        let result = downloader
            .download_with_options(
                "wiktionary/ger",
                DownloadOptions::default().out_dir(&nested_dir),
            )
            .await;

        assert!(result.is_ok());
        assert!(nested_dir.join("ger.odict").exists());
    }

    #[tokio::test]
    async fn test_downloader_default() {
        let downloader = Downloader::default();
        assert_eq!(
            downloader.base_url,
            "https://pub-520e4751d2374bc5bc14265c6e02e06e.r2.dev"
        );
    }

    #[tokio::test]
    async fn test_downloader_new() {
        let custom_url = "https://custom.example.com";
        let downloader = Downloader::new(custom_url.to_string());
        assert_eq!(downloader.base_url, custom_url);
    }

    #[test]
    fn test_download_options_default() {
        let options = DownloadOptions::default();
        assert!(options.caching);
    }

    #[tokio::test]
    async fn test_fetch_with_etag_success() {
        let mock_server = MockServer::start().await;
        let test_data = b"fetch test data";

        Mock::given(method("GET"))
            .and(path("/fetch-test.odict"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(test_data)
                    .insert_header("etag", "\"fetch-etag\""),
            )
            .mount(&mock_server)
            .await;

        let url = format!("{}/fetch-test.odict", mock_server.uri());
        let result = Downloader::fetch_with_etag(&url, None).await;

        assert!(result.is_ok());
        let (bytes, etag) = result.unwrap();
        assert_eq!(bytes, test_data);
        assert_eq!(etag, Some("fetch-etag".to_string()));
    }

    #[tokio::test]
    async fn test_fetch_with_etag_not_modified() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/not-modified.odict"))
            .and(header("If-None-Match", "\"existing-etag\""))
            .respond_with(ResponseTemplate::new(304))
            .mount(&mock_server)
            .await;

        let url = format!("{}/not-modified.odict", mock_server.uri());
        let result = Downloader::fetch_with_etag(&url, Some("\"existing-etag\"")).await;

        assert!(result.is_ok());
        let (bytes, etag) = result.unwrap();
        assert!(bytes.is_empty());
        assert!(etag.is_none());
    }

    #[tokio::test]
    async fn test_fetch_with_etag_error_status() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error.odict"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let url = format!("{}/error.odict", mock_server.uri());
        let result = Downloader::fetch_with_etag(&url, None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Other(msg) => assert!(msg.contains("Unexpected response status")),
            _ => panic!("Expected Error::Other"),
        }
    }
}
