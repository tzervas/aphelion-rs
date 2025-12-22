// Integration tests for Aphelion-rs

#[cfg(test)]
mod tests {
    use aphelion_rs::package;

    #[test]
    fn test_install_package() {
        // Test logic for installing a package
        assert!(package::install("test_package").is_ok());
    }

    #[test]
    fn test_remove_package() {
        // Test logic for removing a package
        assert!(package::remove("test_package").is_ok());
    }

    // ... other tests
}
