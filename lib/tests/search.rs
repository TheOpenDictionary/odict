mod helpers;

#[cfg(test)]
mod index_tests {

    use insta::assert_snapshot;
    use odict::json::ToJSON;
    use odict::search::{IndexOptions, SearchOptions};

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_search() {
        let dict = EXAMPLE_DICT_1.to_dictionary().unwrap();
        let dir = ".odict/.idx";

        dict.index(IndexOptions::default().dir(dir)).unwrap();

        let result = dict
            .search("a dog", SearchOptions::default().dir(dir))
            .expect("Search failed");

        assert_snapshot!(result.to_json(true).unwrap());
    }
}
