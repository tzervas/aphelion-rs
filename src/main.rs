use aphelion_rs::{config, database, network};

fn main() {
    if let Err(err) = run() {
        eprintln!("aphelion-rs error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), aphelion_rs::AphelionError> {
    let config = config::init()?;
    let db_conn = database::connect(&config)?;
    // Scaffold: block_on not wired; sync empty index for now.
    let repo_data = async_std::task::block_on(network::fetch_repo_data(&config))?;
    database::sync(&db_conn, &repo_data)?;
    Ok(())
}
