use crate::serializable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

serializable! {
  pub struct Example {
    #[serde(rename = "$text")]
    pub value: String,
  }
}

/// Deserialize a vector of examples from XML that has the following structure:
/// <examples>
///   <example>This is an example.</example>
/// </examples>
pub fn unwrap_examples<'de, D>(deserializer: D) -> Result<Vec<Example>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Examples {
        #[serde(default)]
        example: Vec<Example>,
    }

    let wrapper = Option::<Examples>::deserialize(deserializer)?;

    Ok(wrapper.map(|examples| examples.example).unwrap_or_default())
}

/// Serialize a vector of examples to XML with the following structure:
/// <examples>
///   <example>This is an example.</example>
/// </examples>
pub fn wrap_examples<S>(examples: &Vec<Example>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Examples {
        example: Vec<Example>,
    }

    if examples.is_empty() {
        // Don't serialize anything if there are no examples
        serializer.serialize_none()
    } else {
        let wrapped_examples = Examples {
            example: examples.clone(),
        };

        wrapped_examples.serialize(serializer)
    }
}
