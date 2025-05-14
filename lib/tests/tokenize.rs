mod helpers;

#[cfg(test)]
mod tokenize_tests {

    use std::str::FromStr;

    use odict::lookup::{Language, TokenizeOptions};

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_tokenize() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize("poo run", &TokenizeOptions::default());
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 2);

        assert_eq!(res[0].entries.len(), 1);
        assert_eq!(res[0].entries[0].entry.term, "poo");
        assert_eq!(res[0].lemma, "poo");

        assert_eq!(res[1].entries.len(), 1);
        assert_eq!(res[1].entries[0].entry.term, "run");
        assert_eq!(res[1].lemma, "run");
    }

    #[test]
    fn test_tokenize_chinese() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize("你不知道的事", &TokenizeOptions::default());
        let res = result.as_ref().unwrap();
        let expected_lemmas = ["你", "不", "知道", "的", "事"];

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), expected_lemmas.len());

        for (i, token) in res.iter().enumerate() {
            assert_eq!(token.language, Some(Language::Cmn));
            assert_eq!(token.lemma, expected_lemmas[i]);
        }
    }

    #[test]
    fn test_tokenize_accents() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize(
            "Chào bạn, hôm nay trời đẹp quá!",
            &TokenizeOptions::default().allow_list(vec![
                Language::Vie,
                Language::Cmn,
                Language::Eng,
            ]),
        );

        let res = result.as_ref().unwrap();
        let expected_lemmas = ["Chào", "bạn", "hôm", "nay", "trời", "đẹp", "quá"];

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), expected_lemmas.len());

        for (i, token) in res.iter().enumerate() {
            assert_eq!(token.language, Some(Language::Vie));
            assert_eq!(token.lemma, expected_lemmas[i]);
        }
    }

    #[test]
    fn test_tokenize_case_sensitive() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        // Default behavior (case-sensitive)
        let result = dict.tokenize("DOG run", &TokenizeOptions::default().insensitive(false));
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 2);

        // "DOG" should not match "dog", so entry list should be empty
        assert_eq!(res[0].entries.len(), 0);
        assert_eq!(res[0].lemma, "DOG");

        // "run" matches exactly
        assert_eq!(res[1].entries.len(), 1);
        assert_eq!(res[1].entries[0].entry.term, "run");
    }

    #[test]
    fn test_tokenize_case_insensitive() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        // With case insensitivity enabled
        let result = dict.tokenize("DOG RUN", &TokenizeOptions::default().insensitive(true));
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 2);

        // "DOG" should match "dog" with insensitive flag
        assert_eq!(res[0].entries.len(), 1);
        assert_eq!(res[0].entries[0].entry.term, "dog");
        assert_eq!(res[0].lemma, "DOG");

        // "RUN" should match "run" with insensitive flag
        assert_eq!(res[1].entries.len(), 1);
        assert_eq!(res[1].entries[0].entry.term, "run");
        assert_eq!(res[1].lemma, "RUN");
    }

    #[test]
    fn test_tokenize_mixed_case() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        // Test with mixed case text
        let result = dict.tokenize("DoG CaT", &TokenizeOptions::default().insensitive(true));
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 2);

        assert_eq!(res[0].entries.len(), 1);
        assert_eq!(res[0].entries[0].entry.term, "dog");
        assert_eq!(res[0].lemma, "DoG");

        assert_eq!(res[1].entries.len(), 1);
        assert_eq!(res[1].entries[0].entry.term, "cat");
        assert_eq!(res[1].lemma, "CaT");
    }

    #[test]
    fn test_tokenize_insensitive_with_follow() {
        // Create a dictionary with aliases for this test
        let xml = r#"
        <dictionary>
            <entry term="run">
                <definition>To move quickly</definition>
            </entry>
            <entry term="runs" see="run" />
        </dictionary>
        "#;

        let dict = odict::Dictionary::from_str(xml).unwrap();

        // Test case insensitivity combined with follow option
        let result = dict.tokenize(
            "RUNS",
            &TokenizeOptions::default().follow(true).insensitive(true),
        );
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 1);

        // Should find "runs" and follow to "run"
        assert_eq!(res[0].entries.len(), 1);
        assert_eq!(res[0].entries[0].entry.term, "run");
        assert_eq!(res[0].entries[0].directed_from.is_some(), true);
        assert_eq!(res[0].entries[0].directed_from.unwrap().term, "runs");
    }
}
