mod helpers;

#[cfg(test)]
mod lookup_tests {

    use insta::assert_snapshot;
    use odict::{
        format::json::ToJSON,
        lookup::{LookupOptions, LookupStrategy},
    };
    use std::{str::FromStr, u32};

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
                    .follow(u32::MAX),
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
                    .follow(u32::MAX),
            )
            .unwrap();

        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0].entry.term, "runner");
    }

    #[test]
    fn test_lookup_follow_limit() {
        // Create a dictionary with a chain of redirects: alias1 -> alias2 -> target
        let xml = r#"
        <dictionary>
            <entry term="target">
                <ety>
                    <sense pos="n">
                        <definition value="The final destination" />
                    </sense>
                </ety>
            </entry>
            <entry term="alias2" see="target" />
            <entry term="alias1" see="alias2" />
        </dictionary>
        "#;

        let dict = odict::schema::Dictionary::from_str(xml).unwrap();

        // Test with follow limit of 0 (no following)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(0))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "alias1");
        assert_eq!(result[0].directed_from.is_none(), true);

        // Test with follow limit of 1 (follows one level)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(1))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "alias2");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias1");

        // Test with follow limit of 2 (follows to the end)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(2))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias1");

        // Test with infinite follow (should reach the end)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(u32::MAX))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias1");
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
                LookupOptions::default().follow(u32::MAX).insensitive(true),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "runners");
        assert_eq!(result[0].entry.term, "runner");
    }
}
