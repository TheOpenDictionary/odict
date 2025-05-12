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
    T: odict::EnumIdentifier + Display,
{
    fn to_enum_wrapper(&self) -> EnumWrapper {
        let value = self.to_string();
        let variant = self.id();

        EnumWrapper { variant, value }
    }
}
