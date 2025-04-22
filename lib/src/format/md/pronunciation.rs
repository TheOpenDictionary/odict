use crate::{Entry, Pronunciation};

pub fn write_pronunciation(
    lines: &mut Vec<String>,
    pronunciation: &Pronunciation,
    _entry: &Entry, // Prefix with underscore to indicate intentional non-use
) -> crate::Result<()> {
    // Use the kind_str method to get the proper display string (including custom kinds)
    let kind_str = pronunciation.kind.to_string();

    // Capitalize the first letter for display
    let kind_display = kind_str
        .chars()
        .next()
        .map(|c| c.to_uppercase().collect::<String>() + &kind_str[1..])
        .unwrap_or_else(|| kind_str.to_string());

    let pron_text = format!("*{}:* {}", kind_display, pronunciation.value);
    lines.push(pron_text);

    // Add URLs as markdown links if they exist
    if !pronunciation.urls.is_empty() {
        let url_lines: Vec<String> = pronunciation
            .urls
            .iter()
            .map(|url| {
                let desc = url.description.as_deref().unwrap_or("Audio");
                format!("- [{}]({})", desc, url.src)
            })
            .collect();

        lines.push(String::new()); // Empty line
        lines.push(String::from("*Audio:*"));
        lines.extend(url_lines);
        lines.push(String::new()); // Another empty line
    }

    Ok(())
}
