#[cfg(test)]
mod form_tests {
    use odict::{FormKind, MediaURL};
    use quick_xml::de::from_str;

    #[test]
    fn test_form_kind_serialization() {
        let kind = FormKind::Comparative;
        let serialized = serde_json::to_string(&kind).unwrap();

        assert_eq!(serialized, "\"comparative\"");

        let deserialized: FormKind = serde_json::from_str("\"comparative\"").unwrap();

        assert!(matches!(deserialized, FormKind::Comparative));

        let expected = FormKind::Other("third-person".into());

        let deserialized: FormKind = serde_json::from_str("\"third-person\"").unwrap();

        assert!(matches!(deserialized, expected));
    }

    #[test]
    fn test_parse_entry_with_form() {
        let xml = r#"
        <entry term="most">
          <ety>
            <sense>
              <form kind="comparative" term="most" />
            </sense>
          </ety>
        </entry>
        "#;

        let entry: odict::Entry = from_str(xml).unwrap();

        assert_eq!(entry.term, "most");

        assert_eq!(entry.etymologies.len(), 1);

        assert!(entry.etymologies[0]
            .senses
            .get(&odict::PartOfSpeech::Un)
            .is_some());

        assert_eq!(
            entry.etymologies[0]
                .senses
                .get(&odict::PartOfSpeech::Un)
                .unwrap()
                .forms
                .len(),
            1
        );

        let form = &entry.etymologies[0]
            .senses
            .get(&odict::PartOfSpeech::Un)
            .unwrap()
            .forms[0];

        assert!(matches!(form.kind, Some(FormKind::Comparative)));
        assert_eq!(form.term, "most".into());
    }

    #[test]
    fn test_entry_with_multiple_forms() {
        let xml = r#"
          <entry term="run">
              <ety>
                <sense>
                  <form kind="conjugation" term="ran" />
                  <form kind="conjugation" term="running" />
                </sense>
              </ety>
            </entry>
        "#;

        let entry: odict::Entry = from_str(xml).unwrap();

        assert_eq!(entry.etymologies.len(), 1);
        assert_eq!(entry.etymologies[0].senses.len(), 1);

        let sense = entry.etymologies[0].senses.get(&odict::PartOfSpeech::Un);

        assert!(sense.is_some());
        assert_eq!(sense.unwrap().forms.len(), 2);

        let first = &sense.unwrap().forms[0];

        assert!(matches!(first.kind, Some(FormKind::Conjugation)));
        assert_eq!(first.term, "ran".into());

        let second = &sense.unwrap().forms[1];

        assert!(matches!(second.kind, Some(FormKind::Conjugation)));
        assert_eq!(second.term, "running".into());
    }
}
