use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Translation))]
pub struct Translation {
  pub lang: String,
  pub value: String,
}
