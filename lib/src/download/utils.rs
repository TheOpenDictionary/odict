use regex::Regex;
use std::sync::LazyLock;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum NetworkError {
    #[strum(serialize = "E_TIMEOUT")]
    Timeout,
    #[strum(serialize = "E_CONNECT")]
    Connect,
    #[strum(serialize = "E_DNS")]
    Dns,
    #[strum(serialize = "E_TLS")]
    Tls,
    #[strum(serialize = "E_BODY")]
    Body,
    #[strum(serialize = "E_HTTP_{0}")]
    Http(u16),
    #[strum(serialize = "E_NETWORK")]
    Network,
}

// Classify reqwest errors into a stable, diagnostic code and message
pub fn classify_reqwest_error(url: &str, e: &reqwest_middleware::Error) -> crate::Error {
    let msg = format!("{e}");
    let lower = msg.to_ascii_lowercase();

    let kind = if e.is_timeout() {
        NetworkError::Timeout
    } else if let Some(status) = e.status() {
        NetworkError::Http(status.as_u16())
    } else if lower.contains("tls") || lower.contains("certificate") || lower.contains("handshake")
    {
        NetworkError::Tls
    } else if e.is_connect() {
        if lower.contains("dns")
            || lower.contains("name or service not known")
            || lower.contains("failed to lookup")
            || lower.contains("resolve")
            || lower.contains("lookup")
        {
            NetworkError::Dns
        } else {
            NetworkError::Connect
        }
    } else if e.is_body() {
        NetworkError::Body
    } else {
        NetworkError::Network
    };

    crate::Error::DownloadFailed(kind, format!("Failed to fetch from {url}: {msg}"))
}

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
