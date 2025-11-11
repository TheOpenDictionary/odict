use indexmap::indexset;
use odict::format::json::ToJSON;
use odict::{
    schema::{DefinitionType, Entry, Example, Translation},
    ToDictionary,
};

#[test]
fn test_translation_in_example() {
    // Create an example with translations
    let example = Example {
        pronunciations: vec![],
        value: "This is an example sentence.".to_string(),
        translations: vec![
            Translation {
                lang: "es".to_string(),
                value: "Esta es una frase de ejemplo.".to_string(),
            },
            Translation {
                lang: "fr".to_string(),
                value: "C'est une phrase d'exemple.".to_string(),
            },
        ],
    };

    // Test that translations are correctly stored
    assert_eq!(example.translations.len(), 2);
    assert_eq!(example.translations[0].lang, "es");
    assert_eq!(
        example.translations[0].value,
        "Esta es una frase de ejemplo."
    );
    assert_eq!(example.translations[1].lang, "fr");
    assert_eq!(example.translations[1].value, "C'est une phrase d'exemple.");
}

#[test]
fn test_translation_in_entry() {
    // Create translations
    let translations = vec![
        Translation {
            lang: "es".to_string(),
            value: "hola".to_string(),
        },
        Translation {
            lang: "fr".to_string(),
            value: "bonjour".to_string(),
        },
    ];

    // Create a sense with translations
    let sense = odict::schema::Sense {
        pos: odict::schema::PartOfSpeech::N,
        lemma: None,
        definitions: vec![],
        tags: vec![],
        translations: translations.clone(),
        forms: vec![],
    };

    // Create an etymology with the sense
    let mut senses = indexset![];

    senses.insert(sense);

    let etymology = odict::schema::Etymology {
        id: None,
        pronunciations: vec![],
        description: None,
        senses,
    };

    // Create an entry with the etymology
    let entry = Entry {
        media: vec![],
        rank: None,
        term: "hello".to_string(),
        see_also: None,
        etymologies: vec![etymology],
    };

    // Test that translations are correctly stored
    let stored_translations = &entry.etymologies[0]
        .senses
        .get(&odict::schema::PartOfSpeech::N)
        .unwrap()
        .translations;
    assert_eq!(stored_translations.len(), 2);
    assert_eq!(stored_translations[0].lang, "es");
    assert_eq!(stored_translations[0].value, "hola");
    assert_eq!(stored_translations[1].lang, "fr");
    assert_eq!(stored_translations[1].value, "bonjour");
}

#[test]
fn test_xml_serialization() {
    // Define a simple XML with translations - now translations should be inside a sense
    let xml = r#"<dictionary>
        <entry term="hello">
            <ety>
                <sense pos="n">
                    <translation lang="es" value="hola" />
                    <translation lang="fr" value="bonjour" />
                </sense>
            </ety>
        </entry>
    </dictionary>"#;

    // Parse the XML
    let dictionary = xml.to_dictionary().unwrap();

    // Get the entry by key from the HashMap
    let entry = dictionary.entries.get("hello").unwrap();

    // Get the translations from the first etymology's first sense
    let translations = &entry.etymologies[0]
        .senses
        .iter()
        .next()
        .unwrap()
        .translations;

    // Test that translations are parsed correctly
    assert_eq!(translations.len(), 2);
    assert_eq!(translations[0].lang, "es");
    assert_eq!(translations[0].value, "hola");
    assert_eq!(translations[1].lang, "fr");
    assert_eq!(translations[1].value, "bonjour");
}

#[test]
fn test_example_with_translation() {
    // Define a simple XML with examples containing translations
    let xml = r#"<dictionary>
        <entry term="example">
            <ety>
                <sense>
                    <definition value="A sample">
                      <example value="This is an example.">
                          <translation lang="es" value="Este es un ejemplo." />
                          <translation lang="fr" value="C'est un exemple." />
                      </example>
                    </definition>
                </sense>
            </ety>
        </entry>
    </dictionary>"#;

    // Parse the XML
    let dictionary = xml.to_dictionary().unwrap();

    // Navigate to the example - correctly access the HashMap structure
    let entry = dictionary.entries.get("example").unwrap();

    // Get examples using the definition's examples() method
    if let Some(definition) = entry.etymologies[0]
        .senses
        .iter()
        .next()
        .and_then(|sense| sense.definitions.first())
        .and_then(|def_type| match def_type {
            DefinitionType::Definition(def) => Some(def),
            _ => None,
        })
    {
        let examples = &definition.examples;
        assert_eq!(examples.len(), 1);

        let example = &examples[0];

        // Test that translations are parsed correctly from examples
        assert_eq!(example.translations.len(), 2);
        assert_eq!(example.translations[0].lang, "es");
        assert_eq!(example.translations[0].value, "Este es un ejemplo.");
        assert_eq!(example.translations[1].lang, "fr");
        assert_eq!(example.translations[1].value, "C'est un exemple.");
    } else {
        panic!("Could not find example in dictionary");
    }
}

#[test]
fn test_json_format() {
    // Define a simple XML with translations now in the sense structure
    let xml = r#"<dictionary>
        <entry term="hello">
            <ety>
                <sense pos="n">
                    <translation lang="es" value="hola" />
                    <translation lang="fr" value="bonjour" />
                </sense>
            </ety>
        </entry>
    </dictionary>"#;

    // Parse the XML
    let dictionary = xml.to_dictionary().unwrap();

    // Convert to JSON
    let json = dictionary.to_json(false).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Access entry translations through the entries array
    if let Some(entries) = json_value["entries"].as_object() {
        let entry = entries.get("hello").unwrap();

        // Validate translations in the sense structure
        if let Some(etymologies) = entry["etymologies"].as_array() {
            if let Some(senses) = etymologies[0]["senses"].as_array() {
                let translations = &senses[0]["translations"];

                // Validate entry translations
                assert_eq!(translations.as_array().unwrap().len(), 2);
                assert_eq!(translations[0]["lang"], "es");
                assert_eq!(translations[0]["value"], "hola");
                assert_eq!(translations[1]["lang"], "fr");
                assert_eq!(translations[1]["value"], "bonjour");
            }
        }

        // The rest of the test for example translations remains the same
        if let Some(etymologies) = entry["etymologies"].as_array() {
            if let Some(senses) = etymologies[0]["senses"].as_array() {
                if let Some(definitions) = senses[0]["definitions"].as_array() {
                    if let Some(examples) = definitions[0]["examples"].as_array() {
                        let example_translations = &examples[0]["translations"];

                        assert_eq!(example_translations[0]["lang"], "es");
                        assert_eq!(example_translations[0]["value"], "Hola, mundo!");
                        assert_eq!(example_translations[1]["lang"], "fr");
                        assert_eq!(example_translations[1]["value"], "Bonjour, monde!");
                    }
                }
            }
        }
    }
}
