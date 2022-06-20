//! Module containing type definitions used by the library
//!
//! TODO: Improve Documentation

mod loader;
mod minecraft;

// Rexport types
pub use loader::Loader;
pub use minecraft::Minecraft;

use semver::Version;
use serde::{Deserialize, Serialize};

/// The versions of minecraft and the launcher for this instance of the pack
///
/// TODO: Document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct Versions {
    /// The version of minecraft this pack works with
    pub minecraft: Minecraft,
    /// The loader this pack works with
    pub loader: Loader,
}

/// The metadata for the pack
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Metadata {
    /// The name of this mod pack
    name: String,
    /// An optional description for this mod pack
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// The author of the pack
    author: String,
    /// The version of the pack
    version: Version,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "My super cool modpack!".to_string(),
            description: Some("Totally a real mod pack!".to_string()),
            author: "Your name here!".to_string(),
            version: Version::parse("0.0.1").unwrap(),
        }
    }
}
