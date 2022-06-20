//! Type wrapper for dealing with files

use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use url::Url;

/// Marker to determine if this mod is needed on the server, the client, or both
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash, PartialOrd, Ord)]
pub enum Side {
    /// Client side
    Client,
    /// Server side
    Server,
    /// Both server and client side
    Both,
}

impl Default for Side {
    fn default() -> Self {
        Side::Both
    }
}

/// Description of a managed file in the pack
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct ManagedFile {
    /// The name of the mod/file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional description of this mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The filename to download
    pub filename: String,
    /// Should this mod be installed in the development profile of the pack
    pub devel: bool,
    /// The relative path to the file from the minecraft directory
    pub path: RelativePathBuf,
    /// Is this mod server side or client side?
    pub side: Side,
    /// The source of this file
    pub source: Source,
}

impl PartialOrd for ManagedFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ManagedFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl Default for ManagedFile {
    fn default() -> Self {
        Self {
            name: Some("My totally awesome mode".to_string()),
            description: Some("It makes trees blue".to_string()),
            filename: "My Awesome Mod.jar".to_string(),
            devel: true,
            path: RelativePathBuf::from_path("mods/MyAwesomeMod.jar").unwrap(),
            side: Side::default(),
            source: Source::default(),
        }
    }
}

/// Sources a file can come from
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash, PartialOrd, Ord)]
pub enum Source {
    /// Raw URL with no version management
    Url {
        /// The url to download the file from
        url: Url,
        /// The blake3 hash of this url
        #[serde(with = "hex::serde")]
        blake3: [u8; 32],
    },
    /// Path to a file in the repository
    Path {
        /// The path the file is located at relative to the directory the manifest is in
        path: RelativePathBuf,
        /// The blake3 hash of this url
        #[serde(with = "hex::serde")]
        blake3: [u8; 32],
    },
    /// Git repoistory
    Git {
        /// The url of the repository
        url: Url,
        /// The branch to work off of, defaulting to the default branch
        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,
    },
    /// Slug for a supported forge (`github:username/project`, `gitlab:username/project`, etc)
    Slug {
        /// The slug
        slug: String,
        /// The branch to work off of, defaulting to the default branch
        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,
    },
    /// Slug for a supported forge (`github:username/project`, `gitlab:username/project`, etc), but
    /// hitting the releases page instead of the repo
    ///
    /// This allows you to specify a regex to find an artifact for the correct version, and the
    /// library will use the latest release that has a matching artifact. This regex must only
    /// return one result
    ///
    /// You can optionally specify a release regex to match a specific range of releases
    SlugReleases {
        /// The slug
        slug: String,
        /// The regex for matching artifact name
        artifact_regex: String,
        /// The regex for matching release name
        #[serde(skip_serializing_if = "Option::is_none")]
        release_regex: Option<String>,
    },
    /// Modrinth mod
    Modrinth {
        /// Slug for the mod
        slug: String,
    },
    /// Curseforge mod
    Curseforge {
        /// Slug for the mod
        slug: String,
    },
}

impl Default for Source {
    fn default() -> Self {
        Self::Url {
            url: Url::parse("https://example.org/mods/MyAwesomeMod-1.2.3.jar").unwrap(),
            blake3: Default::default(),
        }
    }
}
