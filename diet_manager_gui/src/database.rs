use std::fs;
use serde_json;
use crate::models::Database;

const DB_FILE: &str = "database.json";

pub fn load_database() -> Database {
    if let Ok(data) = fs::read_to_string(DB_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Database::default())
    } else {
        Database::default()
    }
}

pub fn save_database(db: &Database) -> std::io::Result<()> {
    // Try to load the existing database to preserve foods if file exists
    let mut final_db = if let Ok(data) = fs::read_to_string(DB_FILE) {
        if let Ok(existing_db) = serde_json::from_str::<Database>(&data) {
            // If we have foods in the db parameter, use those
            // Otherwise, keep the foods from the existing database
            let basic_foods = if !db.basic_foods.is_empty() {
                db.basic_foods.clone()
            } else {
                existing_db.basic_foods
            };

            let composite_foods = if !db.composite_foods.is_empty() {
                db.composite_foods.clone()
            } else {
                existing_db.composite_foods
            };

            // Always update users and current_user
            Database {
                users: db.users.clone(),
                basic_foods,
                composite_foods,
                food_logs: db.food_logs.clone(),
                current_user: db.current_user.clone(),
            }
        } else {
            db.clone()
        }
    } else {
        db.clone()
    };

    // If we still have empty food maps, use the default hardcoded values
    if final_db.basic_foods.is_empty() {
        final_db.basic_foods = Database::default().basic_foods;
    }

    if final_db.composite_foods.is_empty() {
        final_db.composite_foods = Database::default().composite_foods;
    }

    let data = serde_json::to_string_pretty(&final_db).unwrap();
    fs::write(DB_FILE, data)
}