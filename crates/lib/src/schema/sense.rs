use std::borrow::Borrow;
use std::hash::Hash;

use rkyv::with::{AsBox, MapNiche};

use crate::schema::{Form, Translation};
use crate::serializable;

use super::ArchivedPartOfSpeech;
use super::{pos::PartOfSpeech, Definition, EntryRef, Group};

/// Creates a `SenseSet` from a list of elements.
///
/// This macro provides a convenient way to create a `SenseSet` (which is a type alias for `IndexSet<Sense>`)
/// with pre-allocated capacity based on the number of elements provided.
///
/// # Examples
///
/// ```ignore
/// use odict::senseset;
/// use odict::schema::Sense;
///
/// let set = senseset![sense1, sense2, sense3];
/// ```
#[macro_export]
macro_rules! senseset {
    ($($value:expr,)+) => { senseset!($($value),+) };
    ($($value:expr),*) => {
        {
            const CAP: usize = <[()]>::len(&[$({ stringify!($value); }),*]);
            let mut set = $crate::schema::SenseSet::with_capacity(CAP);
            $(
                set.insert($value);
            )*
            set
        }
    };
}

serializable! {
  #[repr(u8)]
  pub enum DefinitionType {
    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "definition")]
    Definition(Definition),
  }
}

serializable! {
  #[derive(Default)]
  #[serde(rename = "sense")]
  pub struct Sense {
    #[serde(rename = "@pos", default)]
    pub pos: PartOfSpeech,

    #[serde(rename = "@lemma")]
    #[rkyv(with = MapNiche<AsBox>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRef>,

    #[serde(default, rename = "$value")]
    pub definitions: Vec<DefinitionType>,

    #[serde(default, rename = "tag")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
    pub tags: Vec<String>,

    #[serde(default, rename = "translation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,

    #[serde(default, rename = "form")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<Form>,
  }
}

impl Hash for Sense {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Hash for ArchivedSense {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Borrow<PartOfSpeech> for Sense {
    fn borrow(&self) -> &PartOfSpeech {
        &self.pos
    }
}

impl Borrow<ArchivedPartOfSpeech> for ArchivedSense {
    fn borrow(&self) -> &ArchivedPartOfSpeech {
        &self.pos
    }
}
