//! Type wrapper for the minecraft version scheme

use std::{cmp::Ordering, fmt::Display, num::ParseIntError};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing::{debug, instrument, trace};

/// A decoded Minecraft version
///
/// This currently treats a snapshot version as greater than any release version
///
///
/// TODO: Document
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
#[serde(tag = "type")]
pub enum Minecraft {
    /// Release version of the game
    ///
    /// This does not include snapshots post 1.0
    Release {
        /// Major version (`x` in `x.y.z`)
        major: u16,
        /// Minor version (`y` in `x.y.z`)
        minor: u16,
        /// Patch version (`z` in `x.y.z`)
        ///
        /// This value will be `None` if not specified
        #[serde(skip_serializing_if = "Option::is_none")]
        patch: Option<u16>,
    },
    /// Snapshot version of the game after the release
    Snapshot {
        /// The year component of the snapshot (`AA` in `AAwBBx`)
        year: u16,
        /// The week component of the snapshot (`BB` in `AAwBBx`)
        week: u16,
        /// The specifier component of the snapshot (`x` in `AAwBBx`)
        ///
        /// We use a string here to not be screwed in the unexpected event Mojang releases a double
        /// letter snapshot version
        specifier: String,
    },
}

impl Minecraft {
    /// Create a new version from a string, verifying it in the process
    ///
    /// TODO: Document
    #[instrument(skip(from), fields(raw = from.as_ref()), err)]
    pub fn new(from: impl AsRef<str>) -> Result<Minecraft, MinecraftVersionError> {
        // Build our regexes (lazily)
        /// Regex for matching a release version (`x.y.z` or `x.y`)
        static RELEASE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\d+)\.(\d+)(?:\.(\d+))?$").unwrap());
        /// Regex for matching a snapshot version (`XXwYYZ`)
        static SNAPSHOT_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\d+)w(\d+)(\w+)$").unwrap());
        debug!("Parsing Version");
        let from = from.as_ref();
        // Attempt to match a Release Version
        if let Some(captures) = RELEASE_REGEX.captures(from) {
            trace!("Parsing a release version");
            let major = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .context(InvalidComponentSnafu)?;
            let minor = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .context(InvalidComponentSnafu)?;
            let patch = if let Some(patch_raw) = captures.get(3) {
                Some(patch_raw.as_str().parse().context(InvalidComponentSnafu)?)
            } else {
                None
            };
            Ok(Self::Release {
                major,
                minor,
                patch,
            })
        } else if let Some(captures) = SNAPSHOT_REGEX.captures(from) {
            // Attempt to match a snapshot
            let year = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .context(InvalidComponentSnafu)?;
            let week = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .context(InvalidComponentSnafu)?;
            let specifier = captures.get(3).unwrap().as_str().to_string();
            Ok(Self::Snapshot {
                year,
                week,
                specifier,
            })
        } else {
            // No matching pattern
            NoSupportedPatternSnafu {
                version: from.to_string(),
            }
            .fail()
        }
    }

    /// Internal function used for simplifying ordering
    fn order_priority(&self) -> usize {
        // This must always return values that are different for each version
        match self {
            Minecraft::Release { .. } => 1,
            Minecraft::Snapshot { .. } => 2,
        }
    }
}

