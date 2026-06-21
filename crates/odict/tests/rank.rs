mod helpers;

#[cfg(test)]
mod rank_tests {

    use odict::schema::{Dictionary, Entry, EntrySet, Etymology};

    use crate::helpers::{EXAMPLE_DICT_1, EXAMPLE_DICT_2};

    fn ranked_entry(term: &str, rank: Option<u32>) -> Entry {
        Entry {
            term: term.to_string(),
            rank,
            etymologies: vec![Etymology::default()],
            ..Entry::default()
        }
    }

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
        let mut entries = EntrySet::new();
        entries.insert(ranked_entry("low", Some(10)));
        entries.insert(ranked_entry("high", Some(100)));
        entries.insert(ranked_entry("medium", Some(50)));
        entries.insert(ranked_entry("norank", None));

        let dict = Dictionary {
            name: Some("Test Dictionary".to_string()),
            entries,
            ..Dictionary::default()
        };

        assert_eq!(dict.min_rank(), Some(10));
        assert_eq!(dict.max_rank(), Some(100));
    }

    #[test]
    fn test_rank_single_value() {
        let mut entries = EntrySet::new();
        entries.insert(ranked_entry("single", Some(42)));
        entries.insert(ranked_entry("norank", None));

        let dict = Dictionary {
            name: Some("Single Rank Test".to_string()),
            entries,
            ..Dictionary::default()
        };

        assert_eq!(dict.min_rank(), Some(42));
        assert_eq!(dict.max_rank(), Some(42));
    }

    #[test]
    fn test_rank_empty_dictionary() {
        let dict = Dictionary::default();

        assert_eq!(dict.min_rank(), None);
        assert_eq!(dict.max_rank(), None);
    }

    #[test]
    fn test_rank_extreme_values() {
        let mut entries = EntrySet::new();
        entries.insert(ranked_entry("min", Some(0)));
        entries.insert(ranked_entry("max", Some(u32::MAX)));

        let dict = Dictionary {
            name: Some("Extreme Values Test".to_string()),
            entries,
            ..Dictionary::default()
        };

        assert_eq!(dict.min_rank(), Some(0));
        assert_eq!(dict.max_rank(), Some(u32::MAX));
    }
}
