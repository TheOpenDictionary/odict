use std::sync::LazyLock;

use super::semver::SemanticVersion;

pub const SIGNATURE: &[u8] = b"ODICT";

pub const VERSION: LazyLock<SemanticVersion> =
    LazyLock::new(|| SemanticVersion::from(env!("CARGO_PKG_VERSION")));
