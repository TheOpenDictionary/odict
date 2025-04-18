use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Form {
    #[pyo3(get)]
    pub form_type: Option<String>,
    #[pyo3(get)]
    pub text: String,
}

impl From<odict::Form> for Form {
    fn from(form: odict::Form) -> Self {
        let odict::Form { form_type, text } = form;

        let form_type = form_type.map(|t| match t {
            odict::FormType::Conjugation => "conjugation".to_string(),
            odict::FormType::Inflection => "inflection".to_string(),
            odict::FormType::Plural => "plural".to_string(),
            odict::FormType::Irregular => "irregular".to_string(),
            odict::FormType::Variant => "variant".to_string(),
            odict::FormType::Other => "other".to_string(),
        });

        Self { form_type, text }
    }
}
