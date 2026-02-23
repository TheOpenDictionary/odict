use std::fmt::{Debug, Display};

use crate::schema::PronunciationKind;

use super::{FormKind, PartOfSpeech};

pub trait EnumIdentifier {
    fn id(&self) -> String;
}

fn get_tag_name<T: serde::Serialize + Display + Debug>(value: &T) -> String {
    serde_json::to_value(value)
        .map(|v| match v {
            serde_json::Value::String(s) => s,
            _ => format!("{value}"),
        })
        .unwrap_or_else(|_| format!("Error serializing enum: {value:?}"))
}

macro_rules! impl_enum_identifier {
    ($enum_type:ident) => {
        impl EnumIdentifier for $enum_type {
            fn id(&self) -> String {
                match self {
                    $enum_type::Other(_) => "other".to_string(),
                    _ => get_tag_name(self),
                }
            }
        }
    };
}

impl_enum_identifier!(PartOfSpeech);
impl_enum_identifier!(FormKind);
impl_enum_identifier!(PronunciationKind);
