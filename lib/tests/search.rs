mod helpers;

#[cfg(test)]
mod index_tests {

    use super::helpers::EXAMPLE_DICTIONARY_1;
    use insta::assert_snapshot;
    use odict::dump::ToJSON;
    use odict::search::{IndexOptions, SearchOptions};

    #[test]
    fn test_search() {
        let dict = EXAMPLE_DICTIONARY_1.to_dictionary().unwrap();
        let dir = ".odict/.idx";

        dict.index(IndexOptions::default().dir(dir)).unwrap();

        let result = dict.search("a dog", SearchOptions::default().dir(dir));

        assert_eq!(result.is_err(), false);
        assert_snapshot!(result.unwrap().to_json(true).unwrap());
    }
}
