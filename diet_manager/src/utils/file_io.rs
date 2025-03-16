use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

use crate::models::{DailyLog, UserProfile};

// Add these basic functions that are being imported elsewhere
pub fn read_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(contents)
}

pub fn write_file(path: &Path, contents: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("Failed to open file for writing: {}", e))?;

    file.write_all(contents.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let contents = read_file(path)?;
    serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))
}

pub fn write_json<T: Serialize>(path: &Path, data: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
    write_file(path, &json)
}

// Remove this function or modify it to work without FoodDatabase
/*
pub fn load_food_database(path: &Path) -> Result<FoodDatabase, String> {
    // Implementation
}
*/

pub fn load_daily_logs(path: &Path) -> Result<HashMap<String, DailyLog>, String> {
    if path.exists() {
        read_json(path)
    } else {
        // Create empty logs if they don't exist
        let logs: HashMap<String, DailyLog> = HashMap::new();
        write_json(path, &logs)?;
        Ok(logs)
    }
}

pub fn save_daily_logs(path: &Path, logs: &HashMap<String, DailyLog>) -> Result<(), String> {
    write_json(path, logs)
}

pub fn load_user_profile(path: &Path) -> Result<UserProfile, String> {
    if path.exists() {
        read_json(path)
    } else {
        Err("User profile not found".to_string())
    }
}

pub fn save_user_profile(path: &Path, profile: &UserProfile) -> Result<(), String> {
    write_json(path, profile)
}