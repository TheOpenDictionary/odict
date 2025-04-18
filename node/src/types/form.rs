use napi::bindgen_prelude::*;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Form {
  pub form_type: Option<String>,
  pub text: String,
}

impl Form {
  pub fn from(form: odict::Form) -> Result<Self> {
    let odict::Form { form_type, text } = form;

    let form_type = form_type.map(|t| match t {
      odict::FormType::Conjugation => "conjugation".to_string(),
      odict::FormType::Inflection => "inflection".to_string(),
      odict::FormType::Plural => "plural".to_string(),
      odict::FormType::Irregular => "irregular".to_string(),
      odict::FormType::Variant => "variant".to_string(),
      odict::FormType::Other => "other".to_string(),
    });

    Ok(Self { form_type, text })
  }
}
