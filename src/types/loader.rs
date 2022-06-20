//! Type wrappers for launchers and their version schemes

use std::fmt::Display;

use semver::Version;
use serde::{Deserialize, Serialize};

/// A decoded loader + version combo
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Hash)]
#[serde(tag = "loader", content = "version")]
pub enum Loader {
    /// A version of the [Quilt](https://quiltmc.org/) loader
    Quilt(Version),
    /// A version of the [Fabric](https://fabricmc.net/) loader
    Fabric(Version),
    /// A version of the [Forge](https://forums.minecraftforge.net/) loader
    Forge(Version),
}

impl Loader {
    /// Creates a Quilt loader from a [`Version`]
    pub fn new_quilt(version: Version) -> Self {
        Self::Quilt(version)
    }
    /// Creates a Fabric loader from a [`Version`]
    pub fn new_fabric(version: Version) -> Self {
        Self::Fabric(version)
    }
    /// Creates a Forge loader from a [`Version`]
    pub fn new_forge(version: Version) -> Self {
        Self::Forge(version)
    }

    /// Returns the name of this loader
    pub fn name(&self) -> &'static str {
        match self {
            Loader::Quilt(_) => "Quilt",
            Loader::Fabric(_) => "Fabric",
            Loader::Forge(_) => "Forge",
        }
    }

    /// Returns the version of this loader
    pub fn version(&self) -> &Version {
        match self {
            Loader::Quilt(version) | Loader::Fabric(version) | Loader::Forge(version) => version,
        }
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::Quilt(Version::parse("0.17.1-beta.3").unwrap())
    }
}

impl Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.version())
    }
}
