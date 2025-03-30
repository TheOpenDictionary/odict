mod helpers;

#[cfg(test)]
mod tokenize_tests {

    use odict::tokenize::TokenizeOptions;

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_tokenize() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();
        let result = dict.tokenize("catdog run", &TokenizeOptions::default().threshold(1));
        let res = result.as_ref().unwrap();

        println!("{:?}", res);
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
}
