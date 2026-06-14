mod helpers;

#[cfg(test)]
mod lookup_tests {

    use insta::assert_snapshot;
    use odict::{
        format::json::ToJSON,
        lookup::{LookupOptions, LookupStrategy},
    };

    use crate::helpers::{EXAMPLE_DICT_1, EXAMPLE_DICT_2};

    #[test]
    fn test_lookup() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .lookup(&vec!["dog", "cat"], LookupOptions::default())
            .unwrap();

        assert_snapshot!(result.to_json(true).unwrap());
    }

    #[test]
    fn test_lookup_splitting() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        let result = dict
            .lookup(
                &vec!["catdog"],
                LookupOptions::default().strategy(LookupStrategy::Split(3)),
            )
            .unwrap();

        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_lookup_follow() {
        let dict = EXAMPLE_DICT_2.contents().unwrap();

        let control = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default().strategy(LookupStrategy::Split(2)),
            )
            .unwrap();

        assert_eq!(control.len(), 1);
        assert_eq!(control[0].entry.term, "runners");

        let basic = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default()
                    .strategy(LookupStrategy::Split(2))
                    .follow(true),
            )
            .unwrap();

        assert_eq!(basic.len(), 1);
        assert_eq!(basic[0].directed_from.is_some(), true);
        assert_eq!(basic[0].directed_from.unwrap().term, "runners");
        assert_eq!(basic[0].entry.term, "runner");

        let fallback = dict
            .lookup(
                &vec!["unfindable (runners)"],
                LookupOptions::default()
                    .strategy(LookupStrategy::Split(2))
                    .follow(true),
            )
            .unwrap();

        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0].entry.term, "runner");
    }

    #[test]
    fn test_lookup_case_sensitive() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // Should find "dog" but not "DOG" with case sensitivity (default)
        let result = dict
            .lookup(&vec!["dog"], LookupOptions::default().insensitive(false))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "dog");

        let result = dict
            .lookup(&vec!["DOG"], LookupOptions::default().insensitive(false))
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_lookup_case_insensitive() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // Should find entries regardless of case when insensitive is true
        let result = dict
            .lookup(&vec!["DOG"], LookupOptions::default().insensitive(true))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "dog");

        let result = dict
            .lookup(&vec!["Cat"], LookupOptions::default().insensitive(true))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "cat");
    }

    #[test]
    fn test_lookup_case_insensitive_mixed() {
        let dict = EXAMPLE_DICT_1.contents().unwrap();

        // Test mixed-case queries
        let result = dict
            .lookup(
                &vec!["DoG", "cAt"],
                LookupOptions::default().insensitive(true),
            )
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].entry.term, "dog");
        assert_eq!(result[1].entry.term, "cat");
    }

    #[test]
    fn test_lookup_case_insensitive_with_follow() {
        let dict = EXAMPLE_DICT_2.contents().unwrap();

        // Test case insensitive lookup combined with follow option
        let result = dict
            .lookup(
                &vec!["RuNnErS"],
                LookupOptions::default().follow(true).insensitive(true),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "runners");
        assert_eq!(result[0].entry.term, "runner");
    }
}
