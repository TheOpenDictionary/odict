use std::{path::Path, time::SystemTime};

use crate::{
    download::{
        client::get_client,
        metadata::{get_metadata, set_metadata, DictionaryMetadata},
        parse_dictionary_name,
    },
    Error,
};

pub struct DownloadOptions {
    caching: bool,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self { caching: true }
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

    pub async fn download<P: AsRef<Path>>(
        &self,
        dictionary_name: &str,
        out_dir: P,
    ) -> crate::Result<Vec<u8>> {
        self.download_with_options(dictionary_name, out_dir, &DownloadOptions::default())
            .await
    }

    pub async fn download_with_options<P: AsRef<Path>>(
        &self,
        dictionary_name: &str,
        out_dir: P,
        options: &DownloadOptions,
    ) -> crate::Result<Vec<u8>> {
        let (dictionary, language) = parse_dictionary_name(dictionary_name)?;

        let out_path = out_dir.as_ref().join(format!("{}.odict", language));

        let url = format!("{}/{}.odict", self.base_url, dictionary);

        let etag = if options.caching {
            get_metadata(&out_path)?.map(|meta| meta.etag)
        } else {
            None
        };

        let (bytes, new_etag) = Self::fetch_with_etag(&url, etag.as_deref()).await?;

        if let Some(etag) = new_etag {
            if options.caching {
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
