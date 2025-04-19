use crate::{Form, FormKind};
use serde::Serialize;

use super::EntryRefJSON;

#[derive(Serialize)]
pub struct FormJSON {
    pub term: EntryRefJSON,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,
}

impl From<Form> for FormJSON {
    fn from(form: Form) -> Self {
        Self {
            term: EntryRefJSON(form.term.0),
            kind: form.kind,
        }
    }
}

impl From<&Form> for FormJSON {
    fn from(form: &Form) -> Self {
        Self {
            term: EntryRefJSON(form.term.0.clone()),
            kind: form.kind.clone(),
        }
    }
}
