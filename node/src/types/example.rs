use napi::bindgen_prelude::*;

use super::mdstring::MDString;

#[napi(object)]
pub struct Example {
  pub value: ClassInstance<MDString>,
}

impl Example {
  pub fn from(env: napi::Env, note: odict::Example) -> Result<Self> {
    let odict::Example { value } = note;

    Ok(Self {
      value: MDString::from(value).into_instance(env).unwrap(),
    })
  }
}
