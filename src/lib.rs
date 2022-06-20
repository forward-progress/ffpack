#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    missing_docs,
    clippy::missing_docs_in_private_items
)]
#![allow(
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::shadow_unrelated,
    clippy::must_use_candidate,
    clippy::implicit_hasher
)]

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub mod types;

use types::{ManagedFile, Metadata, Versions};

/// High level representation of a modpack
///
/// TODO: Document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Pack {
    /// The metadata for this pack
    pub metadata: Metadata,
    /// The versions of minecraft and the launcher that this pack works with
    pub versions: Versions,
    /// The list of managed files
    pub managed_files: BTreeSet<ManagedFile>,
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            versions: Versions::default(),
            managed_files: [ManagedFile::default()].into_iter().collect(),
        }
    }
}
