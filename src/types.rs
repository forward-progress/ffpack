//! Module containing type definitions used by the library
//!
//! TODO: Improve Documentation

mod loader;
mod minecraft;

// Rexport types
pub use loader::Loader;
pub use minecraft::Minecraft;

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
