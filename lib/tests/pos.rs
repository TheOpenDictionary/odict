#[cfg(test)]
mod pos_tests {

    use std::str::FromStr;

    use indexmap::indexset;
    use insta::assert_snapshot;
    use odict::{
        format::xml::ToXML,
        schema::{Dictionary, Entry, EnumIdentifier, Etymology, PartOfSpeech, Sense},
        senseset,
    };

    #[test]
    fn test_id() {
        assert_eq!(PartOfSpeech::AdjKari.id(), "adj_kari");
        assert_eq!(PartOfSpeech::Other("custom".into()).to_string(), "custom");
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", PartOfSpeech::AdjKari),
            "'kari' adjective (archaic)"
        );
        assert_eq!(PartOfSpeech::Other("custom".into()).to_string(), "custom");
    }

    #[test]
    fn test_se_custom() {
        let mut dict = Dictionary::default();

        dict.id = "2ee2a1ae-f7ff-4590-ba2d-de857ba7857f".try_into().unwrap();

        dict.entries.insert(Entry {
            term: "dog".into(),
            etymologies: vec![Etymology {
                description: None,
                senses: senseset![Sense {
                    pos: PartOfSpeech::Other("cusTom".into()),
                    ..Sense::default()
                }],
                ..Etymology::default()
            }],
            ..Entry::default()
        });

        assert_snapshot!(dict.to_xml(true).unwrap());
    }

    #[test]
    fn test_de_custom() {
        let xml = "
          <dictionary>
            <entry term=\"dog\">
              <ety>
                <sense pos=\"custom\" />
              </ety>
            </entry>
          </dictionary>
        ";

        let dict = Dictionary::from_str(xml).unwrap();

        let expected = PartOfSpeech::Other("custom".into());

        let sense = dict.entries.get("dog").unwrap().etymologies[0]
            .senses
            .get(&expected);

        assert!(sense.is_some());
        assert_eq!(sense.unwrap().pos, expected);
    }

    #[test]
    fn test_se() {
        let mut dict = Dictionary::default();

        dict.id = "2ee2a1ae-f7ff-4590-ba2d-de857ba7857f".try_into().unwrap();

        dict.entries.insert(Entry {
            term: "dog".into(),
            etymologies: vec![Etymology {
                senses: senseset![Sense {
                    pos: PartOfSpeech::AdjKari,
                    ..Sense::default()
                }],
                ..Etymology::default()
            }],
            ..Entry::default()
        });

        assert_snapshot!(dict.to_xml(true).unwrap());
    }

    #[test]
    fn test_de() {
        let xml = "
          <dictionary>
            <entry term=\"dog\">
              <ety>
                <sense pos=\"adj_kari\" />
              </ety>
            </entry>
          </dictionary>
        ";

        let dict = Dictionary::from_str(xml).unwrap();

        let sense = dict.entries.get("dog").unwrap().etymologies[0]
            .senses
            .get(&PartOfSpeech::AdjKari);

        assert!(sense.is_some());
        assert_eq!(sense.unwrap().pos, PartOfSpeech::AdjKari);
    }
}
