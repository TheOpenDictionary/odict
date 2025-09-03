use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::schema::Translation))]
pub struct Translation {
  pub lang: String,
  pub value: String,
}
