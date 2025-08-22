use std::{
    fs::{canonicalize, File},
    io::{Cursor, Read},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};

use super::consts::{SIGNATURE, VERSION};
use crate::{compress::decompress, error::Error, semver::SemanticVersion, OpenDictionary};

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
        Self {}
    }
}

impl OpenDictionary {
    pub fn from_bytes<T>(data: T) -> crate::Result<OpenDictionary>
    where
        T: AsRef<[u8]>,
    {
        let mut reader = Cursor::new(data);

        let signature = read_signature(&mut reader)?;
        let version = read_version(&mut reader)?;
        let content = read_content(&mut reader)?;

        Ok(Self {
            signature,
            version,
            content,
            total_size: reader.position(),
            path: None,
        })
    }

    pub fn from_path(path: &str) -> crate::Result<OpenDictionary> {
        let pb = canonicalize(PathBuf::from(path))?;

        let mut file = File::open(&pb)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let mut result = Self::from_bytes(&buffer)?;

        result.path = Some(pb);

        Ok(result)
    }
}
