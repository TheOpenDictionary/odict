mod helpers;

#[cfg(test)]
mod lookup_tests {

    use insta::assert_snapshot;
    use odict::{format::json::ToJSON, lookup::LookupOptions};

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
            .lookup(&vec!["catdog"], LookupOptions::default().split(3))
            .unwrap();

        assert_eq!(result[0][0].term, "cat");
        assert_eq!(result[0][1].term, "dog");
    }

    #[test]
    fn test_lookup_fallback() {
        let dict = EXAMPLE_DICT_1.to_archive().unwrap();

        let result1 = dict
            .lookup(&vec!["catdog(run)"], LookupOptions::default())
            .unwrap();

        assert_eq!(result1[0][0].term, "run");

        let result2 = dict
            .lookup(&vec!["(run)"], LookupOptions::default())
            .unwrap();

        assert_eq!(result2[0][0].term, "run");
    }

    #[test]
    fn test_lookup_follow() {
        let dict = EXAMPLE_DICT_2.to_archive().unwrap();

        let control = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default().split(2).follow(false),
            )
            .unwrap();

        assert_eq!(control.len(), 1);
        assert_eq!(control[0][0].term, "runners");

        let basic = dict
            .lookup(
                &vec!["runners"],
                LookupOptions::default().split(2).follow(true),
            )
            .unwrap();

        assert_eq!(basic.len(), 1);
        assert_eq!(basic[0][0].term, "runner");

        let fallback = dict
            .lookup(
                &vec!["unfindable (runners)"],
                LookupOptions::default().split(2).follow(true),
            )
            .unwrap();

        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0][0].term, "runner");
    }
}
