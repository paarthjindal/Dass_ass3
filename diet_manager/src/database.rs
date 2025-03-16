use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::models::{BasicFood, CompositeFood, DailyLog, UserProfile, FoodTrait};
use crate::utils::file_io::{read_json, write_json}; // Updated imports

pub struct DatabaseManager {
    food_db_path: PathBuf,
    logs_path: PathBuf,
    profile_path: PathBuf,
    pub food_db: Vec<Box<dyn FoodTrait>>,
    pub logs: HashMap<String, DailyLog>,
    pub profile: Option<UserProfile>,
}

impl DatabaseManager {
    pub fn new(
        food_db_path: &Path,
        logs_path: &Path,
        profile_path: &Path,
    ) -> Result<Self, String> {
        // Initialize with empty collections
        let mut manager = DatabaseManager {
            food_db_path: food_db_path.to_path_buf(),
            logs_path: logs_path.to_path_buf(),
            profile_path: profile_path.to_path_buf(),
            food_db: Vec::new(),
            logs: HashMap::new(),
            profile: None,
        };

        // Load data from files if they exist
        if food_db_path.exists() {
            // For simplicity, just initialize with some sample data
            manager.food_db.push(Box::new(BasicFood::new(
                "Apple".to_string(),
                vec!["fruit".to_string(), "sweet".to_string()],
                95.0,
            )));

            manager.food_db.push(Box::new(BasicFood::new(
                "Banana".to_string(),
                vec!["fruit".to_string(), "sweet".to_string()],
                105.0,
            )));

            // Add more sample foods
            manager.food_db.push(Box::new(BasicFood::new(
                "Chicken Breast".to_string(),
                vec!["meat".to_string(), "protein".to_string()],
                165.0,
            )));

            manager.food_db.push(Box::new(BasicFood::new(
                "Brown Rice".to_string(),
                vec!["grain".to_string(), "carbs".to_string()],
                215.0,
            )));

            // Load logs if they exist
            if logs_path.exists() {
                match read_json::<HashMap<String, DailyLog>>(logs_path) {
                    Ok(loaded_logs) => manager.logs = loaded_logs,
                    Err(e) => eprintln!("Warning: Failed to load logs: {}", e),
                }
            }

            // Load profile if it exists
            if profile_path.exists() {
                match read_json::<UserProfile>(profile_path) {
                    Ok(profile) => manager.profile = Some(profile),
                    Err(e) => eprintln!("Warning: Failed to load profile: {}", e),
                }
            }
        }

        Ok(manager)
    }

    pub fn search_foods_by_name(&self, name: &str) -> Vec<&Box<dyn FoodTrait>> {
        let name_lower = name.to_lowercase();
        self.food_db
            .iter()
            .filter(|food| food.name().to_lowercase().contains(&name_lower))
            .collect()
    }

    pub fn get_food_by_name(&self, name: &str) -> Option<&Box<dyn FoodTrait>> {
        self.food_db.iter().find(|food| food.name() == name)
    }

    pub fn get_composite_food(&self, name: &str) -> Option<&CompositeFood> {
        self.food_db.iter()
            .filter_map(|food| {
                if food.name() == name {
                    food.as_any().downcast_ref::<CompositeFood>()
                } else {
                    None
                }
            })
            .next()
    }

    pub fn add_basic_food(&mut self, food: BasicFood) {
        self.food_db.push(Box::new(food));
    }

    pub fn add_composite_food(&mut self, food: CompositeFood) {
        self.food_db.push(Box::new(food));
    }

    pub fn save_to_files(&self) -> Result<(), String> {
        // Save logs
        write_json(&self.logs_path, &self.logs)?;

        // Save profile if it exists
        if let Some(profile) = &self.profile {
            write_json(&self.profile_path, profile)?;
        }

        // We don't save food_db since it's complex with trait objects
        // In a real app, you'd serialize the specific implementations

        Ok(())
    }
}