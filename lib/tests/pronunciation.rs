#[cfg(test)]
mod pronunciation_tests {
    use odict::schema::{MediaURL, PronunciationKind};
    use quick_xml::de::from_str;

    #[test]
    fn test_pronunciation_kind_serialization() {
        let kind = PronunciationKind::IPA;
        let serialized = serde_json::to_string(&kind).unwrap();
        assert_eq!(serialized, "\"ipa\"");

        // Test deserialization including case-insensitivity
        let deserialized: PronunciationKind = serde_json::from_str("\"ipa\"").unwrap();

        assert!(matches!(deserialized, PronunciationKind::IPA));

        let expected = PronunciationKind::Other("wagegiles".into());

        // Test Other variant
        let deserialized: PronunciationKind = serde_json::from_str("\"wadegiles\"").unwrap();
        assert!(matches!(deserialized, expected));
    }

    #[test]
    fn test_media_url_validation() {
        let valid_urls = vec![
            "https://example.com/audio.mp3",
            "http://localhost:8080/audio.mp3",
            "./relative/path.mp3",
            "/absolute/path.mp3",
        ];

        for url in valid_urls {
            let media_url = MediaURL::new(url);
            assert!(media_url.validate().is_ok());
        }

        let invalid_url = "not://a.valid/url";
        let media_url = MediaURL::new(invalid_url);
        assert!(media_url.validate().is_err());
    }

    #[test]
    fn test_parse_entry_with_pronunciation() {
        let xml = r#"
        <entry term="你好">
          <ety>
            <pronunciation kind="pinyin" value="ni hao">
              <url src="./audio.mp3" type="audio/mpeg" />
            </pronunciation>
          </ety>
        </entry>
        "#;

        let entry: odict::schema::Entry = from_str(xml).unwrap();

        assert_eq!(entry.term, "你好");
        assert_eq!(entry.etymologies.len(), 1);
        assert_eq!(entry.etymologies[0].pronunciations.len(), 1);

        let pronunciation = &entry.etymologies[0].pronunciations[0];
        assert!(matches!(
            pronunciation.kind,
            Some(PronunciationKind::Pinyin)
        ));
        assert_eq!(pronunciation.value, "ni hao");
        assert_eq!(pronunciation.media.len(), 1);
        assert_eq!(pronunciation.media[0].src, "./audio.mp3");
        assert_eq!(
            pronunciation.media[0].mime_type,
            Some("audio/mpeg".to_string())
        );
    }

    #[test]
    fn test_parse_example_with_pronunciation() {
        let xml = r#"
        <example value="Hello, world!">
          <pronunciation kind="ipa" value="həˈləʊ wɜːld">
            <url src="./hello.mp3" />
          </pronunciation>
        </example>
        "#;

        let example: odict::schema::Example = from_str(xml).unwrap();
        assert_eq!(example.value, "Hello, world!");
        assert_eq!(example.pronunciations.len(), 1);
        let pronunciation = &example.pronunciations[0];
        assert!(matches!(pronunciation.kind, Some(PronunciationKind::IPA)));
        assert_eq!(pronunciation.value, "həˈləʊ wɜːld");
        assert_eq!(pronunciation.media.len(), 1);
        assert_eq!(pronunciation.media[0].src, "./hello.mp3");
    }

    #[test]
    fn test_entry_with_multiple_pronunciations() {
        let xml = r#"
        <entry term="你好">
          <ety>
            <pronunciation kind="pinyin" value="ni hao">
              <url src="./audio1.mp3" />
            </pronunciation>
            <pronunciation kind="ipa" value="ni˨˩ xɑʊ̯˧˥">
              <url src="./audio2.mp3" />
            </pronunciation>
          </ety>
        </entry>
        "#;

        let entry: odict::schema::Entry = from_str(xml).unwrap();

        assert_eq!(entry.etymologies.len(), 1);
        assert_eq!(entry.etymologies[0].pronunciations.len(), 2);

        let pinyin = &entry.etymologies[0].pronunciations[0];
        assert!(matches!(pinyin.kind, Some(PronunciationKind::Pinyin)));
        assert_eq!(pinyin.value, "ni hao");

        let ipa = &entry.etymologies[0].pronunciations[1];
        assert!(matches!(ipa.kind, Some(PronunciationKind::IPA)));
        assert_eq!(ipa.value, "ni˨˩ xɑʊ̯˧˥");
    }
}
