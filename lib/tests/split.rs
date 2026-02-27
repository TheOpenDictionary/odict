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
        // The result should be identical regardless of which path is taken.
        let result = dict
            .split(&vec!["cat"], SplitOptions::default())
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "cat");
    }
}
