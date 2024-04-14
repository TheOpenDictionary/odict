mod helpers;

#[cfg(test)]
mod index_tests {
    use super::helpers::EXAMPLE_DICTIONARY_1;
    use insta::assert_snapshot;
    use odict::{SQLDialect, ToSQL};
    use regex::Regex;

    #[test]
    fn test_sql() {
        let dict = EXAMPLE_DICTIONARY_1.to_dictionary().unwrap();
        let result = dict.to_sql(SQLDialect::Postgres);
        let re = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();

        assert_eq!(result.is_err(), false);
        assert_snapshot!(re.replace_all(result.unwrap().as_str(), ""));
    }
}
