mod helpers;

#[cfg(test)]
mod index_tests {

    use insta::assert_snapshot;
    use odict::format::sql::{SQLDialect, ToSQL};
    use regex::Regex;

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_sql() {
        let result = EXAMPLE_DICT_1
            .deserialize()
            .unwrap()
            .clone()
            .to_sql(SQLDialect::Postgres);

        let re = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
        .unwrap();

        assert_eq!(result.is_err(), false);
        assert_snapshot!(re.replace_all(result.unwrap().as_str(), ""));
    }
}
