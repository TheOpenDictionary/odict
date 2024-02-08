use crate::Dictionary;

impl Dictionary {
    pub fn merge_in(&mut self, dictionaries: &[&Dictionary]) {
        for src in dictionaries {
            for (term, entry) in src.entries.iter() {
                if !self.entries.contains_key(term.as_str()) {
                    self.entries.insert(term.clone(), entry.clone());
                }
            }
        }
    }
}
