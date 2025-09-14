mod helpers;

#[cfg(test)]
mod index_tests {

    use std::sync::LazyLock;

    use odict::{index::IndexOptions, schema::Dictionary};

    use crate::helpers::get_example_dict;

    static EXAMPLE_DICT: LazyLock<Dictionary> = LazyLock::new(|| {
        get_example_dict("example1")
            .expect("Failed to get example dictionary")
            .contents()
            .unwrap()
            .deserialize()
            .unwrap()
    });

    #[test]
    fn test_index() {
        let opts = IndexOptions::default().dir(".odict/.idx");
        let result1 = EXAMPLE_DICT.index(&opts);

        assert!(result1.is_ok());

        let result2 = EXAMPLE_DICT.index(&opts);

        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err().to_string(), "Index already exists");
    }
}
