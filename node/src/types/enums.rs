use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(Debug, PartialEq, Clone, StructuralConvert)]
#[convert(from(internal::EnumWrapper))]
pub struct EnumWrapper {
  pub variant: String,
  pub value: String,
}
