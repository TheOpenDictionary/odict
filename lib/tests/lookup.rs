mod helpers;

#[cfg(test)]
mod lookup_tests {

    use insta::assert_snapshot;
    use odict::{
        format::json::ToJSON,
        lookup::{LookupOptions, LookupStrategy},
    };
    use std::str::FromStr;

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

        // Test with follow=false (no following)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(false))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "alias1");
        assert_eq!(result[0].directed_from.is_none(), true);

        // Test with follow=true (follows until entry with etymologies found)
        // Should follow alias1 -> alias2 -> target and stop at target since it has etymologies
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(true))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias1");

        // Test starting from alias2 should also reach target
        let result = dict
            .lookup(&vec!["alias2"], LookupOptions::default().follow(true))
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias2");
    }

    #[test]
    fn test_lookup_redirect_loop_detection() {
        // Create a dictionary with circular redirects: loop1 -> loop2 -> loop1
        let xml = r#"
        <dictionary>
            <entry term="loop1" see="loop2" />
            <entry term="loop2" see="loop1" />
        </dictionary>
        "#;

        let dict = odict::schema::Dictionary::from_str(xml).unwrap();

        // Test that circular redirects are detected and return an error
        let result = dict.lookup(&vec!["loop1"], LookupOptions::default().follow(true));

        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();

        assert_eq!(
            error_message,
            "Redirect loop detected: loop1 -> loop2 -> loop1"
        );

        assert!(error_message.contains("loop1"));
        assert!(error_message.contains("loop2"));
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

    #[test]
    fn test_lookup_redirect_case_insensitive() {
        // Create a dictionary with redirects where case differs in queries
        let xml = r#"
        <dictionary>
            <entry term="target">
                <ety>
                    <sense pos="n">
                        <definition value="The final destination" />
                    </sense>
                </ety>
            </entry>
            <entry term="alias" see="target" />
        </dictionary>
        "#;

        let dict = odict::schema::Dictionary::from_str(xml).unwrap();

        // Test case insensitive redirect following with uppercase query
        let result = dict
            .lookup(
                &vec!["ALIAS"],
                LookupOptions::default().follow(true).insensitive(true),
            )
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias");

        // Test case insensitive redirect following with mixed case query
        let result = dict
            .lookup(
                &vec!["Alias"],
                LookupOptions::default().follow(true).insensitive(true),
            )
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias");

        // Test that case sensitive mode doesn't find mismatched case
        let result = dict
            .lookup(
                &vec!["ALIAS"],
                LookupOptions::default().follow(true).insensitive(false),
            )
            .unwrap();
        assert_eq!(result.len(), 0);

        // Test that exact case match works in case sensitive mode
        let result = dict
            .lookup(
                &vec!["alias"],
                LookupOptions::default().follow(true).insensitive(false),
            )
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
    }
}
