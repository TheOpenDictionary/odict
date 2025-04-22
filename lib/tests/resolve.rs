mod helpers;

#[cfg(test)]
mod resolve_tests {
    use map_macro::hash_map;

    use odict::{
        Definition, DefinitionType, Dictionary, Entry, Etymology, PartOfSpeech, Sense, ID,
    };

    #[test]
    fn test_resolve() {
        let dict = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_map! {
              String::from("dog") => Entry {
                forms: vec![],
                term: "dog".to_string(),
                see_also: None,
                translations: vec![],
                etymologies: vec![
                  Etymology {
                    id: None,
                    description: None,
                    pronunciation: None,
                    senses: hash_map! {
                      PartOfSpeech::n => Sense {
                        pos: PartOfSpeech::n,
                        lemma: None,
                        definitions: vec![
                        DefinitionType::Definition(
                            Definition {
                              id: None,
                              value: "A domesticated carnivorous mammal.".into(),
                              examples: vec![],
                              notes: vec![],
                            }
                          )
                        ],
                        tags: vec![],
                      }
                    }
                  }
                ]
              }
            },
        };

        // Test resolving an existing entry
        let dog_entry = dict.resolve("dog");
        assert!(dog_entry.is_some());
        assert_eq!(dog_entry.unwrap().term, "dog");

        // Test resolving a non-existent entry
        let cat_entry = dict.resolve("cat");
        assert!(cat_entry.is_none());
    }
}
