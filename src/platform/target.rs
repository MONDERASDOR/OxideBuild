use crate::error::OxideError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    Other(String),
}

impl FromStr for Platform {
    type Err = OxideError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linux" => Ok(Platform::Linux),
            "windows" => Ok(Platform::Windows),
            "macos" => Ok(Platform::MacOS),
            other => Ok(Platform::Other(other.to_string())),
        }
    }
}

pub trait TargetPlatform {
    fn detect() -> anyhow::Result<Platform>;
    fn build(&self, config: &super::config::OxideConfig, profile: Option<&str>) -> anyhow::Result<()>;
}

pub fn detect_or_select(target: Option<String>) -> anyhow::Result<Box<dyn TargetPlatform>> {
    let platform = match target {
        Some(t) => t.parse()?,
        None => Platform::detect()?,
    };

    match platform {
        Platform::Linux => Ok(Box::new(super::linux::LinuxPlatform)),
        Platform::Windows => Ok(Box::new(super::windows::WindowsPlatform)),
        Platform::MacOS => Ok(Box::new(super::macos::MacOSPlatform)),
        Platform::Other(_) => Err(OxideError::UnsupportedPlatform.into()),
    }
}
