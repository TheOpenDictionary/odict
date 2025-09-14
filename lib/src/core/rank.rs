use crate::schema::{ArchivedDictionary, Dictionary};

impl ArchivedDictionary {
    fn rank_iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries
            .iter()
            .filter_map(|entry| entry.rank.as_ref().map(|r| r.to_native()))
    }
}

impl Dictionary {
    fn rank_iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries.iter().filter_map(|entry| entry.rank)
    }
}

macro_rules! rank {
    ($t:ident) => {
        impl $t {
            pub fn min_rank(&self) -> Option<u32> {
                self.rank_iter().min()
            }

            pub fn max_rank(&self) -> Option<u32> {
                self.rank_iter().max()
            }
        }
    };
}

rank!(Dictionary);
rank!(ArchivedDictionary);
