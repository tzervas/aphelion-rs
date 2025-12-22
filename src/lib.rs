// Common structs and traits to be used across modules

/// Common Configuration struct
pub struct Config {
    // Fields here
}

/// Common Error enum for robust error handling
#[derive(Debug)]
pub enum AphelionError {
    NetworkError,
    DatabaseError,
    // ... other errors
}

// Re-export modules
pub mod config;
pub mod network;
pub mod database;
pub mod package;
pub mod debian;
pub mod utils;
