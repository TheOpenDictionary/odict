mod helpers;

#[cfg(test)]
mod index_tests {
    use super::helpers::EXAMPLE_DICTIONARY_1;
    use insta::assert_snapshot;
    use odict::{SQLDialect, ToSQL, ID};

    #[test]
    fn test_sql() {
        let mut dict = EXAMPLE_DICTIONARY_1.to_dictionary().unwrap();

        dict.id = ID::parse("2ee2a1ae-f7ff-4590-ba2d-de857ba7857f").unwrap();

        let result = dict.to_sql(SQLDialect::Postgres);

        assert_eq!(result.is_err(), false);
        assert_snapshot!(result.unwrap());
    }
}
