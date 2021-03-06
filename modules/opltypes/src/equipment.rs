//! Defines the Equipment field.

use std::fmt;

/// The Equipment field.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize, PartialOrd)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename = "Single-ply")]
    #[strum(serialize = "Single-ply")]
    Single,
    #[serde(rename = "Multi-ply")]
    #[strum(serialize = "Multi-ply")]
    Multi,
    Straps,
}

impl Default for Equipment {
    fn default() -> Equipment {
        Equipment::Multi
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Equipment::Raw => write!(f, "Raw"),
            Equipment::Wraps => write!(f, "Wraps"),
            Equipment::Single => write!(f, "Single-ply"),
            Equipment::Multi => write!(f, "Multi-ply"),
            Equipment::Straps => write!(f, "Straps"),
        }
    }
}
