use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxideError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("TOML parsing error")]
    TomlError(#[from] toml::de::Error),
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    #[error("Invalid configuration")]
    ConfigError(String),
    #[error("Build failed")]
    BuildError,
    #[error("Command execution failed")]
    CommandError,
}
