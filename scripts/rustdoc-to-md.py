#!/usr/bin/env python3
"""
Extracts Rust doc comments (///) from source files and generates Markdown documentation.

This script parses Rust source files for structs, functions, and their fields/methods,
then formats the extracted documentation as Markdown suitable for inclusion in a
documentation site.

Usage:
    python scripts/rustdoc-to-md.py python/src       # Generate docs for Python bindings
    python scripts/rustdoc-to-md.py node/src          # Generate docs for Node bindings

Alternatively, use cargo rustdoc with JSON output and the `rustdoc-md` tool:
    RUSTC_BOOTSTRAP=1 cargo rustdoc -p odict_python -- -Z unstable-options --output-format json
    rustdoc-md target/doc/theopendictionary.json -o docs/python-api.md
"""

import re
import sys
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class DocItem:
    """A documented item extracted from Rust source."""
    name: str
    kind: str  # "struct", "function", "method", "field", "getter"
    doc: str
    signature: str = ""
    fields: list = field(default_factory=list)
    methods: list = field(default_factory=list)


def extract_doc_comment(lines: list[str], end_idx: int) -> str:
    """Extract consecutive /// doc comments ending at or before the given line.

    Skips over attribute lines (#[...]) to find the doc comment block.
    """
    # First, skip backwards over attribute lines
    i = end_idx
    while i >= 0:
        stripped = lines[i].strip()
        if stripped.startswith("///"):
            break
        elif stripped.startswith("#[") or stripped == "":
            i -= 1
            continue
        else:
            return ""
        i -= 1

    # Now extract the doc comment lines
    doc_lines = []
    while i >= 0 and lines[i].strip().startswith("///"):
        comment = lines[i].strip().removeprefix("///")
        # Preserve leading space for indented content, but strip the first space
        if comment.startswith(" "):
            comment = comment[1:]
        doc_lines.append(comment)
        i -= 1
    doc_lines.reverse()
    return "\n".join(doc_lines).strip()


