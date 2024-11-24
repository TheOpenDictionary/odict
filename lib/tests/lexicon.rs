mod helpers;

#[cfg(test)]
mod lexicon_tests {
    use crate::helpers::get_example_dict;

    #[test]
    fn test_lexicon() {
        let binding = get_example_dict("example1").expect("Failed to get example dictionary");
        let dict = binding.to_archive().unwrap();
        let result = dict.lexicon();
        assert_eq!(result, vec!["cat", "dog", "poo", "ran", "run"]);
    }
}
