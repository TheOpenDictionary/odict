use odict::format::json::ToJSON;
use odict::{DefinitionType, Entry, Example, ToDictionary, Translation};

#[test]
fn test_translation_in_example() {
    // Create an example with translations
    let example = Example {
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
    // Create an entry with translations
    let entry = Entry {
        term: "hello".to_string(),
        see_also: None,
        etymologies: vec![],
        forms: vec![],
        translations: vec![
            Translation {
                lang: "es".to_string(),
                value: "hola".to_string(),
            },
            Translation {
                lang: "fr".to_string(),
                value: "bonjour".to_string(),
            },
        ],
    };

    // Test that translations are correctly stored
    assert_eq!(entry.translations.len(), 2);
    assert_eq!(entry.translations[0].lang, "es");
    assert_eq!(entry.translations[0].value, "hola");
    assert_eq!(entry.translations[1].lang, "fr");
    assert_eq!(entry.translations[1].value, "bonjour");
}

#[test]
fn test_xml_serialization() {
    // Define a simple XML with translations
    let xml = r#"<dictionary>
        <entry term="hello">
            <translation lang="es" value="hola" />
            <translation lang="fr" value="bonjour" />
        </entry>
    </dictionary>"#;

    // Parse the XML
    let dictionary = xml.to_dictionary().unwrap();

    // Get the entry by key from the HashMap
    let entry = dictionary.entries.get("hello").unwrap();

    // Test that translations are parsed correctly
    assert_eq!(entry.translations.len(), 2);
    assert_eq!(entry.translations[0].lang, "es");
    assert_eq!(entry.translations[0].value, "hola");
    assert_eq!(entry.translations[1].lang, "fr");
    assert_eq!(entry.translations[1].value, "bonjour");
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
        .values()
        .next()
        .and_then(|sense| sense.definitions.get(0))
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
    // Define a simple XML with translations
    let xml = r#"<dictionary>
        <entry term="hello">
            <translation lang="es" value="hola" />
            <translation lang="fr" value="bonjour" />
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
        let translations = &entry["translations"];

        // Validate entry translations
        assert_eq!(translations.as_array().unwrap().len(), 2);
        assert_eq!(translations[0]["lang"], "es");
        assert_eq!(translations[0]["value"], "hola");
        assert_eq!(translations[1]["lang"], "fr");
        assert_eq!(translations[1]["value"], "bonjour");

        // Validate example translations
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