def parse_rust_file(filepath: Path) -> list[DocItem]:
    """Parse a Rust file and extract documented items."""
    content = filepath.read_text()
    lines = content.splitlines()
    items = []

    i = 0
    while i < len(lines):
        line = lines[i].strip()

        # Detect #[pyfunction]
        if line == "#[pyfunction]":
            doc = extract_doc_comment(lines, i - 1)
            # Find the function signature
            j = i + 1
            while j < len(lines) and not lines[j].strip().startswith("pub fn "):
                j += 1
            if j < len(lines):
                sig = extract_fn_signature(lines, j)
                name = re.search(r"pub fn (\w+)", lines[j])
                if name:
                    items.append(DocItem(
                        name=name.group(1),
                        kind="function",
                        doc=doc,
                        signature=sig,
                    ))

        # Detect #[pyclass] or #[napi(object)]
        if line == "#[pyclass]" or line.startswith("#[pyclass") or line.startswith("#[napi"):
            # Only match struct-level napi, not method-level
            if line.startswith("#[napi") and "object" not in line and "constructor" not in line:
                i += 1
                continue

            doc = extract_doc_comment(lines, i - 1)
            # Find the struct name
            j = i + 1
            while j < len(lines) and not lines[j].strip().startswith("pub struct "):
                j += 1
            if j < len(lines):
                name_match = re.search(r"pub struct (\w+)", lines[j])
                if name_match:
                    struct_name = name_match.group(1)
                    struct_item = DocItem(
                        name=struct_name,
                        kind="struct",
                        doc=doc,
                    )
                    # Extract fields from struct body
                    if lines[j].strip().endswith("{"):
                        k = j + 1
                        while k < len(lines) and not lines[k].strip().startswith("}"):
                            field_line = lines[k].strip()
                            if field_line.startswith("pub "):
                                # Look back for doc comment, skipping #[pyo3(...)] attrs
                                field_doc = extract_doc_comment(lines, k - 1)
                                field_match = re.match(
                                    r"pub\s+(\w+):\s*(.+?),?\s*$", field_line
                                )
                                if field_match:
                                    struct_item.fields.append(DocItem(
                                        name=field_match.group(1),
                                        kind="field",
                                        doc=field_doc,
                                        signature=field_match.group(2),
                                    ))
                            k += 1
                    items.append(struct_item)

        # Detect #[pymethods] impl blocks
        if line == "#[pymethods]":
            j = i + 1
            while j < len(lines) and not lines[j].strip().startswith("impl "):
                j += 1
            if j < len(lines):
                impl_match = re.search(r"impl (\w+)", lines[j])
                if impl_match:
                    impl_name = impl_match.group(1)
                    # Find the matching struct in items
                    target = None
                    for item in items:
                        if item.name == impl_name and item.kind == "struct":
                            target = item
                            break

                    # Parse methods in the impl block
                    brace_depth = 0
                    k = j
                    while k < len(lines):
                        if "{" in lines[k]:
                            brace_depth += lines[k].count("{")
                        if "}" in lines[k]:
                            brace_depth -= lines[k].count("}")
                        if brace_depth == 0 and k > j:
                            break

                        mline = lines[k].strip()

                        # Check for pub fn (but skip dunder methods)
                        if mline.startswith("pub fn ") and "__" not in mline:
                            # Extract doc comment, skipping attribute lines
                            method_doc = extract_doc_comment(lines, k - 1)

                            # Check for #[getter] in preceding attribute lines
                            is_getter = False
                            is_staticmethod = False
                            is_new = False
                            for back in range(max(0, k - 10), k):
                                attr = lines[back].strip()
                                if attr == "#[getter]":
                                    is_getter = True
                                elif attr == "#[staticmethod]":
                                    is_staticmethod = True
                                elif attr == "#[new]":
                                    is_new = True
                                elif attr.startswith("pub fn ") or (attr.startswith("///") and back < k - 1):
                                    break  # stop looking back past another function or doc

                            sig = extract_fn_signature(lines, k)

                            name_match = re.search(r"pub fn (\w+)", mline)
                            if name_match and method_doc:
                                kind = "getter" if is_getter else "method"
                                if is_new:
                                    kind = "constructor"

                                method = DocItem(
                                    name=name_match.group(1),
                                    kind=kind,
                                    doc=method_doc,
                                    signature=sig,
                                )
                                if target:
                                    target.methods.append(method)
                        k += 1

        i += 1

    return items


def extract_fn_signature(lines: list[str], start: int) -> str:
    """Extract a function signature starting from the given line."""
    sig_lines = []
    paren_depth = 0
    i = start
    while i < len(lines):
        line = lines[i]
        sig_lines.append(line.rstrip())
        paren_depth += line.count("(") - line.count(")")
        if paren_depth <= 0 and ")" in line:
            # Check for return type on next line
            if i + 1 < len(lines) and lines[i + 1].strip().startswith("->"):
                sig_lines.append(lines[i + 1].rstrip())
            break
        i += 1

    sig = " ".join(l.strip() for l in sig_lines)
    # Clean up: extract just the function part
    match = re.search(r"(pub fn \w+.*?)(?:\s*\{|\s*where)", sig)
    if match:
        return match.group(1).strip()
    match = re.search(r"(pub fn \w+[^{]*)", sig)
    if match:
        return match.group(1).strip().rstrip("{").strip()
    return sig


