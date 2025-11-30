use std::{path::Path, time::SystemTime};

use crate::{
    config::DEFAULT_CONFIG_DIR,
    download::{
        constants::{DEFAULT_BASE_URL, DEFAULT_DICTIONARIES_DIR},
        metadata::{delete_metadata, get_metadata, set_metadata, DictionaryMetadata},
        options::{DownloadOptions, ProgressCallback},
        utils::parse_remote_dictionary_name,
    },
    Error, OpenDictionary,
};

use futures_util::StreamExt;
use reqwest::Client;
use reqwest_middleware::ClientWithMiddleware;

#[derive(Debug, Clone)]
pub struct DictionaryDownloader<'a> {
    base_url: String,
    client: ClientWithMiddleware,
    pub(crate) options: DownloadOptions<'a>,
}

impl<'a> AsRef<DictionaryDownloader<'a>> for DictionaryDownloader<'a> {
    fn as_ref(&self) -> &DictionaryDownloader<'a> {
        self
    }
}

impl<'a> DictionaryDownloader<'a> {
    fn build_client_with_retries(retries: u32) -> ClientWithMiddleware {
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(retries);

        let retry = reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy);

        let base_client = reqwest::Client::builder()
            .user_agent("ODICT/3")
            .build()
            .expect("Failed to create HTTP client");

        reqwest_middleware::ClientBuilder::new(base_client)
            .with(retry)
            .build()
    }
}

impl<'a> Default for DictionaryDownloader<'a> {
    fn default() -> Self {
        let options = DownloadOptions::default();
        let client = Self::build_client_with_retries(options.retries);

        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            client,
            options,
        }
    }
}

