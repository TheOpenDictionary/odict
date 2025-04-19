mod helpers;

#[cfg(test)]
mod resolve_tests {
    use map_macro::hash_map;

    use odict::{
        Definition, DefinitionType, Dictionary, Entry, EntryRef, Etymology, PartOfSpeech, Sense, ID,
    };

    #[test]
    fn test_resolve() {
        let dict = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_map! {
              String::from("dog") => Entry {
                lemma: None,
                forms: vec![],
                term: "dog".to_string(),
                see_also: None,
                etymologies: vec![
                  Etymology {
                    id: None,
                    description: None,
                    pronunciation: None,
                    senses: hash_map! {
                      PartOfSpeech::n => Sense {
                        pos: PartOfSpeech::n,
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
                      }
                    }
                  }
                ]
              }
            },
        };

        // Test resolving an existing entry
        let dog_ref = EntryRef::new("dog");
        let dog_entry = dict.resolve(&dog_ref);
        assert!(dog_entry.is_some());
        assert_eq!(dog_entry.unwrap().term, "dog");

        // Test resolving a non-existent entry
        let cat_ref = EntryRef::new("cat");
        let cat_entry = dict.resolve(&cat_ref);
        assert!(cat_entry.is_none());
    }
}
