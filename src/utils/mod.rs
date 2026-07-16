//! Shared helpers.

/// Validate a package name (minimal non-empty check).
pub fn validate_package_name(name: &str) -> bool {
    !name.trim().is_empty()
}

/// Split repository payload into lines (placeholder parser).
pub fn parse_repo_data(data: &str) -> Vec<&str> {
    data.lines().collect()
}
