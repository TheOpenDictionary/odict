use std::str::FromStr;

use crate::Dictionary;

pub trait ToDictionary {
    fn to_dictionary(&self) -> crate::Result<Dictionary>;
}

impl ToDictionary for String {
    fn to_dictionary(&self) -> crate::Result<Dictionary> {
        Dictionary::from_str(self.as_str())
    }
}

impl ToDictionary for str {
    fn to_dictionary(&self) -> crate::Result<Dictionary> {
        Dictionary::from_str(self)
    }
}
