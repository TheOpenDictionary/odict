use anyhow::Result;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;
use std::path::Path;
use std::process::Command;

use rustdoc_types::{
    Abi, AssocItemConstraintKind, Crate, Enum, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Generics, Id, Impl, Item, ItemEnum, MacroKind, Module,
    PreciseCapturingArg, Struct, StructKind, Term, Trait, TraitBoundModifier, Type, Union,
    VariantKind, Visibility, WherePredicate,
};

/// Generates Markdown documentation for the crate at the given `Cargo.toml`
/// path and writes it to `output_path`.
///
/// Requires a nightly toolchain, since the underlying rustdoc JSON output is an
/// unstable rustdoc option.
pub fn generate_docs(
    manifest_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let crate_data = get_json(manifest_path)?;
    let markdown = json_to_markdown(crate_data);
    std::fs::write(output_path, markdown)?;
    Ok(())
}

/// Generates rustdoc JSON for the crate at the given `Cargo.toml` path and
/// deserializes it into a [`Crate`].
///
/// Requires a nightly toolchain, since `--output-format json` is an unstable
/// rustdoc option.
fn get_json(manifest_path: impl AsRef<Path>) -> Result<Crate, Box<dyn Error>> {
    let manifest_path = manifest_path.as_ref();

    // Resolve the target directory and the crate's documented name before
    // invoking rustdoc, since the JSON file is written to
    // `<target_dir>/doc/<crate_name>.json`.
    let metadata_output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .arg("--manifest-path")
        .arg(manifest_path)
        .output()?;

    if !metadata_output.status.success() {
        return Err(format!(
            "`cargo metadata` failed: {}",
            String::from_utf8_lossy(&metadata_output.stderr)
        )
        .into());
    }

    let metadata: serde_json::Value = serde_json::from_slice(&metadata_output.stdout)?;

    let target_directory = metadata["target_directory"]
        .as_str()
        .ok_or("missing `target_directory` in cargo metadata")?;

    // `cargo metadata --no-deps` still lists every workspace member, so select
    // the package whose manifest matches the one requested rather than blindly
    // taking the first.
    let canonical_manifest = std::fs::canonicalize(manifest_path)?;
    let package = metadata["packages"]
        .as_array()
        .and_then(|packages| {
            packages.iter().find(|pkg| {
                pkg["manifest_path"]
                    .as_str()
                    .and_then(|path| std::fs::canonicalize(path).ok())
                    == Some(canonical_manifest.clone())
            })
        })
        .ok_or("no package matching the manifest path found in cargo metadata")?;

    // rustdoc names the JSON file after the documented target (the library if
    // present, otherwise the package itself), with dashes normalized to
    // underscores.
    let crate_name = package["targets"]
        .as_array()
        .and_then(|targets| {
            targets.iter().find(|target| {
                target["kind"]
                    .as_array()
                    .is_some_and(|kinds| kinds.iter().any(|kind| kind == "lib"))
            })
        })
        .and_then(|target| target["name"].as_str())
        .or_else(|| package["name"].as_str())
        .ok_or("could not determine crate name from cargo metadata")?
        .replace('-', "_");

    let status = Command::new("cargo")
        .args(["doc", "--no-deps"])
        .arg("--manifest-path")
        .arg(manifest_path)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .status()?;

    if !status.success() {
        return Err(format!("`cargo doc` failed with status: {status}").into());
    }

    let json_path = Path::new(target_directory)
        .join("doc")
        .join(format!("{crate_name}.json"));

    let json = std::fs::read_to_string(&json_path).map_err(|e| {
        format!(
            "failed to read rustdoc JSON at {}: {e}",
            json_path.display()
        )
    })?;

    let crate_data: Crate = serde_json::from_str(&json)?;

    Ok(crate_data)
}

/// Markdown heading prefix of `level` hashes, capped at the maximum valid
/// heading depth of 6.
fn heading(level: usize) -> &'static str {
    const HASHES: [&str; 6] = ["#", "##", "###", "####", "#####", "######"];
    HASHES[level.clamp(1, 6) - 1]
}

/// First non-blank line of a doc comment, if any.
fn first_doc_line(docs: &str) -> Option<&str> {
    docs.lines().next().filter(|line| !line.trim().is_empty())
}

/// Pushes a visibility modifier (with a trailing space when non-default),
/// prefixed by `indent`.
fn format_visibility(output: &mut String, visibility: &Visibility, indent: &str) {
    output.push_str(indent);
    match visibility {
        Visibility::Public => output.push_str("pub "),
        Visibility::Crate => output.push_str("pub(crate) "),
        Visibility::Restricted { path, .. } => {
            let _ = write!(output, "pub(in {path}) ");
        }
        Visibility::Default => {}
    }
}

/// Pushes an ABI prefix (e.g. `extern "C" `) for non-Rust ABIs.
fn format_abi(output: &mut String, abi: &Abi) {
    let name = match abi {
        Abi::Rust => return,
        Abi::C { unwind } => unwind_abi("C", *unwind),
        Abi::Cdecl { unwind } => unwind_abi("cdecl", *unwind),
        Abi::Stdcall { unwind } => unwind_abi("stdcall", *unwind),
        Abi::Fastcall { unwind } => unwind_abi("fastcall", *unwind),
        Abi::Aapcs { unwind } => unwind_abi("aapcs", *unwind),
        Abi::Win64 { unwind } => unwind_abi("win64", *unwind),
        Abi::SysV64 { unwind } => unwind_abi("sysv64", *unwind),
        Abi::System { unwind } => unwind_abi("system", *unwind),
        Abi::Other(abi) => abi.clone(),
    };
    let _ = write!(output, "extern \"{name}\" ");
}

fn unwind_abi(name: &str, unwind: bool) -> String {
    if unwind {
        format!("{name}-unwind")
    } else {
        name.to_string()
    }
}

/// Pushes a `for<...>` higher-ranked binder (with a trailing space) when
/// `params` is non-empty.
fn format_for_binder(output: &mut String, params: &[GenericParamDef]) {
    if params.is_empty() {
        return;
    }

    output.push_str("for<");
    for (i, param) in params.iter().enumerate() {
        match &param.kind {
            GenericParamDefKind::Lifetime { .. } => {
                let _ = write!(output, "'{}", param.name);
            }
            _ => output.push_str(&param.name),
        }
        if i < params.len() - 1 {
            output.push_str(", ");
        }
    }
    output.push_str("> ");
}

