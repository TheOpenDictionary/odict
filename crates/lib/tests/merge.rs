#[cfg(test)]
mod merge_tests {

    use odict::{
        entryset,
        schema::{
            Definition, DefinitionType, Dictionary, Entry, Etymology, PartOfSpeech, Sense, ID,
        },
        senseset,
    };

    #[test]
    fn test_merge() {
        let dict1 = Dictionary {
            id: ID::new(),
            name: None,
            entries: entryset![Entry {
                rank: None,
                media: vec![],
                term: "dog".to_string(),
                see_also: None,
                etymologies: vec![Etymology {
                    id: None,
                    description: None,
                    pronunciations: vec![],
                    senses: senseset![Sense {
                        tags: vec![],
                        translations: vec![],
                        forms: vec![],
                        pos: PartOfSpeech::N,
                        lemma: None,
                        definitions: vec![DefinitionType::Definition(Definition {
                            id: None,
                            value: "some definition".into(),
                            examples: vec![],
                            notes: vec![],
                        })],
                    }]
                }]
            }],
        };

        let dict2 = Dictionary {
            id: ID::new(),
            name: None,
            entries: entryset! {
              Entry {
                  rank: None,
                  media: vec![],
                  see_also: None,
                  term: "cat".to_string(),
                  etymologies: vec![Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: None,
                      senses: senseset! [
                          Sense {
                              tags: vec![],
                              pos: PartOfSpeech::N,
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
                      ]
                  }],
              },
            },
        };

        let expected = Dictionary {
            id: ID::new(),
            name: None,
            entries: entryset![
                Entry {
                    term: "dog".to_string(),
                    etymologies: vec![Etymology {
                        senses: senseset![Sense {
                            pos: PartOfSpeech::N,
                            definitions: vec![DefinitionType::Definition(Definition {
                                id: None,
                                value: "some definition".into(),
                                examples: vec![],
                                notes: vec![],
                            })],
                            ..Sense::default()
                        }],
                        ..Etymology::default()
                    }],
                    ..Entry::default()
                },
                Entry {
                    term: "cat".to_string(),
                    etymologies: vec![Etymology {
                        senses: senseset![Sense {
                            pos: PartOfSpeech::N,
                            definitions: vec![DefinitionType::Definition(Definition {
                                id: None,
                                value: "some other definition".into(),
                                examples: vec![],
                                notes: vec![],
                            })],
                            ..Sense::default()
                        }],
                        ..Etymology::default()
                    }],
                    ..Entry::default()
                },
            ],
        };

        let mut result = dict1;

        result.merge(&dict2);
        result.id = expected.id.clone(); // Prevent failures based solely on UUIDs

        assert_eq!(expected, result);
    }
}
