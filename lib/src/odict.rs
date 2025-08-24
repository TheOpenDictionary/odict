/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

use std::path::PathBuf;

use rkyv::access_unchecked;

use crate::version::SemanticVersion;

#[derive(Clone)]
pub struct OpenDictionary {
    pub(crate) signature: String,
    pub(crate) version: SemanticVersion,
    pub(crate) path: Option<PathBuf>,
    pub(crate) total_size: u64,
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

    pub fn size(&self) -> u64 {
        self.total_size
    }

    pub fn contents(&self) -> crate::Result<&crate::schema::ArchivedDictionary> {
        Ok(unsafe { access_unchecked::<crate::schema::ArchivedDictionary>(&self.bytes[..]) })
    }
}
