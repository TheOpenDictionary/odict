use std::{error::Error, ops::Deref};

use rkyv::string::ArchivedString;
use uuid::Uuid;

use crate::serializable;

serializable! {
  pub struct ID(String);
}

impl ID {
    pub fn new() -> Self {
        ID(Uuid::new_v4().to_string())
    }

    pub fn parse(value: &str) -> Result<Self, Box<dyn Error>> {
        Ok(ID(Uuid::parse_str(value)?.to_string()))
    }
}

impl Default for ID {
    fn default() -> Self {
        ID::new()
    }
}

impl Deref for ID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ArchivedID {
    type Target = ArchivedString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