def rust_type_to_display(ty: str) -> str:
    """Convert a Rust type to a more readable display format."""
    ty = ty.strip().rstrip(",")
    # Option<T> -> T | None
    m = re.match(r"Option<(.+)>$", ty)
    if m:
        inner = rust_type_to_display(m.group(1))
        return f"{inner} | None"
    # Vec<T> -> list[T]
    m = re.match(r"Vec<(.+)>$", ty)
    if m:
        inner = rust_type_to_display(m.group(1))
        return f"list[{inner}]"
    # Either<A, B> -> A | B
    m = re.match(r"Either<(.+),\s*(.+)>$", ty)
    if m:
        a = rust_type_to_display(m.group(1))
        b = rust_type_to_display(m.group(2))
        return f"{a} | {b}"
    # Basic type mappings
    mappings = {
        "String": "str",
        "&str": "str",
        "u32": "int",
        "u64": "int",
        "i32": "int",
        "i64": "int",
        "usize": "int",
        "bool": "bool",
        "f32": "float",
        "f64": "float",
    }
    return mappings.get(ty, ty)


def items_to_markdown(items: list[DocItem], title: str) -> str:
    """Convert extracted items to Markdown format."""
    md = []
    md.append(f"# {title}\n")
    md.append("*Auto-generated from Rust doc comments.*\n")
    md.append("---\n")

    # Separate functions and structs
    functions = [i for i in items if i.kind == "function"]
    structs = [i for i in items if i.kind == "struct"]

    if functions:
        md.append("## Functions\n")
        for func in functions:
            md.append(f"### `{func.name}()`\n")
            if func.doc:
                md.append(f"{func.doc}\n")

    for struct in structs:
        md.append(f"## `{struct.name}`\n")
        if struct.doc:
            md.append(f"{struct.doc}\n")

        # Constructors
        constructors = [m for m in struct.methods if m.kind == "constructor"]
        if constructors:
            md.append("### Constructors\n")
            for c in constructors:
                md.append(f"#### `{struct.name}()`\n")
                if c.doc:
                    md.append(f"{c.doc}\n")

        # Static methods
        statics = [m for m in struct.methods if m.kind == "static"]
        for s in statics:
            md.append(f"### `{struct.name}.{s.name}()`\n")
            if s.doc:
                md.append(f"{s.doc}\n")

        # Properties (fields + getters)
        getters = [m for m in struct.methods if m.kind == "getter"]
        if struct.fields or getters:
            md.append("### Properties\n")
            md.append("| Property | Type | Description |")
            md.append("|----------|------|-------------|")
            for f in struct.fields:
                ty = rust_type_to_display(f.signature)
                doc = f.doc.replace("\n", " ") if f.doc else ""
                md.append(f"| `{f.name}` | `{ty}` | {doc} |")
            for g in getters:
                doc = g.doc.replace("\n", " ") if g.doc else ""
                md.append(f"| `{g.name}` | — | {doc} |")
            md.append("")

        # Methods (non-getter, non-constructor)
        methods = [m for m in struct.methods if m.kind == "method"]
        if methods:
            md.append("### Methods\n")
            for m in methods:
                md.append(f"#### `{m.name}()`\n")
                if m.doc:
                    md.append(f"{m.doc}\n")

        md.append("---\n")

    return "\n".join(md)


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <source_directory> [output_file]")
        print(f"Example: {sys.argv[0]} python/src docs/python-api-generated.md")
        sys.exit(1)

    src_dir = Path(sys.argv[1])
    output_file = sys.argv[2] if len(sys.argv) > 2 else None

    if not src_dir.exists():
        print(f"Error: directory {src_dir} does not exist")
        sys.exit(1)

    # Collect all .rs files
    rs_files = sorted(src_dir.rglob("*.rs"))
    all_items = []

    for rs_file in rs_files:
        items = parse_rust_file(rs_file)
        all_items.extend(items)

    if not all_items:
        print("No documented items found.")
        sys.exit(1)

    # Determine title from directory name
    dir_name = src_dir.parent.name if src_dir.name == "src" else src_dir.name
    title_map = {"python": "Python API", "node": "JavaScript API"}
    title = title_map.get(dir_name, f"{dir_name} API")

    md = items_to_markdown(all_items, title)

    if output_file:
        Path(output_file).write_text(md)
        print(f"Generated {output_file} ({len(all_items)} items)")
    else:
        print(md)


if __name__ == "__main__":
    main()
