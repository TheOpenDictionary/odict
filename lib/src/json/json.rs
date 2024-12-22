use serde::Serialize;
use serde_json::{to_string, to_string_pretty};

use super::dictionary::DictionaryJSON;
use super::entry::EntryJSON;

use crate::{ArchivedEntry, Dictionary, Entry};

pub struct JSONSerializer {}

pub fn stringify<T>(value: &T, pretty: bool) -> crate::Result<String>
where
    T: ?Sized + Serialize,
{
    match pretty {
        true => Ok(to_string_pretty(value)?),
        false => Ok(to_string(value)?),
    }
}

pub trait ToJSON {
    fn to_json(self, pretty: bool) -> crate::Result<String>;
}

impl ToJSON for Dictionary {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = DictionaryJSON::from(self);
        stringify(&json, pretty)
    }
}

impl ToJSON for Entry {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = EntryJSON::from(self);
        stringify(&json, pretty)
    }
}

impl ToJSON for Vec<Entry> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| EntryJSON::from(v))
            .collect::<Vec<EntryJSON>>();

        stringify(&json, pretty)
    }
}

impl ToJSON for Vec<Vec<Entry>> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| v.into_iter().map(|v| EntryJSON::from(v)).collect())
            .collect::<Vec<Vec<EntryJSON>>>();

        stringify(&json, pretty)
    }
}

impl ToJSON for &ArchivedEntry {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = EntryJSON::from(self.to_entry().unwrap());
        stringify(&json, pretty)
    }
}

impl ToJSON for Vec<&ArchivedEntry> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| EntryJSON::from(v))
            .collect::<Vec<EntryJSON>>();

        stringify(&json, pretty)
    }
}

impl ToJSON for Vec<Vec<&ArchivedEntry>> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| v.into_iter().map(|v| EntryJSON::from(v)).collect())
            .collect::<Vec<Vec<EntryJSON>>>();

        stringify(&json, pretty)
    }
}
