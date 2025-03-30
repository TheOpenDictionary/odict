mod helpers;

#[cfg(test)]
mod tokenize_tests {

    use odict::tokenize::{Language, TokenizeOptions};

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_tokenize() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize("catdog run", &TokenizeOptions::default().threshold(1));
        let res = result.as_ref().unwrap();

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), 2);

        assert_eq!(res[0].entries.len(), 2);
        assert_eq!(res[0].entries[0].term, "cat");
        assert_eq!(res[0].entries[1].term, "dog");
        assert_eq!(res[0].lemma, "catdog");

        assert_eq!(res[1].entries.len(), 1);
        assert_eq!(res[1].entries[0].term, "run");
        assert_eq!(res[1].lemma, "run");
    }

    #[test]
    fn test_tokenize_chinese() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize("你不知道的事", &TokenizeOptions::default().threshold(1));
        let res = result.as_ref().unwrap();
        let expected_lemmas = ["你", "不", "知道", "的", "事"];

        assert_eq!(result.is_ok(), true);
        assert_eq!(res.len(), expected_lemmas.len());

        for (i, token) in res.iter().enumerate() {
            assert_eq!(token.language, Some("cmn".to_string()));
            assert_eq!(token.lemma, expected_lemmas[i]);
        }
    }

    #[test]
    fn test_tokenize_accents() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize(
            "Chào bạn, hôm nay trời đẹp quá!",
            &TokenizeOptions::default().threshold(1).allow_list(&[
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
            assert_eq!(token.language, Some("vie".to_string()));
            assert_eq!(token.lemma, expected_lemmas[i]);
        }
    }
}