fn json_to_markdown(data: Crate) -> String {
    let mut output = String::new();

    // Add crate header and basic info
    output.push_str("# Crate Documentation\n\n");

    if let Some(version) = &data.crate_version {
        output.push_str(&format!("**Version:** {}\n\n", version));
    }

    output.push_str(&format!("**Format Version:** {}\n\n", data.format_version));

    // Process the root module to start
    let root_id = data.root;
    if let Some(root_item) = data.index.get(&root_id)
        && let ItemEnum::Module(module) = &root_item.inner
    {
        if let Some(name) = &root_item.name {
            output.push_str(&format!("# Module `{}`\n\n", name));
        } else if module.is_crate {
            output.push_str("# Crate Root\n\n");
        }

        // Add root documentation if available
        if let Some(docs) = &root_item.docs {
            output.push_str(&format!("{}\n\n", docs));
        }

        // Process all items in the module with consistent heading levels
        // starting at level 2 for top-level categories
        process_items(&mut output, &module.items, &data, 2);
    }

    output
}

fn process_items(output: &mut String, item_ids: &[Id], data: &Crate, level: usize) {
    let heading_level = level.min(6);

    // Group items by kind for better organization
    let mut modules = Vec::new();
    let mut types = Vec::new();
    let mut traits = Vec::new();
    let mut functions = Vec::new();
    let mut constants = Vec::new();
    let mut macros = Vec::new();
    let mut reexports = Vec::new(); // New category for re-exports
    let mut other_items = Vec::new();

    for &id in item_ids {
        if let Some(item) = data.index.get(&id) {
            match &item.inner {
                ItemEnum::Module(_) => modules.push(id),
                ItemEnum::Struct(_)
                | ItemEnum::Enum(_)
                | ItemEnum::Union(_)
                | ItemEnum::TypeAlias(_) => types.push(id),
                ItemEnum::Trait(_) | ItemEnum::TraitAlias(_) => traits.push(id),
                ItemEnum::Function(_) => functions.push(id),
                ItemEnum::Constant { .. } | ItemEnum::Static(_) => constants.push(id),
                ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => macros.push(id),
                ItemEnum::Use(_) => reexports.push(id), // Categorize re-exports
                _ => other_items.push(id),
            }
        }
    }

    // Process each group in order
    let groups: [(&str, &[Id]); 8] = [
        ("Modules", &modules),
        ("Types", &types),
        ("Traits", &traits),
        ("Functions", &functions),
        ("Constants and Statics", &constants),
        ("Macros", &macros),
        ("Re-exports", &reexports),
        ("Other Items", &other_items),
    ];

    for (label, ids) in groups {
        if ids.is_empty() {
            continue;
        }
        let _ = write!(output, "{} {label}\n\n", heading(heading_level));
        for &id in ids {
            process_item(output, data.index.get(&id).unwrap(), data, level + 1);
        }
    }
}

/// The keyword shown in an item's heading (`Struct`, `Enum`, …), or `None` for
/// kinds rendered with bespoke headings.
fn item_kind_label(inner: &ItemEnum) -> Option<&'static str> {
    Some(match inner {
        ItemEnum::Struct(_) => "Struct",
        ItemEnum::Enum(_) => "Enum",
        ItemEnum::Union(_) => "Union",
        ItemEnum::Trait(_) => "Trait",
        ItemEnum::TraitAlias(_) => "Trait Alias",
        ItemEnum::Function(_) => "Function",
        ItemEnum::TypeAlias(_) => "Type Alias",
        ItemEnum::Constant { .. } => "Constant",
        ItemEnum::Static(_) => "Static",
        ItemEnum::Macro(_) => "Macro",
        ItemEnum::ProcMacro(_) => "Procedural Macro",
        _ => return None,
    })
}

fn process_item(output: &mut String, item: &Item, data: &Crate, level: usize) {
    let heading_level = level.min(6);
    let h = heading(heading_level);

    // Add item heading with name and kind
    match &item.inner {
        // Check for re-exports first, regardless of whether they have a name
        ItemEnum::Use(use_item) => {
            // Extract the meaningful part of the source path
            let source_name = use_item
                .source
                .split("::")
                .last()
                .unwrap_or(&use_item.source);

            // Format the heading based on the type of re-export
            if use_item.is_glob {
                let _ = write!(output, "{h} Re-export `{}::*`\n\n", use_item.source);
            } else if let Some(name) = &item.name {
                if name != source_name {
                    let _ = write!(output, "{h} Re-export `{source_name}` as `{name}`\n\n");
                } else {
                    let _ = write!(output, "{h} Re-export `{name}`\n\n");
                }
            } else {
                let _ = write!(output, "{h} Re-export `{source_name}`\n\n");
            }
        }
        // Modules always use a consistent level so they stand out.
        ItemEnum::Module(_) => {
            if let Some(name) = &item.name {
                let _ = write!(output, "## Module `{name}`\n\n");
            }
        }
        ItemEnum::ExternCrate {
            name: crate_name, ..
        } => {
            let _ = write!(output, "{h} Extern Crate `{crate_name}`\n\n");
        }
        // Nameless impl blocks.
        ItemEnum::Impl(impl_) if item.name.is_none() => {
            if let Some(trait_) = &impl_.trait_ {
                let _ = write!(
                    output,
                    "{h} Implementation of `{}` for `{}`\n\n",
                    trait_.path,
                    format_type(&impl_.for_, data)
                );
            } else {
                let _ = write!(
                    output,
                    "{h} Implementation for `{}`\n\n",
                    format_type(&impl_.for_, data)
                );
            }
        }
        _ => {
            if let Some(name) = &item.name {
                match item_kind_label(&item.inner) {
                    Some(label) => {
                        let _ = write!(output, "{h} {label} `{name}`\n\n");
                    }
                    None => {
                        let _ = write!(output, "{h} `{name}`\n\n");
                    }
                }
            } else {
                let _ = write!(output, "{h} Unnamed Item\n\n");
            }
        }
    }

    // Add item attributes if present
    if !item.attrs.is_empty() {
        output.push_str("**Attributes:**\n\n");
        for attr in &item.attrs {
            output.push_str(&format!("- `{:?}`\n", attr));
        }
        output.push('\n');
    }

    // Add deprecation info if present
    if let Some(deprecation) = &item.deprecation {
        output.push_str("**⚠️ Deprecated");
        if let Some(since) = &deprecation.since {
            output.push_str(&format!(" since {}", since));
        }
        output.push_str("**");

        if let Some(note) = &deprecation.note {
            output.push_str(&format!(": {}", note));
        }
        output.push_str("\n\n");
    }

    // Add documentation if available
    if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", docs));
    }

    // Add code block with item signature
    output.push_str("```rust\n");
    format_item_signature(output, item, data);
    output.push_str("\n```\n\n");

    // Process additional details based on item kind
    match &item.inner {
        ItemEnum::Module(module) => process_module_details(output, module, data, level + 1),
        ItemEnum::Struct(struct_) => process_struct_details(output, struct_, data, level + 1),
        ItemEnum::Enum(enum_) => process_enum_details(output, enum_, data, level + 1),
        ItemEnum::Union(union_) => process_union_details(output, union_, data, level + 1),
        ItemEnum::Trait(trait_) => process_trait_details(output, trait_, data, level + 1),
        ItemEnum::Impl(impl_) => process_impl_details(output, impl_, data, level + 1),
        _ => {}
    }
}

