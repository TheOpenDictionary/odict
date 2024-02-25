use std::error::Error;

use quick_xml::se::to_string;
use serde::Serialize;

pub trait ToXML {
    fn to_xml(self) -> Result<String, Box<dyn Error>>;
}

impl<S> ToXML for S
where
    S: Serialize,
{
    fn to_xml(self) -> Result<String, Box<dyn Error>> {
        let v = to_string(&self)?;
        Ok(v)
    }
}
