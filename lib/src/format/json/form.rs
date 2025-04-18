use crate::Form;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormJSON {
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl From<Form> for FormJSON {
    fn from(form: Form) -> Self {
        Self {
            value: form.value.0,
            type_: form.type_.map(|ft| ft.to_string()),
        }
    }
}

impl From<&Form> for FormJSON {
    fn from(form: &Form) -> Self {
        Self {
            value: form.value.0.clone(),
            type_: form.type_.as_ref().map(|ft| ft.to_string()),
        }
    }
}
