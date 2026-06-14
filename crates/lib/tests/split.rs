mod helpers;

#[cfg(test)]
mod split_tests {

    use odict::split::SplitOptions;

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_split() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .split(&vec!["catdog"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_split_with_numbers() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .split(&vec!["2cat8dog"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_split_does_not_attempt_whole_word_lookup() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // "cat" exists as a whole word in the dictionary.
        // split() should still find it via the segmentation path, not a short-circuit exact match.
        let result = dict
            .split(&vec!["cat"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "cat");
    }

    #[test]
    fn test_split_no_matches() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .split(&vec!["xyz"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_split_empty_string() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .split(&vec![""], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_split_threshold_skips_short_segments() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // "cat" and "dog" are each 3 chars; threshold=4 means segments of ≤4 chars
        // are discarded when no match is found, so neither word is ever tried at length 3
        let result = dict
            .split(&vec!["catdog"], SplitOptions::default().threshold(4))
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_split_multiple_queries_preserves_order() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // Results from each query are appended in input order
        let result = dict
            .split(&vec!["catdog", "dogcat"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
        assert_eq!(result[2].entry.term, "dog");
        assert_eq!(result[3].entry.term, "cat");
    }

    #[test]
    fn test_split_without_follow() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // "ran" has a see_also pointing to "run"; without follow, "ran" is returned as-is
        let result = dict
            .split(&vec!["randog"], SplitOptions::default().follow(false))
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "ran");
        assert!(result[0].directed_from.is_none());
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_split_with_follow() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // "ran" see_also "run"; with follow=true, the split should resolve to "run"
        let result = dict
            .split(&vec!["randog"], SplitOptions::default().follow(true))
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "run");
        assert_eq!(result[0].directed_from.unwrap().term, "ran");
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_split_case_sensitive_by_default() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // "CAT" does not match "cat" with default (case-sensitive) settings;
        // only "dog" (lowercase) should be found
        let result = dict
            .split(&vec!["CATdog"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "dog");
    }

    #[test]
    fn test_split_case_insensitive() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .split(&vec!["CATdog"], SplitOptions::default().insensitive(true))
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
    }
}