fn format_item_signature(output: &mut String, item: &Item, data: &Crate) {
    format_visibility(output, &item.visibility, "");

    // Format item based on its kind
    match &item.inner {
        ItemEnum::Module(_) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("mod {} {{ /* ... */ }}", name));
            }
        }
        ItemEnum::Struct(struct_) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("struct {}", name));
                format_generics(output, &struct_.generics, data);

                match &struct_.kind {
                    StructKind::Unit => output.push(';'),
                    StructKind::Tuple(fields) => {
                        output.push('(');
                        for (i, field_opt) in fields.iter().enumerate() {
                            if let Some(field_id) = field_opt {
                                if let Some(field_item) = data.index.get(field_id)
                                    && let ItemEnum::StructField(field_type) = &field_item.inner
                                {
                                    format_visibility(output, &field_item.visibility, "");
                                    output.push_str(&format_type(field_type, data));
                                }
                                if i < fields.len() - 1 {
                                    output.push_str(", ");
                                }
                            } else {
                                // For stripped fields
                                output.push_str("/* private field */");
                                if i < fields.len() - 1 {
                                    output.push_str(", ");
                                }
                            }
                        }
                        output.push_str(");");
                    }
                    StructKind::Plain {
                        fields,
                        has_stripped_fields,
                    } => {
                        output.push_str(" {\n");
                        for &field_id in fields {
                            if let Some(field_item) = data.index.get(&field_id)
                                && let Some(field_name) = &field_item.name
                                && let ItemEnum::StructField(field_type) = &field_item.inner
                            {
                                format_visibility(output, &field_item.visibility, "    ");
                                output.push_str(&format!(
                                    "{}: {},\n",
                                    field_name,
                                    format_type(field_type, data)
                                ));
                            }
                        }
                        if *has_stripped_fields {
                            output.push_str("    // Some fields omitted\n");
                        }
                        output.push('}');
                    }
                }
            }
        }
        ItemEnum::Enum(enum_) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("enum {}", name));
                format_generics(output, &enum_.generics, data);
                output.push_str(" {\n");

                for &variant_id in &enum_.variants {
                    if let Some(variant_item) = data.index.get(&variant_id)
                        && let Some(variant_name) = &variant_item.name
                    {
                        output.push_str(&format!("    {}", variant_name));

                        if let ItemEnum::Variant(variant) = &variant_item.inner {
                            match &variant.kind {
                                VariantKind::Plain => {}
                                VariantKind::Tuple(fields) => {
                                    output.push('(');
                                    for (i, field_opt) in fields.iter().enumerate() {
                                        if let Some(field_id) = field_opt {
                                            if let Some(field_item) = data.index.get(field_id)
                                                && let ItemEnum::StructField(field_type) =
                                                    &field_item.inner
                                            {
                                                output.push_str(&format_type(field_type, data));
                                            }
                                            if i < fields.len() - 1 {
                                                output.push_str(", ");
                                            }
                                        } else {
                                            // For stripped fields
                                            output.push_str("/* private field */");
                                            if i < fields.len() - 1 {
                                                output.push_str(", ");
                                            }
                                        }
                                    }
                                    output.push(')');
                                }
                                VariantKind::Struct {
                                    fields,
                                    has_stripped_fields,
                                } => {
                                    output.push_str(" {\n");
                                    for &field_id in fields {
                                        if let Some(field_item) = data.index.get(&field_id)
                                            && let Some(field_name) = &field_item.name
                                            && let ItemEnum::StructField(field_type) =
                                                &field_item.inner
                                        {
                                            output.push_str(&format!(
                                                "        {}: {},\n",
                                                field_name,
                                                format_type(field_type, data)
                                            ));
                                        }
                                    }
                                    if *has_stripped_fields {
                                        output.push_str("        // Some fields omitted\n");
                                    }
                                    output.push_str("    }");
                                }
                            }

                            if let Some(discriminant) = &variant.discriminant {
                                output.push_str(&format!(" = {}", discriminant.expr));
                            }
                        }

                        output.push_str(",\n");
                    }
                }

                if enum_.has_stripped_variants {
                    output.push_str("    // Some variants omitted\n");
                }

                output.push('}');
            }
        }
        ItemEnum::Union(union_) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("union {}", name));
                format_generics(output, &union_.generics, data);
                output.push_str(" {\n");

                for &field_id in &union_.fields {
                    if let Some(field_item) = data.index.get(&field_id)
                        && let Some(field_name) = &field_item.name
                        && let ItemEnum::StructField(field_type) = &field_item.inner
                    {
                        format_visibility(output, &field_item.visibility, "    ");
                        output.push_str(&format!(
                            "{}: {},\n",
                            field_name,
                            format_type(field_type, data)
                        ));
                    }
                }

                if union_.has_stripped_fields {
                    output.push_str("    // Some fields omitted\n");
                }

                output.push('}');
            }
        }
        ItemEnum::Function(function) => {
            // Function header
            if function.header.is_const {
                output.push_str("const ");
            }
            if function.header.is_unsafe {
                output.push_str("unsafe ");
            }
            if function.header.is_async {
                output.push_str("async ");
            }

            format_abi(output, &function.header.abi);

            // Function name
            if let Some(name) = &item.name {
                output.push_str(&format!("fn {}", name));

                // Generic parameters
                format_generics(output, &function.generics, data);

                // Parameters
                output.push('(');
                for (i, (param_name, param_type)) in function.sig.inputs.iter().enumerate() {
                    output.push_str(&format!(
                        "{}: {}",
                        param_name,
                        format_type(param_type, data)
                    ));
                    if i < function.sig.inputs.len() - 1 || function.sig.is_c_variadic {
                        output.push_str(", ");
                    }
                }

                // Variadic
                if function.sig.is_c_variadic {
                    output.push_str("...");
                }

                output.push(')');

                // Return type
                if let Some(return_type) = &function.sig.output {
                    output.push_str(&format!(" -> {}", format_type(return_type, data)));
                }

                // Where clause
                format_where_clause(output, &function.generics.where_predicates, data);

                // Function body indication
                if function.has_body {
                    output.push_str(" { /* ... */ }");
                } else {
                    output.push(';');
                }
            }
        }
        ItemEnum::Trait(trait_) => {
            // Trait modifiers
            if trait_.is_auto {
                output.push_str("auto ");
            }
            if trait_.is_unsafe {
                output.push_str("unsafe ");
            }

            // Trait definition
            if let Some(name) = &item.name {
                output.push_str(&format!("trait {}", name));
                format_generics(output, &trait_.generics, data);

                // Trait bounds
                if !trait_.bounds.is_empty() {
                    output.push_str(": ");
                    format_bounds(output, &trait_.bounds, data);
                }

                // Where clause
                format_where_clause(output, &trait_.generics.where_predicates, data);

                output.push_str(" {\n    /* Associated items */\n}");
            }
        }
        ItemEnum::TraitAlias(trait_alias) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("trait {}", name));
                format_generics(output, &trait_alias.generics, data);
                output.push_str(" = ");
                format_bounds(output, &trait_alias.params, data);
                format_where_clause(output, &trait_alias.generics.where_predicates, data);
                output.push(';');
            }
        }
        ItemEnum::Impl(impl_) => {
            // Impl modifiers
            if impl_.is_unsafe {
                output.push_str("unsafe ");
            }

            output.push_str("impl");

            // Generics
            format_generics(output, &impl_.generics, data);

            // Trait reference if this is a trait impl
            if let Some(trait_) = &impl_.trait_ {
                if impl_.is_negative {
                    output.push_str(" !");
                } else {
                    output.push(' ');
                }

                output.push_str(&trait_.path);
                if let Some(args) = &trait_.args {
                    format_generic_args(output, args, data);
                }

                output.push_str(" for ");
            }

            // For type
            output.push_str(&format_type(&impl_.for_, data));

            // Where clause
            format_where_clause(output, &impl_.generics.where_predicates, data);

            output.push_str(" {\n    /* Associated items */\n}");

            // Add note if this is a compiler-generated impl
            if impl_.is_synthetic {
                output.push_str("\n// Note: This impl is compiler-generated");
            }
        }
        ItemEnum::TypeAlias(type_alias) => {
            if let Some(name) = &item.name {
                output.push_str(&format!("type {}", name));
                format_generics(output, &type_alias.generics, data);
                format_where_clause(output, &type_alias.generics.where_predicates, data);
                output.push_str(&format!(" = {};", format_type(&type_alias.type_, data)));
            }
        }
        ItemEnum::Constant { type_, const_ } => {
            if let Some(name) = &item.name {
                output.push_str(&format!(
                    "const {}: {} = {};",
                    name,
                    format_type(type_, data),
                    const_.expr
                ));
            }
        }
        ItemEnum::Static(static_) => {
            if let Some(name) = &item.name {
                output.push_str("static ");
                if static_.is_mutable {
                    output.push_str("mut ");
                }
                if static_.is_unsafe {
                    output.push_str("/* unsafe */ ");
                }
                output.push_str(&format!(
                    "{}: {} = {};",
                    name,
                    format_type(&static_.type_, data),
                    static_.expr
                ));
            }
        }
        ItemEnum::Macro(macro_body) => {
            if let Some(name) = &item.name {
                output.push_str(&format!(
                    "macro_rules! {} {{\n    /* {} */\n}}",
                    name, macro_body
                ));
            }
        }
        ItemEnum::ProcMacro(proc_macro) => {
            if let Some(name) = &item.name {
                output.push_str("#[proc_macro");
                match proc_macro.kind {
                    MacroKind::Bang => output.push(']'),

                    MacroKind::Attr => output.push_str("_attribute]"),
                    MacroKind::Derive => {
                        output.push_str("_derive]");
                        if !proc_macro.helpers.is_empty() {
                            output.push_str("\n// Helpers: ");
                            for (i, helper) in proc_macro.helpers.iter().enumerate() {
                                output.push_str(&format!("#[{}]", helper));
                                if i < proc_macro.helpers.len() - 1 {
                                    output.push_str(", ");
                                }
                            }
                        }
                    }
                }
                output.push_str(&format!(
                    "\npub fn {}(/* ... */) -> /* ... */ {{\n    /* ... */\n}}",
                    name
                ));
            }
        }
        ItemEnum::ExternCrate { name, rename } => {
            output.push_str(&format!("extern crate {}", name));
            if let Some(rename_val) = rename {
                output.push_str(&format!(" as {}", rename_val));
            }
            output.push(';');
        }
        ItemEnum::Use(use_item) => {
            output.push_str(&format!("use {}", use_item.source));
            if use_item.is_glob {
                output.push_str("::*");
            } else if use_item.name
                != use_item
                    .source
                    .split("::")
                    .last()
                    .unwrap_or(&use_item.source)
            {
                output.push_str(&format!(" as {}", use_item.name));
            }
            output.push(';');
        }
        ItemEnum::StructField(field_type) => {
            // For struct fields, just output the type
            if let Some(name) = &item.name {
                format_visibility(output, &item.visibility, "");
                output.push_str(&format!("{}: {}", name, format_type(field_type, data)));
            } else {
                output.push_str(&format_type(field_type, data));
            }
        }
        ItemEnum::Variant(variant) => {
            // For enum variants
            if let Some(name) = &item.name {
                output.push_str(name);

                match &variant.kind {
                    VariantKind::Plain => {}
                    VariantKind::Tuple(fields) => {
                        output.push('(');
                        for (i, field_opt) in fields.iter().enumerate() {
                            if let Some(field_id) = field_opt {
                                if let Some(field_item) = data.index.get(field_id)
                                    && let ItemEnum::StructField(field_type) = &field_item.inner
                                {
                                    output.push_str(&format_type(field_type, data));
                                }
                                if i < fields.len() - 1 {
                                    output.push_str(", ");
                                }
                            } else {
                                // For stripped fields
                                output.push_str("/* private field */");
                                if i < fields.len() - 1 {
                                    output.push_str(", ");
                                }
                            }
                        }
                        output.push(')');
                    }
                    VariantKind::Struct {
                        fields,
                        has_stripped_fields,
                    } => {
                        output.push_str(" {\n");
                        for &field_id in fields {
                            if let Some(field_item) = data.index.get(&field_id)
                                && let Some(field_name) = &field_item.name
                                && let ItemEnum::StructField(field_type) = &field_item.inner
                            {
                                output.push_str(&format!(
                                    "    {}: {},\n",
                                    field_name,
                                    format_type(field_type, data)
                                ));
                            }
                        }
                        if *has_stripped_fields {
                            output.push_str("    // Some fields omitted\n");
                        }
                        output.push('}');
                    }
                }

                if let Some(discriminant) = &variant.discriminant {
                    output.push_str(&format!(" = {}", discriminant.expr));
                }
            }
        }
        ItemEnum::Primitive(primitive) => {
            output.push_str(&format!("// Primitive type: {}", primitive.name));
        }
        ItemEnum::ExternType => {
            if let Some(name) = &item.name {
                output.push_str(&format!("extern {{ type {}; }}", name));
            }
        }
        ItemEnum::AssocConst { type_, value } => {
            if let Some(name) = &item.name {
                output.push_str(&format!("const {}: {}", name, format_type(type_, data)));
                if let Some(val) = value {
                    output.push_str(&format!(" = {}", val));
                }
                output.push(';');
            }
        }
        ItemEnum::AssocType {
            generics,
            bounds,
            type_,
        } => {
            if let Some(name) = &item.name {
                output.push_str(&format!("type {}", name));
                format_generics(output, generics, data);

                if !bounds.is_empty() {
                    output.push_str(": ");
                    format_bounds(output, bounds, data);
                }

                if let Some(ty) = type_ {
                    output.push_str(&format!(" = {}", format_type(ty, data)));
                }

                format_where_clause(output, &generics.where_predicates, data);
                output.push(';');
            }
        }
    }
}

