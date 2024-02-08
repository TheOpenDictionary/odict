use crate::{ArchivedDictionary, Dictionary};

macro_rules! lexicon {
    ($type:ty) => {
        impl $type {
            pub fn lexicon(&self) -> Vec<&str> {
                let mut vec: Vec<&str> = self
                    .entries
                    .iter()
                    .map(|(_, entry)| entry.term.as_str())
                    .collect();
                vec.sort();
                vec
            }
        }
    };
}

lexicon!(Dictionary);
lexicon!(ArchivedDictionary);
