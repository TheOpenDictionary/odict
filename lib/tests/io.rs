#[cfg(test)]
mod io_tests {

    use odict::schema::ID;
    use regex::Regex;
    use rkyv::deserialize;

    #[tokio::test]
    async fn test_empty() {
        let loader = DictionaryLoader::default();
        let writer = DictionaryWriter::default();

        let write = writer.compile_xml("../examples/empty.xml", "../examples/empty.odict");

        assert_eq!(write.is_ok(), true);

        let read = loader.load("../examples/empty.odict").await;

        assert_eq!(read.is_err(), false);

        let archive = read.as_ref().unwrap().contents().unwrap();

        let id: ID = deserialize::<ID, rkyv::rancor::Error>(&archive.id).unwrap();

        // Ensure a UUID ID is generated for the dictionary
        let id_regex = Regex::new(
            r"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
        )
        .unwrap();

        assert!(id_regex.is_match(&id));
    }
}
