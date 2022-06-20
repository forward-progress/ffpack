//! Module containing type definitions used by the library
//!
//! TODO: Improve Documentation

mod minecraft_version;

// Rexport types
pub use minecraft_version::MinecraftVersion;
use serde::{Deserialize, Serialize};

/// The versions of minecraft and the launcher for this instance of the pack
///
/// TODO: Document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct Versions {
    /// The version of minecraft this pack works with
    pub minecraft: MinecraftVersion,
}
