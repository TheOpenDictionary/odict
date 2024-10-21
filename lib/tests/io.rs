#[cfg(test)]
mod io_tests {

    use odict::{DictionaryReader, DictionaryWriter, ID};
    use regex::Regex;
    use rkyv::deserialize;

    #[test]
    fn test_empty() {
        let reader = DictionaryReader::default();
        let writer = DictionaryWriter::default();

        let write = writer.compile_xml("../examples/empty.xml", "../examples/empty.odict");

        assert_eq!(write.is_ok(), true);

        let read = reader.read_from_path("../examples/empty.odict");

        assert_eq!(read.is_err(), false);

        let archive = read.as_ref().unwrap().to_archive().unwrap();

        let id: ID = deserialize::<ID, rkyv::rancor::Error>(&archive.id).unwrap();

        // Ensure a UUID ID is generated for the dictionary
        let id_regex = Regex::new(
            r"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
        )
        .unwrap();

        assert!(id_regex.is_match(&id));
    }
}
