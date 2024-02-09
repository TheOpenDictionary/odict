use std::ops::Deref;

use uuid::Uuid;

use crate::serializable;

serializable! {
  pub struct ID(String);
}

impl ID {
    pub fn new() -> Self {
        ID(Uuid::new_v4().to_string())
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
