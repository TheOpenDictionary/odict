use once_cell::sync::Lazy;

use super::semver::SemanticVersion;

pub const SIGNATURE: &[u8] = b"ODICT";

pub const VERSION: Lazy<SemanticVersion> =
    Lazy::new(|| SemanticVersion::from(env!("CARGO_PKG_VERSION")));
