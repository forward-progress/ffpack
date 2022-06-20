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

use serde::{Deserialize, Serialize};

pub mod types;

use types::{Metadata, Versions};

/// High level representation of a modpack
///
/// TODO: Document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct Pack {
    /// The metadata for this pack
    pub metadata: Metadata,
    /// The versions of minecraft and the launcher that this pack works with
    pub versions: Versions,
}
