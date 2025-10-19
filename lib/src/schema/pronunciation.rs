use icu_locale_core::LanguageIdentifier;
use rkyv::{
    rancor::{Fallible, Source},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, Place, Resolver, Serialize,
};

use super::MediaURL;
use crate::serializable;

// Custom wrapper for LanguageIdentifier to serialize as string and deserialize with try_from
pub struct LanguageIdentifierAsString;

impl ArchiveWith<LanguageIdentifier> for LanguageIdentifierAsString {
    type Archived = Archived<String>;
    type Resolver = Resolver<String>;

    fn resolve_with(
        field: &LanguageIdentifier,
        resolver: Self::Resolver,
        out: Place<Self::Archived>,
    ) {
        let string_repr = field.to_string();
        string_repr.resolve(resolver, out);
    }
}

impl<S: Fallible + ?Sized> SerializeWith<LanguageIdentifier, S> for LanguageIdentifierAsString
where
    String: Serialize<S>,
{
    fn serialize_with(
        field: &LanguageIdentifier,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let string_repr = field.to_string();
        string_repr.serialize(serializer)
    }
}

impl<D> DeserializeWith<Archived<String>, LanguageIdentifier, D> for LanguageIdentifierAsString
where
    D: Fallible + ?Sized,
    D::Error: Source,
    String: Archive,
    Archived<String>: Deserialize<String, D>,
{
    fn deserialize_with(
        field: &Archived<String>,
        deserializer: &mut D,
    ) -> Result<LanguageIdentifier, D::Error> {
        let string_repr: String = field.deserialize(deserializer)?;
        LanguageIdentifier::try_from_str(string_repr.as_str())
            .map_err(|e| <D::Error as Source>::new(e))
    }
}

serializable! {
  pub struct Pronunciation {
    #[serde(rename = "@kind")]
    #[rkyv(with = LanguageIdentifierAsString)]
    pub kind: LanguageIdentifier,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename = "url")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaURL>,
  }
}
