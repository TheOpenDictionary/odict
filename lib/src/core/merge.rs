use crate::Dictionary;

impl Dictionary {
    pub fn merge_multi(&mut self, dictionaries: &[&Dictionary]) {
        for src in dictionaries {
            self.merge(src);
        }
    }

    pub fn merge(&mut self, dictionary: &Dictionary) {
        for (term, entry) in dictionary.entries.iter() {
            if !self.entries.contains_key(term.as_str()) {
                self.entries.insert(term.clone(), entry.clone());
            }
        }
    }
}
