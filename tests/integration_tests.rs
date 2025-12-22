// Integration tests for Aphelion-rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_package() {
        // Test logic for installing a package
        assert_eq!(package::install("test_package").is_ok(), true);
    }

    #[test]
    fn test_remove_package() {
        // Test logic for removing a package
        assert_eq!(package::remove("test_package").is_ok(), true);
    }

    // ... other tests
}