fn format_generics(output: &mut String, generics: &Generics, data: &Crate) {
    if generics.params.is_empty() {
        return;
    }

    output.push('<');
    for (i, param) in generics.params.iter().enumerate() {
        match &param.kind {
            GenericParamDefKind::Lifetime { outlives } => {
                output.push_str(&format!("'{}", param.name));
                if !outlives.is_empty() {
                    output.push_str(": ");
                    for (j, lifetime) in outlives.iter().enumerate() {
                        output.push_str(&format!("'{}", lifetime));
                        if j < outlives.len() - 1 {
                            output.push_str(" + ");
                        }
                    }
                }
            }
            GenericParamDefKind::Type {
                bounds,
                default,
                is_synthetic,
            } => {
                // If synthetic, add a note
                if *is_synthetic {
                    output.push_str("/* synthetic */ ");
                }

                output.push_str(&param.name);
                if !bounds.is_empty() {
                    output.push_str(": ");
                    format_bounds(output, bounds, data);
                }
                if let Some(default_type) = default {
                    output.push_str(&format!(" = {}", format_type(default_type, data)));
                }
            }
            GenericParamDefKind::Const { type_, default } => {
                output.push_str(&format!(
                    "const {}: {}",
                    param.name,
                    format_type(type_, data)
                ));
                if let Some(default_value) = default {
                    output.push_str(&format!(" = {}", default_value));
                }
            }
        }

        if i < generics.params.len() - 1 {
            output.push_str(", ");
        }
    }
    output.push('>');
}

