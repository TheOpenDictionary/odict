mod helpers;

#[cfg(test)]
mod lexicon_tests {
    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_lexicon() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.lexicon();
        assert_eq!(result, vec!["cat", "dog", "poo", "ran", "run"]);
    }
}
