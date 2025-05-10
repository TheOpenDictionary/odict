#[cfg(test)]
mod pos_tests {

    use insta::assert_snapshot;
    use map_macro::hash_map;
    use odict::{format::xml::ToXML, Dictionary, Entry, Etymology, PartOfSpeech, Sense};

    #[test]
    fn test_display() {
        assert_eq!(
            PartOfSpeech::AdjKari.to_string(),
            "'kari' adjective (archaic)"
        );
        assert_eq!(PartOfSpeech::Other("custom".into()).to_string(), "custom");
    }

    #[test]
    fn test_se_custom() {
        let mut dict = Dictionary::default();

        dict.id = "2ee2a1ae-f7ff-4590-ba2d-de857ba7857f".try_into().unwrap();

        dict.entries.insert(
            "dog".into(),
            Entry {
                term: "dog".into(),
                see_also: None,
                etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                    senses: hash_map! {
                      PartOfSpeech::Other("cusTom".into()) => Sense {
                        lemma: None,
                        definitions: vec![],
                        tags: vec![],
                        translations: vec![],
                        forms: vec![],
                        pos: PartOfSpeech::Other("cusTom".into())
                      }
                    },
                }],
            },
        );

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

        let dict = Dictionary::try_from(xml).unwrap();

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

        dict.entries.insert(
            "dog".into(),
            Entry {
                term: "dog".into(),
                see_also: None,
                etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                    senses: hash_map! {
                      PartOfSpeech::AdjKari => Sense {
                        lemma: None,
                        definitions: vec![],
                        tags: vec![],
                        translations: vec![],
                        forms: vec![],
                        pos: PartOfSpeech::AdjKari
                      }
                    },
                }],
            },
        );

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

        let dict = Dictionary::try_from(xml).unwrap();

        let sense = dict.entries.get("dog").unwrap().etymologies[0]
            .senses
            .get(&PartOfSpeech::AdjKari);

        assert!(sense.is_some());
        assert_eq!(sense.unwrap().pos, PartOfSpeech::AdjKari);
    }
}
