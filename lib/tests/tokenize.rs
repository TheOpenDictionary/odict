mod helpers;

#[cfg(test)]
mod tokenize_tests {

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
}
