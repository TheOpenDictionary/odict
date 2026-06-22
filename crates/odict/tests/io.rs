#[cfg(test)]
mod io_tests {

    use odict::{
        schema::{Dictionary, ID},
        OpenDictionary,
    };
    use regex::Regex;
    use rkyv::deserialize;

    #[test]
    fn test_empty() {
        let write = Dictionary::from_path("../../examples/empty.xml")
            .unwrap()
            .build()
            .unwrap()
            .to_disk("../../examples/empty.odict");

        assert!(write.is_ok());

        let read = OpenDictionary::from_path("../../examples/empty.odict");

        assert!(read.is_ok());

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
