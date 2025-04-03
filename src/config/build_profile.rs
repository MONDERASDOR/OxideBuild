use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuildProfile {
    Debug,
    Release,
    Custom(String),
}

impl Default for BuildProfile {
    fn default() -> Self {
        BuildProfile::Debug
    }
}

impl std::fmt::Display for BuildProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildProfile::Debug => write!(f, "debug"),
            BuildProfile::Release => write!(f, "release"),
            BuildProfile::Custom(s) => write!(f, "{}", s),
        }
    }
}
