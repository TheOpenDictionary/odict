use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

/// Serialize a vector of tags to XML with the following structure:
/// <tags>
///   <tag>possessive</tag>
/// </tags>
pub fn wrap_tags<S>(tags: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Tag {
        #[serde(rename = "$text")]
        value: String,
    }

    #[derive(Serialize)]
    struct Tags {
        tag: Vec<Tag>,
    }

    if tags.is_empty() {
        // Don't serialize anything if there are no tags
        serializer.serialize_none()
    } else {
        let wrapped_tags = Tags {
            tag: tags.iter().map(|t| Tag { value: t.clone() }).collect(),
        };

        wrapped_tags.serialize(serializer)
    }
}
