use std::error::Error;

use serde::Serialize;

use crate::json::EntryJSON;
use crate::{Dictionary, Entry};

pub struct JSONSerializer {}

pub trait ToJSON {
    fn to_json(self, pretty: bool) -> Result<String, Box<dyn Error>>;
}

impl ToJSON for Vec<Entry> {
    fn to_json(self, pretty: bool) -> Result<String, Box<dyn Error>> {
        let json = self
            .into_iter()
            .map(|v| EntryJSON::from(v))
            .collect::<Vec<EntryJSON>>();

        Ok("".to_string())
    }
}

impl ToJSON for Vec<Vec<Entry>> {
    fn to_json(self, pretty: bool) -> Result<String, Box<dyn Error>> {
        let json = self
            .into_iter()
            .map(|v| v.into_iter().map(|v| EntryJSON::from(v)).collect())
            .collect::<Vec<Vec<EntryJSON>>>();

        Ok("".to_string())
    }
}
