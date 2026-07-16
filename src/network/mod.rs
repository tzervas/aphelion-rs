//! Repository metadata fetch helpers.

use crate::{AphelionError, Config};

/// Fetch repository index data (placeholder; returns empty payload).
pub async fn fetch_repo_data(_config: &Config) -> Result<String, AphelionError> {
    // Networking is scaffolded; return empty index until real mirrors are wired.
    Ok(String::new())
}
