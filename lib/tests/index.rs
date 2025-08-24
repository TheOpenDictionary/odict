mod helpers;

#[cfg(test)]
mod index_tests {

    use std::sync::LazyLock;

    use odict::{schema::Dictionary, search::IndexOptions};

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

        assert_eq!(result1.is_err(), false);

        let result2 = EXAMPLE_DICT.index(&opts);

        assert_eq!(result2.is_err(), true);
        assert_eq!(result2.unwrap_err().to_string(), "Index already exists");
    }
}
