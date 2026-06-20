use std::path::Path;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    cli::generate_docs(workspace_root.join("docs/src/content/docs/cli/reference.md"))?;

    Ok(())
}
