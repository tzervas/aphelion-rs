//! Configuration loading and defaults.

use crate::{AphelionError, Config};

/// Initialize configuration with defaults.
pub fn init() -> Result<Config, AphelionError> {
    Ok(Config {
        db_path: "aphelion.db".to_string(),
        repo_url: String::new(),
    })
}
