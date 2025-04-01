mod helpers;

mod markdown_tests {

    use insta::assert_snapshot;

    use odict::{format::md::ToMarkdown, lookup::LookupOptions};

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_markdown() {
        let dict = EXAMPLE_DICT_1.to_dictionary().unwrap();

        let run = dict.lookup(&vec!["dog"], LookupOptions::default()).unwrap();

        let markdown = run[0].entry.to_markdown().unwrap();

        assert_snapshot!(markdown);
    }
}
