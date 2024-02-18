use std::error::Error;

use quick_xml::se::to_string;
use serde::Serialize;

pub fn to_xml<S>(value: S) -> Result<String, Box<dyn Error>>
where
    S: Serialize,
{
    let v = to_string(&value)?;
    Ok(v)
}
