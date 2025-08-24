use crate::schema::Dictionary;

impl Dictionary {
    pub fn merge_multi(&mut self, dictionaries: Vec<&Dictionary>) {
        for src in dictionaries {
            self.merge(src);
        }
    }

    pub fn merge(&mut self, dictionary: &Dictionary) {
        for entry in dictionary.entries.iter() {
            if !self.entries.contains(entry) {
                self.entries.insert(entry.clone());
            }
        }
    }
}
