mod helpers;

#[cfg(test)]
mod split_tests {
    use odict::SplitOptions;

    use crate::helpers::EXAMPLE_DICTIONARY_1;

    #[test]
    fn test_split() {
        let dict = EXAMPLE_DICTIONARY_1.to_archive().unwrap();
        let result = dict.split("catdog", &SplitOptions::default().threshold(1));

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.as_ref().unwrap().len(), 2);
        assert_eq!(result.as_ref().unwrap()[0].term, "cat");
        assert_eq!(result.as_ref().unwrap()[1].term, "dog");
    }

    #[test]
    fn test_split_with_numbers() {
        let dict = EXAMPLE_DICTIONARY_1.to_archive().unwrap();
        let result = dict.split("2cat8dog", &SplitOptions::default().threshold(1));

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.as_ref().unwrap().len(), 2);
        assert_eq!(result.as_ref().unwrap()[0].term, "cat");
        assert_eq!(result.as_ref().unwrap()[1].term, "dog");
    }
}
