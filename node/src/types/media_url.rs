use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::schema::MediaURL))]
pub struct MediaURL {
  pub src: String,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}
