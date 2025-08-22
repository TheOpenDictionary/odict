/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

use std::path::PathBuf;

use rkyv::access_unchecked;

use crate::{semver::SemanticVersion, ArchivedDictionary, Dictionary};

#[derive(Clone)]
pub struct OpenDictionary {
    pub signature: String,
    pub version: SemanticVersion,
    pub path: Option<PathBuf>,
    pub total_size: u64,

    pub(super) content: Vec<u8>,
}

impl OpenDictionary {
    pub fn access(&self) -> crate::Result<&ArchivedDictionary> {
        let archived = unsafe { access_unchecked::<crate::ArchivedDictionary>(&self.content[..]) };
        Ok(archived)
    }

    pub fn deserialize(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = self.access()?.deserialize()?;
        Ok(dict)
    }
}
