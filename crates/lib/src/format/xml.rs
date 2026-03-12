use quick_xml::se::{to_string, Serializer};
use serde::Serialize;

use crate::schema::{Dictionary, Entry};

pub trait ToXML {
    fn to_xml(self, pretty: bool) -> crate::Result<String>
    where
        Self: Sized + Serialize,
    {
        match pretty {
            true => {
                // See https://github.com/tafia/quick-xml/issues/361#issuecomment-1509724435
                let mut buffer = String::new();
                let mut ser = Serializer::new(&mut buffer);

                ser.indent(' ', 2);

                self.serialize(ser)
                    .map_err(|e| crate::Error::Serialize(e.to_string()))?;

                Ok(buffer.to_string())
            }
            false => Ok(to_string(&self).map_err(|e| crate::Error::Serialize(e.to_string()))?),
        }
    }
}

impl ToXML for Dictionary {}

impl ToXML for Entry {}

impl ToXML for &Entry {}
