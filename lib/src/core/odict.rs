/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

use std::path::PathBuf;

use rkyv::access_unchecked;

use crate::{ArchivedDictionary, SemanticVersion};

#[derive(Clone)]
pub struct OpenDictionary {
    pub signature: String,
    pub version: SemanticVersion,
    pub path: Option<PathBuf>,
    pub total_size: u64,
    pub(super) content: Vec<u8>,
}

impl OpenDictionary {
    pub fn content(&self) -> crate::Result<&ArchivedDictionary> {
        Ok(unsafe { access_unchecked::<crate::ArchivedDictionary>(&self.content[..]) })
    }
}
