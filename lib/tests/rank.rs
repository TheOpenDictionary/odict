mod helpers;

#[cfg(test)]
mod rank_tests {
    use indexmap::IndexSet;

    use crate::helpers::{EXAMPLE_DICT_1, EXAMPLE_DICT_2};

    #[test]
    fn test_min_rank_with_ranks() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();
        let result = dict.min_rank();

        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_max_rank_with_ranks() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();
        let result = dict.max_rank();

        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_min_rank_no_ranks() {
        let dict = EXAMPLE_DICT_2.contents().unwrap();
        let result = dict.min_rank();

        assert_eq!(result, None);
    }

    #[test]
    fn test_max_rank_no_ranks() {
        let dict = EXAMPLE_DICT_2.contents().unwrap();
        let result = dict.max_rank();

        assert_eq!(result, None);
    }

    #[test]
    fn test_min_rank_dictionary_type() {
        let dict = EXAMPLE_DICT_1.contents().unwrap().deserialize().unwrap();
        let result = dict.min_rank();

        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_max_rank_dictionary_type() {
        let dict = EXAMPLE_DICT_1.contents().unwrap().deserialize().unwrap();
        let result = dict.max_rank();

        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_min_rank_dictionary_type_no_ranks() {
        let dict = EXAMPLE_DICT_2.contents().unwrap().deserialize().unwrap();
        let result = dict.min_rank();

        assert_eq!(result, None);
    }

    #[test]
    fn test_max_rank_dictionary_type_no_ranks() {
        let dict = EXAMPLE_DICT_2.contents().unwrap().deserialize().unwrap();
        let result = dict.max_rank();

        assert_eq!(result, None);
    }

    #[test]
    fn test_rank_with_multiple_values() {
        use odict::schema::{Dictionary, Entry, Etymology};

        let mut dict = Dictionary::default();

        dict.name = Some("Test Dictionary".to_string());

        let mut entries = IndexSet::new();

        let mut entry1 = Entry::default();
        entry1.term = "low".to_string();
        entry1.rank = Some(10);
        entry1.etymologies = vec![Etymology::default()];
        entries.insert(entry1);

        let mut entry2 = Entry::default();
        entry2.term = "high".to_string();
        entry2.rank = Some(100);
        entry2.etymologies = vec![Etymology::default()];
        entries.insert(entry2);

        let mut entry3 = Entry::default();
        entry3.term = "medium".to_string();
        entry3.rank = Some(50);
        entry3.etymologies = vec![Etymology::default()];
        entries.insert(entry3);

        let mut entry4 = Entry::default();
        entry4.term = "norank".to_string();
        entry4.rank = None;
        entry4.etymologies = vec![Etymology::default()];
        entries.insert(entry4);

        dict.entries = entries;

        assert_eq!(dict.min_rank(), Some(10));
        assert_eq!(dict.max_rank(), Some(100));
    }

    #[test]
    fn test_rank_single_value() {
        use odict::schema::{Dictionary, Entry, Etymology};

        let mut dict = Dictionary::default();
        dict.name = Some("Single Rank Test".to_string());

        let mut entries = IndexSet::new();

        let mut entry1 = Entry::default();
        entry1.term = "single".to_string();
        entry1.rank = Some(42);
        entry1.etymologies = vec![Etymology::default()];
        entries.insert(entry1);

        let mut entry2 = Entry::default();
        entry2.term = "norank".to_string();
        entry2.rank = None;
        entry2.etymologies = vec![Etymology::default()];
        entries.insert(entry2);

        dict.entries = entries;

        assert_eq!(dict.min_rank(), Some(42));
        assert_eq!(dict.max_rank(), Some(42));
    }

    #[test]
    fn test_rank_empty_dictionary() {
        use odict::schema::Dictionary;

        let dict = Dictionary::default();

        assert_eq!(dict.min_rank(), None);
        assert_eq!(dict.max_rank(), None);
    }

    #[test]
    fn test_rank_extreme_values() {
        use odict::schema::{Dictionary, Entry, Etymology};

        let mut dict = Dictionary::default();

        dict.name = Some("Extreme Values Test".to_string());

        let mut entries = IndexSet::new();
        let mut entry1 = Entry::default();

        entry1.term = "min".to_string();
        entry1.rank = Some(0);
        entry1.etymologies = vec![Etymology::default()];
        entries.insert(entry1);

        let mut entry2 = Entry::default();
        entry2.term = "max".to_string();
        entry2.rank = Some(u32::MAX);
        entry2.etymologies = vec![Etymology::default()];
        entries.insert(entry2);

        dict.entries = entries;

        assert_eq!(dict.min_rank(), Some(0));
        assert_eq!(dict.max_rank(), Some(u32::MAX));
    }
}
