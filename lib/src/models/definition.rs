use crate::serializable;

use super::{example::Example, note::Note};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::Deserialize;
use std::fmt;
use std::io::Cursor;

/// A segment of text that can be either plain text or a reference to another entry
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvDeserialize, RkyvSerialize)]
#[rkyv(derive(Debug))]
pub enum TextSegment {
    /// Regular text content
    Plain(String),
    /// A reference to another dictionary entry
    Reference(String),
}

/// Rich text that can contain references to other dictionary entries
#[derive(Debug, Clone, PartialEq, Eq, Default, Archive, RkyvDeserialize, RkyvSerialize)]
#[rkyv(derive(Debug))]
pub struct RichText {
    /// The segments that make up the rich text
    pub segments: Vec<TextSegment>,
}

// Remove the manual Debug implementations for Archived types as they conflict with derived implementations

// Implement Display for RichText
impl fmt::Display for RichText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_plain_string())
    }
}

impl RichText {
    /// Create a new empty RichText
    pub fn new() -> Self {
        RichText {
            segments: Vec::new(),
        }
    }

    /// Add plain text to the rich text
    pub fn add_plain(&mut self, text: &str) {
        self.segments.push(TextSegment::Plain(text.to_string()));
    }

    /// Add a reference to another entry
    pub fn add_reference(&mut self, term: &str) {
        self.segments.push(TextSegment::Reference(term.to_string()));
    }

    /// Convert to a plain string (references are included as-is)
    pub fn to_plain_string(&self) -> String {
        self.segments
            .iter()
            .map(|segment| match segment {
                TextSegment::Plain(text) => text.clone(),
                TextSegment::Reference(term) => term.clone(),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Parse text with embedded <ref> tags into RichText
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut result = RichText::new();
        let mut remaining = text;

        while !remaining.is_empty() {
            if let Some(start) = remaining.find("<ref>") {
                // Add text before reference
                if start > 0 {
                    result.add_plain(&remaining[..start]);
                }

                // Find end of reference
                if let Some(end) = remaining[start + 5..].find("</ref>") {
                    let term = &remaining[start + 5..start + 5 + end];
                    result.add_reference(term);
                    remaining = &remaining[start + 5 + end + 6..];
                } else {
                    // Unclosed reference tag - treat as plain text
                    result.add_plain(remaining);
                    break;
                }
            } else {
                // No more references
                result.add_plain(remaining);
                break;
            }
        }

        Ok(result)
    }

    /// Get the contents as a string
    /// Note: Returns a string clone rather than a true string slice.
    /// This is needed because RichText doesn't have an underlying
    /// contiguous string to reference.
    pub fn as_str(&self) -> String {
        self.to_plain_string()
    }
}

// Module for serializing and deserializing RichText
pub mod rich_text {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(rich_text: &RichText, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        for segment in &rich_text.segments {
            match segment {
                TextSegment::Plain(text) => {
                    writer
                        .write_event(Event::Text(BytesText::new(text)))
                        .map_err(serde::ser::Error::custom)?;
                }
                TextSegment::Reference(term) => {
                    // Write <ref> start tag
                    writer
                        .write_event(Event::Start(BytesStart::new("ref")))
                        .map_err(serde::ser::Error::custom)?;

                    // Write reference content
                    writer
                        .write_event(Event::Text(BytesText::new(term)))
                        .map_err(serde::ser::Error::custom)?;

                    // Write </ref> end tag
                    writer
                        .write_event(Event::End(BytesEnd::new("ref")))
                        .map_err(serde::ser::Error::custom)?;
                }
            }
        }

        let result = writer.into_inner().into_inner();
        let xml_string = String::from_utf8(result).map_err(serde::ser::Error::custom)?;

        serializer.serialize_str(&xml_string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<RichText, D::Error>
    where
        D: Deserializer<'de>,
    {
        let xml_str = String::deserialize(deserializer)?;

        // Use quick-xml's reader to parse the mixed content
        let mut reader = Reader::from_str(&xml_str);
        // Configure reader settings
        reader.config_mut().trim_text(true);

        let mut rich_text = RichText::new();
        let mut buf = Vec::new();
        let mut in_ref = false;
        let mut current_ref = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    let text = e.unescape().map_err(serde::de::Error::custom)?;
                    if in_ref {
                        current_ref.push_str(&text);
                    } else {
                        rich_text.add_plain(&text);
                    }
                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"ref" => {
                    in_ref = true;
                }
                Ok(Event::End(e)) if e.name().as_ref() == b"ref" => {
                    rich_text.add_reference(&current_ref);
                    current_ref.clear();
                    in_ref = false;
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(serde::de::Error::custom(format!(
                        "Error parsing XML: {}",
                        e
                    )))
                }
                _ => (),
            }
        }

        Ok(rich_text)
    }
}

serializable! {
  pub struct Definition {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "value", with = "rich_text")]
    pub value: RichText,

    #[serde(default, rename="example")]
    pub examples: Vec<Example>,

    #[serde(default, rename="note")]
    pub notes: Vec<Note>,
  }
}

// Implement conversion from String to RichText for backward compatibility
impl From<String> for RichText {
    fn from(s: String) -> Self {
        let mut rich_text = RichText::new();
        rich_text.add_plain(&s);
        rich_text
    }
}

// Implement conversion from RichText to String for backward compatibility
impl From<RichText> for String {
    fn from(rt: RichText) -> Self {
        rt.to_plain_string()
    }
}