fn format_where_clause(output: &mut String, predicates: &[WherePredicate], data: &Crate) {
    if predicates.is_empty() {
        return;
    }

    output.push_str("\nwhere\n    ");
    for (i, predicate) in predicates.iter().enumerate() {
        match predicate {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => {
                format_for_binder(output, generic_params);

                output.push_str(&format_type(type_, data));

                if !bounds.is_empty() {
                    output.push_str(": ");
                    format_bounds(output, bounds, data);
                }
            }
            WherePredicate::LifetimePredicate { lifetime, outlives } => {
                output.push_str(&format!("'{}", lifetime));
                if !outlives.is_empty() {
                    output.push_str(": ");
                    for (j, outlive) in outlives.iter().enumerate() {
                        output.push_str(&format!("'{}", outlive));
                        if j < outlives.len() - 1 {
                            output.push_str(" + ");
                        }
                    }
                }
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                output.push_str(&format_type(lhs, data));
                output.push_str(" = ");
                match rhs {
                    Term::Type(type_) => output.push_str(&format_type(type_, data)),
                    Term::Constant(constant) => output.push_str(&constant.expr),
                }
            }
        }

        if i < predicates.len() - 1 {
            output.push_str(",\n    ");
        }
    }
}

fn format_bounds(output: &mut String, bounds: &[GenericBound], data: &Crate) {
    for (i, bound) in bounds.iter().enumerate() {
        match bound {
            GenericBound::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => {
                match modifier {
                    TraitBoundModifier::None => {}
                    TraitBoundModifier::Maybe => output.push('?'),
                    TraitBoundModifier::MaybeConst => output.push_str("~const "),
                }

                format_for_binder(output, generic_params);

                output.push_str(&trait_.path);
                if let Some(args) = &trait_.args {
                    format_generic_args(output, args, data);
                }
            }
            GenericBound::Outlives(lifetime) => {
                output.push_str(&format!("'{}", lifetime));
            }
            GenericBound::Use(args) => {
                output.push_str("use<");
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        PreciseCapturingArg::Lifetime(lifetime) => {
                            output.push_str(&format!("'{}", lifetime))
                        }
                        PreciseCapturingArg::Param(param) => output.push_str(param),
                    }

                    if i < args.len() - 1 {
                        output.push_str(", ");
                    }
                }
                output.push('>');
            }
        }

        if i < bounds.len() - 1 {
            output.push_str(" + ");
        }
    }
}

