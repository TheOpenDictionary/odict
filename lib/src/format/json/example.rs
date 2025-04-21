use serde::Serialize;

use crate::Example;

#[derive(Serialize, Clone)]
pub struct ExampleJSON {
    pub value: String,
}

impl From<Example> for ExampleJSON {
    fn from(example: Example) -> Self {
        Self {
            value: example.value,
        }
    }
}

impl From<&Example> for ExampleJSON {
    fn from(example: &Example) -> Self {
        Self {
            value: example.value.clone(),
        }
    }
}
