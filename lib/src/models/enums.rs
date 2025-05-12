use std::fmt::{Debug, Display};

pub trait EnumIdentifier {
    fn id(&self) -> String;
}

impl<T> EnumIdentifier for T
where
    T: strum::VariantNames + serde::Serialize + Debug + Display,
{
    fn id(&self) -> String {
        serde_json::to_value(self)
            .map(|v| match v {
                serde_json::Value::String(s) => s,
                _ => format!("{}", self),
            })
            .unwrap_or_else(|_| format!("Error serializing enum: {:?}", self))
    }
}
