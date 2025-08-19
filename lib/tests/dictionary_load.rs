// #[cfg(all(feature = "download", feature = "config"))]
// mod load_tests {
//     use odict::Dictionary;

//     #[tokio::test]
//     async fn test_load_invalid_format() {
//         // Test invalid dictionary/language formats
//         let invalid_formats = vec![
//             "wiktionary",
//             "wiktionary/",
//             "/eng",
//             "wiktionary/eng/extra",
//             "",
//             "/",
//         ];

//         for format in invalid_formats {
//             let result = Dictionary::load(format).await;
//             assert!(result.is_err(), "Expected error for format: {}", format);

//             if let Err(e) = result {
//                 assert!(e.to_string().contains("Invalid dictionary/language format"));
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_load_empty_names() {
//         // Test empty dictionary name
//         let result = Dictionary::load("/eng").await;
//         assert!(result.is_err());

//         // Test empty language name
//         let result = Dictionary::load("wiktionary/").await;
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_load_valid_format_parsing() {
//         // This test just verifies the parsing logic without actually downloading
//         let dict_lang = "wiktionary/eng";
//         let parts: Vec<&str> = dict_lang.split('/').collect();
//         assert_eq!(parts.len(), 2);
//         assert_eq!(parts[0], "wiktionary");
//         assert_eq!(parts[1], "eng");
//         assert!(!parts[0].is_empty());
//         assert!(!parts[1].is_empty());
//     }
// }
