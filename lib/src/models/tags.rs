use serde::{Deserialize, Deserializer};

/// Deserialize a vector of tags from XML that has the following structure:
/// <tags>
///   <tag>possessive</tag>
/// </tags>
pub fn unwrap_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Tags {
        #[serde(default)]
        tag: Vec<String>,
    }

    let wrapper = Option::<Tags>::deserialize(deserializer)?;

    Ok(wrapper.map(|tags| tags.tag).unwrap_or_default())
}
