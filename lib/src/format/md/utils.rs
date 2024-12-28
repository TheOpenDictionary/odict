pub fn divider() -> String {
    "─".repeat(32)
}

pub fn index_to_alpha(i: usize) -> char {
    (i as u8 + b'a') as char
}

pub fn underline_target(example: &str, target: &str) -> String {
    let mut parts: Vec<String> = vec![];
    let mut last_index = 0;

    for (index, _) in example.match_indices(target) {
        parts.push(example[last_index..index].into());
        parts.push(format!("*{}*", target));

        last_index = index + target.len();
    }

    if last_index < example.len() {
        parts.push(example[last_index..].into());
    }

    let modified_string = format!("_{}_", parts.concat());

    modified_string
}

pub fn indent(s: &str, width: usize) -> String {
    let padding = " ".repeat(width);

    s.lines()
        .into_iter()
        .map(|l| format!("{}{}", padding, l))
        .collect::<Vec<String>>()
        .join("\n")
        .into()
}
