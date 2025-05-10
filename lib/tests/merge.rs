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
                rank: None,
                media: vec![],
                term: "dog".to_string(),
                see_also: None,
                etymologies: vec![
                  Etymology {
                    id:None,
                    description:None,
                    pronunciations: vec![],
                    senses: hash_map! {
                      PartOfSpeech::n => Sense {
                        tags: vec![],
                        translations: vec![],
                        forms: vec![],
                        pos: PartOfSpeech::n,
                        lemma: None,
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
                  rank: None,
                  media: vec![],
                  see_also: None,
                  term: "cat".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                      senses:hash_map! {
                          PartOfSpeech::n=>
                          Sense {
                              tags: vec![],
                              pos: PartOfSpeech::n,
                              forms: vec![],
                              translations: vec![],
                              lemma: None,
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
                  rank: None,
                  see_also: None,
                  media: vec![],
                  term: "dog".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                      senses:hash_map! {
                        PartOfSpeech::n=>
                        Sense {
                            tags: vec![],
                            pos: PartOfSpeech::n,
                            forms: vec![],
                            translations: vec![],
                            lemma: None,
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
                  rank: None,
                  media: vec![],
                  term: "cat".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                      senses:hash_map! {
                          PartOfSpeech::n=>
                          Sense {
                              tags: vec![],
                              pos: PartOfSpeech::n,
                              forms: vec![],
                              translations: vec![],
                              lemma: None,
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