fn format_generic_args(output: &mut String, args: &GenericArgs, data: &Crate) {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            if args.is_empty() && constraints.is_empty() {
                return;
            }

            output.push('<');

            // Format args
            for (i, arg) in args.iter().enumerate() {
                match arg {
                    GenericArg::Lifetime(lifetime) => output.push_str(&format!("'{}", lifetime)),
                    GenericArg::Type(type_) => output.push_str(&format_type(type_, data)),
                    GenericArg::Const(constant) => output.push_str(&constant.expr),
                    GenericArg::Infer => output.push('_'),
                }

                if i < args.len() - 1 || !constraints.is_empty() {
                    output.push_str(", ");
                }
            }

            // Format constraints
            for (i, constraint) in constraints.iter().enumerate() {
                output.push_str(&constraint.name.to_string());

                // Format constraint args if present
                if let Some(args) = &constraint.args {
                    let mut args_str = String::new();
                    format_generic_args(&mut args_str, args, data);
                    if !args_str.is_empty() && args_str != "<>" {
                        output.push_str(&args_str);
                    }
                }

                match &constraint.binding {
                    AssocItemConstraintKind::Equality(term) => {
                        output.push_str(" = ");
                        match term {
                            Term::Type(type_) => output.push_str(&format_type(type_, data)),
                            Term::Constant(constant) => output.push_str(&constant.expr),
                        }
                    }
                    AssocItemConstraintKind::Constraint(bounds) => {
                        output.push_str(": ");
                        format_bounds(output, bounds, data);
                    }
                }

                if i < constraints.len() - 1 {
                    output.push_str(", ");
                }
            }

            output.push('>');
        }
        GenericArgs::Parenthesized {
            inputs,
            output: output_type,
        } => {
            output.push('(');

            for (i, input) in inputs.iter().enumerate() {
                output.push_str(&format_type(input, data));
                if i < inputs.len() - 1 {
                    output.push_str(", ");
                }
            }

            output.push(')');

            if let Some(output_ty) = output_type {
                output.push_str(&format!(" -> {}", format_type(output_ty, data)));
            }
        }
        GenericArgs::ReturnTypeNotation => {
            output.push_str("::method(..)");
        }
    }
}

fn format_type(ty: &Type, data: &Crate) -> String {
    let mut output = String::new();

    match ty {
        Type::ResolvedPath(path) => {
            output.push_str(&path.path);
            if let Some(args) = &path.args {
                format_generic_args(&mut output, args, data);
            }
        }
        Type::DynTrait(dyn_trait) => {
            output.push_str("dyn ");

            for (i, trait_) in dyn_trait.traits.iter().enumerate() {
                format_for_binder(&mut output, &trait_.generic_params);

                output.push_str(&trait_.trait_.path);
                if let Some(args) = &trait_.trait_.args {
                    format_generic_args(&mut output, args, data);
                }

                if i < dyn_trait.traits.len() - 1 {
                    output.push_str(" + ");
                }
            }

            // Lifetime bound if present
            if let Some(lifetime) = &dyn_trait.lifetime {
                output.push_str(&format!(" + '{}", lifetime));
            }
        }
        Type::Generic(name) => {
            output.push_str(name);
        }
        Type::Primitive(name) => {
            output.push_str(name);
        }
        Type::FunctionPointer(fn_ptr) => {
            format_for_binder(&mut output, &fn_ptr.generic_params);

            // Function header (const, unsafe, extern, etc.)
            if fn_ptr.header.is_const {
                output.push_str("const ");
            }
            if fn_ptr.header.is_unsafe {
                output.push_str("unsafe ");
            }

            format_abi(&mut output, &fn_ptr.header.abi);

            output.push_str("fn(");

            // Parameters
            for (i, (_, param_type)) in fn_ptr.sig.inputs.iter().enumerate() {
                output.push_str(&format_type(param_type, data));
                if i < fn_ptr.sig.inputs.len() - 1 || fn_ptr.sig.is_c_variadic {
                    output.push_str(", ");
                }
            }

            // Variadic
            if fn_ptr.sig.is_c_variadic {
                output.push_str("...");
            }

            output.push(')');

            // Return type
            if let Some(return_type) = &fn_ptr.sig.output {
                output.push_str(&format!(" -> {}", format_type(return_type, data)));
            }
        }
        Type::Tuple(types) => {
            if types.is_empty() {
                output.push_str("()");
            } else {
                output.push('(');
                for (i, ty) in types.iter().enumerate() {
                    output.push_str(&format_type(ty, data));
                    if i < types.len() - 1 {
                        output.push_str(", ");
                    }
                }
                output.push(')');
            }
        }
        Type::Slice(ty) => {
            output.push_str(&format!("[{}]", format_type(ty, data)));
        }
        Type::Array { type_, len } => {
            output.push_str(&format!("[{}; {}]", format_type(type_, data), len));
        }
        Type::Pat {
            type_,
            __pat_unstable_do_not_use,
        } => {
            output.push_str(&format!(
                "{} is {}",
                format_type(type_, data),
                __pat_unstable_do_not_use
            ));
        }
        Type::ImplTrait(bounds) => {
            output.push_str("impl ");
            format_bounds(&mut output, bounds, data);
        }
        Type::Infer => {
            output.push('_');
        }
        Type::RawPointer { is_mutable, type_ } => {
            if *is_mutable {
                output.push_str("*mut ");
            } else {
                output.push_str("*const ");
            }
            output.push_str(&format_type(type_, data));
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            output.push('&');
            if let Some(lt) = lifetime {
                output.push_str(&format!("'{} ", lt));
            }
            if *is_mutable {
                output.push_str("mut ");
            }
            output.push_str(&format_type(type_, data));
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            output.push('<');
            output.push_str(&format_type(self_type, data));

            if let Some(trait_path) = trait_ {
                let _ = write!(output, " as {}", trait_path.path);
                if let Some(trait_args) = &trait_path.args {
                    format_generic_args(&mut output, trait_args, data);
                }
            }

            output.push_str(&format!(">::{}", name));

            if let Some(args) = args {
                let mut args_str = String::new();
                format_generic_args(&mut args_str, args, data);
                if args_str != "<>" && !args_str.is_empty() {
                    output.push_str(&args_str);
                }
            }
        }
    }

    output
}

fn process_module_details(output: &mut String, module: &Module, data: &Crate, _level: usize) {
    if module.is_stripped {
        output.push_str(
            "> **Note:** This module is marked as stripped. Some items may be omitted.\n\n",
        );
    }

    // Reset level when entering a module to avoid excessive nesting
    // This ensures that module contents are always at a reasonable heading level
    process_items(output, &module.items, data, 3);
}

/// Renders a `| Name | Type | Documentation |` table for named fields.
fn render_named_fields_table(
    output: &mut String,
    field_ids: &[Id],
    data: &Crate,
    has_stripped_fields: bool,
) {
    output.push_str("| Name | Type | Documentation |\n|------|------|---------------|\n");

    for &field_id in field_ids {
        let Some(field_item) = data.index.get(&field_id) else {
            continue;
        };
        let Some(field_name) = &field_item.name else {
            continue;
        };
        let ItemEnum::StructField(field_type) = &field_item.inner else {
            continue;
        };
        let docs = field_item
            .docs
            .as_deref()
            .unwrap_or("")
            .replace("\n", "<br>");
        let _ = writeln!(
            output,
            "| `{field_name}` | `{}` | {docs} |",
            format_type(field_type, data)
        );
    }

    if has_stripped_fields {
        output.push_str("| *private fields* | ... | *Some fields have been omitted* |\n");
    }

    output.push('\n');
}

