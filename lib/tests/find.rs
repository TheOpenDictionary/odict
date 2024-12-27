mod helpers;

#[cfg(test)]
mod find_tests {
    use odict::find::FindOptions;

    use crate::helpers::get_example_dict;

    #[test]
    fn test_find_default() {
        let binding = get_example_dict("example1").expect("Failed to get example dictionary");
        let dict = binding.to_archive().unwrap();
        let result = dict
            .find(
                "R",
                FindOptions::default().case_matching(odict::find::CaseMatching::Ignore),
            )
            .iter()
            .map(|e| e.term.as_str())
            .collect::<Vec<&str>>();

        assert_eq!(result, vec!["ran", "run"]);
    }

    #[test]
    fn test_find_casing() {
        let binding = get_example_dict("example1").expect("Failed to get example dictionary");
        let dict = binding.to_archive().unwrap();
        let result = dict
            .find(
                "RA",
                FindOptions::default().case_matching(odict::find::CaseMatching::Respect),
            )
            .iter()
            .map(|e| e.term.as_str())
            .collect::<Vec<&str>>();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_normalization() {
        let binding = get_example_dict("example1").expect("Failed to get example dictionary");
        let dict = binding.to_archive().unwrap();
        let result = dict
            .find(
                "ŕ",
                FindOptions::default().normalization(odict::find::Normalization::Never),
            )
            .iter()
            .map(|e| e.term.as_str())
            .collect::<Vec<&str>>();

        assert_eq!(result.len(), 0);
    }
}
