use crate::Form;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormJSON {
    pub value: String,
}

impl From<Form> for FormJSON {
    fn from(form: Form) -> Self {
        Self {
            value: form.value.0,
        }
    }
}

impl From<&Form> for FormJSON {
    fn from(form: &Form) -> Self {
        Self {
            value: form.value.0.clone(),
        }
    }
}
