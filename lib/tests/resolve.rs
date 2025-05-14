mod helpers;

#[cfg(test)]
mod resolve_tests {
    use map_macro::hash_set;

    use odict::{
        Definition, DefinitionType, Dictionary, Entry, Etymology, PartOfSpeech, Sense, ID,
    };

    #[test]
    fn test_resolve() {
        let dict = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_set! {
              Entry {
                rank: None,
                media: vec![],
                term: "dog".to_string(),
                see_also: None,
                etymologies: vec![
                  Etymology {
                    id: None,
                    description: None,
                    pronunciations: vec![],
                    senses: hash_set![ Sense {
                        pos: PartOfSpeech::N,
                        lemma: None,
                        forms: vec![],
                        translations: vec![],
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
                      }]
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
