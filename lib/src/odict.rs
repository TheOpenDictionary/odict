/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

use std::path::PathBuf;

use crate::version::SemanticVersion;

#[derive(Clone)]
pub struct OpenDictionary {
    pub(crate) signature: String,
    pub(crate) version: SemanticVersion,
    pub(crate) path: Option<PathBuf>,
    pub(crate) bytes: Vec<u8>,
}

impl OpenDictionary {
    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub fn version(&self) -> &SemanticVersion {
        &self.version
    }

    pub fn size(&self) -> crate::Result<u64> {
        if let Some(path) = &self.path {
            if let Ok(metadata) = std::fs::metadata(path) {
                return Ok(metadata.len());
            }
        }

        let bytes = self.to_bytes()?;

        Ok(bytes.len().try_into()?)
    }

    pub fn contents(&self) -> crate::Result<&crate::schema::ArchivedDictionary> {
        Ok(rkyv::access::<
            crate::schema::ArchivedDictionary,
            rkyv::rancor::Error,
        >(&self.bytes[..])?)
    }
}
