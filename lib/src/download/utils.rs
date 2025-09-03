use regex::Regex;
use std::sync::LazyLock;

static DICTIONARY_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)/([a-z-]+)$").unwrap());

pub fn parse_remote_dictionary_name(name: &str) -> super::Result<(String, String)> {
    let captures = DICTIONARY_NAME_REGEX.captures(name).ok_or_else(|| {
    crate::error::Error::Other(format!(
      "Invalid dictionary/language format '{name}'. Expected format: 'dictionary/language' (lowercase letters, dash allowed in language)"
    ))
  })?;

    let dictionary = captures.get(1).unwrap().as_str();
    let language = captures.get(2).unwrap().as_str();

    Ok((dictionary.to_string(), language.to_string()))
}

pub(super) fn extract_etag(response: &reqwest::Response) -> Option<String> {
    response
        .headers()
        .get("etag")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_matches('"').to_string())
}
