use map_macro::hash_set;
use odict::{Entry, EntryRef, Form, FormKind, PartOfSpeech, Sense};

#[test]
fn test_form_tags_serialization() {
    let form = Form {
        term: EntryRef("word".to_string()),
        kind: Some(FormKind::Plural),
        tags: vec!["possessive".to_string(), "archaic".to_string()],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&form).unwrap();

    // Check that the XML has the expected tags element structure
    assert!(xml.contains("<tag>possessive</tag>"));
    assert!(xml.contains("<tag>archaic</tag>"));

    // Deserialize from XML
    let deserialized: Form = quick_xml::de::from_str(&xml).unwrap();

    // Check that the tags were correctly deserialized
    assert_eq!(deserialized.tags.len(), 2);
    assert!(deserialized.tags.contains(&"possessive".to_string()));
    assert!(deserialized.tags.contains(&"archaic".to_string()));
}

#[test]
fn test_sense_tags_serialization() {
    let sense = Sense {
        pos: PartOfSpeech::N,
        lemma: None,
        definitions: vec![],
        tags: vec!["informal".to_string(), "slang".to_string()],
        translations: vec![],
        forms: vec![],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&sense).unwrap();

    // Check that the XML has the expected tags element structure
    assert!(xml.contains("<tag>informal</tag>"));
    assert!(xml.contains("<tag>slang</tag>"));

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
    assert!(!xml.contains("<tag"));

    // Deserializing XML without tags should result in an empty vector
    let deserialized: Form = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(deserialized.tags.len(), 0);
}

#[test]
fn test_form_with_tags() {
    // Create a form with tags
    let form = Form {
        term: EntryRef("words".to_string()),
        kind: Some(FormKind::Plural),
        tags: vec!["plural".to_string()],
    };

    // Create a sense with the form
    let sense = Sense {
        pos: PartOfSpeech::N,
        lemma: None,
        definitions: vec![],
        tags: vec![],
        translations: vec![],
        forms: vec![form],
    };

    // Create an etymology with the sense
    let mut senses = hash_set! {};
    senses.insert(sense);

    let etymology = odict::Etymology {
        id: None,
        pronunciations: vec![],
        description: None,
        senses,
    };

    // Create an entry with the etymology
    let entry = Entry {
        media: vec![],
        rank: None,
        term: "word".to_string(),
        see_also: None,
        etymologies: vec![etymology],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&entry).unwrap();

    // Check for form tags
    assert!(xml.contains("<form"));
    assert!(xml.contains("<tag>plural</tag>"));

    // Deserialize from XML
    let deserialized: Entry = quick_xml::de::from_str(&xml).unwrap();

    // Check that form tags were correctly deserialized
    assert_eq!(
        deserialized.etymologies[0]
            .senses
            .get(&PartOfSpeech::N)
            .unwrap()
            .forms[0]
            .tags
            .len(),
        1
    );
    assert_eq!(
        deserialized.etymologies[0]
            .senses
            .get(&PartOfSpeech::N)
            .unwrap()
            .forms[0]
            .tags[0],
        "plural"
    );
}

#[test]
fn test_forms_in_sense() {
    // Create forms with tags
    let form1 = Form {
        term: EntryRef("words".to_string()),
        kind: Some(FormKind::Plural),
        tags: vec!["plural".to_string()],
    };

    let form2 = Form {
        term: EntryRef("worded".to_string()),
        kind: Some(FormKind::Comparative),
        tags: vec!["past".to_string()],
    };

    // Create a sense with forms
    let sense = Sense {
        pos: PartOfSpeech::N,
        lemma: None,
        definitions: vec![],
        tags: vec![],
        translations: vec![],
        forms: vec![form1, form2],
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&sense).unwrap();

    // Check for form tags
    assert!(xml.contains("<form"));
    assert!(xml.contains("<tag>plural</tag>"));
    assert!(xml.contains("<tag>past</tag>"));

    // Deserialize from XML
    let deserialized: Sense = quick_xml::de::from_str(&xml).unwrap();

    // Check that forms were correctly deserialized
    assert_eq!(deserialized.forms.len(), 2);
    assert_eq!(deserialized.forms[0].tags.len(), 1);
    assert_eq!(deserialized.forms[0].tags[0], "plural");
    assert_eq!(deserialized.forms[1].tags.len(), 1);
    assert_eq!(deserialized.forms[1].tags[0], "past");
}
