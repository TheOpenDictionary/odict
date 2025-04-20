use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry, EntryRef};
use rkyv::option::ArchivedOption;

macro_rules! resolve {
    ($t:ident, $ret:ident, $opt:ident) => {
        impl $t {
            pub fn resolve<'a>(&'a self, entry_ref: &EntryRef) -> $opt<&'a $ret> {
                self.entries
                    .get(entry_ref.as_ref())
                    .map(|e| $opt::Some(e))
                    .unwrap_or($opt::None)
            }
        }
    };
}

resolve!(Dictionary, Entry, Option);
resolve!(ArchivedDictionary, ArchivedEntry, ArchivedOption);
