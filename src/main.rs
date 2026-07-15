use aphelion_rs::{config, network, database};

#[tokio::main]
async fn main() {
    // Initialize config
    let config = config::init();
    
    // Initialize database
    let db_conn = database::connect(&config).expect("Failed to connect to database");
    
    // Fetch repository data
    let repo_data = network::fetch_repo_data(&config).await.expect("Failed to fetch repo data");
    
    // Sync local package database
    database::sync(&db_conn, &repo_data).expect("Failed to sync database");
    
    // CLI interaction logic here
    println!("Aphelion-rs initialized successfully!");
}
