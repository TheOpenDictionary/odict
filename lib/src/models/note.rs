use crate::serializable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::example::{unwrap_examples, wrap_examples, Example};

serializable! {
  pub struct Note {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "unwrap_examples")]
    #[serde(serialize_with = "wrap_examples")]
    pub examples: Vec<Example>,
  }
}

/// Deserialize a vector of notes from XML that has the following structure:
/// <notes>
///   <note value="This is a note">...</note>
/// </notes>
pub fn unwrap_notes<'de, D>(deserializer: D) -> Result<Vec<Note>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Notes {
        #[serde(default)]
        note: Vec<Note>,
    }

    let wrapper = Option::<Notes>::deserialize(deserializer)?;

    Ok(wrapper.map(|notes| notes.note).unwrap_or_default())
}

/// Serialize a vector of notes to XML with the following structure:
/// <notes>
///   <note value="This is a note">...</note>
/// </notes>
pub fn wrap_notes<S>(notes: &Vec<Note>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Notes {
        note: Vec<Note>,
    }

    if notes.is_empty() {
        // Don't serialize anything if there are no notes
        serializer.serialize_none()
    } else {
        let wrapped_notes = Notes {
            note: notes.clone(),
        };

        wrapped_notes.serialize(serializer)
    }
}
