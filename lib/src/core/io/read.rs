use std::{
    fs::{canonicalize, File},
    io::{Cursor, Read},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rkyv::access_unchecked;

use super::consts::{SIGNATURE, VERSION};
use crate::semver::SemanticVersion;
use crate::{compress::decompress, error::Error, ArchivedDictionary, Dictionary};

/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct DictionaryFile {
    pub signature: String,
    pub version: SemanticVersion,
    pub path: Option<PathBuf>,
    pub total_size: u64,
    content: Vec<u8>,
}

impl DictionaryFile {
    pub fn to_archive(&self) -> crate::Result<&ArchivedDictionary> {
        let archived = unsafe { access_unchecked::<crate::ArchivedDictionary>(&self.content[..]) };
        Ok(archived)
    }

    pub fn to_dictionary(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = self.to_archive()?.to_dictionary()?;
        Ok(dict)
    }
}

/* -------------------------------------------------------------------------- */
/*                               Helper Methods                               */
/* -------------------------------------------------------------------------- */

fn read_signature<T>(reader: &mut Cursor<T>) -> crate::Result<String>
where
    T: AsRef<[u8]>,
{
    let mut signature_bytes = [0; 5];

    reader.read_exact(&mut signature_bytes)?;

    let signature = signature_bytes.to_vec();

    if signature != SIGNATURE {
        return Err(Error::InvalidSignature);
    }

    Ok(String::from_utf8(signature)?)
}

fn read_version<T>(reader: &mut Cursor<T>) -> crate::Result<SemanticVersion>
where
    T: AsRef<[u8]>,
{
    let version_len = reader.read_u64::<LittleEndian>()?;
    let mut version_bytes = vec![0; version_len as usize];

    reader.read_exact(&mut version_bytes)?;

    let version = SemanticVersion::from(version_bytes);

    if !version.is_compatible(&VERSION) {
        return Err(Error::Incompatible);
    }

    Ok(version)
}

fn read_content<T>(reader: &mut Cursor<T>) -> crate::Result<Vec<u8>>
where
    T: AsRef<[u8]>,
{
    let content_size = reader.read_u64::<LittleEndian>()?;
    let mut content_bytes = vec![0; content_size as usize];

    reader.read_exact(&mut content_bytes)?;

    let content = decompress(&content_bytes)?;

    Ok(content)
}

/* -------------------------------------------------------------------------- */
/*                              DictionaryReader                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
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

    pub fn read_from_bytes<T>(&self, data: T) -> crate::Result<DictionaryFile>
    where
        T: AsRef<[u8]>,
    {
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

    pub fn read_from_path(&self, path: &str) -> crate::Result<DictionaryFile> {
        let pb = canonicalize(PathBuf::from(path))?;

        let mut file = File::open(&pb)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let mut result = self.read_from_bytes(&buffer)?;

        result.path = Some(pb);

        Ok(result)
    }
}