impl PartialOrd for Minecraft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Minecraft {
    fn cmp(&self, other: &Self) -> Ordering {
        // Only attempt a comparison if the version is of the same order priority
        match self.order_priority().cmp(&other.order_priority()) {
            // We know that the two values must be of the same "type", so we have to do a deep
            // comparison
            Ordering::Equal => match self {
                Minecraft::Release {
                    major: major_self,
                    minor: minor_self,
                    patch: patch_self,
                } => {
                    if let Minecraft::Release {
                        major: major_other,
                        minor: minor_other,
                        patch: patch_other,
                    } = other
                    {
                        // do a semver sytle comparison
                        match (
                            major_self.cmp(major_other),
                            minor_self.cmp(minor_other),
                            patch_self.cmp(patch_other),
                        ) {
                            // If the major versions are equal, and the minor versions are equal,
                            // compare on the patch version
                            (Ordering::Equal, Ordering::Equal, patch) => patch,
                            // If the major versions are equal, but the minor versions aren't, then
                            // compare on the minor version
                            (Ordering::Equal, minor, _) => minor,
                            // If the major version aren't equal, just compare on those
                            (major, _, _) => major,
                        }
                    } else {
                        // This is unreachable as `order_priority` _must_ return unique values for
                        // each variant
                        unreachable!()
                    }
                }
                Minecraft::Snapshot {
                    year: year_self,
                    week: week_self,
                    specifier: specifier_self,
                } => {
                    if let Minecraft::Snapshot {
                        year: year_other,
                        week: week_other,
                        specifier: specifer_other,
                    } = other
                    {
                        match (
                            year_self.cmp(year_other),
                            week_self.cmp(week_other),
                            specifier_self.cmp(specifer_other),
                        ) {
                            // If the years are equal, and the weeks are equal, compare on the
                            // specifier character/characters. These are in lexigraphical order,
                            // hopefully
                            (Ordering::Equal, Ordering::Equal, specifier) => specifier,
                            // If the years are equal, but the weeks aren't, then compare on the
                            // week
                            (Ordering::Equal, week, _) => week,
                            // If the years aren't equal, just compare on those
                            (year, _, _) => year,
                        }
                    } else {
                        // This is unreachable as `order_priority` _must_ return unique values for
                        // each variant
                        unreachable!()
                    }
                }
            },
            // If they aren't of the same type, we only need the shallow comparison provided by `order_priority`
            x => x,
        }
    }
}

impl Display for Minecraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Minecraft::Release {
                major,
                minor,
                patch,
            } => {
                if let Some(patch) = patch {
                    write!(f, "{}.{}.{}", major, minor, patch)
                } else {
                    write!(f, "{}.{}", major, minor)
                }
            }
            Minecraft::Snapshot {
                year,
                week,
                specifier,
            } => write!(f, "{}w{}{}", year, week, specifier),
        }
    }
}

impl Default for Minecraft {
    fn default() -> Self {
        // Current minecraft version at time of writing
        Minecraft::Release {
            major: 1,
            minor: 19,
            patch: None,
        }
    }
}

/// Error that occurs during version parsing
///
/// TODO: Document
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum MinecraftVersionError {
    /// Version provided did not match any supported pattern
    #[snafu(display("Version provided did not match any supplied pattern: {}", version))]
    NoSupportedPattern {
        /// The provided version
        version: String,
    },
    /// Invalid version component
    InvalidComponent {
        /// Underlying parse error
        source: ParseIntError,
    },
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    // Sanity check a few hand picked versions
    #[test]
    fn smoke() {
        let pairs = vec![
            (
                "1.18.2",
                Minecraft::Release {
                    major: 1,
                    minor: 18,
                    patch: Some(2),
                },
            ),
            (
                "1.19",
                Minecraft::Release {
                    major: 1,
                    minor: 19,
                    patch: None,
                },
            ),
            (
                "18w10d",
                Minecraft::Snapshot {
                    year: 18,
                    week: 10,
                    specifier: "d".to_string(),
                },
            ),
        ];
        for (raw, version) in pairs {
            match Minecraft::new(raw) {
                Ok(parsed) => assert_eq!(version, parsed),
                Err(e) => {
                    println!("Failed to parse version: {}", raw);
                    println!("Error: {:?}", e);
                    panic!("Test failed");
                }
            }
        }
    }

    // Test the ordering
    #[test]
    fn order() {
        // An ordered list of test version
        let versions: Vec<Minecraft> = vec![
            "1.1", "1.6.2", "1.18", "1.18.1", "1.18.2", "1.19", "18w10d", "22w28a", "22w28b",
        ]
        .into_iter()
        .map(|x| Minecraft::new(x).unwrap())
        .collect();
        // Since the test list is ordered, we can use a comparison of the indexes to check against
        for (index_1, version_1) in versions.iter().enumerate() {
            for (index_2, version_2) in versions.iter().enumerate() {
                assert_eq!(index_1.cmp(&index_2), version_1.cmp(version_2));
            }
        }
    }

    // Test the display implementation by round tripping
    #[test]
    fn display() {
        let versions_raw = vec![
            "1.1", "1.6.2", "1.18", "1.18.1", "1.18.2", "1.19", "18w10d", "22w28a", "22w28b",
        ];
        for version_raw in versions_raw {
            let parsed = Minecraft::new(version_raw).unwrap();
            let displayed = format!("{}", parsed);
            assert_eq!(version_raw, &displayed);
        }
    }
}
