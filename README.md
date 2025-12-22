# aphelion-rs

Aphelion-rs aims to revolutionize Linux package management by offering a modern, optimized, and user-friendly solution written in Rust.

## Features

- 🚀 Fast and efficient package management
- 🔒 Built with Rust for memory safety and performance
- 🎨 Modern CLI interface with intuitive commands
- 📦 Core package operations: install, remove, update, upgrade, search, and list

## Installation

### From Source

```bash
git clone https://github.com/tzervas/aphelion-rs.git
cd aphelion-rs
cargo build --release
```

The binary will be available at `target/release/aphelion`.

## Usage

```bash
# Display help
aphelion --help

# Install a package
aphelion install <package-name>

# Remove a package
aphelion remove <package-name>

# Update package database
aphelion update

# Upgrade installed packages
aphelion upgrade

# Search for packages
aphelion search <query>

# List installed packages
aphelion list
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the Development Version

```bash
cargo run -- [COMMAND]
```

## Project Status

This project is in early development. Core functionality is being implemented.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Package database management
- [ ] Package installation and removal
- [ ] Dependency resolution
- [ ] Package repository support
- [ ] Configuration management
- [ ] Progress indicators and better output
- [ ] Package verification and signatures
- [ ] Parallel downloads
- [ ] Transaction management
