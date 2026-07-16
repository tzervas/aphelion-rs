//! Local package database (SQLite).

use sqlite::Connection;

use crate::{AphelionError, Config};

/// Connect to the local SQLite package database.
pub fn connect(config: &Config) -> Result<Connection, AphelionError> {
    Ok(Connection::open(&config.db_path)?)
}

/// Sync local package database with fetched repository data.
pub fn sync(_conn: &Connection, _repo_data: &str) -> Result<(), AphelionError> {
    Ok(())
}
