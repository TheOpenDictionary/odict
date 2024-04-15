use std::{
    error::Error,
    fs::{canonicalize, File},
    io::{Cursor, Read, Seek},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rkyv::archived_root;

use super::constants::FILE_VERSION;
use crate::{lz4::decompress, ArchivedDictionary, Dictionary};

/* -------------------------------------------------------------------------- */
/*                               DictionaryFile                               */
/* -------------------------------------------------------------------------- */

pub struct DictionaryFile {
    pub signature: String,
    pub version: u16,
    pub path: Option<PathBuf>,
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
/*                              DictionaryReader                              */
/* -------------------------------------------------------------------------- */

pub struct DictionaryReader {}

impl DictionaryReader {
    pub fn new() -> Self {
        // TODO: maybe add a config?
        Self {}
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn read_from_bytes(&self, data: &[u8]) -> Result<DictionaryFile, Box<dyn Error>> {
        let mut reader = Cursor::new(data);
        let mut sig_bytes = [0; 5];

        reader.read_exact(&mut sig_bytes)?;

        reader.seek(std::io::SeekFrom::Start(5))?;

        let version = reader.read_u16::<LittleEndian>()?;

        if version != FILE_VERSION {}

        reader.seek(std::io::SeekFrom::Start(7))?;

        let content_size = reader.read_u64::<LittleEndian>()?;

        reader.seek(std::io::SeekFrom::Start(15))?;

        let signature = String::from_utf8(sig_bytes.to_vec())?;

        if signature != "ODICT" {
            return Err("This is not an ODict file".into());
        }

        let mut content_bytes = vec![0; content_size as usize];

        reader.read_exact(&mut content_bytes)?;

        let content = decompress(&content_bytes)?;

        Ok(DictionaryFile {
            signature,
            version,
            content,
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
