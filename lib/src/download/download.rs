use std::{path::PathBuf, time::SystemTime};

use crate::{
    config::get_config_dir,
    download::{
        metadata::{get_metadata, set_metadata, DictionaryMetadata},
        options::{DownloadOptions, ProgressCallback},
        utils::parse_remote_dictionary_name,
    },
    Error,
};

use futures_util::StreamExt;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct DictionaryDownloader {
    base_url: String,
    client: Client,
}

impl AsRef<DictionaryDownloader> for DictionaryDownloader {
    fn as_ref(&self) -> &DictionaryDownloader {
        self
    }
}

impl Default for DictionaryDownloader {
    fn default() -> Self {
        Self {
            base_url: "https://dictionaries.odict.org".to_string(),
            client: Client::builder()
                .user_agent("odict/2.9.1")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
}

impl DictionaryDownloader {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    pub fn with_client<F>(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    pub async fn download(&self, dictionary_name: &str) -> crate::Result<PathBuf> {
        self.download_with_options(dictionary_name, &DownloadOptions::default())
            .await
    }

    pub async fn download_with_options<'a, Options: AsRef<DownloadOptions<'a>>>(
        &self,
        dictionary_name: &str,
        options: Options,
    ) -> crate::Result<PathBuf> {
        let (dictionary, language) = parse_remote_dictionary_name(dictionary_name)?;

        let opts = options.as_ref();

        let out_dir = match opts.out_dir {
            Some(ref dir) => dir.clone(),
            None => get_config_dir()?.join("dictionaries").join(&dictionary),
        };

        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir).map_err(crate::error::Error::Io)?;
        }

        let url = format!("{}/{}/{}.odict", self.base_url, dictionary, language);
        let out_path = out_dir.join(format!("{language}.odict"));

        let etag = if opts.caching {
            get_metadata(&out_path)?.map(|meta| meta.etag)
        } else {
            None
        };

        let (bytes, new_etag) = self
            .fetch_with_etag(&url, etag.as_deref(), opts.on_progress.as_ref())
            .await?;

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

        match bytes.is_empty() {
            true => {
                // File already exists from cache, no need to write
            }
            false => {
                std::fs::write(&out_path, &bytes)?;
            }
        };

        Ok(out_path)
    }

    async fn fetch_with_etag<'a>(
        &self,
        url: &str,
        etag: Option<&str>,
        on_progress: Option<&ProgressCallback<'a>>,
    ) -> crate::Result<(Vec<u8>, Option<String>)> {
        let client = &self.client;
        let mut request = client.get(url);

        if let Some(tag) = etag {
            request = request.header("If-None-Match", tag);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch from {url}: {e}")))?;

        match response.status().as_u16() {
            304 => {
                // Not Modified - return empty bytes (caller should handle this case)
                Ok((Vec::new(), None))
            }
            200 => {
                // Success - extract etag and bytes
                let etag = super::utils::extract_etag(&response);
                let content_length = response.content_length();

                let mut bytes = Vec::new();
                let mut downloaded = 0u64;
                let mut stream = response.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk
                        .map_err(|e| Error::Other(format!("Failed to read response chunk: {e}")))?;

                    bytes.extend_from_slice(&chunk);
                    downloaded += chunk.len() as u64;

                    // Call progress callback if provided
                    if let Some(callback) = on_progress {
                        let progress = if let Some(total) = content_length {
                            downloaded as f64 / total as f64
                        } else {
                            0.0 // Unknown total size
                        };
                        callback(downloaded, content_length, progress);
                    }
                }

                Ok((bytes, etag))
            }
            status => Err(Error::Other(format!(
                "Unexpected response status from {url}: {status}"
            ))),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::download::options::DownloadOptions;

    use super::*;

    use tempfile::TempDir;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_downloader(base_url: String) -> DictionaryDownloader {
        DictionaryDownloader::new(reqwest::Client::new(), base_url)
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
            .await
            .unwrap();

        let output_file = temp_dir.path().join("eng.odict");

        assert_eq!(result, output_file);
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
        let options = DownloadOptions::default()
            .caching(true)
            .out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/eng", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap();
        let output_file = temp_dir.path().join("eng.odict");
        assert_eq!(result_path, output_file);
        assert_eq!(fs::read(output_file).unwrap(), test_data);
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
        let options = DownloadOptions::default()
            .caching(true)
            .out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/de", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap();
        let output_file = temp_dir.path().join("de.odict");
        assert_eq!(result_path, output_file);
        assert_eq!(fs::read(output_file).unwrap(), test_data);
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
        let options = DownloadOptions::default()
            .caching(true)
            .out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/es", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap();
        let output_file = temp_dir.path().join("es.odict");
        assert_eq!(result_path, output_file);
        assert_eq!(fs::read(output_file).unwrap(), test_data);
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
        let downloader = DictionaryDownloader::default();
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
        let downloader = DictionaryDownloader::default();
        assert_eq!(downloader.base_url, "https://dictionaries.odict.org");
    }

    #[tokio::test]
    async fn test_downloader_new() {
        let custom_url = "https://custom.example.com";
        let client = reqwest::Client::new();
        let downloader = DictionaryDownloader::new(client, custom_url.to_string());
        assert_eq!(downloader.base_url, custom_url);
    }

    #[tokio::test]
    async fn test_download_with_progress_callback() {
        let mock_server = MockServer::start().await;
        let test_data = b"test data with progress tracking";

        Mock::given(method("GET"))
            .and(path("/wiktionary/progress.odict"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(test_data)
                    .insert_header("content-length", test_data.len().to_string()),
            )
            .mount(&mock_server)
            .await;

        let downloader = create_test_downloader(mock_server.uri());
        let temp_dir = TempDir::new().unwrap();

        // Track progress calls
        use std::sync::{Arc, Mutex};
        let progress_calls = Arc::new(Mutex::new(Vec::new()));
        let progress_calls_clone = progress_calls.clone();

        let options = DownloadOptions::default()
            .out_dir(temp_dir.path())
            .on_progress(move |downloaded, total, progress| {
                let mut calls = progress_calls_clone.lock().unwrap();
                calls.push((downloaded, total, progress));
            });

        let result = downloader
            .download_with_options("wiktionary/progress", &options)
            .await
            .unwrap();

        let expected_path = temp_dir.path().join("progress.odict");
        assert_eq!(result, expected_path);
        assert!(expected_path.exists());
        assert_eq!(fs::read(&expected_path).unwrap(), test_data);

        // Verify progress was tracked
        let calls = progress_calls.lock().unwrap();
        assert!(
            !calls.is_empty(),
            "Progress callback should have been called"
        );

        // Check the final call
        let final_call = calls.last().unwrap();
        assert_eq!(final_call.0, test_data.len() as u64); // downloaded bytes
        assert_eq!(final_call.1, Some(test_data.len() as u64)); // total bytes
        assert_eq!(final_call.2, 1.0); // progress should be 100%
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
        let result = DictionaryDownloader::default()
            .fetch_with_etag(&url, None, None)
            .await;

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
        let result = DictionaryDownloader::default()
            .fetch_with_etag(&url, Some("\"existing-etag\""), None)
            .await;

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
        let result = DictionaryDownloader::default()
            .fetch_with_etag(&url, None, None)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Other(msg) => assert!(msg.contains("Unexpected response status")),
            _ => panic!("Expected Error::Other"),
        }
    }
}
