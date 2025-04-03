pub mod target;
pub mod linux;
pub mod windows;
pub mod macos;
pub mod cross_compile;

pub use target::{Platform, TargetPlatform, detect_or_select};
