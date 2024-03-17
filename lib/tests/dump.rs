mod helpers;

mod dump_tests {

    use insta::assert_snapshot;

    use odict::{
        dump::{ToJSON, ToXML},
        Dictionary, ID,
    };

    use crate::helpers::EXAMPLE_DICTIONARY_1;

    #[test]
    fn test_xml() {
        let dictionary1 = EXAMPLE_DICTIONARY_1.to_dictionary().unwrap();
        let dumped = dictionary1.clone().to_xml(true).unwrap();
        let mut dictionary2 = Dictionary::from(dumped.as_str()).unwrap();

        dictionary2.id = dictionary1.id.clone();

        assert_eq!(dictionary1, dictionary2);
    }

    #[test]
    fn test_json() {
        let dictionary1 = EXAMPLE_DICTIONARY_1.to_dictionary().unwrap();
        let mut mutable = dictionary1.clone();

        // Keep it consistent
        mutable.id = ID::parse("2ee2a1ae-f7ff-4590-ba2d-de857ba7857f").unwrap();

        let dumped = mutable.to_json(true).unwrap();

        assert_snapshot!(dumped);
    }
}
