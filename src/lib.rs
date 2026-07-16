//! Aphelion-rs: modern Linux package manager scaffold (Debian/Ubuntu focus).

pub mod config;
pub mod database;
pub mod debian;
pub mod network;
pub mod package;
pub mod utils;

/// Runtime configuration for the package manager.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub db_path: String,
    pub repo_url: String,
}

/// Common error type for package-manager operations.
#[derive(Debug)]
pub enum AphelionError {
    NetworkError(String),
    DatabaseError(String),
    ConfigError(String),
    Other(String),
}

impl std::fmt::Display for AphelionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(m) | Self::DatabaseError(m) | Self::ConfigError(m) | Self::Other(m) => {
                write!(f, "{m}")
            }
        }
    }
}

impl std::error::Error for AphelionError {}

impl From<sqlite::Error> for AphelionError {
    fn from(err: sqlite::Error) -> Self {
        Self::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AphelionError {
    fn from(err: reqwest::Error) -> Self {
        Self::NetworkError(err.to_string())
    }
}
