pub fn parse_dictionary_name(name: &str) -> super::Result<(String, String)> {
    let parts: Vec<&str> = name.split('/').collect();

    if parts.len() != 2 {
        return Err(crate::error::Error::Other(format!(
            "Invalid dictionary/language format '{}'. Expected format: 'dictionary/language'",
            name
        )));
    }

    let dictionary = parts[0];
    let language = parts[1];

    Ok((dictionary.to_string(), language.to_string()))
}

pub fn extract_etag(response: &reqwest::Response) -> Option<String> {
    response
        .headers()
        .get("etag")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_matches('"').to_string())
}
