mod helpers;

#[cfg(test)]
mod index_tests {

    use insta::assert_snapshot;
    use odict::format::json::ToJSON;
    use odict::index::IndexOptions;
    use odict::search::SearchOptions;

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_search() {
        let dict = EXAMPLE_DICT_1.contents().unwrap().deserialize().unwrap();
        let dir = ".odict/.idx";

        dict.index(IndexOptions::default().dir(dir)).unwrap();

        let result = dict
            .search("a dog", SearchOptions::default().dir(dir))
            .expect("Search failed");

        assert_snapshot!(result.to_json(true).unwrap());
    }
}
