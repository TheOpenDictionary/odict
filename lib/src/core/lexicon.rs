use crate::{ArchivedDictionary, Dictionary};

macro_rules! lexicon {
    ($t:ident) => {
        impl $t {
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
