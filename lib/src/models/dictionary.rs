use rkyv::{
    deserialize, to_bytes,
    with::{AsBox, MapNiche},
};
use std::collections::HashSet;
use std::str::FromStr;

use crate::{error::Error, serializable};

use super::{entry::Entry, id::ID};

serializable! {
  #[derive(Default)]
  #[serde(rename = "dictionary")]
  pub struct Dictionary {
      #[serde(default, rename = "@id")]
      pub id: ID,

      #[serde(rename = "@name")]
      #[rkyv(with = MapNiche<AsBox>)]
      #[serde(skip_serializing_if = "Option::is_none")]
      pub name: Option<String>,

      #[serde(default, rename = "entry", with = "entries")]
      pub entries: HashSet<Entry>,
  }
}

mod entries {
    use std::collections::HashSet;

    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use crate::models::Entry;

    pub fn serialize<S>(set: &HashSet<Entry>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(set.iter())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Entry>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut set = HashSet::new();

        for item in Vec::<Entry>::deserialize(deserializer)? {
            set.insert(item);
        }

        Ok(set)
    }
}

impl Dictionary {
    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let bytes =
            to_bytes::<rkyv::rancor::Error>(self).map_err(|e| Error::Serialize(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    /// Downloads and loads a dictionary from the ODict repository
    ///
    /// # Arguments
    ///
    /// * `dict_lang` - Dictionary and language string in format "dictionary/language" (e.g., "wiktionary/eng")
    ///
    /// # Returns
    ///
    /// Returns a loaded Dictionary on success
    ///
    /// # Example
    ///
    /// ```rust
    /// use odict::Dictionary;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let dict = Dictionary::load("wiktionary/eng").await?;
    /// println!("Loaded dictionary with {} entries", dict.entries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "download", feature = "config"))]
    pub async fn load(dict_lang: &str) -> crate::Result<Dictionary> {
        Self::load_with_options(dict_lang, &crate::download::DownloadOptions::default()).await
    }

    /// Downloads and loads a dictionary from the ODict repository with custom options
    ///
    /// # Arguments
    ///
    /// * `dict_lang` - Dictionary and language string in format "dictionary/language" (e.g., "wiktionary/eng")
    /// * `options` - Download options (caching, custom base URL, etc.)
    ///
    /// # Returns
    ///
    /// Returns a loaded Dictionary on success
    ///
    /// # Example
    ///
    /// ```rust
    /// use odict::{Dictionary, download::DownloadOptions};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load with caching (default)
    /// let dict = Dictionary::load_with_options("wiktionary/eng", &DownloadOptions::new()).await?;
    ///
    /// // Force download, bypassing cache
    /// let dict = Dictionary::load_with_options("wiktionary/eng", &DownloadOptions::new().force(true)).await?;
    ///
    /// println!("Loaded dictionary with {} entries", dict.entries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "download", feature = "config"))]
    pub async fn load_with_options(
        dict_lang: &str,
        options: &crate::download::DownloadOptions,
    ) -> crate::Result<Dictionary> {
        // Get config directory and create dictionaries subdirectory

        use crate::{download::Downloader, DictionaryReader};
        let config_dir = crate::config::get_config_dir()?;
        let dictionaries_dir = config_dir.join("dictionaries");

        // Parse dict_lang to get dictionary name for directory structure
        let parts: Vec<&str> = dict_lang.split('/').collect();

        if parts.len() != 2 {
            return Err(Error::Other(format!(
                "Invalid dictionary/language format '{}'. Expected format: 'dictionary/language'",
                dict_lang
            )));
        }

        let dictionary = parts[0];

        // Create the full path preserving dictionary/language structure
        let dict_dir = dictionaries_dir.join(dictionary);

        // Download the dictionary file with the specified options
        let content = Downloader::default()
            .download_with_options(dict_lang, &dict_dir, options)
            .await?;

        // Parse and return the dictionary
        DictionaryReader::default()
            .read_from_bytes(&content)?
            .to_dictionary()
    }
}

impl FromStr for Dictionary {
    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(xml).map_err(|e| crate::Error::Deserialize(e.to_string()))
    }

    type Err = crate::Error;
}

impl ArchivedDictionary {
    pub fn to_dictionary(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = deserialize::<Dictionary, rkyv::rancor::Error>(self)
            .map_err(|e| Error::Deserialize(e.to_string()))?;

        Ok(dict)
    }
}

impl &ArchivedDictionary {
    pub fn to_dictionary(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = deserialize::<Dictionary, rkyv::rancor::Error>(self)
            .map_err(|e| Error::Deserialize(e.to_string()))?;

        Ok(dict)
    }
}
