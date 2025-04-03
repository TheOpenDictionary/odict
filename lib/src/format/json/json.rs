use serde::Serialize;
use serde_json::{to_string, to_string_pretty};

use super::dictionary::DictionaryJSON;
use super::entry::EntryJSON;

#[cfg(feature = "tokenize-latin")]
use super::token::TokenJSON;

use crate::{lookup::LookupResult, ArchivedEntry, Dictionary, Entry};

#[cfg(feature = "tokenize-latin")]
use crate::Token;

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

impl ToJSON for Vec<LookupResult<Entry>> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| EntryJSON::from(v.entry))
            .collect::<Vec<EntryJSON>>();
        stringify(&json, pretty)
    }
}

impl ToJSON for Vec<LookupResult<&ArchivedEntry>> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| EntryJSON::from(v.entry))
            .collect::<Vec<EntryJSON>>();
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

#[cfg(feature = "tokenize-latin")]
impl<'a> ToJSON for Token<&ArchivedEntry> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = TokenJSON::from(self);
        stringify(&json, pretty)
    }
}

#[cfg(feature = "tokenize-latin")]
impl<'a> ToJSON for Vec<Token<&ArchivedEntry>> {
    fn to_json(self, pretty: bool) -> crate::Result<String> {
        let json = self
            .into_iter()
            .map(|v| TokenJSON::from(v))
            .collect::<Vec<TokenJSON>>();

        stringify(&json, pretty)
    }
}
