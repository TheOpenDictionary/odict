use std::{
    error::Error,
    fs::{canonicalize, File},
    io::{Cursor, Read},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rkyv::archived_root;

use crate::{
    constants::{SIGNATURE, VERSION},
    lz4::decompress,
    ArchivedDictionary, Dictionary,
};

use super::semver::SemanticVersion;

/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

pub struct DictionaryFile {
    pub signature: String,
    pub version: SemanticVersion,
    pub path: Option<PathBuf>,
    pub total_size: u64,
    content: Vec<u8>,
}

impl DictionaryFile {
    pub fn to_archive(&self) -> Result<&ArchivedDictionary, Box<dyn Error>> {
        let archived = unsafe { archived_root::<Dictionary>(&self.content[..]) };
        Ok(archived)
    }

    pub fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>> {
        let dict: Dictionary = self.to_archive()?.to_dictionary()?;
        Ok(dict)
    }
}

/* -------------------------------------------------------------------------- */
/*                               Helper Methods                               */
/* -------------------------------------------------------------------------- */

fn read_signature(reader: &mut Cursor<&[u8]>) -> Result<String, Box<dyn Error>> {
    let mut signature_bytes = [0; 5];

    reader.read_exact(&mut signature_bytes)?;

    let signature = signature_bytes.to_vec();

    if signature != SIGNATURE {
        return Err("This is not an ODict file".into());
    }

    Ok(String::from_utf8(signature)?)
}

fn read_version(reader: &mut Cursor<&[u8]>) -> Result<SemanticVersion, Box<dyn Error>> {
    let version_len = reader.read_u64::<LittleEndian>()?;
    let mut version_bytes = vec![0; version_len as usize];

    reader.read_exact(&mut version_bytes).map_err(|_| {
        "Failed to read version (is it possible this file was compiled with ODict V1?)"
    })?;

    let version = SemanticVersion::from(version_bytes);

    if !version.is_compatible(&VERSION) {
        return Err("This is not compatible with the current version of ODict".into());
    }

    Ok(version)
}

fn read_content(reader: &mut Cursor<&[u8]>) -> Result<Vec<u8>, Box<dyn Error>> {
    let content_size = reader.read_u64::<LittleEndian>()?;
    let mut content_bytes = vec![0; content_size as usize];

    reader.read_exact(&mut content_bytes)?;

    let content = decompress(&content_bytes)?;

    Ok(content)
}

/* -------------------------------------------------------------------------- */
/*                              DictionaryReader                              */
/* -------------------------------------------------------------------------- */

pub struct DictionaryReader {}

impl Default for DictionaryReader {
    fn default() -> Self {
        Self::new()
    }
}

impl DictionaryReader {
    pub fn new() -> Self {
        // TODO: maybe add a config?
        Self {}
    }

    pub fn read_from_bytes(&self, data: &[u8]) -> Result<DictionaryFile, Box<dyn Error>> {
        let mut reader = Cursor::new(data);

        let signature = read_signature(&mut reader)?;
        let version = read_version(&mut reader)?;
        let content = read_content(&mut reader)?;

        Ok(DictionaryFile {
            signature,
            version,
            content,
            total_size: reader.position(),
            path: None,
        })
    }

    pub fn read_from_path(&self, path: &str) -> Result<DictionaryFile, Box<dyn Error>> {
        let pb = canonicalize(PathBuf::from(path))?;
        let mut file = File::open(&pb)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let mut result = self.read_from_bytes(&buffer)?;

        result.path = Some(pb);

        Ok(result)
    }
}