/// Renders a `| Index | Type | Documentation |` table for tuple fields, where a
/// `None` entry denotes a stripped (private) field.
fn render_tuple_fields_table(output: &mut String, fields: &[Option<Id>], data: &Crate) {
    output.push_str("| Index | Type | Documentation |\n|-------|------|---------------|\n");

    for (i, field_opt) in fields.iter().enumerate() {
        let Some(field_id) = field_opt else {
            let _ = writeln!(output, "| {i} | `private` | *Private field* |");
            continue;
        };
        if let Some(field_item) = data.index.get(field_id)
            && let ItemEnum::StructField(field_type) = &field_item.inner
        {
            let docs = field_item
                .docs
                .as_deref()
                .unwrap_or("")
                .replace("\n", "<br>");
            let _ = writeln!(
                output,
                "| {i} | `{}` | {docs} |",
                format_type(field_type, data)
            );
        }
    }

    output.push('\n');
}

/// Renders a method as a fenced `rust` code block followed by its first doc
/// line. `bullet` is the list marker (e.g. `"- "`) and `indent` the
/// continuation indent for the fenced block and doc line.
fn render_method_signature(
    output: &mut String,
    method_item: &Item,
    data: &Crate,
    bullet: &str,
    indent: &str,
) {
    let mut signature = String::new();
    format_item_signature(&mut signature, method_item, data);
    let _ = write!(
        output,
        "{bullet}```rust\n{indent}{}\n{indent}```",
        signature.trim()
    );

    if let Some(line) = method_item.docs.as_deref().and_then(first_doc_line) {
        let _ = write!(output, "\n{indent}{line}");
    }
    output.push_str("\n\n");
}

/// Renders the function items of a single impl block as a bulleted list.
fn render_impl_methods(output: &mut String, impl_id: Id, data: &Crate, bullet: &str, indent: &str) {
    let Some(impl_item) = data.index.get(&impl_id) else {
        return;
    };
    let ItemEnum::Impl(impl_) = &impl_item.inner else {
        return;
    };

    for &item_id in &impl_.items {
        let Some(method_item) = data.index.get(&item_id) else {
            continue;
        };
        if matches!(&method_item.inner, ItemEnum::Function(_)) {
            render_method_signature(output, method_item, data, bullet, indent);
        }
    }
}

/// Renders a bulleted list of items as `- `name`: <first doc line>`.
fn render_name_doc_list(output: &mut String, item_ids: &[Id], data: &Crate) {
    for &id in item_ids {
        let Some(item) = data.index.get(&id) else {
            continue;
        };
        let Some(name) = &item.name else {
            continue;
        };
        let _ = write!(output, "- `{name}`");
        if let Some(line) = item.docs.as_deref().and_then(first_doc_line) {
            let _ = write!(output, ": {line}");
        }
        output.push('\n');
    }
    output.push('\n');
}

/// Renders the `Implementations` section (inherent methods, then trait impls
/// sorted alphabetically) shared by structs, enums, and unions.
fn render_impls(output: &mut String, impl_ids: &[Id], data: &Crate, heading_level: usize) {
    if impl_ids.is_empty() {
        return;
    }

    let _ = write!(output, "{} Implementations\n\n", heading(heading_level));

    let mut trait_impls: BTreeMap<String, Vec<Id>> = BTreeMap::new();
    let mut inherent_impls: Vec<Id> = Vec::new();

    for &impl_id in impl_ids {
        let Some(impl_item) = data.index.get(&impl_id) else {
            continue;
        };
        let ItemEnum::Impl(impl_) = &impl_item.inner else {
            continue;
        };
        if let Some(trait_) = &impl_.trait_ {
            trait_impls
                .entry(trait_.path.clone())
                .or_default()
                .push(impl_id);
        } else {
            inherent_impls.push(impl_id);
        }
    }

    let sub_heading = heading(heading_level + 1);

    if !inherent_impls.is_empty() {
        let _ = write!(output, "{sub_heading} Methods\n\n");
        for &impl_id in &inherent_impls {
            render_impl_methods(output, impl_id, data, "- ", "  ");
        }
    }

    if !trait_impls.is_empty() {
        let _ = write!(output, "{sub_heading} Trait Implementations\n\n");
        for (trait_name, impls) in &trait_impls {
            let _ = writeln!(output, "- **{trait_name}**");
            for &impl_id in impls {
                render_impl_methods(output, impl_id, data, "  - ", "    ");
            }
        }
    }
}

fn process_struct_details(output: &mut String, struct_: &Struct, data: &Crate, level: usize) {
    let heading_level = level.min(6);

    match &struct_.kind {
        StructKind::Unit => {
            // Nothing to detail for unit structs
        }
        StructKind::Tuple(fields) => {
            let _ = write!(output, "{} Fields\n\n", heading(heading_level));
            render_tuple_fields_table(output, fields, data);
        }
        StructKind::Plain {
            fields,
            has_stripped_fields,
        } => {
            let _ = write!(output, "{} Fields\n\n", heading(heading_level));
            render_named_fields_table(output, fields, data, *has_stripped_fields);
        }
    }

    render_impls(output, &struct_.impls, data, heading_level);
}

fn process_enum_details(output: &mut String, enum_: &Enum, data: &Crate, level: usize) {
    let heading_level = level.min(6);
    let _ = write!(output, "{} Variants\n\n", heading(heading_level));

    for &variant_id in &enum_.variants {
        let Some(variant_item) = data.index.get(&variant_id) else {
            continue;
        };
        let Some(variant_name) = &variant_item.name else {
            continue;
        };

        let _ = write!(
            output,
            "{} `{variant_name}`\n\n",
            heading(heading_level + 1)
        );

        if let Some(docs) = &variant_item.docs {
            let _ = write!(output, "{docs}\n\n");
        }

        let ItemEnum::Variant(variant) = &variant_item.inner else {
            continue;
        };

        match &variant.kind {
            VariantKind::Plain => {
                if let Some(discriminant) = &variant.discriminant {
                    let _ = write!(output, "Discriminant: `{}`\n\n", discriminant.expr);
                }
            }
            VariantKind::Tuple(fields) => {
                output.push_str("Fields:\n\n");
                render_tuple_fields_table(output, fields, data);
            }
            VariantKind::Struct {
                fields,
                has_stripped_fields,
            } => {
                output.push_str("Fields:\n\n");
                render_named_fields_table(output, fields, data, *has_stripped_fields);
            }
        }

        if let Some(discriminant) = &variant.discriminant {
            let _ = write!(output, "Discriminant value: `{}`\n\n", discriminant.value);
        }
    }

    if enum_.has_stripped_variants {
        output.push_str(
            "*Note: Some variants have been omitted because they are private or hidden.*\n\n",
        );
    }

    render_impls(output, &enum_.impls, data, heading_level);
}

