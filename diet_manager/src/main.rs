use std::path::PathBuf;
use crate::cli::CLI;
use crate::database::DatabaseManager;

mod cli;
mod database;
mod models;
mod food;
mod log;
mod user;
mod utils;

fn main() -> Result<(), String> {
    println!("=== Welcome to YADA (Yet Another Diet Assistant) ===");
    println!("Loading data...");

    // Define paths for data files
    let food_db_path = PathBuf::from("food_data.json");
    let logs_path = PathBuf::from("log.json");
    let profile_path = PathBuf::from("user_profile.json");

    // Initialize database manager
    let db_manager = match DatabaseManager::new(
        &food_db_path,
        &logs_path,
        &profile_path,
    ) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Err(format!("Database initialization error: {}", e));
        }
    };

    // Initialize CLI
    let mut cli = CLI::new(db_manager);

    // Run the application
    match cli.run() {
        Ok(_) => {
            println!("Thank you for using YADA!");
            Ok(())
        },
        Err(e) => {
            eprintln!("Application error: {}", e);
            Err(format!("Application error: {}", e))
        }
    }
}