use crate::{Form, FormKind};
use serde::Serialize;

use super::EntryRefJSON;

#[derive(Serialize)]
pub struct FormJSON {
    pub term: EntryRefJSON,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl From<Form> for FormJSON {
    fn from(form: Form) -> Self {
        Self {
            term: EntryRefJSON(form.term.0),
            kind: form.kind,
            tags: form.tags,
        }
    }
}

impl From<&Form> for FormJSON {
    fn from(form: &Form) -> Self {
        Self {
            term: EntryRefJSON(form.term.0.clone()),
            kind: form.kind.clone(),
            tags: form.tags.clone(),
        }
    }
}
