mod helpers;

mod dump_tests {

    use std::str::FromStr;

    use insta::assert_snapshot;

    use odict::{
        format::{json::ToJSON, xml::ToXML},
        Dictionary, ID,
    };

    use crate::helpers::EXAMPLE_DICT_1;

    #[test]
    fn test_xml() {
        let dict = EXAMPLE_DICT_1.deserialize().unwrap();

        let dumped = dict.clone().to_xml(true).unwrap();

        let mut dictionary2 = Dictionary::from_str(dumped.as_str()).unwrap();

        dictionary2.id = dict.id.clone();

        assert_eq!(dict, dictionary2);
    }

    #[test]
    fn test_json() {
        let mut mutable = EXAMPLE_DICT_1.deserialize().unwrap().clone();

        // Keep it consistent
        mutable.id = ID::parse("2ee2a1ae-f7ff-4590-ba2d-de857ba7857f").unwrap();

        let dumped = mutable.to_json(true).unwrap();

        assert_snapshot!(dumped);
    }
}
