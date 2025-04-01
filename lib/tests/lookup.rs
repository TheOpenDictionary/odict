mod helpers;

#[cfg(test)]
mod lookup_tests {

    use insta::assert_snapshot;
    use odict::{
        format::json::ToJSON,
        lookup::{LookupOptions, LookupStrategy},
    };

    use crate::helpers::{EXAMPLE_DICT_1, EXAMPLE_DICT_2};

    #[test]
    fn test_lookup() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        let result = dict
            .lookup(&vec!["dog", "cat"], LookupOptions::default())
            .unwrap();

        assert_snapshot!(result.to_json(true).unwrap());
    }

    #[test]
    fn test_lookup_splitting() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        let result = dict
            .lookup(
                &vec!["catdog"],
                LookupOptions::default().strategy(LookupStrategy::Split(3)),
            )
            .unwrap();

        assert_eq!(result[0].entry.term, "cat");
        assert_eq!(result[1].entry.term, "dog");
    }

    #[test]
    fn test_lookup_follow() {
        let dict = EXAMPLE_DICT_2.to_archive().unwrap();

        let control = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default()
                    .strategy(LookupStrategy::Split(2))
                    .follow(false),
            )
            .unwrap();

        assert_eq!(control.len(), 1);
        assert_eq!(control[0].entry.term, "runners");

        let basic = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default()
                    .strategy(LookupStrategy::Split(2))
                    .follow(true),
            )
            .unwrap();

        assert_eq!(basic.len(), 1);
        assert_eq!(basic[0].directed_from.is_some(), true);
        assert_eq!(basic[0].directed_from.unwrap().term, "runners");
        assert_eq!(basic[0].entry.term, "runner");

        let fallback = dict
            .lookup(
                &vec!["unfindable (runners)"],
                LookupOptions::default()
                    .strategy(LookupStrategy::Split(2))
                    .follow(true),
            )
            .unwrap();

        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0].entry.term, "runner");
    }
}
