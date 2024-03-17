mod helpers;

#[cfg(test)]
mod index_tests {

    use super::helpers::EXAMPLE_DICTIONARY_1;
    use odict::search::IndexOptions;

    #[test]
    fn test_index() {
        let archive = EXAMPLE_DICTIONARY_1.to_archive().unwrap();
        let opts = IndexOptions::default().dir(".odict/.idx");
        let result1 = archive.index(&opts);

        assert_eq!(result1.is_err(), false);

        let result2 = archive.index(&opts);

        assert_eq!(result2.is_err(), true);
        assert_eq!(result2.unwrap_err().to_string(), "Index already exists");
    }
}