fn process_union_details(output: &mut String, union_: &Union, data: &Crate, level: usize) {
    let heading_level = level.min(6);

    let _ = write!(output, "{} Fields\n\n", heading(heading_level));
    render_named_fields_table(output, &union_.fields, data, union_.has_stripped_fields);

    render_impls(output, &union_.impls, data, heading_level);
}

fn process_trait_details(output: &mut String, trait_: &Trait, data: &Crate, level: usize) {
    let heading_level = level.min(6);
    // Special traits info
    if trait_.is_auto {
        output.push_str("> This is an auto trait.\n\n");
    }
    if trait_.is_unsafe {
        output.push_str("> This trait is unsafe to implement.\n\n");
    }
    if !trait_.is_dyn_compatible {
        output.push_str(
            "> This trait is not object-safe and cannot be used in dynamic trait objects.\n\n",
        );
    }

    // Associated items
    if !trait_.items.is_empty() {
        // Group items by kind
        let mut required_methods = Vec::new();
        let mut provided_methods = Vec::new();
        let mut assoc_types = Vec::new();
        let mut assoc_consts = Vec::new();

        for &item_id in &trait_.items {
            if let Some(item) = data.index.get(&item_id) {
                match &item.inner {
                    ItemEnum::Function(function) => {
                        if function.has_body {
                            provided_methods.push(item_id);
                        } else {
                            required_methods.push(item_id);
                        }
                    }
                    ItemEnum::AssocType { .. } => assoc_types.push(item_id),
                    ItemEnum::AssocConst { value, .. } => {
                        if value.is_some() {
                            // Has a default value
                            provided_methods.push(item_id);
                        } else {
                            assoc_consts.push(item_id);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Required items
        if !required_methods.is_empty() || !assoc_types.is_empty() || !assoc_consts.is_empty() {
            let _ = write!(output, "{} Required Items\n\n", heading(heading_level));
            let sub_heading = heading(heading_level + 1);

            if !assoc_types.is_empty() {
                let _ = write!(output, "{sub_heading} Associated Types\n\n");
                render_name_doc_list(output, &assoc_types, data);
            }

            if !assoc_consts.is_empty() {
                let _ = write!(output, "{sub_heading} Associated Constants\n\n");
                render_name_doc_list(output, &assoc_consts, data);
            }

            if !required_methods.is_empty() {
                let _ = write!(output, "{sub_heading} Required Methods\n\n");
                render_name_doc_list(output, &required_methods, data);
            }
        }

        // Provided items
        if !provided_methods.is_empty() {
            let _ = write!(output, "{} Provided Methods\n\n", heading(heading_level));
            for &method_id in &provided_methods {
                let Some(method_item) = data.index.get(&method_id) else {
                    continue;
                };
                if matches!(&method_item.inner, ItemEnum::Function(_)) {
                    render_method_signature(output, method_item, data, "- ", "  ");
                }
            }
        }
    }

    // Implementations
    if !trait_.implementations.is_empty() {
        let _ = write!(output, "{} Implementations\n\n", heading(heading_level));
        output.push_str("This trait is implemented for the following types:\n\n");

        for &impl_id in &trait_.implementations {
            let Some(impl_item) = data.index.get(&impl_id) else {
                continue;
            };
            let ItemEnum::Impl(impl_) = &impl_item.inner else {
                continue;
            };
            let _ = write!(output, "- `{}`", format_type(&impl_.for_, data));
            if !impl_.generics.params.is_empty() {
                output.push_str(" with ");
                format_generics(output, &impl_.generics, data);
            }
            output.push('\n');
        }
        output.push('\n');
    }
}

fn process_impl_details(output: &mut String, impl_: &Impl, data: &Crate, level: usize) {
    let heading_level = level.min(6);
    // List all items in the impl
    if !impl_.items.is_empty() {
        let _ = write!(output, "{} Associated Items\n\n", heading(heading_level));
        let sub_heading = heading(heading_level + 1);

        // Group by kind
        let mut methods = Vec::new();
        let mut assoc_types = Vec::new();
        let mut assoc_consts = Vec::new();

        for &item_id in &impl_.items {
            if let Some(item) = data.index.get(&item_id) {
                match &item.inner {
                    ItemEnum::Function(_) => methods.push(item_id),
                    ItemEnum::AssocType { .. } => assoc_types.push(item_id),
                    ItemEnum::AssocConst { .. } => assoc_consts.push(item_id),
                    _ => {}
                }
            }
        }

        if !assoc_types.is_empty() {
            let _ = write!(output, "{sub_heading} Associated Types\n\n");
            for &type_id in &assoc_types {
                process_item(output, data.index.get(&type_id).unwrap(), data, level + 1);
            }
        }

        if !assoc_consts.is_empty() {
            let _ = write!(output, "{sub_heading} Associated Constants\n\n");
            for &const_id in &assoc_consts {
                process_item(output, data.index.get(&const_id).unwrap(), data, level + 1);
            }
        }

        if !methods.is_empty() {
            let _ = write!(output, "{sub_heading} Methods\n\n");
            for &method_id in &methods {
                process_item(output, data.index.get(&method_id).unwrap(), data, level + 1);
            }
        }
    }

    // If this is a trait impl, list the provided trait methods that aren't overridden
    if impl_.trait_.is_some() && !impl_.provided_trait_methods.is_empty() {
        let _ = write!(
            output,
            "{} Provided Trait Methods\n\n",
            heading(heading_level)
        );
        output.push_str("The following methods are available through the trait but not explicitly implemented:\n\n");

        for provided_method in &impl_.provided_trait_methods {
            let _ = writeln!(output, "- `{provided_method}`");
        }

        output.push('\n');
    }

    // If this is a blanket impl, mention it
    if let Some(blanket_type) = &impl_.blanket_impl {
        let _ = write!(
            output,
            "This is a blanket implementation for all types that match: `{}`\n\n",
            format_type(blanket_type, data)
        );
    }
}
