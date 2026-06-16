use crate::rustdoc::generate_docs;

mod rustdoc;

fn main() {
    let manifest_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../odict/Cargo.toml");
    generate_docs(manifest_path, "test.md").unwrap();
}
