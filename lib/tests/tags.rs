use odict::{Entry, EntryRef, Form, FormKind, PartOfSpeech, Sense};

#[test]
fn test_form_tags_serialization() {
    let mut form = Form {
        term: EntryRef("word".to_string()),
        kind: Some(FormKind::Plural),
        tags: vec!["possessive".to_string(), "archaic".to_string()],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&form).unwrap();
    
    // Check that the XML has the expected tags element structure
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("<tag>possessive</tag>"));
    assert!(xml.contains("<tag>archaic</tag>"));
    assert!(xml.contains("</tags>"));
    
    // Deserialize from XML
    let deserialized: Form = quick_xml::de::from_str(&xml).unwrap();
    
    // Check that the tags were correctly deserialized
    assert_eq!(deserialized.tags.len(), 2);
    assert!(deserialized.tags.contains(&"possessive".to_string()));
    assert!(deserialized.tags.contains(&"archaic".to_string()));
}

#[test]
fn test_sense_tags_serialization() {
    let mut sense = Sense {
        pos: PartOfSpeech::Noun,
        lemma: None,
        definitions: vec![],
        tags: vec!["informal".to_string(), "slang".to_string()],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&sense).unwrap();
    
    // Check that the XML has the expected tags element structure
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("<tag>informal</tag>"));
    assert!(xml.contains("<tag>slang</tag>"));
    assert!(xml.contains("</tags>"));
    
    // Deserialize from XML
    let deserialized: Sense = quick_xml::de::from_str(&xml).unwrap();
    
    // Check that the tags were correctly deserialized
    assert_eq!(deserialized.tags.len(), 2);
    assert!(deserialized.tags.contains(&"informal".to_string()));
    assert!(deserialized.tags.contains(&"slang".to_string()));
}

#[test]
fn test_empty_tags() {
    let form = Form {
        term: EntryRef("word".to_string()),
        kind: None,
        tags: vec![],
    };

    let xml = quick_xml::se::to_string(&form).unwrap();
    
    // Empty tags should be skipped in serialization
    assert!(!xml.contains("<tags>"));
    
    // Deserializing XML without tags should result in an empty vector
    let deserialized: Form = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(deserialized.tags.len(), 0);
}

#[test]
fn test_entry_with_form_and_sense_tags() {
    // Create a form with tags
    let form = Form {
        term: EntryRef("words".to_string()),
        kind: Some(FormKind::Plural),
        tags: vec!["plural".to_string()],
    };

    // Create a sense with tags
    let sense = Sense {
        pos: PartOfSpeech::Noun,
        lemma: None,
        definitions: vec![],
        tags: vec!["countable".to_string()],
    };
    
    // Create an entry with both form and sense
    let entry = Entry {
        id: Some("word".to_string()),
        term: "word".to_string(),
        pronunciations: vec![],
        forms: vec![form],
        etymologies: vec![],
        senses: vec![sense],
        notes: vec![],
    };
    
    // Serialize to XML
    let xml = quick_xml::se::to_string(&entry).unwrap();
    
    // Check for both form and sense tags
    assert!(xml.contains("<form"));
    assert!(xml.contains("<tags><tag>plural</tag></tags>"));
    assert!(xml.contains("<sense"));
    assert!(xml.contains("<tags><tag>countable</tag></tags>"));
    
    // Deserialize from XML
    let deserialized: Entry = quick_xml::de::from_str(&xml).unwrap();
    
    // Check that both form tags and sense tags were correctly deserialized
    assert_eq!(deserialized.forms[0].tags.len(), 1);
    assert_eq!(deserialized.forms[0].tags[0], "plural");
    assert_eq!(deserialized.senses[0].tags.len(), 1);
    assert_eq!(deserialized.senses[0].tags[0], "countable");
}