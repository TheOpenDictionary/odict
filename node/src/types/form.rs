use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(Clone, StructuralConvert)]
#[convert(from(odict::Form))]
pub struct Form {
  pub term: String,
  pub kind: Option<String>,
  pub tags: Vec<String>,
}
