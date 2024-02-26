#[cfg(test)]
mod merge_tests {
    use map_macro::hash_map;

    use odict::{
        Definition, DefinitionType, Dictionary, Entry, Etymology, PartOfSpeech, Sense, ID,
    };

    #[test]
    fn test_merge() {
        let dict1 = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_map! {
              String::from("dog") => Entry {
                term: "dog".to_string(),
                see_also: None,
                etymologies: vec![
                  Etymology {
                    id:None,
                    description:None,
                    pronunciation:None,
                    senses: hash_map! {
                      PartOfSpeech::n => Sense {
                        pos: PartOfSpeech::n,
                        definitions: vec![
                        DefinitionType::Definition(
                            Definition {
                              id: None,
                              value: "some definition".into(),
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

        let dict2 = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_map! {
              "cat".to_string()=>
              Entry {
                  see_also: None,
                  term: "cat".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciation: None,
                    description: None,
                      senses:hash_map! {
                          PartOfSpeech::n=>
                          Sense {
                              pos: PartOfSpeech::n,
                              definitions: vec![
                                DefinitionType::Definition(
                                  Definition {
                                    id: None,
                                    value: "some other definition".into(),
                                    examples: vec![],
                                    notes: vec![],
                                }
                              )
                            ],
                          },
                        }
                  }],
              },
            },
        };

        let expected = Dictionary {
            id: ID::new(),
            name: None,
            entries: hash_map! {
              "dog".to_string() => Entry {
                  see_also: None,
                  term: "dog".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciation: None,
                    description: None,
                      senses:hash_map! {
                        PartOfSpeech::n=>
                        Sense {
                            pos: PartOfSpeech::n,
                            definitions: vec![
                              DefinitionType::Definition(
                                Definition {
                                  id: None,
                                  value: "some definition".into(),
                                  examples: vec![],
                                  notes: vec![],
                              }
                            )
                          ],
                        },
                      }
                  }],
              },
              "cat".to_string() => Entry {
                  see_also: None,
                  term: "cat".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciation: None,
                    description: None,
                      senses:hash_map! {
                          PartOfSpeech::n=>
                          Sense {
                              pos: PartOfSpeech::n,
                              definitions: vec![DefinitionType::Definition(
                                Definition{
                                  id: None,
                                  value: "some other definition".into(),
                                  examples: vec![],
                                  notes: vec![],
                                }
                              )],
                          },
                        }
                  }],
              },
            },
        };

        let mut result = dict1;

        result.merge(&dict2);
        result.id = expected.id.clone(); // Prevent failures based solely on UUIDs

        assert_eq!(expected, result);
    }
}
