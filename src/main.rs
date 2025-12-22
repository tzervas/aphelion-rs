use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aphelion")]
#[command(about = "A modern Linux package manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        /// Name of the package to install
        package: String,
    },
    /// Remove a package
    Remove {
        /// Name of the package to remove
        package: String,
    },
    /// Update package database
    Update,
    /// Upgrade installed packages
    Upgrade,
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// List installed packages
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Install { package }) => {
            println!("Installing package: {}", package);
            // TODO: Implement package installation
        }
        Some(Commands::Remove { package }) => {
            println!("Removing package: {}", package);
            // TODO: Implement package removal
        }
        Some(Commands::Update) => {
            println!("Updating package database...");
            // TODO: Implement database update
        }
        Some(Commands::Upgrade) => {
            println!("Upgrading installed packages...");
            // TODO: Implement package upgrade
        }
        Some(Commands::Search { query }) => {
            println!("Searching for: {}", query);
            // TODO: Implement package search
        }
        Some(Commands::List) => {
            println!("Listing installed packages...");
            // TODO: Implement package listing
        }
        None => {
            println!("Aphelion - Modern Linux Package Manager");
            println!("Use --help for more information");
        }
    }

    Ok(())
}
