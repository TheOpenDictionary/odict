use crate::schema::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

macro_rules! resolve {
    ($t:ident, $ret:ident) => {
        impl $t {
            pub fn resolve<'a>(&'a self, term: &str) -> Option<&'a $ret> {
                self.entries.get(term)
            }
        }
    };
}

resolve!(Dictionary, Entry);
resolve!(ArchivedDictionary, ArchivedEntry);
