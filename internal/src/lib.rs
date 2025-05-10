use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumWrapper {
    pub variant: String,
    pub value: String,
}

pub trait ToEnumWrapper {
    fn to_enum_wrapper(&self) -> EnumWrapper;
}

impl<T> ToEnumWrapper for T
where
    T: strum::VariantNames + Display + Debug,
{
    fn to_enum_wrapper(&self) -> EnumWrapper {
        let value = self.to_string();

        let variant = {
            let debug_str = format!("{:?}", self);
            if let Some(variant_name) = debug_str.split('(').next() {
                variant_name.to_string()
            } else {
                debug_str
            }
        }
        .to_string();

        EnumWrapper { variant, value }
    }
}
