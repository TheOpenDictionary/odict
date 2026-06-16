# Crate Documentation

**Version:** 3.2.2

**Format Version:** 57

# Module `odict`

## Modules

## Module `config`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"config\"), span: crates/odict/src/lib.rs:1:7: 1:25 (#0) }])]")`

```rust
pub mod config { /* ... */ }
```

### Constants and Statics

#### Constant `DEFAULT_CONFIG_DIR`

```rust
pub const DEFAULT_CONFIG_DIR: std::sync::LazyLock<std::path::PathBuf> = _;
```

## Module `format`

```rust
pub mod format { /* ... */ }
```

### Modules

## Module `json`

```rust
pub mod json { /* ... */ }
```

### Re-exports

#### Re-export `json::*`

```rust
pub use json::*;
```

## Module `sql`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"sql\"), span: crates/odict/src/format/mod.rs:6:7: 6:22 (#0) }])]")`

```rust
pub mod sql { /* ... */ }
```

### Re-exports

#### Re-export `SQLDialect`

```rust
pub use self::utils::SQLDialect;
```

#### Re-export `self::sql::*`

```rust
pub use self::sql::*;
```

## Module `xml`

```rust
pub mod xml { /* ... */ }
```

### Traits

#### Trait `ToXML`

```rust
pub trait ToXML {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn to_xml(self: Self, pretty: bool) -> crate::Result<String>
where
    Self: Sized + Serialize { /* ... */ }
  ```

##### Implementations

This trait is implemented for the following types:

- `crate::schema::Dictionary`
- `crate::schema::Entry`
- `&crate::schema::Entry`

## Module `fs`

```rust
pub mod fs { /* ... */ }
```

### Functions

#### Function `infer_path`

```rust
pub fn infer_path<P: Into<std::path::PathBuf> + AsRef<std::ffi::OsStr>>(path: P) -> std::path::PathBuf { /* ... */ }
```

## Module `schema`

```rust
pub mod schema { /* ... */ }
```

### Re-exports

#### Re-export `self::definition::*`

```rust
pub use self::definition::*;
```

#### Re-export `self::dictionary::*`

```rust
pub use self::dictionary::*;
```

#### Re-export `self::entry::*`

```rust
pub use self::entry::*;
```

#### Re-export `self::entry_ref::*`

```rust
pub use self::entry_ref::*;
```

#### Re-export `self::enums::*`

```rust
pub use self::enums::*;
```

#### Re-export `self::etymology::*`

```rust
pub use self::etymology::*;
```

#### Re-export `self::example::*`

```rust
pub use self::example::*;
```

#### Re-export `self::form::*`

```rust
pub use self::form::*;
```

#### Re-export `self::form_kind::*`

```rust
pub use self::form_kind::*;
```

#### Re-export `self::group::*`

```rust
pub use self::group::*;
```

#### Re-export `self::id::*`

```rust
pub use self::id::*;
```

#### Re-export `self::media_url::*`

```rust
pub use self::media_url::*;
```

#### Re-export `self::note::*`

```rust
pub use self::note::*;
```

#### Re-export `self::pos::*`

```rust
pub use self::pos::*;
```

#### Re-export `self::pronunciation::*`

```rust
pub use self::pronunciation::*;
```

#### Re-export `self::pronunciation_kind::*`

```rust
pub use self::pronunciation_kind::*;
```

#### Re-export `self::sense::*`

```rust
pub use self::sense::*;
```

#### Re-export `self::translation::*`

```rust
pub use self::translation::*;
```

## Macros

### Macro `entryset`

**Attributes:**

- `MacroExport`

Creates an `EntrySet` from a list of elements.

This macro provides a convenient way to create an `EntrySet` (which is a type alias for `IndexSet<Entry>`)
with pre-allocated capacity based on the number of elements provided.

# Examples

```ignore
use odict::entryset;
use odict::schema::Entry;

let set = entryset![entry1, entry2, entry3];
```

```rust
pub macro_rules! entryset {
    /* macro_rules! entryset {
    ($($value:expr,)+) => { ... };
    ($($value:expr),*) => { ... };
} */
}
```

### Macro `senseset`

**Attributes:**

- `MacroExport`

Creates a `SenseSet` from a list of elements.

This macro provides a convenient way to create a `SenseSet` (which is a type alias for `IndexSet<Sense>`)
with pre-allocated capacity based on the number of elements provided.

# Examples

```ignore
use odict::senseset;
use odict::schema::Sense;

let set = senseset![sense1, sense2, sense3];
```

```rust
pub macro_rules! senseset {
    /* macro_rules! senseset {
    ($($value:expr,)+) => { ... };
    ($($value:expr),*) => { ... };
} */
}
```

### Macro `serializable`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! serializable {
    /* macro_rules! serializable {
    ($i:item) => { ... };
} */
}
```

### Macro `serializable_test`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! serializable_test {
    /* macro_rules! serializable_test {
    ($i:item) => { ... };
} */
}
```

### Macro `serializable_custom`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! serializable_custom {
    /* macro_rules! serializable_custom {
    ($i:item) => { ... };
} */
}
```

### Macro `serializable_enum`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! serializable_enum {
    /* macro_rules! serializable_enum {
    ($i:item) => { ... };
} */
}
```

## Re-exports

### Re-export `CompressOptions`

```rust
pub use self::compress::CompressOptions;
```

### Re-export `self::core::*`

```rust
pub use self::core::*;
```

### Re-export `self::error::*`

```rust
pub use self::error::*;
```

### Re-export `self::ext::*`

```rust
pub use self::ext::*;
```

### Re-export `self::load::*`

```rust
pub use self::load::*;
```

### Re-export `self::odict::*`

```rust
pub use self::odict::*;
```

