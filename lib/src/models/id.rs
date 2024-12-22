use std::ops::Deref;

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

    pub fn parse(value: &str) -> crate::Result<Self> {
        Ok(ID(Uuid::parse_str(value)
            .map_err(|e| crate::Error::InvalidID(e.to_string()))?
            .to_string()))
    }
}

impl Into<String> for ID {
    fn into(self) -> String {
        self.0
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