impl<'a> DictionaryDownloader<'a> {
    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client.into();
        self
    }

    pub fn with_base_url<U: AsRef<str>>(mut self, url: U) -> Self {
        self.base_url = url.as_ref().into();
        self
    }

    pub fn with_options(mut self, options: DownloadOptions<'a>) -> Self {
        let retries_changed = self.options.retries != options.retries;

        self.options = options;

        // Rebuild client if retries changed
        if retries_changed {
            self.client = Self::build_client_with_retries(self.options.retries);
        }

        self
    }

    pub fn with_caching(mut self, caching: bool) -> Self {
        self.options = self.options.with_caching(caching);
        self
    }

    pub fn with_out_dir<P: AsRef<Path>>(mut self, out_dir: P) -> Self {
        self.options = self.options.with_out_dir(out_dir);
        self
    }

    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(u64, Option<u64>, f64) + Send + Sync + 'a,
    {
        self.options = self.options.on_progress(callback);
        self
    }

    pub fn with_client_middleware<F>(mut self, client: ClientWithMiddleware) -> Self {
        self.client = client;
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.options = self.options.with_retries(retries);
        // Rebuild client with new retry policy
        self.client = Self::build_client_with_retries(retries);
        self
    }

    pub async fn download(&self, dictionary_name: &str) -> crate::Result<OpenDictionary> {
        self.download_with_options(dictionary_name, &self.options)
            .await
    }

    fn merge_download_options<Options: AsRef<DownloadOptions<'a>>>(
        &self,
        options: Options,
    ) -> DownloadOptions<'a> {
        let mut opts = self.options.clone();
        let override_opts = options.as_ref();

        if opts.caching != override_opts.caching {
            opts.caching = override_opts.caching;
        }

        if let Some(dir) = &override_opts.config_dir {
            opts.config_dir = Some(dir.clone());
        }

        if let Some(dir) = &override_opts.out_dir {
            opts.out_dir = Some(dir.clone())
        }

        if let Some(cb) = override_opts.on_progress.as_ref() {
            opts.on_progress = Some(cb.clone());
        }

        if opts.retries != override_opts.retries {
            opts.retries = override_opts.retries;
        }

        opts
    }

    async fn get_dictionary(
        &self,
        url: &str,
        out_path: &Path,
        opts: &DownloadOptions<'a>,
    ) -> crate::Result<OpenDictionary> {
        let etag = match opts.caching {
            true => get_metadata(&out_path)?.map(|meta| meta.etag),
            false => None,
        };

        let (bytes, new_etag) = self
            .fetch_with_etag(&url, etag.as_deref(), opts.on_progress.as_ref())
            .await?;

        // If the upstream was modified
        if !bytes.is_empty() {
            if let Some(etag) = new_etag {
                if opts.caching {
                    set_metadata(
                        &out_path,
                        DictionaryMetadata {
                            etag,
                            last_modified: SystemTime::now(),
                            url: url.into(),
                        },
                    )?;
                }
            }

            std::fs::write(&out_path, &bytes)?;
        }

        OpenDictionary::from_path(&out_path)
    }

    pub async fn download_with_options<Options: AsRef<DownloadOptions<'a>>>(
        &self,
        dictionary_name: &str,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        let (dictionary, language) = parse_remote_dictionary_name(dictionary_name)?;

        let opts = self.merge_download_options(&options);

        let out_dir = match opts.clone().out_dir {
            Some(dir) => dir,
            None => opts
                .clone()
                .config_dir
                .unwrap_or(DEFAULT_CONFIG_DIR.to_path_buf())
                .join(DEFAULT_DICTIONARIES_DIR),
        };

        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir).map_err(crate::error::Error::Io)?;
        }

        let mut retries_remaining = opts.retries;
        let url = format!("{}/{}/{}.odict", self.base_url, dictionary, language);
        let out_path = out_dir.join(format!("{language}.odict"));

        if !out_path.exists() {
            // Don't have a metadata file if the dictionary doesn't exist
            // This should handle cases where a dictionary is deleted but its metadata isn't
            delete_metadata(&out_path)?;
        }

        loop {
            let open_dictionary = self.get_dictionary(&url, &out_path, &opts).await;

            // Attempt to load the file to verify it's not corrupted
            match open_dictionary {
                Ok(dict) => {
                    // File is valid, return success
                    return Ok(dict);
                }
                Err(e) => {
                    // Check if this is a corruption error that we should retry
                    let is_corrupted = matches!(
                        &e,
                        Error::InvalidBuffer(_)
                            | Error::InvalidSignature
                            | Error::Incompatible(_, _)
                            | Error::Deserialize(_)
                            | Error::Io(_)
                    );

                    if is_corrupted && retries_remaining > 0 {
                        // Delete corrupted file and metadata to force fresh download
                        if out_path.exists() {
                            std::fs::remove_file(&out_path)?;
                        }

                        delete_metadata(&out_path)?;

                        retries_remaining -= 1;

                        // Continue loop to retry
                        continue;
                    } else {
                        // Either not a corruption error or no retries left
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn fetch_with_etag(
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
            .map_err(|e| super::utils::classify_reqwest_error(url, &e))?;

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
                    let chunk = chunk.map_err(|e| {
                        Error::DownloadFailed(
                            super::utils::NetworkError::Body,
                            format!("Failed to read response chunk: {e}"),
                        )
                    })?;

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
            status => Err(Error::DownloadFailed(
                super::utils::NetworkError::Http(status),
                format!("Unexpected response status from {url}: {status}"),
            )),
        }
    }
}

impl OpenDictionary {
    pub async fn download(dictionary_name: &str) -> Result<OpenDictionary, Error> {
        DictionaryDownloader::default()
            .download(dictionary_name)
            .await
    }

    pub async fn download_with_options<'a, Options: AsRef<DownloadOptions<'a>>>(
        dictionary_name: &str,
        options: Options,
    ) -> Result<OpenDictionary, Error> {
        DictionaryDownloader::default()
            .download_with_options(dictionary_name, options)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::download::options::DownloadOptions;
    use crate::download::NetworkError;

    use super::*;

    use tempfile::TempDir;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_downloader<'a>(base_url: String) -> DictionaryDownloader<'a> {
        DictionaryDownloader::default()
            .with_client(reqwest::Client::new().into())
            .with_base_url(base_url)
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
                DownloadOptions::default().with_out_dir(temp_dir.path()),
            )
            .await
            .unwrap();

        let output_file = temp_dir.path().join("eng.odict");

        assert_eq!(result.path.unwrap(), output_file);
        assert!(output_file.exists());
        assert_eq!(fs::read(output_file).unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_caching_enabled() {
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
            .with_caching(true)
            .with_out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/eng", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap().path.unwrap();
        let output_file = temp_dir.path().join("eng.odict");
        assert_eq!(result_path, output_file);
        assert_eq!(fs::read(output_file).unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_download_caching_disabled() {
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
            .with_caching(true)
            .with_out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/de", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap().path.unwrap();
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
            .with_caching(true)
            .with_out_dir(temp_dir.path());

        let result = downloader
            .download_with_options("wiktionary/es", &options)
            .await;

        assert!(result.is_ok());
        let result_path = result.unwrap().path.unwrap();
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
            Error::DownloadFailed(NetworkError::Http(status), _) => assert_eq!(status, 500),
            v => panic!("Expected NetworkError::Http, Received: {:?}", v),
        }
    }

    #[tokio::test]
    async fn test_download_invalid_dictionary_name() {
        let downloader = DictionaryDownloader::default();
        let temp_dir = TempDir::new().unwrap();

        let result = downloader
            .download_with_options(
                "invalid-name",
                DownloadOptions::default().with_out_dir(temp_dir.path()),
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
                DownloadOptions::default().with_out_dir(&nested_dir),
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
        let downloader = DictionaryDownloader::default()
            .with_client(client)
            .with_base_url(custom_url.to_string());
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
            .with_out_dir(temp_dir.path())
            .on_progress(move |downloaded, total, progress| {
                let mut calls = progress_calls_clone.lock().unwrap();
                calls.push((downloaded, total, progress));
            });

        let result = downloader
            .download_with_options("wiktionary/progress", &options)
            .await
            .unwrap();

        let expected_path = temp_dir.path().join("progress.odict");
        assert_eq!(result.path().unwrap(), expected_path);
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
            Error::DownloadFailed(NetworkError::Http(status), _) => assert!(status == 404),
            v => panic!("Expected Error::Http, Received: {:?}", v),
        }
    }
}
