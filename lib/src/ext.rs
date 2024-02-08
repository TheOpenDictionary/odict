use std::error::Error;

use crate::Dictionary;

pub trait ToDictionary {
    fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>>;
}

impl ToDictionary for String {
    fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>> {
        Ok(Dictionary::from(self.as_str()))
    }
}
